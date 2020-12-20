use std::collections::HashMap;

pub struct Frame {
    variables: HashMap<i8, i8>,
    pub return_address: usize
}

impl Frame {
    pub fn new(return_address: usize) -> Frame {
        Frame { variables: HashMap::new(), return_address }
    }

    pub fn store(&mut self, key: i8, value: i8) {
        self.variables.insert(key, value);
    }

    pub fn load(&self, key: &i8) -> Option<&i8> {
        self.variables.get(key)
    }
}