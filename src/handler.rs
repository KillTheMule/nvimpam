//! The handler for the rpc events sent by `neovim_lib`
use std::sync::mpsc;

use failure;
use failure::Error;
use neovim_lib::neovim_api::Buffer;
use neovim_lib::{Handler, Value};

use event::Event;

/// The handler containing the sending end of a channel. The receiving end is
/// the main [event loop](../event/enum.Event.html#method.event_loop).
pub struct NeovimHandler(pub mpsc::Sender<Event>);

impl NeovimHandler {
  /// Parse a nvim_buf_updates_start notification into a
  /// [UpdatesStart](../event/enum.Event.html#variant.UpdatesStart) event
  pub fn parse_updates_start(
    &mut self,
    mut args: Vec<Value>,
  ) -> Result<Event, Error> {
    let more = parse_bool(&last_arg(
      &mut args,
      "Not enough arguments in nvim_buf_updates_start!",
    )?)?;
    let linedata = parse_vecstr(last_arg(
      &mut args,
      "Not enough arguments in nvim_buf_updates_start!",
    )?)?;
    let changedtick = parse_u64(&last_arg(
      &mut args,
      "Not enough arguments in nvim_buf_updates_start!",
    )?)?;
    let buf = parse_buf(last_arg(
      &mut args,
      "Not enough arguments in nvim_buf_updates_start!",
    )?);

    Ok(Event::UpdatesStart {
      buf,
      changedtick,
      linedata,
      more,
    })
  }

  /// Parse a nvim_buf_update notification into a
  /// [Update](../event/enum.Event.html#variant.Update) event
  pub fn parse_update(&mut self, mut args: Vec<Value>) -> Result<Event, Error> {
    let linedata = parse_vecstr(last_arg(
      &mut args,
      "Not enough arguments in nvim_buf_update!",
    )?)?;
    let numreplaced = parse_u64(&last_arg(
      &mut args,
      "Not enough arguments in nvim_buf_update!",
    )?)?;
    let firstline = parse_u64(&last_arg(
      &mut args,
      "Not enough arguments in nvim_buf_update!",
    )?)?;
    let changedtick = parse_u64(&last_arg(
      &mut args,
      "Not enough arguments in nvim_buf_update!",
    )?)?;
    let buf = parse_buf(last_arg(
      &mut args,
      "Not enough arguments in nvim_buf_update!",
    )?);

    Ok(Event::Update {
      buf,
      changedtick,
      firstline,
      numreplaced,
      linedata,
    })
  }

  /// Parse a nvim_buf_changedtick notification into a
  /// [ChangedTick](../event/enum.Event.html#variant.ChangedTick) event
  pub fn parse_changedtick(
    &mut self,
    mut args: Vec<Value>,
  ) -> Result<Event, Error> {
    let changedtick = parse_u64(&last_arg(
      &mut args,
      "Not enough arguments in nvim_buf_changedtick!",
    )?)?;
    let buf = parse_buf(last_arg(
      &mut args,
      "Not enough arguments in nvim_buf_changedtick!",
    )?);
    Ok(Event::ChangedTick { buf, changedtick })
  }

  /// Parse a nvim_buf_updates_end notification into a
  /// [UpdatesEnd](../event/enum.Event.html#variant.UpdatesEnd) event
  pub fn parse_updates_end(
    &mut self,
    mut args: Vec<Value>,
  ) -> Result<Event, Error> {
    let buf = parse_buf(last_arg(
      &mut args,
      "Not enough arguments in nvim_buf_updates_end!",
    )?);
    Ok(Event::UpdatesEnd { buf })
  }
}

impl Handler for NeovimHandler {
  fn handle_notify(&mut self, name: &str, args: Vec<Value>) {
    match name {
      "nvim_buf_updates_start" => {
        if let Ok(event) = self.parse_updates_start(args) {
          info!("{:?}", event);
          if let Err(reason) = self.0.send(event) {
            error!("{}", reason);
          }
        }
      }
      "nvim_buf_update" => {
        if let Ok(event) = self.parse_update(args) {
          info!("{:?}", event);
          if let Err(reason) = self.0.send(event) {
            error!("{}", reason);
          }
        }
      }
      "nvim_buf_changedtick" => {
        if let Ok(event) = self.parse_changedtick(args) {
          info!("{:?}", event);
          if let Err(reason) = self.0.send(event) {
            error!("{}", reason);
          }
        }
      }
      "nvim_buf_updates_end" => {
        if let Ok(event) = self.parse_updates_end(args) {
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
      unknown => {
        error!("Receveid unknown event: {}!", unknown);
      }
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
