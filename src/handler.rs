//! The handler for the rpc events sent by `neovim_lib`
use std::sync::mpsc;

use neovim_lib::{Handler, Value};
use neovim_lib::neovim_api::Buffer;
use failure;
use failure::Error;

use event::Event;

/// The handler containing the sending end of a channel. The receiving end is
/// the main [event loop](../event/enum.Event.html#method.event_loop).
pub struct NeovimHandler(pub mpsc::Sender<Event>);

impl NeovimHandler {
  /// Parse a LiveUpdateStart notification into a
  /// [LiveUpdateStart](../event/enum.Event.html) event
  pub fn parse_liveupdatestart(
    &mut self,
    mut args: Vec<Value>,
  ) -> Result<Event, Error> {
    let more = parse_bool(&last_arg(
      &mut args,
      "Not enough arguments in LiveUpdateStart!",
    )?)?;
    let linedata = parse_vecstr(last_arg(
      &mut args,
      "Not enough arguments in LiveUpdateStart!",
    )?)?;
    let changedtick = parse_u64(&last_arg(
      &mut args,
      "Not enough arguments in LiveUpdateStart!",
    )?)?;
    let buf = parse_buf(last_arg(
      &mut args,
      "Not enough arguments in LiveUpdateStart!",
    )?);

    Ok(Event::LiveUpdateStart {
      buf,
      changedtick,
      linedata,
      more,
    })
  }

  /// Parse a LiveUpdate notification into a
  /// [LiveUpdate](../event/enum.Event.html) event
  pub fn parse_liveupdate(
    &mut self,
    mut args: Vec<Value>,
  ) -> Result<Event, Error> {
    let linedata = parse_vecstr(last_arg(
      &mut args,
      "Not enough arguments in LiveUpdate!",
    )?)?;
    let numreplaced =
      parse_u64(&last_arg(&mut args, "Not enough arguments in LiveUpdate!")?)?;
    let firstline =
      parse_u64(&last_arg(&mut args, "Not enough arguments in LiveUpdate!")?)?;
    let changedtick =
      parse_u64(&last_arg(&mut args, "Not enough arguments in LiveUpdate!")?)?;
    let buf =
      parse_buf(last_arg(&mut args, "Not enough arguments in LiveUpdate!")?);

    Ok(Event::LiveUpdate {
      buf,
      changedtick,
      firstline,
      numreplaced,
      linedata,
    })
  }

  /// Parse a LiveUpdateTick notification into a
  /// [LiveUpdateTick](../event/enum.Event.html) event
  pub fn parse_liveupdatetick(
    &mut self,
    mut args: Vec<Value>,
  ) -> Result<Event, Error> {
    let changedtick =
      parse_u64(&last_arg(&mut args, "Not enough arguments in LiveUpdate!")?)?;
    let buf =
      parse_buf(last_arg(&mut args, "Not enough arguments in LiveUpdate!")?);
    Ok(Event::LiveUpdateTick { buf, changedtick })
  }

  /// Parse a LiveUpdateEnd notification into a
  /// [LiveUpdateEnd](../event/enum.Event.html) event
  pub fn parse_liveupdateend(
    &mut self,
    mut args: Vec<Value>,
  ) -> Result<Event, Error> {
    let buf =
      parse_buf(last_arg(&mut args, "Not enough arguments in LiveUpdate!")?);
    Ok(Event::LiveUpdateEnd { buf })
  }
}

impl Handler for NeovimHandler {
  fn handle_notify(&mut self, name: &str, args: Vec<Value>) {
    match name {
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
      "RefreshFolds" => {
        info!("RefreshFolds");
        if let Err(reason) = self.0.send(Event::RefreshFolds) {
          error!("{}", reason);
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

  /// As of now, our handler cannot handle requests (only notifications). It
  /// doesn't need to.
  fn handle_request(
    &mut self,
    _name: &str,
    _args: Vec<Value>,
  ) -> Result<Value, Value> {
    Err(Value::from("not implemented"))
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

/// Parse a `neovim_lib::Value` into a u64
pub fn parse_u64(value: &Value) -> Result<u64, Error> {
  value
    .as_u64()
    .ok_or_else(|| failure::err_msg("cannot parse usize"))
}

/// Parse a `neovim_lib::Value` into a bool
pub fn parse_bool(value: &Value) -> Result<bool, Error> {
  value
    .as_bool()
    .ok_or_else(|| failure::err_msg("cannot parse bool"))
}

/// Pare a `neovim_lib::Value` into a Vec<String>. Note that this method takes
/// ownership of the value so it does not need to copy out the contained strings
pub fn parse_vecstr(value: Value) -> Result<Vec<String>, Error> {
  let mut res: Vec<String>;
  if let Value::Array(v) = value {
    res = Vec::with_capacity(v.len());

    for val in v {
      if let Value::String(s) = val {
        match s.into_str() {
          Some(string) => res.push(string),
          None => return Err(failure::err_msg("non-utf8 values in array")),
        }
      } else {
        return Err(failure::err_msg("non-String value in array"));
      }
    }
  } else {
    return Err(failure::err_msg("cannot parse array"));
  }

  Ok(res)
}

/// Parse a `neovim_lib::Value` into a `neovim_lib::Buffer`. This cannot fail,
/// but if the Value was not obtained from the rpc api, this will probably not
/// be a valid buffer to send commands to.
pub fn parse_buf(value: Value) -> Buffer {
  Buffer::new(value)
}
