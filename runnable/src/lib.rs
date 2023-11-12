#[derive(Clone)]
pub enum ParamTypes {
    Number,
    String,
    Bool,
}

#[derive(Clone)]
pub struct Runnable {
    pub name: String,
    pub inputs: Vec<(String, ParamTypes)>,
    pub outputs: Vec<(String, ParamTypes)>,
}