use session::Session;
use rpc::*;
use rmpv::Value;
use neovim_api::NeovimApi;
use std::fmt;
use std::error::Error;

pub struct Neovim {
    pub session: Session,
}

pub enum UiOption {
    RGB(bool),
    ExtPopupmenu(bool),
    ExtTabline(bool),
}

impl UiOption {
    fn to_value(&self) -> (Value, Value) {
        let name_value = self.to_name_value();
        (name_value.0.into(), name_value.1)
    }

    fn to_name_value(&self) -> (&str, Value) {
        match self {
            &UiOption::RGB(val) => ("rgb", val.into()),
            &UiOption::ExtPopupmenu(val) => ("ext_popupmenu", val.into()),
            &UiOption::ExtTabline(val) => ("ext_tabline", val.into()),
        }
    }
}

pub struct UiAttachOptions {
    rgb: UiOption,
    popupmenu_external: UiOption,
    tabline_external: UiOption,
}

impl UiAttachOptions {
    pub fn new() -> UiAttachOptions {
        UiAttachOptions {
            rgb: UiOption::RGB(true),
            popupmenu_external: UiOption::ExtPopupmenu(false),
            tabline_external: UiOption::ExtTabline(false),
        }
    }

    pub fn set_rgb(&mut self, rgb: bool) {
        self.rgb = UiOption::RGB(rgb);
    }

    pub fn set_popupmenu_external(&mut self, popupmenu_external: bool) {
        self.popupmenu_external = UiOption::ExtPopupmenu(popupmenu_external);
    }

    pub fn set_tabline_external(&mut self, tabline_external: bool) {
        self.tabline_external = UiOption::ExtTabline(tabline_external);
    }

    fn to_value_map(&self) -> Value {
        Value::Map(vec![self.rgb.to_value(),
                        self.popupmenu_external.to_value(),
                        self.tabline_external.to_value()])
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CallError {
    GenericError(String),
    NeovimError(u64, String),
}

impl fmt::Display for CallError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CallError::GenericError(ref s) => write!(f, "Unknown error type: {}", s),
            CallError::NeovimError(id, ref s) => write!(f, "{} - {}", id, s),
        }
    }
}

impl Error for CallError {
    fn description(&self) -> &str {
        match *self {
            CallError::GenericError(ref s) => s,
            CallError::NeovimError(_, ref s) => s,
        }
    }
}


#[doc(hidden)]
pub fn map_generic_error(err: Value) -> CallError {
    match err {
        Value::String(val) => CallError::GenericError(val.as_str().unwrap().to_owned()),
        Value::Array(arr) => {
            if arr.len() == 2 {
                match (&arr[0], &arr[1]) {
                    (&Value::Integer(ref id), &Value::String(ref val)) => {
                        CallError::NeovimError(id.as_u64().unwrap(),
                                               val.as_str().unwrap().to_owned())
                    }
                    _ => CallError::GenericError(format!("{:?}", arr)),
                }
            } else {
                CallError::GenericError(format!("{:?}", arr))
            }
        }
        val => CallError::GenericError(format!("{:?}", val)),
    }
}

#[doc(hidden)]
pub fn map_result<T: FromVal<Value>>(val: Value) -> T {
    T::from_val(val)
}

impl Neovim {
    pub fn new(session: Session) -> Neovim {
        Neovim { session: session }
    }

    /// Register as a remote UI.
    ///
    /// After this method is called, the client will receive redraw notifications.
    pub fn ui_attach(&mut self,
                     width: u64,
                     height: u64,
                     opts: UiAttachOptions)
                     -> Result<(), CallError> {
        self.session
            .call("nvim_ui_attach",
                  &call_args!(width, height, opts.to_value_map()))
            .map_err(map_generic_error)
            .map(|_| ())
    }

    /// Send a quit command to Nvim.
    /// The quit command is 'qa!' which will make Nvim quit without
    /// saving anything.
    pub fn quit_no_save(&mut self) -> Result<(), CallError> {
        self.command("qa!")
    }

    /// Same as `ui_set_option` but use `UiOption` as argument to check type at compile time
    pub fn set_option(&mut self, option: UiOption) -> Result<(), CallError> {
        let name_value = option.to_name_value();
        self.ui_set_option(name_value.0, name_value.1)
    }
}
