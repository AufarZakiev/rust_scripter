use ordered_hash_map::OrderedHashMap;

#[derive(Clone)]
pub enum ParamTypes {
    Number,
    String,
    Bool,
}

#[derive(Clone)]
pub struct Runnable {
    pub name: String,
    pub inputs: OrderedHashMap<String, ParamTypes>,
    pub outputs: OrderedHashMap<String, ParamTypes>,
}

impl Default for Runnable {
    fn default() -> Self {
        let mut inputs = OrderedHashMap::new();
        inputs.insert("Input1".into(), ParamTypes::String);
        inputs.insert("Input2".into(), ParamTypes::Number);
        inputs.insert("Input3".into(), ParamTypes::Bool);

        let mut outputs = OrderedHashMap::new();
        outputs.insert("Output1".into(), ParamTypes::String);
        outputs.insert("Output2".into(), ParamTypes::Bool);

        Runnable {
            name: "Function #0".to_owned(),
            inputs,
            outputs,
        }
    }
}