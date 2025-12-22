use std::collections::HashMap;
use super::value::Value;

#[derive(Debug, Clone)]
pub struct RuntimeEnv {
    vars: HashMap<String, Value>,
}

impl RuntimeEnv {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        self.vars.get(name).cloned()
    }

    pub fn set(&mut self, name: impl Into<String>, value: Value) {
        self.vars.insert(name.into(), value);
    }
}
