//! Extending the rpc api provided by `neovim_lib` with the capabilities
//! provided by [Neovim#7917](https://github.com/neovim/neovim/pull/7917)
use neovim_lib::neovim::map_generic_error;
use neovim_lib::neovim::map_result;
use neovim_lib::neovim::CallError;
use neovim_lib::neovim_api::Buffer;
use neovim_lib::Neovim;
use neovim_lib::Value;

/// Trait to extend the Buffer API of neovim-lib
pub trait BufferExt {
  /// Subscribe to buffer events provided by neovim, see
  /// https://github.com/neovim/neovim/pull/7917
  fn event_sub(
    &self,
    neovim: &mut Neovim,
    send_buffer: bool,
  ) -> Result<(), CallError>;

  /// Unsubscribe from buffer events
  fn event_unsub(&self, neovim: &mut Neovim) -> Result<(), CallError>;
}

impl BufferExt for Buffer {
  /// since: xxxx
  fn event_sub(
    &self,
    neovim: &mut Neovim,
    send_buffer: bool,
  ) -> Result<(), CallError> {
    let mut v = Vec::new();
    v.push(self.get_value().clone());
    v.push(Value::from(send_buffer));
    neovim
      .session
      .call("nvim_buf_event_sub", v)
      .map(map_result)
      .map_err(map_generic_error)
  }

  /// since: xxxx
  fn event_unsub(&self, neovim: &mut Neovim) -> Result<(), CallError> {
    let mut v = Vec::new();
    v.push(self.get_value().clone());
    neovim
      .session
      .call("nvim_buf_event_unsub", v)
      .map(map_result)
      .map_err(map_generic_error)
  }
}
