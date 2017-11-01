use neovim_lib::Value;
use neovim_lib::neovim_api::Buffer;

pub fn parse_u64(value: &Value) -> Result<u64, String> {
    value.as_u64().ok_or_else(
        || "cannot parse usize".to_owned(),
    )
}

pub fn parse_bool(value: &Value) -> Result<bool, String> {
    value.as_bool().ok_or_else(
        || "cannot parse bool".to_owned(),
    )
}

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

pub fn parse_buf(value: &Value) -> Buffer {
    Buffer::new(value.clone())
}
