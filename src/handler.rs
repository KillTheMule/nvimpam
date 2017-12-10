//! The handler for the rpc events sent by neovim_lib
use event::Event;
use neovim_lib::{Handler, Value};
use neovim_lib::neovim_api::Buffer;
use std::sync::mpsc;

/// The handler containing the sending end of a channel. The receiving end is
/// the main [event loop](../event/enum.Event.html#method.event_loop).
pub struct NeovimHandler(pub mpsc::Sender<Event>);

impl NeovimHandler {
  /// Parse a LiveUpdateStart notification into a
  /// [LiveUpdateStart](../event/enum.Event.html) event
  pub fn parse_liveupdatestart(
    &mut self,
    mut args: Vec<Value>,
  ) -> Result<Event, String> {
    let buf = parse_buf(&args[0]);
    let changedtick = parse_u64(&args[1])?;
    let more = parse_bool(&args[3])?;
    let linedata = parse_vecstr(args.remove(2))?;
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
  ) -> Result<Event, String> {
    let buf = parse_buf(&args[0]);
    let changedtick = parse_u64(&args[1])?;
    let firstline = parse_u64(&args[2])?;
    let numreplaced = parse_u64(&args[3])?;
    let linedata = parse_vecstr(args.remove(4))?;
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
    args: Vec<Value>,
  ) -> Result<Event, String> {
    let buf = parse_buf(&args[0]);
    let changedtick = parse_u64(&args[1])?;
    Ok(Event::LiveUpdateTick { buf, changedtick })
  }

  /// Parse a LiveUpdateEnd notification into a
  /// [LiveUpdateEnd](../event/enum.Event.html) event
  pub fn parse_liveupdateend(
    &mut self,
    args: Vec<Value>,
  ) -> Result<Event, String> {
    let buf = parse_buf(&args[0]);
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

/// Parse a neovim_lib::Value into a u64
pub fn parse_u64(value: &Value) -> Result<u64, String> {
  value.as_u64().ok_or_else(
    || "cannot parse usize".to_owned(),
  )
}

/// Parse a neovim_lib::Value into a bool
pub fn parse_bool(value: &Value) -> Result<bool, String> {
  value.as_bool().ok_or_else(
    || "cannot parse bool".to_owned(),
  )
}

/// Pare a neovim_lib::Value into a Vec<String>. Note that this method takes
/// ownership of the value so it does not need to copy out the contained strings
pub fn parse_vecstr(value: Value) -> Result<Vec<String>, String> {
  if let Value::Array(v) = value {
    v.into_iter()
      .map(move |e| match e {
        Value::String(s) => {
          s.into_str().ok_or_else(
            || "non-utf8 values in array".to_owned(),
          )
        }
        _ => return Err("nonstring value in array".to_owned()),
      })
      .collect()
  } else {
    Err("cannot parse array".to_owned())
  }
}

/// Parse a neovim_lib::Value into a neovim_lib::Buffer. This cannot fail, but
/// if the Value was not obtained from the rpc api, this will probably not be a
/// valid buffer to send commands to.
pub fn parse_buf(value: &Value) -> Buffer {
  Buffer::new(value.clone())
}
