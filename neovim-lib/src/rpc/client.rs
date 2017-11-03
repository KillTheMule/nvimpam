use std::io::{Read, Write};
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::sync::{mpsc, Mutex, Arc};

use super::handler::{DefaultHandler, Handler};
use rmpv::Value;

use super::model;

type Queue = Arc<Mutex<HashMap<u64, mpsc::Sender<Result<Value, Value>>>>>;

pub struct Client<R, W>
    where R: Read + Send + 'static,
          W: Write + Send + 'static
{
    reader: Option<R>,
    writer: Arc<Mutex<W>>,
    dispatch_guard: Option<JoinHandle<()>>,
    event_loop_started: bool,
    queue: Queue,
    msgid_counter: u64,
}

impl<R, W> Client<R, W>
    where R: Read + Send + 'static,
          W: Write + Send + 'static
{
    pub fn take_dispatch_guard(&mut self) -> JoinHandle<()> {
        self.dispatch_guard.take().expect("Can only take join handle after running event loop")
    }

    pub fn start_event_loop_handler<H>(&mut self, handler: H)
        where H: Handler + Send + 'static
    {
        self.dispatch_guard = Some(Self::dispatch_thread(self.queue.clone(),
                                                         self.reader.take().unwrap(),
                                                         self.writer.clone(),
                                                         handler));
        self.event_loop_started = true;
    }

    pub fn start_event_loop(&mut self) {
        self.dispatch_guard = Some(Self::dispatch_thread(self.queue.clone(),
                                                         self.reader.take().unwrap(),
                                                         self.writer.clone(),
                                                         DefaultHandler()));
        self.event_loop_started = true;
    }

    pub fn new(reader: R, writer: W) -> Self {
        let queue = Arc::new(Mutex::new(HashMap::new()));
        Client {
            reader: Some(reader),
            writer: Arc::new(Mutex::new(writer)),
            msgid_counter: 0,
            queue: queue.clone(),
            dispatch_guard: None,
            event_loop_started: false,
        }
    }

    pub fn call_timeout(&mut self,
                        method: &str,
                        args: &Vec<Value>,
                        dur: Duration)
                        -> Result<Value, Value> {
        if !self.event_loop_started {
            return Err(Value::from("Event loop not started"));
        }

        let instant = Instant::now();
        let delay = Duration::from_millis(1);

        let receiver = self.send_msg(method, args);

        loop {
            match receiver.try_recv() {
                Err(mpsc::TryRecvError::Empty) => {
                    thread::sleep(delay);
                    if instant.elapsed() >= dur {
                        return Err(Value::from("Wait timeout"));
                    }
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    return Err(Value::from("Channel disconnected"))
                }
                Ok(val) => return val,
            };
        }
    }

    fn send_msg(&mut self,
                method: &str,
                args: &Vec<Value>)
                -> mpsc::Receiver<Result<Value, Value>> {
        let msgid = self.msgid_counter;
        self.msgid_counter += 1;

        let req = model::RpcMessage::RpcRequest {
            msgid: msgid,
            method: method.to_owned(),
            params: args.clone(),
        };

        let (sender, receiver) = mpsc::channel();
        self.queue.lock().unwrap().insert(msgid, sender);

        let ref mut writer = *self.writer.lock().unwrap();
        model::encode(writer, &req).expect("Error sending message");

        receiver
    }

    pub fn call(&mut self,
                method: &str,
                args: &Vec<Value>,
                dur: Option<Duration>)
                -> Result<Value, Value> {
        match dur {
            Some(dur) => self.call_timeout(method, args, dur),
            None => self.call_inf(method, args),
        }
    }

    pub fn call_inf(&mut self, method: &str, args: &Vec<Value>) -> Result<Value, Value> {
        if !self.event_loop_started {
            return Err(Value::from("Event loop not started"));
        }

        let receiver = self.send_msg(method, args);

        receiver.recv().unwrap()
    }

    fn dispatch_thread<H>(queue: Queue,
                          mut reader: R,
                          writer: Arc<Mutex<W>>,
                          mut handler: H)
                          -> JoinHandle<()>
        where H: Handler + Send + 'static
    {
        thread::spawn(move || loop {
            let msg = match model::decode(&mut reader) {
                Ok(msg) => msg,
                Err(e) => {
                    debug!("Error decoding: {}", e);
                    return;
                }
            };
            //debug!("Get message {:?}", msg);
            match msg {
                model::RpcMessage::RpcRequest { msgid, method, params } => {
                    let response = match handler.handle_request(&method, &params) {
                        Ok(result) => {
                            model::RpcMessage::RpcResponse {
                                msgid: msgid,
                                result: result,
                                error: Value::Nil,
                            }
                        }
                        Err(error) => {
                            model::RpcMessage::RpcResponse {
                                msgid: msgid,
                                result: Value::Nil,
                                error: error,
                            }
                        }
                    };

                    let ref mut writer = *writer.lock().unwrap();
                    model::encode(writer, &response).expect("Error sending RPC response");
                }
                model::RpcMessage::RpcResponse { msgid, result, error } => {
                    let sender = queue.lock().unwrap().remove(&msgid).unwrap();
                    if error != Value::Nil {
                        sender.send(Err(error)).unwrap();
                    }
                    sender.send(Ok(result)).unwrap();
                }
                model::RpcMessage::RpcNotification { method, params } => {
                    handler.handle_notify(method, params);
                }
            };
        })
    }
}
