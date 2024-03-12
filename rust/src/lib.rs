use godot::engine::ISprite2D;
use godot::engine::Sprite2D;
use godot::prelude::*;
use std::str::FromStr;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Player {
    speed: f64,
    angular_speed: f64,

    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for Player {
    fn init(base: Base<Sprite2D>) -> Self {
        godot_print!("Hello, World!");

        Self {
            speed: 400.0,
            angular_speed: std::f64::consts::PI,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let input = Input::singleton();
        let mut direction = 0.0;
        if Input::is_action_pressed(&input, StringName::from_str("ui_left").unwrap()) {
            direction = -1.0;
        }
        if Input::is_action_pressed(&input, StringName::from_str("ui_right").unwrap()) {
            direction = 1.0;
        }

        let radians = (self.angular_speed * direction * delta) as f32;
        self.base_mut().rotate(radians);

        let rotation = self.base().get_rotation();
        let mut velocity = Vector2::ZERO;
        if Input::is_action_pressed(&input, StringName::from_str("ui_up").unwrap()) {
            velocity = Vector2::UP.rotated(rotation) * self.speed as f32;
        }

        self.base_mut().translate(velocity * delta as f32);
    }
}

#[godot_api]
impl Player {
    #[func]
    fn increase_speed(&mut self, amount: f64) {
        self.speed += amount;
        self.base_mut().emit_signal("speed_increased".into(), &[]);
    }

    #[signal]
    fn speed_increased();
}
