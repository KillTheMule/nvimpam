// Auto generated {{date}}

use neovim::*;
use rpc::*;

{% for etype in exttypes %}
#[derive(PartialEq, Clone, Debug)]
pub struct {{ etype.name }} {
    code_data: Value,
}

impl {{ etype.name }} {
    pub fn new(code_data: Value) -> {{ etype.name }} {
        {{ etype.name }} {
            code_data: code_data,
        }
    }

    /// Internal value, that represent type
    pub fn get_value(&self) -> &Value {
        &self.code_data
    }

    {% for f in functions if f.ext and f.name.startswith(etype.prefix) %}
    /// since: {{f.since}}
    pub fn {{f.name|replace(etype.prefix, '')}}(&self, neovim: &mut Neovim, {{f.argstring}}) -> Result<{{f.return_type.native_type_ret}}, CallError> {
        neovim.session.call("{{f.name}}",
                          &call_args![self.code_data.clone()
                          {% if f.parameters|count > 0 %}
                          , {{ f.parameters|map(attribute = "name")|join(", ") }}
                          {% endif %}
                          ])
                    .map(map_result)
                    .map_err(map_generic_error)
    }
    {% endfor %}
}

{% endfor %}

{% for etype in exttypes %}
impl FromVal<Value> for {{ etype.name }} {
    fn from_val(val: Value) -> Self {
        {{ etype.name }}::new(val)
    }
}

impl <'a> IntoVal<Value> for &'a {{etype.name}} {
    fn into_val(self) -> Value {
        self.code_data.clone()
    }
}
{% endfor %}

pub trait NeovimApi {
    {% for f in functions if not f.ext %}
    /// since: {{f.since}}
    fn {{f.name|replace('nvim_', '')}}(&mut self, {{f.argstring}}) -> Result<{{f.return_type.native_type_ret}}, CallError>;
    {% endfor %}
}

impl NeovimApi for Neovim {
    {% for f in functions if not f.ext %}
    fn {{f.name|replace('nvim_', '')}}(&mut self, {{f.argstring}}) -> Result<{{f.return_type.native_type_ret}}, CallError> {
        self.session.call("{{f.name}}",
                          &call_args![{{ f.parameters|map(attribute = "name")|join(", ") }}])
                    .map(map_result)
                    .map_err(map_generic_error)
    }

    {% endfor %}
}
