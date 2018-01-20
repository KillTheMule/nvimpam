//! Extending the rpc api provided by `neovim_lib` with the capabilities
//! provided by <https://github.com/neovim/neovim/pull/5269>
use neovim_lib::neovim_api::Buffer;
use neovim_lib::neovim::CallError;
use neovim_lib::neovim::map_result;
use neovim_lib::neovim::map_generic_error;
use neovim_lib::Neovim;
use neovim_lib::Value;

/// Trait to extend the Buffer API of neovim-lib
pub trait BufferExt {
  /// Subscribe to live updates provided by neovim, see
  /// https://github.com/neovim/neovim/pull/5269
  fn live_updates(
    &self,
    neovim: &mut Neovim,
    enabled: bool,
  ) -> Result<(), CallError>;
}

impl BufferExt for Buffer {
  /// since: xxxx
  fn live_updates(
    &self,
    neovim: &mut Neovim,
    enabled: bool,
  ) -> Result<(), CallError> {
    let mut v = Vec::new();
    v.push(self.get_value().clone());
    v.push(Value::from(enabled));
    neovim
      .session
      .call("nvim_buf_live_updates", v)
      .map(map_result)
      .map_err(map_generic_error)
  }
}
