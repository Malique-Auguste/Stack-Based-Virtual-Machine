use std::collections::HashMap;

pub struct Frame {
    variables: HashMap<i16, i16>,
    pub return_address: usize
}

impl Frame {
    pub fn new(return_address: usize) -> Frame {
        Frame { variables: HashMap::new(), return_address }
    }

    pub fn store(&mut self, key: i16, value: i16) {
        self.variables.insert(key, value);
    }

    pub fn load(&self, key: &i16) -> Option<&i16> {
        self.variables.get(key)
    }
}