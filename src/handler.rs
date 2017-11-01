use args;
use event::Event;
use neovim_lib::{Handler, Value};
use std::sync::mpsc;

pub struct NeovimHandler(pub mpsc::Sender<Event>);

impl NeovimHandler {
    pub fn parse_liveupdatestart(&mut self, mut args: Vec<Value>) -> Result<Event, String> {
        let buf = args::parse_buf(&args[0]);
        let changedtick = args::parse_u64(&args[1])?;
        let more = args::parse_bool(&args[3])?;
        let linedata = args::parse_vecstr(args.remove(2))?;
        Ok(Event::LiveUpdateStart {
            buf,
            changedtick,
            linedata,
            more,
        })
    }

    pub fn parse_liveupdate(&mut self, mut args: Vec<Value>) -> Result<Event, String> {
        let buf = args::parse_buf(&args[0]);
        let changedtick = args::parse_u64(&args[1])?;
        let firstline = args::parse_u64(&args[2])?;
        let numreplaced = args::parse_u64(&args[3])?;
        let linedata = args::parse_vecstr(args.remove(4))?;
        Ok(Event::LiveUpdate {
            buf,
            changedtick,
            firstline,
            numreplaced,
            linedata,
        })
    }

    pub fn parse_liveupdatetick(&mut self, args: Vec<Value>) -> Result<Event, String> {
        let buf = args::parse_buf(&args[0]);
        let changedtick = args::parse_u64(&args[1])?;
        Ok(Event::LiveUpdateTick { buf, changedtick })
    }

    pub fn parse_liveupdateend(&mut self, args: Vec<Value>) -> Result<Event, String> {
        let buf = args::parse_buf(&args[0]);
        Ok(Event::LiveUpdateEnd { buf })
    }
}

impl Handler for NeovimHandler {
    fn handle_notify(&mut self, name: String, args: Vec<Value>) {
        //info!("event: {}", name);
        //print_args(&args);
        match name.as_ref() {
            "LiveUpdateStart" => {
                if let Ok(event) = self.parse_liveupdatestart(args) {
                    info!("{:?}", event);
                    if let Err(reason) = self.0.send(event) {
                        error!("{}", reason);
                    }
                }
            }
            "LiveUpdate" => {
                if let Ok(event) = self.parse_liveupdate(args) {
                    info!("{:?}", event);
                    if let Err(reason) = self.0.send(event) {
                        error!("{}", reason);
                    }
                }
            }
            "LiveUpdateTick" => {
                if let Ok(event) = self.parse_liveupdatetick(args) {
                    info!("{:?}", event);
                    if let Err(reason) = self.0.send(event) {
                        error!("{}", reason);
                    }
                }
            }
            "LiveUpdateEnd" => {
                if let Ok(event) = self.parse_liveupdateend(args) {
                    info!("{:?}", event);
                    if let Err(reason) = self.0.send(event) {
                        error!("{}", reason);
                    }
                }
            }
            "quit" => {
                if let Err(reason) = self.0.send(Event::Quit) {
                    error!("{}", reason);
                }
            }
            _ => {}
        }
    }

    fn handle_request(&mut self, _name: &str, _args: &Vec<Value>) -> Result<Value, Value> {
        Err(Value::from("not implemented"))
    }
}
