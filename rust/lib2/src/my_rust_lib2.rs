// NOTE: This module is not meant to be used directly, but rather to be used as a library.
pub mod my_rust_lib2 {
    use godot::engine::Node;
    use godot::prelude::*;

    #[derive(GodotClass)]
    #[class(base=TileMap)]
    pub struct MyObjectTileMap {
        my_i64: i64,
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

}
