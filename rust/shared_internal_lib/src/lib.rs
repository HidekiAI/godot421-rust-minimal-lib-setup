mod my_rust_lib;

pub struct MySharedStructLib {
    my_i64: i64,
    my_f64: f64,
    my_node: my_rust_lib::my_rust_lib::MyStruct,
}

impl MySharedStructLib {
    pub fn get_i64(&self) -> i64 {
        self.my_i64
    }

    pub fn set_i64(&mut self, val: i64) {
        self.my_i64 = val;
    }

    pub fn get_f64(&self) -> f64 {
        self.my_f64
    }

    pub fn set_f64(&mut self, val: f64) {
        self.my_f64 = val;
    }
}
