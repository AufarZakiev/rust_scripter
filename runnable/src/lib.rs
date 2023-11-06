use std::collections::HashMap;

pub enum ParamTypes {
    Int,
    Float,
    String,
}

pub struct Runnable {
    pub inputs: HashMap<String, ParamTypes>,
    pub outputs: HashMap<String, ParamTypes>,
}
