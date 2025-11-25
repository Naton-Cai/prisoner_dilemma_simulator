use fyrox::{
    core::{
        algebra::Vector2, log::Log, pool::Handle, reflect::prelude::*, type_traits::prelude::*,
        visitor::prelude::*,
    },
    graph::{BaseSceneGraph, SceneGraph},
    gui::{message::MessageDirection, text::TextMessage, widget::WidgetMessage},
    scene::{
        dim2::{collider::Collider, rigidbody::RigidBody},
        node::Node,
    },
    script::{ScriptContext, ScriptTrait},
};
use rand::random_range;
use std::cmp;

use crate::Game;

const MAX_SPEED: f32 = 15.0;
const MAX_WAIT_TIME: f32 = 5.0;
const MIN_WAIT_TIME: f32 = 3.0;

//our values to calcuate health gain
const GREEDGREED_HEALTH_GAIN: i64 = -1;
const GREEDCOOP_HEALTH_GAIN: i64 = 3;
const COOPGREED_HEALTH_GAIN: i64 = -2;
const COOPCOOP_HEALTH_GAIN: i64 = 2;

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
    move_time_since_last_change: f32,
    move_change_interval: f32,
    collision_time_since_last_change: f32,
    collision_change_interval: f32,
    rigid_body_handle: Handle<Node>,
    collision_handle: Handle<Node>,
    detector_handle: Handle<Node>,
}

impl Bugsters {
    //create a new bugster with the passed in args
    pub fn new(
        healthpoints: i64,
        personality: PersonalityType,
        rigid_body: Handle<Node>,
        collision: Handle<Node>,
        detector: Handle<Node>,
    ) -> Self {
        Self {
            healthpoints,
            personality,
            speed: MAX_SPEED,
            x_speed: 0.0,
            y_speed: 0.0,
            move_time_since_last_change: 2.0,
            move_change_interval: 1.0,
            collision_time_since_last_change: 1.0,
            collision_change_interval: 0.5,

            rigid_body_handle: rigid_body,
            collision_handle: collision,
            detector_handle: detector,
        }
    }

    //checks for entity collision
    fn entity_contact(&mut self, context: &mut ScriptContext) {
        //gets all intersected colliders
        let graph = &mut context.scene.graph;
        let Some(detector) = graph.try_get_of_type::<Collider>(self.detector_handle) else {
            return;
        };

        let intersections: Vec<_> = detector
            .intersects(&graph.physics2d)
            .filter(|i| i.has_any_active_contact)
            .collect();
        for intersection in intersections {
            //get the collider that this collider interesected
            let collided = if self.detector_handle == intersection.collider1 {
                intersection.collider2
            } else {
                intersection.collider1
            };

            //gets the parent of the collider which should be a rigid body
            let parent_rigid = {
                let graph = &context.scene.graph;
                if let Some(collider) = graph.try_get_of_type::<Collider>(collided) {
                    collider.parent()
                } else {
                    return;
                }
            };

            //gets the personality of  the contacted bugster from its script then apply the health change to our bugster
            if let Some(script) = context
                .scene
                .graph
                .try_get_script_of_mut::<Bugsters>(parent_rigid)
            {
                let health_change = self.health_calculation(&script.personality);
                let actual_health_change = cmp::max(health_change, self.healthpoints * -1); //calcuates the actual amount lost, accounting for health dropping to 0
                self.healthpoints += health_change;

                //apply the change to the overall game counters
                let game = context.plugins.get_mut::<Game>();
                match self.personality {
                    PersonalityType::Greedy => {
                        game.change_greed_hp(actual_health_change);
                        context
                            .user_interfaces
                            .first()
                            .send_message(TextMessage::text(
                                game.greed_counter,
                                MessageDirection::ToWidget,
                                format!("Greed Total: {}", game.greed_hp).to_owned(),
                            ));
                    }
                    PersonalityType::Cooperative => {
                        game.change_coop_hp(actual_health_change);
                        context
                            .user_interfaces
                            .first()
                            .send_message(TextMessage::text(
                                game.coop_counter,
                                MessageDirection::ToWidget,
                                format!("Coop Total: {}", game.coop_hp).to_owned(),
                            ));
                    }
                }

                //if hp drops to 0, remove this node
                if self.healthpoints <= 0 {
                    context.scene.graph.remove_node(self.rigid_body_handle);
                }
            } else {
                Log::err("NO BUGSTER SCRIPT FOUND");
            }

            //get the direction of where the the two colliders touch
            let direction = {
                let graph = &context.scene.graph;
                let contracted_node = match graph.try_get(collided) {
                    Some(node) => node,
                    None => return,
                };
                let self_node = match graph.try_get(self.detector_handle) {
                    Some(node) => node,
                    None => return,
                };

                let self_position = self_node.global_position();
                let contracted_position = contracted_node.global_position();

                contracted_position - self_position
            };

            let Some(rigid_body) = context
                .scene
                .graph
                .try_get_mut_of_type::<RigidBody>(self.rigid_body_handle)
            else {
                Log::info("Not a Rigid Body!");
                return;
            };
            //use the direction to apply a knockback force that knocks the two nodes away from eachother
            rigid_body.apply_impulse(Vector2::new(direction.x * -6.0, direction.y * -6.0));
        }
    }

    //calculates the health changes when contacting another bugster
    pub fn health_calculation(&mut self, contact_personality: &PersonalityType) -> i64 {
        match (&self.personality, contact_personality) {
            (PersonalityType::Greedy, PersonalityType::Greedy) => GREEDGREED_HEALTH_GAIN,
            (PersonalityType::Greedy, PersonalityType::Cooperative) => GREEDCOOP_HEALTH_GAIN,
            (PersonalityType::Cooperative, PersonalityType::Greedy) => COOPGREED_HEALTH_GAIN,
            (PersonalityType::Cooperative, PersonalityType::Cooperative) => COOPCOOP_HEALTH_GAIN,
        }
    }
}

impl ScriptTrait for Bugsters {
    fn on_init(&mut self, _context: &mut ScriptContext) {}

    fn on_update(&mut self, context: &mut ScriptContext) {
        //check for collision
        if self.collision_time_since_last_change >= self.collision_change_interval {
            self.collision_time_since_last_change = 0.0;
            self.entity_contact(context);
        }

        let Some(rigid_body) = context
            .scene
            .graph
            .try_get_mut_of_type::<RigidBody>(self.rigid_body_handle)
        else {
            Log::info("Not a Rigid Body!");
            return;
        };

        self.move_time_since_last_change += context.dt;
        self.collision_time_since_last_change += context.dt;

        //when the time since last change exceeds the change interval, change direction and apply impulse
        if self.move_time_since_last_change >= self.move_change_interval {
            //randomly generate new x and y speeds within the speed limit
            self.x_speed = random_range(-self.speed..=self.speed);
            self.y_speed = random_range(-1.0..=1.0) * (self.speed - self.x_speed.abs());
            //reset the timer
            self.move_time_since_last_change = 0.0;
            //set a new random change interval
            self.move_change_interval = random_range(MIN_WAIT_TIME..=MAX_WAIT_TIME);

            //apply the new speeds as an impulse to the rigid body
            rigid_body.apply_impulse(Vector2::new(self.x_speed, self.y_speed));
        }
    }
}
