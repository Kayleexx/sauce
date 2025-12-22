use super::value::Value;

#[derive(Debug)]
pub struct Effect {
    pub name: String,
    pub payload: Option<Value>,
}
