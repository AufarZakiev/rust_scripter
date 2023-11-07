pub enum ParamTypes {
    Number,
    String,
    Bool,
}

pub struct Runnable {
    pub inputs: Vec<(String, ParamTypes)>,
    pub outputs: Vec<(String, ParamTypes)>,
}