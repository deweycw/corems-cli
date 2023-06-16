use std::collections::HashMap;

#[derive(Debug)]
pub struct AssignParams<'a>{
    pub params: Parameters<'a>,
    pub elements: Elements,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct ParamIndex{
    pub i: i32,
}
#[derive(Debug)]
pub struct ParamValue<'a>{
    pub py_param: &'a str,
    pub value: &'a str,
}

pub type Parameters<'a> = HashMap<ParamIndex, ParamValue<'a>,>;

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct ElementIndex{
    pub i: i32,
}
#[derive(Debug)]
pub struct ElementValue{
    pub symbol: String,
    pub range: String,
}

pub type Elements= HashMap<ElementIndex, ElementValue>;


#[derive(PartialEq, Eq, Hash)]
pub struct CalIndex{
    pub i: i32,
}
#[derive(Debug)]
pub struct CalValue{
    pub py_param: String,
    pub value: String,
}


pub fn make_param_hash<'a>(param_vector: &Vec<(&'a str, &'a str)>) -> Parameters<'a> {

    let mut params_hash: Parameters = HashMap::new();

    let mut i: i32 = 0;

    for tuple in param_vector.iter() {
        let pname: &str = tuple.0;
        let pvalue: &str = tuple.1;
        let line_order = i;

        let param_index = ParamIndex {
            i: line_order,
        };            

        let param_value = ParamValue {
            py_param: pname,
            value: pvalue,
        };

        params_hash.insert(param_index, param_value);
        i = i + 1;

    }

    return params_hash;
}