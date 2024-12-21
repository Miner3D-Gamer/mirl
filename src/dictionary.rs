use serde_json::{json, Value};
//use std::any::Any;

pub fn empty_dict() -> Value {
    return json!({});
}
pub fn set_dict_value(dict: &mut Value, index: &str, item: impl Into<Value>) {
    dict[index] = item.into();
}
pub fn get_dict_value<'a>(dict: &'a Value, index: &str) -> Option<&'a Value> {
    return dict.get(index)
}