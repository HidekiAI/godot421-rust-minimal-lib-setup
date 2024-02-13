// NOTE: This module is not meant to be used directly, but rather to be used as a library.
pub mod my_rust_lib {
    use std::collections::HashMap;

    use godot::engine::Node;
    use godot::prelude::*;

    // primitive struct that has no impl (OK, it does have new())
    // uses Node
    #[derive(GodotClass)]
    #[class(base=Node)]
    pub struct MyStruct {
        base: Base<Node>,
        //#[var]  // Do we really need this (this lib/module does not have Godot entry-point, so it'll never show up in the Inspector)
        #[init(default = 666)]
        my_i64: i64,

        #[init(default = 777.0)]
        my_f64: f64,

        // Q: Should we use GString (godot::builtin::GString) instead here?  I recall reading something about using into() method to allow &str as param for GString conversion?
        #[init(default = (HashMap::<i64,String>::new()))]
        my_dictionary: HashMap<i64, String>,

        // an example in which it HasA Node2D
        #[init(default = None)] // don't do Some(Node2D::new()) here, even if it is possible, because we're not going to be sure of race-condition
        my_node_2d: Option<Node2D>,
    }

    #[godot_api]
    impl INode for MyStruct {
        fn init(base: Base<Node>) -> Self {
            Self {
                base,
                my_i64: 0,
                my_f64: 0.0,
                my_dictionary: HashMap::<i64,String>::new(),
                my_node_2d: None,   // assume it inits as None!
            }
        }
    }

    impl MyStruct {
        // NOTE: Do not impl 'pub fn new() -> Self ' because this struct HasA Node2D, hence you have to rely on init() to create the struct
        pub fn set_position(&mut self, pos: Vector2) {
            match self.my_node_2d {
                None => (), // noop
                Some(_) => {
                    self.my_node_2d.as_mut().unwrap().set_position(pos);
                },
            }
        }
        pub fn get_position(&self) -> Option<Vector2> {
            match self.my_node_2d {
                None => None,
                Some(_) => {
                    Some(self.my_node_2d.as_ref().unwrap().get_position())
                },
            }
        }
    }
}
