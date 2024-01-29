use godot::prelude::*;
use shared_internal_lib; // shared crate

mod my_rust_lib;

// CRITICAL: __MUST__ expose entry point or else you will get the error:
// "GDExtension entry point 'gdext_rust_init' not found in library ..."
// NOTE that there are ONE and ONLY ONE entry point per library.
#[gdextension]
unsafe impl ExtensionLibrary for entry_point::Node_MyExtensionStructLib1 {}

// Some reminders:
// * Do not instantiate (HasA pattern) any Godot types in the struct, but rather create an
//   Option<T> variable, in which you would query for (neighbor/sibling, parent, children)
//   nodes and dynamically cast it when found
// * If you need to add a node to the scene tree that this struct depends on, then you'll
//   need to refactor that node so that it is a library of its own and expose its entry point
pub mod entry_point {
    use crate::my_rust_lib;
    use godot::engine::Node;
    use godot::prelude::*;

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
        // NOTE: What is VERY interesting is that by exposing it as Option<>, MyStructNode2DForLib1 becomes visible and available in SceneTree so that you can add this as a (child or sibling) Node in the editor
        // Alternatively, you can do `Gd::from_init_fn(|my_base| {...})` and hold Gd<T>
        internal_obj4: Option<Gd<my_rust_lib::my_rust_lib::MyStructNode2DForLib1>>, // Note that my_rust_lib::MyStructNode2D does not have an entry point, so it's not exposed to the edtitor
    }

    #[godot_api]
    impl INode for Node_MyExtensionStructLib1 {
        fn init(base: Base<Node>) -> Self {
            godot_print!(
                "my_rust_lib::Node_MyExtensionStructLib1::init(): 'godot-rust-hello_world' app"
            ); // Prints to the Godot console

            Self {
                base,
                my_i64: 0,
                my_f64: 0.0,
                internal_i64: 0,
                internal_obj1: my_rust_lib::my_rust_lib::MyStruct::new(Some(1), Some(3.0)),
                internal_obj2: my_rust_lib::my_rust_lib::MyStruct2::new(Some(1), Some(3.0)),
                internal_obj3: my_rust_lib::my_rust_lib::MyStruct3::new(Some(1), Some(3.0)),
                internal_obj4: None,
            }
        }

        fn ready(&mut self) {
            godot_print!(
                "my_rust_lib::Node_MyExtensionStructLib1::_ready(): 'godot-rust-hello_world' app"
            );
            // Prints to the Godot console
        }
    }

    #[godot_api]
    impl Node_MyExtensionStructLib1 {
        #[func]
        pub fn locate_obj4(&mut self) -> Option<Gd<my_rust_lib::my_rust_lib::MyStructNode2DForLib1>> {
            //let child2D = self.base().get_node_as::<Node2D>("Child");
            let class_name: GString = "MyStructNode2DForLib1".into();

            // iterate through Node::get_children() and find the node that matches the type
            for child_node in self.base().get_children().iter_shared() {
                if child_node.get_class() == class_name {
                    let child_node = child_node.cast::<my_rust_lib::my_rust_lib::MyStructNode2DForLib1>();
                    self.internal_obj4 = Some(child_node);

                    // opt out on first found, assume other children are not of the same type
                    break;
                }
            }
            self.internal_obj4.clone()
        }

        #[func]
        pub fn get_obj4_child_node(
            &self,
        ) -> Option<Gd<my_rust_lib::my_rust_lib::MyStructNode2DForLib1>> {
            self.internal_obj4.clone()  // feels weird (and wrong) cloning a ref-count-based smart pointer
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
