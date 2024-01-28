use godot::prelude::*;

// CRITICAL: __MUST__ expose entry point or else you will get the error:
// "GDExtension entry point 'gdext_rust_init' not found in library ..."
#[gdextension]
unsafe impl ExtensionLibrary for my_rust_lib::MyExtensionStructLib1 {}

pub mod my_rust_lib {
    use godot::engine::Node;
    use godot::engine::Node2D;
    use godot::prelude::*;

    #[derive(godot::register::GodotClass)]
    #[class(base = Node)]
    pub struct MyExtensionStructLib1 {
        base: Base<Node>,

        #[var]
        my_i64: i64,

        #[var]
        my_f64: f64,
    }

    #[godot_api]
    impl INode for MyExtensionStructLib1 {
        fn init(base: Base<Node>) -> Self {
            godot_print!("my_rust_lib::MyExtensionStructLib1::init(): Hello, world!"); // Prints to the Godot console

            Self {
                base,
                my_i64: 0,
                my_f64: 0.0,
            }
        }

        fn ready(&mut self) {
            godot_print!("my_rust_lib::MyExtensionStructLib1::_ready(): Hello, world!"); // Prints to the Godot console
        }
    }

    #[godot_api]
    impl MyExtensionStructLib1 {
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
