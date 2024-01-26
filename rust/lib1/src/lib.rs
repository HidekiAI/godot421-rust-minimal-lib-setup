use godot::prelude::*;
use godot::engine::Node2D;
use godot::register;

#[derive(GodotClass)]
#[class(init, base = Node2D)]
struct MyExtensionStructLib1 {
    base: Base<Node2D>,

    #[var]
    my_i64: i64,

    #[var]
    my_f64: f64,
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