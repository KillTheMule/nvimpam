//! The handler for the rpc events sent by `neovim_lib`. Note that this is
//! excuted in another thread, so we use a
//! [`Sender<Event>`](std::sync::mpsc::Sender) to send the parsed event data to
//! the main thread.
use std::sync::mpsc;

use failure::{self, Error};
use neovim_lib::{neovim_api::Buffer, Handler, RequestHandler, Value};

use crate::event::Event;

/// The handler containing the sending end of a channel. The receiving end is
/// the main [`event loop`](::event::Event::event_loop).
pub struct NeovimHandler {
  pub to_main: mpsc::Sender<Event>,
  pub from_main: mpsc::Receiver<Value>,
}

impl NeovimHandler {
  /// Parse a nvim_buf_lines_event notification into a
  /// [`LinesEvent`](::event::Event::LinesEvent) event
  pub fn parse_lines_event(
    &mut self,
    mut args: Vec<Value>,
  ) -> Result<Event, Error> {
    let nea = "Not enough arguments in nvim_buf_lines_event!";

    let more = parse_bool(&last_arg(&mut args, nea)?)?;
    let linedata = parse_vecstr(last_arg(&mut args, nea)?)?;
    let lastline = parse_i64(&last_arg(&mut args, nea)?)?;
    let firstline = parse_i64(&last_arg(&mut args, nea)?)?;
    let changedtick = parse_u64(&last_arg(&mut args, nea)?)?;
    let buf = parse_buf(last_arg(&mut args, nea)?);

    Ok(Event::LinesEvent {
      buf,
      changedtick,
      firstline,
      lastline,
      linedata,
      more,
    })
  }

  /// Parse a nvim_buf_changedtick_event notification into a
  /// [`ChangedTickEvent`](::event::Event::ChangedTickEvent)
  /// event
  pub fn parse_changedtick_event(
    &mut self,
    mut args: Vec<Value>,
  ) -> Result<Event, Error> {
    let nea = "Not enough arguments in nvim_buf_changedtick_event!";

    let changedtick = parse_u64(&last_arg(&mut args, nea)?)?;
    let buf = parse_buf(last_arg(&mut args, nea)?);
    Ok(Event::ChangedTickEvent { buf, changedtick })
  }

  /// Parse a HighlightRegion notification into a
  /// [`HighlightRegion`](::event::Event::HighlightRegion) event
  pub fn parse_highlight_region(
    &mut self,
    mut args: Vec<Value>,
  ) -> Result<Event, Error> {
    let nea = "Not enough arguments in HighlightRegion notification!";

    let lastline = parse_u64(&last_arg(&mut args, nea)?)?;
    let firstline = parse_u64(&last_arg(&mut args, nea)?)?;
    Ok(Event::HighlightRegion {
      firstline,
      lastline,
    })
  }

  /// Parse a nvim_buf_detach_event notification into a
  /// [`DetachEvent`](::event::Event::DetachEvent) event
  pub fn parse_detach_event(
    &mut self,
    mut args: Vec<Value>,
  ) -> Result<Event, Error> {
    let nea = "Not enough arguments in nvim_buf_detach_event!";
    let buf = parse_buf(last_arg(&mut args, nea)?);
    Ok(Event::DetachEvent { buf })
  }
}

impl Handler for NeovimHandler {
  fn handle_notify(&mut self, name: &str, args: Vec<Value>) {
    match name {
      "nvim_buf_lines_event" => {
        if let Ok(event) = self.parse_lines_event(args) {
          info!("{:?}", event);
          if let Err(reason) = self.to_main.send(event) {
            error!("{}", reason);
          }
        }
      }
      "nvim_buf_changedtick_event" => {
        if let Ok(event) = self.parse_changedtick_event(args) {
          info!("{:?}", event);
          if let Err(reason) = self.to_main.send(event) {
            error!("{}", reason);
          }
        }
      }
      "nvim_buf_detach_event" => {
        if let Ok(event) = self.parse_detach_event(args) {
          info!("{:?}", event);
          if let Err(reason) = self.to_main.send(event) {
            error!("{}", reason);
          }
        }
      }
      "HighlightRegion" => {
        if let Ok(event) = self.parse_highlight_region(args) {
          info!("{:?}", event);
          if let Err(reason) = self.to_main.send(event) {
            error!("{}", reason);
          }
        }
      }
      "quit" => {
        if let Err(reason) = self.to_main.send(Event::Quit) {
          error!("{}", reason);
        }
      }
      unknown => {
        error!("Received unknown notification: '{}'!", unknown);
      }
    }
  }
}

impl RequestHandler for NeovimHandler {
  fn handle_request(
    &mut self,
    name: String,
    _args: Vec<Value>,
  ) -> Result<Value, Value> {
    match name.as_str() {
      "RefreshFolds" => {
        if let Err(reason) = self.to_main.send(Event::RefreshFolds) {
          Err(Value::from(format!(
            "Error sending request '{}': {}!",
            name, reason
          )))
        } else {
          self.from_main.recv().map_err(|e| {
            Value::from(format!(
              "Error receiving value for request '{}' from main thread: {}!",
              name, e
            ))
          })
        }
      }
      _ => Err(Value::from(format!("Received unknown Request: '{}'!", name))),
    }
  }
}

/// Helper function to get the last argument of a `Vec<Value>` or return an
/// error message
pub fn last_arg(
  v: &mut Vec<Value>,
  errmsg: &'static str,
) -> Result<Value, Error> {
  v.pop().ok_or_else(|| failure::err_msg(errmsg))
}

/// Parse a [`neovim_lib::Value`](neovim_lib::Value) into a u64
pub fn parse_u64(value: &Value) -> Result<u64, Error> {
  value
    .as_u64()
    .ok_or_else(|| failure::err_msg(format!("Cannot parse '{:?}' as u64", value)))
}
///
/// Parse a [`neovim_lib::Value`](neovim_lib::Value) into a i64
pub fn parse_i64(value: &Value) -> Result<i64, Error> {
  value
    .as_i64()
    .ok_or_else(|| failure::err_msg(format!("Cannot parse '{:?}' as i64", value)))
}

/// Parse a [`neovim_lib::Value`](neovim_lib::Value) into a bool
pub fn parse_bool(value: &Value) -> Result<bool, Error> {
  value
    .as_bool()
    .ok_or_else(|| failure::err_msg(format!("Cannot parse '{:?}' as bool", value)))
}

/// Parse a [`neovim_lib::Value`](neovim_lib::Value) into a `Vec<String>`. Note
/// that this method takes ownership of the value so it does not need to copy
/// out the contained strings
pub fn parse_vecstr(value: Value) -> Result<Vec<String>, Error> {
  let mut res: Vec<String>;
  if let Value::Array(v) = value {
    res = Vec::with_capacity(v.len());

    for val in v {
      if let Value::String(s) = val {
        match s.into_str() {
          Some(string) => res.push(string),
          None => return Err(failure::err_msg("Non-utf8 values in array")),
        }
      } else {
        return Err(failure::err_msg("Non-String value in array"));
      }
    }
  } else {
    return Err(failure::err_msg(format!("Cannot parse '{:?}' as array", value)));
  }

  Ok(res)
}

/// Parse a [`neovim_lib::Value`](neovim_lib::Value) into a
/// [`neovim_lib::Buffer`](neovim_lib::neovim_api::Buffer). This cannot fail,
/// but if the Value was not obtained from the rpc api, this will probably not
/// be a valid buffer to send commands to.
pub fn parse_buf(value: Value) -> Buffer {
  Buffer::new(value)
}
