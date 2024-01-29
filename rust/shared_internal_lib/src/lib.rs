use shared_internal_lib;    // shared crate
use godot::prelude::*;

mod my_rust_lib;

// CRITICAL: __MUST__ expose entry point or else you will get the error:
// "GDExtension entry point 'gdext_rust_init' not found in library ..."
// NOTE that there are ONE and ONLY ONE entry point per library.
#[gdextension]
unsafe impl ExtensionLibrary for entry_point::Node_MyExtensionStructLib1 {}

pub mod entry_point {
    use godot::engine::Node;
    use godot::prelude::*;
    use crate::my_rust_lib;

    // Odd (and annoying) as the naming may be, because Godot Editor does not show namespaces
    // (but it does show up under corect parent that BASE is based off of), for clarity purpose,
    // I will name them as "<ParentNode>_<ObjectName>" so that when adding nodes in Godot Editor,
    // if you have Lib1::MyStruct and Lib2::MyStruct, it will distinguish them
    #[derive(godot::register::GodotClass)]
    #[class(base = Node)]
    pub struct Node_MyExtensionStructLib1 {
        base: Base<Node>,

        #[var]
        my_i64: i64,

        #[var]
        my_f64: f64,

        // vars that are not exposed to Godot Editor
        internal_i64: i64,
        internal_obj1: my_rust_lib::my_rust_lib::MyStruct,
        internal_obj2: my_rust_lib::my_rust_lib::MyStruct2,
        internal_obj3: my_rust_lib::my_rust_lib::MyStruct3,
        internal_obj4: my_rust_lib::my_rust_lib::MyStruct4,
    }

    #[godot_api]
    impl INode for Node_MyExtensionStructLib1 {
        fn init(base: Base<Node>) -> Self {
            godot_print!("my_rust_lib::Node_MyExtensionStructLib1::init(): Hello, world!"); // Prints to the Godot console

            Self {
                base,
                my_i64: 0,
                my_f64: 0.0,
                internal_i64: 0,
                internal_obj1: my_rust_lib::my_rust_lib::MyStruct::new(Some(1), Some(3.0)),
                internal_obj2: my_rust_lib::my_rust_lib::MyStruct2::new(Some(1), Some(3.0)),
                internal_obj3: my_rust_lib::my_rust_lib::MyStruct3::new(Some(1), Some(3.0)),
                internal_obj4: my_rust_lib::my_rust_lib::MyStruct4::new(Some(1), Some(3.0)),
            }
        }

        fn ready(&mut self) {
            godot_print!("my_rust_lib::Node_MyExtensionStructLib1::_ready(): Hello, world!");
            // Prints to the Godot console
        }
    }

    #[godot_api]
    impl Node_MyExtensionStructLib1 {
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

        #[func]
        pub fn get_internal_i64(&self) -> i64 {
            self.internal_i64
        }
    }
}
