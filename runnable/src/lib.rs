use std::collections::HashMap;

#[derive(Clone)]
pub enum ParamTypes {
    Number,
    String,
    Bool,
}

#[derive(Clone)]
pub struct Runnable {
    pub name: String,
    pub inputs: HashMap<String, ParamTypes>,
    pub outputs: HashMap<String, ParamTypes>,
}