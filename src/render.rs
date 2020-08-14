use std::collections::HashMap;
use tera::{to_value, try_get_value, Filter, Result, Value};

pub(crate) struct TypeHeader(pub(crate) HashMap<String, String>);

impl Filter for TypeHeader {
    fn filter(&self, value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
        let ty = try_get_value!("typeheader", "value", String, value);

        to_value(self.0.get(&ty).unwrap_or(&ty)).map_err(Into::into)
    }
}

pub(crate) struct ScopeHeader(pub(crate) HashMap<String, String>);

impl Filter for ScopeHeader {
    fn filter(&self, value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
        let ty = try_get_value!("scopeheader", "value", String, value);

        to_value(self.0.get(&ty).unwrap_or(&ty)).map_err(Into::into)
    }
}

pub(crate) fn indent(value: &Value, args: &HashMap<String, Value>) -> Result<Value> {
    let s = try_get_value!("indent", "value", String, value);
    let n = match args.get("n") {
        Some(val) => try_get_value!("indent", "n", usize, val),
        None => return Err("Filter `indent` expected an arg called `n`".into()),
    };

    let mut out = vec![];
    for line in s.lines() {
        if !line.is_empty() {
            out.push(" ".repeat(n) + line);
        } else {
            out.push(line.to_owned());
        }
    }

    Ok(to_value(&out.join("\n"))?)
}
