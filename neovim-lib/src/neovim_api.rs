// Auto generated 2017-05-28 11:18:20.638588

use neovim::*;
use rpc::*;

#[derive(PartialEq, Clone, Debug)]
pub struct Buffer {
    code_data: Value,
}

impl Buffer {
    pub fn new(code_data: Value) -> Buffer {
        Buffer { code_data: code_data }
    }

    /// Internal value, that represent type
    pub fn get_value(&self) -> &Value {
        &self.code_data
    }

    /// since: 1
    pub fn line_count(&self, neovim: &mut Neovim) -> Result<u64, CallError> {
        neovim
            .session
            .call("nvim_buf_line_count", &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn get_lines(&self,
                     neovim: &mut Neovim,
                     start: u64,
                     end: u64,
                     strict_indexing: bool)
                     -> Result<Vec<String>, CallError> {
        neovim
            .session
            .call("nvim_buf_get_lines",
                  &call_args![self.code_data.clone(), start, end, strict_indexing])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn set_lines(&self,
                     neovim: &mut Neovim,
                     start: u64,
                     end: u64,
                     strict_indexing: bool,
                     replacement: Vec<String>)
                     -> Result<(), CallError> {
        neovim
            .session
            .call("nvim_buf_set_lines",
                  &call_args![self.code_data.clone(),
                              start,
                              end,
                              strict_indexing,
                              replacement])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn get_var(&self, neovim: &mut Neovim, name: &str) -> Result<Value, CallError> {
        neovim
            .session
            .call("nvim_buf_get_var",
                  &call_args![self.code_data.clone(), name])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 2
    pub fn get_changedtick(&self, neovim: &mut Neovim) -> Result<u64, CallError> {
        neovim
            .session
            .call("nvim_buf_get_changedtick",
                  &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn set_var(&self, neovim: &mut Neovim, name: &str, value: Value) -> Result<(), CallError> {
        neovim
            .session
            .call("nvim_buf_set_var",
                  &call_args![self.code_data.clone(), name, value])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn del_var(&self, neovim: &mut Neovim, name: &str) -> Result<(), CallError> {
        neovim
            .session
            .call("nvim_buf_del_var",
                  &call_args![self.code_data.clone(), name])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn get_option(&self, neovim: &mut Neovim, name: &str) -> Result<Value, CallError> {
        neovim
            .session
            .call("nvim_buf_get_option",
                  &call_args![self.code_data.clone(), name])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn set_option(&self,
                      neovim: &mut Neovim,
                      name: &str,
                      value: Value)
                      -> Result<(), CallError> {
        neovim
            .session
            .call("nvim_buf_set_option",
                  &call_args![self.code_data.clone(), name, value])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn get_number(&self, neovim: &mut Neovim) -> Result<u64, CallError> {
        neovim
            .session
            .call("nvim_buf_get_number", &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn get_name(&self, neovim: &mut Neovim) -> Result<String, CallError> {
        neovim
            .session
            .call("nvim_buf_get_name", &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn set_name(&self, neovim: &mut Neovim, name: &str) -> Result<(), CallError> {
        neovim
            .session
            .call("nvim_buf_set_name",
                  &call_args![self.code_data.clone(), name])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn is_valid(&self, neovim: &mut Neovim) -> Result<bool, CallError> {
        neovim
            .session
            .call("nvim_buf_is_valid", &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn get_mark(&self, neovim: &mut Neovim, name: &str) -> Result<(u64, u64), CallError> {
        neovim
            .session
            .call("nvim_buf_get_mark",
                  &call_args![self.code_data.clone(), name])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn add_highlight(&self,
                         neovim: &mut Neovim,
                         src_id: u64,
                         hl_group: &str,
                         line: u64,
                         col_start: u64,
                         col_end: u64)
                         -> Result<u64, CallError> {
        neovim
            .session
            .call("nvim_buf_add_highlight",
                  &call_args![self.code_data.clone(),
                              src_id,
                              hl_group,
                              line,
                              col_start,
                              col_end])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn clear_highlight(&self,
                           neovim: &mut Neovim,
                           src_id: u64,
                           line_start: u64,
                           line_end: u64)
                           -> Result<(), CallError> {
        neovim
            .session
            .call("nvim_buf_clear_highlight",
                  &call_args![self.code_data.clone(), src_id, line_start, line_end])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: xxxx
    pub fn live_updates(&self, neovim: &mut Neovim, enabled: bool) -> Result<(), CallError> {
        let mut v = Vec::new();
        v.push(self.code_data.clone().into_val());
        v.push(enabled.into_val());
        neovim
            .session
            .call(
                "nvim_buf_live_updates",
                &v,
            )
            .map(map_result)
            .map_err(map_generic_error)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Window {
    code_data: Value,
}

impl Window {
    pub fn new(code_data: Value) -> Window {
        Window { code_data: code_data }
    }

    /// Internal value, that represent type
    pub fn get_value(&self) -> &Value {
        &self.code_data
    }

    /// since: 1
    pub fn get_buf(&self, neovim: &mut Neovim) -> Result<Buffer, CallError> {
        neovim
            .session
            .call("nvim_win_get_buf", &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn get_cursor(&self, neovim: &mut Neovim) -> Result<(u64, u64), CallError> {
        neovim
            .session
            .call("nvim_win_get_cursor", &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn set_cursor(&self, neovim: &mut Neovim, pos: (u64, u64)) -> Result<(), CallError> {
        neovim
            .session
            .call("nvim_win_set_cursor",
                  &call_args![self.code_data.clone(), pos])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn get_height(&self, neovim: &mut Neovim) -> Result<u64, CallError> {
        neovim
            .session
            .call("nvim_win_get_height", &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn set_height(&self, neovim: &mut Neovim, height: u64) -> Result<(), CallError> {
        neovim
            .session
            .call("nvim_win_set_height",
                  &call_args![self.code_data.clone(), height])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn get_width(&self, neovim: &mut Neovim) -> Result<u64, CallError> {
        neovim
            .session
            .call("nvim_win_get_width", &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn set_width(&self, neovim: &mut Neovim, width: u64) -> Result<(), CallError> {
        neovim
            .session
            .call("nvim_win_set_width",
                  &call_args![self.code_data.clone(), width])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn get_var(&self, neovim: &mut Neovim, name: &str) -> Result<Value, CallError> {
        neovim
            .session
            .call("nvim_win_get_var",
                  &call_args![self.code_data.clone(), name])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn set_var(&self, neovim: &mut Neovim, name: &str, value: Value) -> Result<(), CallError> {
        neovim
            .session
            .call("nvim_win_set_var",
                  &call_args![self.code_data.clone(), name, value])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn del_var(&self, neovim: &mut Neovim, name: &str) -> Result<(), CallError> {
        neovim
            .session
            .call("nvim_win_del_var",
                  &call_args![self.code_data.clone(), name])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn get_option(&self, neovim: &mut Neovim, name: &str) -> Result<Value, CallError> {
        neovim
            .session
            .call("nvim_win_get_option",
                  &call_args![self.code_data.clone(), name])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn set_option(&self,
                      neovim: &mut Neovim,
                      name: &str,
                      value: Value)
                      -> Result<(), CallError> {
        neovim
            .session
            .call("nvim_win_set_option",
                  &call_args![self.code_data.clone(), name, value])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn get_position(&self, neovim: &mut Neovim) -> Result<(u64, u64), CallError> {
        neovim
            .session
            .call("nvim_win_get_position", &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn get_tabpage(&self, neovim: &mut Neovim) -> Result<Tabpage, CallError> {
        neovim
            .session
            .call("nvim_win_get_tabpage", &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn get_number(&self, neovim: &mut Neovim) -> Result<u64, CallError> {
        neovim
            .session
            .call("nvim_win_get_number", &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn is_valid(&self, neovim: &mut Neovim) -> Result<bool, CallError> {
        neovim
            .session
            .call("nvim_win_is_valid", &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Tabpage {
    code_data: Value,
}

impl Tabpage {
    pub fn new(code_data: Value) -> Tabpage {
        Tabpage { code_data: code_data }
    }

    /// Internal value, that represent type
    pub fn get_value(&self) -> &Value {
        &self.code_data
    }

    /// since: 1
    pub fn list_wins(&self, neovim: &mut Neovim) -> Result<Vec<Window>, CallError> {
        neovim
            .session
            .call("nvim_tabpage_list_wins",
                  &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn get_var(&self, neovim: &mut Neovim, name: &str) -> Result<Value, CallError> {
        neovim
            .session
            .call("nvim_tabpage_get_var",
                  &call_args![self.code_data.clone(), name])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn set_var(&self, neovim: &mut Neovim, name: &str, value: Value) -> Result<(), CallError> {
        neovim
            .session
            .call("nvim_tabpage_set_var",
                  &call_args![self.code_data.clone(), name, value])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn del_var(&self, neovim: &mut Neovim, name: &str) -> Result<(), CallError> {
        neovim
            .session
            .call("nvim_tabpage_del_var",
                  &call_args![self.code_data.clone(), name])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn get_win(&self, neovim: &mut Neovim) -> Result<Window, CallError> {
        neovim
            .session
            .call("nvim_tabpage_get_win", &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn get_number(&self, neovim: &mut Neovim) -> Result<u64, CallError> {
        neovim
            .session
            .call("nvim_tabpage_get_number",
                  &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
    /// since: 1
    pub fn is_valid(&self, neovim: &mut Neovim) -> Result<bool, CallError> {
        neovim
            .session
            .call("nvim_tabpage_is_valid", &call_args![self.code_data.clone()])
            .map(map_result)
            .map_err(map_generic_error)
    }
}


impl FromVal<Value> for Buffer {
    fn from_val(val: Value) -> Self {
        Buffer::new(val)
    }
}

impl<'a> IntoVal<Value> for &'a Buffer {
    fn into_val(self) -> Value {
        self.code_data.clone()
    }
}
impl FromVal<Value> for Window {
    fn from_val(val: Value) -> Self {
        Window::new(val)
    }
}

impl<'a> IntoVal<Value> for &'a Window {
    fn into_val(self) -> Value {
        self.code_data.clone()
    }
}
impl FromVal<Value> for Tabpage {
    fn from_val(val: Value) -> Self {
        Tabpage::new(val)
    }
}

impl<'a> IntoVal<Value> for &'a Tabpage {
    fn into_val(self) -> Value {
        self.code_data.clone()
    }
}

pub trait NeovimApi {
    /// since: 1
    fn ui_detach(&mut self) -> Result<(), CallError>;
    /// since: 1
    fn ui_try_resize(&mut self, width: u64, height: u64) -> Result<(), CallError>;
    /// since: 1
    fn ui_set_option(&mut self, name: &str, value: Value) -> Result<(), CallError>;
    /// since: 1
    fn command(&mut self, command: &str) -> Result<(), CallError>;
    /// since: 1
    fn feedkeys(&mut self, keys: &str, mode: &str, escape_csi: bool) -> Result<(), CallError>;
    /// since: 1
    fn input(&mut self, keys: &str) -> Result<u64, CallError>;
    /// since: 1
    fn replace_termcodes(&mut self,
                         str: &str,
                         from_part: bool,
                         do_lt: bool,
                         special: bool)
                         -> Result<String, CallError>;
    /// since: 1
    fn command_output(&mut self, str: &str) -> Result<String, CallError>;
    /// since: 1
    fn eval(&mut self, expr: &str) -> Result<Value, CallError>;
    /// since: 1
    fn call_function(&mut self, fname: &str, args: Vec<Value>) -> Result<Value, CallError>;
    /// since: 1
    fn strwidth(&mut self, str: &str) -> Result<u64, CallError>;
    /// since: 1
    fn list_runtime_paths(&mut self) -> Result<Vec<String>, CallError>;
    /// since: 1
    fn set_current_dir(&mut self, dir: &str) -> Result<(), CallError>;
    /// since: 1
    fn get_current_line(&mut self) -> Result<String, CallError>;
    /// since: 1
    fn set_current_line(&mut self, line: &str) -> Result<(), CallError>;
    /// since: 1
    fn del_current_line(&mut self) -> Result<(), CallError>;
    /// since: 1
    fn get_var(&mut self, name: &str) -> Result<Value, CallError>;
    /// since: 1
    fn set_var(&mut self, name: &str, value: Value) -> Result<(), CallError>;
    /// since: 1
    fn del_var(&mut self, name: &str) -> Result<(), CallError>;
    /// since: 1
    fn get_vvar(&mut self, name: &str) -> Result<Value, CallError>;
    /// since: 1
    fn get_option(&mut self, name: &str) -> Result<Value, CallError>;
    /// since: 1
    fn set_option(&mut self, name: &str, value: Value) -> Result<(), CallError>;
    /// since: 1
    fn out_write(&mut self, str: &str) -> Result<(), CallError>;
    /// since: 1
    fn err_write(&mut self, str: &str) -> Result<(), CallError>;
    /// since: 1
    fn err_writeln(&mut self, str: &str) -> Result<(), CallError>;
    /// since: 1
    fn list_bufs(&mut self) -> Result<Vec<Buffer>, CallError>;
    /// since: 1
    fn get_current_buf(&mut self) -> Result<Buffer, CallError>;
    /// since: 1
    fn set_current_buf(&mut self, buffer: &Buffer) -> Result<(), CallError>;
    /// since: 1
    fn list_wins(&mut self) -> Result<Vec<Window>, CallError>;
    /// since: 1
    fn get_current_win(&mut self) -> Result<Window, CallError>;
    /// since: 1
    fn set_current_win(&mut self, window: &Window) -> Result<(), CallError>;
    /// since: 1
    fn list_tabpages(&mut self) -> Result<Vec<Tabpage>, CallError>;
    /// since: 1
    fn get_current_tabpage(&mut self) -> Result<Tabpage, CallError>;
    /// since: 1
    fn set_current_tabpage(&mut self, tabpage: &Tabpage) -> Result<(), CallError>;
    /// since: 1
    fn subscribe(&mut self, event: &str) -> Result<(), CallError>;
    /// since: 1
    fn unsubscribe(&mut self, event: &str) -> Result<(), CallError>;
    /// since: 1
    fn get_color_by_name(&mut self, name: &str) -> Result<u64, CallError>;
    /// since: 1
    fn get_color_map(&mut self) -> Result<Vec<(Value, Value)>, CallError>;
    /// since: 2
    fn get_mode(&mut self) -> Result<Vec<(Value, Value)>, CallError>;
    /// since: 1
    fn get_api_info(&mut self) -> Result<Vec<Value>, CallError>;
    /// since: 1
    fn call_atomic(&mut self, calls: Vec<Value>) -> Result<Vec<Value>, CallError>;
}

impl NeovimApi for Neovim {
    fn ui_detach(&mut self) -> Result<(), CallError> {
        self.session
            .call("nvim_ui_detach", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn ui_try_resize(&mut self, width: u64, height: u64) -> Result<(), CallError> {
        self.session
            .call("nvim_ui_try_resize", &call_args![width, height])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn ui_set_option(&mut self, name: &str, value: Value) -> Result<(), CallError> {
        self.session
            .call("nvim_ui_set_option", &call_args![name, value])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn command(&mut self, command: &str) -> Result<(), CallError> {
        self.session
            .call("nvim_command", &call_args![command])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn feedkeys(&mut self, keys: &str, mode: &str, escape_csi: bool) -> Result<(), CallError> {
        self.session
            .call("nvim_feedkeys", &call_args![keys, mode, escape_csi])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn input(&mut self, keys: &str) -> Result<u64, CallError> {
        self.session
            .call("nvim_input", &call_args![keys])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn replace_termcodes(&mut self,
                         str: &str,
                         from_part: bool,
                         do_lt: bool,
                         special: bool)
                         -> Result<String, CallError> {
        self.session
            .call("nvim_replace_termcodes",
                  &call_args![str, from_part, do_lt, special])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn command_output(&mut self, str: &str) -> Result<String, CallError> {
        self.session
            .call("nvim_command_output", &call_args![str])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn eval(&mut self, expr: &str) -> Result<Value, CallError> {
        self.session
            .call("nvim_eval", &call_args![expr])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn call_function(&mut self, fname: &str, args: Vec<Value>) -> Result<Value, CallError> {
        self.session
            .call("nvim_call_function", &call_args![fname, args])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn strwidth(&mut self, str: &str) -> Result<u64, CallError> {
        self.session
            .call("nvim_strwidth", &call_args![str])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn list_runtime_paths(&mut self) -> Result<Vec<String>, CallError> {
        self.session
            .call("nvim_list_runtime_paths", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn set_current_dir(&mut self, dir: &str) -> Result<(), CallError> {
        self.session
            .call("nvim_set_current_dir", &call_args![dir])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_current_line(&mut self) -> Result<String, CallError> {
        self.session
            .call("nvim_get_current_line", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn set_current_line(&mut self, line: &str) -> Result<(), CallError> {
        self.session
            .call("nvim_set_current_line", &call_args![line])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn del_current_line(&mut self) -> Result<(), CallError> {
        self.session
            .call("nvim_del_current_line", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_var(&mut self, name: &str) -> Result<Value, CallError> {
        self.session
            .call("nvim_get_var", &call_args![name])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn set_var(&mut self, name: &str, value: Value) -> Result<(), CallError> {
        self.session
            .call("nvim_set_var", &call_args![name, value])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn del_var(&mut self, name: &str) -> Result<(), CallError> {
        self.session
            .call("nvim_del_var", &call_args![name])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_vvar(&mut self, name: &str) -> Result<Value, CallError> {
        self.session
            .call("nvim_get_vvar", &call_args![name])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_option(&mut self, name: &str) -> Result<Value, CallError> {
        self.session
            .call("nvim_get_option", &call_args![name])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn set_option(&mut self, name: &str, value: Value) -> Result<(), CallError> {
        self.session
            .call("nvim_set_option", &call_args![name, value])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn out_write(&mut self, str: &str) -> Result<(), CallError> {
        self.session
            .call("nvim_out_write", &call_args![str])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn err_write(&mut self, str: &str) -> Result<(), CallError> {
        self.session
            .call("nvim_err_write", &call_args![str])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn err_writeln(&mut self, str: &str) -> Result<(), CallError> {
        self.session
            .call("nvim_err_writeln", &call_args![str])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn list_bufs(&mut self) -> Result<Vec<Buffer>, CallError> {
        self.session
            .call("nvim_list_bufs", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_current_buf(&mut self) -> Result<Buffer, CallError> {
        self.session
            .call("nvim_get_current_buf", &Vec::new())
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn set_current_buf(&mut self, buffer: &Buffer) -> Result<(), CallError> {
        self.session
            .call("nvim_set_current_buf", &call_args![buffer])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn list_wins(&mut self) -> Result<Vec<Window>, CallError> {
        self.session
            .call("nvim_list_wins", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_current_win(&mut self) -> Result<Window, CallError> {
        self.session
            .call("nvim_get_current_win", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn set_current_win(&mut self, window: &Window) -> Result<(), CallError> {
        self.session
            .call("nvim_set_current_win", &call_args![window])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn list_tabpages(&mut self) -> Result<Vec<Tabpage>, CallError> {
        self.session
            .call("nvim_list_tabpages", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_current_tabpage(&mut self) -> Result<Tabpage, CallError> {
        self.session
            .call("nvim_get_current_tabpage", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn set_current_tabpage(&mut self, tabpage: &Tabpage) -> Result<(), CallError> {
        self.session
            .call("nvim_set_current_tabpage", &call_args![tabpage])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn subscribe(&mut self, event: &str) -> Result<(), CallError> {
        self.session
            .call("nvim_subscribe", &call_args![event])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn unsubscribe(&mut self, event: &str) -> Result<(), CallError> {
        self.session
            .call("nvim_unsubscribe", &call_args![event])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_color_by_name(&mut self, name: &str) -> Result<u64, CallError> {
        self.session
            .call("nvim_get_color_by_name", &call_args![name])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_color_map(&mut self) -> Result<Vec<(Value, Value)>, CallError> {
        self.session
            .call("nvim_get_color_map", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_mode(&mut self) -> Result<Vec<(Value, Value)>, CallError> {
        self.session
            .call("nvim_get_mode", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_api_info(&mut self) -> Result<Vec<Value>, CallError> {
        self.session
            .call("nvim_get_api_info", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn call_atomic(&mut self, calls: Vec<Value>) -> Result<Vec<Value>, CallError> {
        self.session
            .call("nvim_call_atomic", &call_args![calls])
            .map(map_result)
            .map_err(map_generic_error)
    }
}
