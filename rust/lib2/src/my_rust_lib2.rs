// NOTE: This module is not meant to be used directly, but rather to be used as a library.
pub mod my_rust_lib2 {
    use godot::engine::Node;
    use godot::prelude::*;

    #[derive(GodotClass)]
    #[class(base=Node)]
    pub struct MyStructForLib2 {
        base: Base<Node>,

        my_i64: i64,

        my_f64: f64,
    }

    #[godot_api]
    impl INode for MyStructForLib2 {
        fn init(base: Base<Node>) -> Self {
            Self {
                base,
                my_i64: 0,
                my_f64: 0.0,
            }
        }
    }

    #[godot_api]
    impl MyStructForLib2 {
        #[func]
        pub fn get_i64(&self) -> i64 {
            self.my_i64
        }

        #[func]
        pub fn set_i64(&mut self, val: i64) {
            self.my_i64 = val;
        }

        #[func]
        pub fn get_f64(&self) -> f64 {
            self.my_f64
        }

        #[func]
        pub fn set_f64(&mut self, val: f64) {
            self.my_f64 = val;
        }
    }
}
