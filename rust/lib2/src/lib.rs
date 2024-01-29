use shared_internal_lib;    // shared crate
use godot::prelude::*;

mod my_rust_lib2;

// CRITICAL: __MUST__ expose entry point or else you will get the error:
// "GDExtension entry point 'gdext_rust_init' not found in library ..."
// NOTE that there are ONE and ONLY ONE entry point per library.
#[gdextension]
unsafe impl ExtensionLibrary for entry_point::Node2D_MyExtensionStructLib2 {}

pub mod entry_point {
    use std::str::FromStr;

    use godot::engine::Node2D;
    use godot::prelude::*;
    use crate::my_rust_lib2;

    // Odd (and annoying) as the naming may be, because Godot Editor does not show namespaces
    // (but it does show up under corect parent that BASE is based off of), for clarity purpose,
    // I will name them as "<ParentNode>_<ObjectName>" so that when adding nodes in Godot Editor,
    // if you have Lib2::MyStruct and Lib2::MyStruct, it will distinguish them
    #[derive(godot::register::GodotClass)]
    #[class(base = Node2D)]
    pub struct Node2D_MyExtensionStructLib2 {
        base: Base<Node2D>,

        #[var]
        my_i64: i64,

        #[var]
        my_f64: f64,

        // vars that are not exposed to Godot Editor
        internal_i64: i64,

        // It has to be Option<> based, for MyStructForLib2::base's init() and this struct's init() are called in unordered fashion
        internal_obj1: Option<Gd<my_rust_lib2::my_rust_lib2::MyStructForLib2>>,
    }

    #[godot_api]
    impl INode2D for Node2D_MyExtensionStructLib2 {
        fn init(base: Base<Node2D>) -> Self {
            godot_print!("my_rust_lib::Node2D_MyExtensionStructLib2::init(): 'godot-rust-hello_world' app"); // Prints to the Godot console

            Self {
                base,
                my_i64: 0,
                my_f64: 0.0,
                internal_i64: 0,
                // It'll be up to the user to query the node to see if this node is attached and assign it as Some(T)
                internal_obj1: None,
            }
        }

        fn ready(&mut self) {
            godot_print!("my_rust_lib::Node2D_MyExtensionStructLib2::_ready(): 'godot-rust-hello_world' app");
            // Prints to the Godot console
        }
    }

    #[godot_api]
    impl Node2D_MyExtensionStructLib2 {
        #[func]
        pub fn locate_internal_obj1(&mut self) -> Option<Gd<my_rust_lib2::my_rust_lib2::MyStructForLib2>> {
            let path = NodePath::from_str("./MyStructForLib2".into()).unwrap(); // TODO: Make this a match statement instead of forcing to unwrap()
            let node = self.base().get_node(path);
            if node.is_none() {
                godot_print!("my_rust_lib::Node2D_MyExtensionStructLib2::locate_internal_obj1(): 'godot-rust-hello_world' app: failed to locate node");
                return None;
            }
            let node = node.unwrap();
            let node = node.try_cast::<my_rust_lib2::my_rust_lib2::MyStructForLib2>();
            if node.is_err() {
                godot_print!("my_rust_lib::Node2D_MyExtensionStructLib2::locate_internal_obj1(): 'godot-rust-hello_world' app: failed to cast node");
                return None;
            }
            let node = node.unwrap();
            self.internal_obj1 = Some(node);
            self.internal_obj1.clone()
        }

        #[func]
        pub fn get_child_node(&self) -> Option<Gd<my_rust_lib2::my_rust_lib2::MyStructForLib2>> {
            self.internal_obj1.clone()
        }

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
