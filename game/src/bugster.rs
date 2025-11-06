use fyrox::{
    core::{
        algebra::Vector2, log::Log, reflect::prelude::*, type_traits::prelude::*,
        visitor::prelude::*,
    },
    event::Event,
    scene::dim2::rigidbody::RigidBody,
    script::{ScriptContext, ScriptDeinitContext, ScriptTrait},
};

//our enum that determines the personality type of our bugster
#[derive(Visit, Reflect, Debug, Clone)]
pub enum PersonalityType {
    Greedy,
    Cooperative,
}

impl Default for PersonalityType {
    fn default() -> Self {
        PersonalityType::Cooperative
    }
}

#[derive(Visit, Reflect, Default, Debug, Clone, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "9b4ca1b0-d66b-472e-9dcc-8700d6a55b55")]
#[visit(optional)]

pub struct Bugsters {
    pub healthpoints: i64,
    pub personality: PersonalityType,
    speed: f32,
    x_speed: f32,
    y_speed: f32,
    time_since_last_change: f32,
    change_interval: f32,
}

impl ScriptTrait for Bugsters {
    fn on_init(&mut self, context: &mut ScriptContext) {
        self.healthpoints = 10;
        self.speed = 2.0;
        self.x_speed = 0.0;
        self.y_speed = 0.0;
        self.time_since_last_change = 2.0;
        self.change_interval = 1.0;
    }

    fn on_start(&mut self, context: &mut ScriptContext) {
        // There should be a logic that depends on other scripts in scene.
        // It is called right after **all** scripts were initialized.
    }

    fn on_deinit(&mut self, context: &mut ScriptDeinitContext) {
        // Put de-initialization logic here.
    }

    fn on_os_event(&mut self, event: &Event<()>, context: &mut ScriptContext) {
        // Respond to OS events here.
    }

    fn on_update(&mut self, context: &mut ScriptContext) {
        self.time_since_last_change += context.dt;

        if let Some(rigid_body) = context.scene.graph[context.handle].cast_mut::<RigidBody>() {
            //when the time since last change exceeds the change interval, change direction
            if self.time_since_last_change >= self.change_interval {
                //randomly generate new x and y speeds within the speed limit
                self.x_speed = rand::random_range(-self.speed..=self.speed);
                self.y_speed = rand::random_range(-1.0..=1.0) * (self.speed - self.x_speed.abs());
                self.time_since_last_change = 0.0;
                Log::info(format!("Bugster X_speed: {}", self.x_speed).as_str());
                Log::info(format!("Bugster Y_speed: {}", self.y_speed).as_str());
            }
            //keep the bugster moving
            rigid_body.set_lin_vel(Vector2::new(self.x_speed, self.y_speed));
        }
    }
}
