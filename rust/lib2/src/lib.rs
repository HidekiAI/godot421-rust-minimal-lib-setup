use godot::prelude::*;

// CRITICAL: __MUST__ expose entry point or else you will get the error:
// "GDExtension entry point 'gdext_rust_init' not found in library ..."
#[gdextension]
unsafe impl ExtensionLibrary for my_rust_lib2::MyExtensionStructLib2 {}

pub mod my_rust_lib2 {
    use godot::engine::Node2D;
    use godot::prelude::*;

    #[derive(GodotClass)]
    #[class(base = Node2D)]
    pub struct MyExtensionStructLib2 {
        base: Base<Node2D>,

        #[var]
        my_i64: i64,

        #[var]
        my_f64: f64,
    }

    #[godot_api]
    impl INode2D for MyExtensionStructLib2 {
        fn init(base: Base<Node2D>) -> Self {
            godot_print!("my_rust_lib2::MyExtensionStructLib2::init(): Hello, world!"); // Prints to the Godot console

            Self {
                base,
                my_i64: 0,
                my_f64: 0.0,
            }
        }

        fn ready(&mut self) {
            godot_print!("my_rust_lib2::MyExtensionStructLib2::ready(): Hello, world!"); // Prints to the Godot console
        }

        //fn physics_process(&mut self, delta: f64) {
        //}
    }

    #[godot_api]
    impl MyExtensionStructLib2 {
        #[func]
        fn get_i64(&self) -> i64 {
            self.my_i64
        }

        #[func]
        fn set_i64(&mut self, val: i64) {
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
