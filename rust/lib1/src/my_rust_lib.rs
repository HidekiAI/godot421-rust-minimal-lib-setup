// NOTE: This module is not meant to be used directly, but rather to be used as a library.
pub mod my_rust_lib {
    use godot::engine::Node;
    use godot::prelude::*;

    // primitive struct that has no impl (OK, it does have new())
    pub struct MyStruct {
        my_i64: i64,
        my_f64: f64,
    }
    // with impl (including new())
    pub struct MyStruct2 {
        my_i64: i64,
        my_f64: f64,
    }
    // uses shared_internal_lib
    pub struct MyStruct3 {
        my_i64: i64,
        my_f64: f64,
    }
    // uses Node2D
    #[derive(GodotClass)]
    #[class(base=Node2D)]
    pub struct MyStructNode2D {
        base: Base<Node2D>,
        #[var]
        my_i64: i64,
        #[var]
        my_f64: f64,
    }

    impl MyStruct {
        pub fn new(parm1: Option<i64>, parm2: Option<f64>) -> Self {
            Self {
                my_i64: parm1.unwrap_or_default(),
                my_f64: parm2.unwrap_or_default(),
            }
        }
    }

    impl MyStruct2 {
        pub fn new(arg1: Option<i64>, arg2: Option<f64>) -> Self {
            Self {
                my_i64: arg1.unwrap_or_default(),
                my_f64: arg2.unwrap_or_default(),
            }
        }
        pub fn set_f64(&mut self, f64: f64) {
            self.my_f64 = f64;
        }
        pub fn get_f64(&self) -> f64 {
            self.my_f64
        }
        pub fn set_i64(&mut self, i64: i64) {
            self.my_i64 = i64;
        }
        pub fn get_i64(&self) -> i64 {
            self.my_i64
        }
    }

    impl MyStruct3 {
        pub fn new(arg1: Option<i64>, arg2: Option<f64>) -> Self {
            Self {
                my_i64: arg1.unwrap_or_default(),
                my_f64: arg2.unwrap_or_default(),
            }
        }
        pub fn set_f64(&mut self, f64: f64) {
            self.my_f64 = f64;
        }
        pub fn get_f64(&self) -> f64 {
            self.my_f64
        }
        pub fn set_i64(&mut self, i64: i64) {
            self.my_i64 = i64;
        }
        pub fn get_i64(&self) -> i64 {
            self.my_i64
        }
    }

    #[godot_api]
    impl INode2D for MyStructNode2D {
        fn init(base: Base<Node2D>) -> Self {
            godot_print!("my_rust_lib::MyStructNode2D::init(): Hello, world!"); // Prints to the Godot console
            Self {
                base,
                my_i64: 0,
                my_f64: 0.0,
            }
        }
        fn ready(&mut self) {
            godot_print!("my_rust_lib::MyStructNode2D::_ready(): Hello, world!");
            // Prints to the Godot console
        }
    }

    impl MyStructNode2D {
        pub fn new(arg1: Option<i64>, arg2: Option<f64>) -> Self {
            Self {
                base: Node2D::new().into(),
                my_i64: arg1.unwrap_or_default(),
                my_f64: arg2.unwrap_or_default(),
            }
        }
        pub fn set_f64(&mut self, f64: f64) {
            self.my_f64 = f64;
        }
        pub fn get_f64(&self) -> f64 {
            self.my_f64
        }
        pub fn set_i64(&mut self, i64: i64) {
            self.my_i64 = i64;
        }
        pub fn get_i64(&self) -> i64 {
            self.my_i64
        }
    }
}
