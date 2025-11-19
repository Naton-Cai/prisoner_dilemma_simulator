use fyrox::{
    asset::{manager::ResourceManager, untyped::ResourceKind},
    core::{
        algebra::{Vector2, Vector3},
        log::{self, Log},
        pool::Handle,
        reflect::prelude::*,
        type_traits::prelude::*,
        uuid::{self, timestamp::context},
        visitor::prelude::*,
    },
    graph::{BaseSceneGraph, SceneGraph},
    gui::texture::Texture,
    material::{Material, MaterialResource},
    scene::{
        base::BaseBuilder,
        dim2::{
            collider::Collider,
            rectangle::RectangleBuilder,
            rigidbody::{self, RigidBody},
        },
        graph::Graph,
        node::Node,
        transform::TransformBuilder,
    },
    script::{ScriptContext, ScriptTrait},
};

const COOPERATIVE_SPRITE_PATH: &str = "data/sprites/bugster_cooperative.png";
const GREEDY_SPRITE_PATH: &str = "data/sprites/bugster_greedy.png";
const MAX_SPEED: f32 = 15.0;
const MAX_WAIT_TIME: f32 = 5.0;
const MIN_WAIT_TIME: f32 = 3.0;

//our values to calcuate health gain
const GREEDGREED_HEALTH_GAIN: i64 = -1;
const GREEDCOOP_HEALTH_GAIN: i64 = 3;
const COOPGREED_HEALTH_GAIN: i64 = -2;
const COOPCOOP_HEALTH_GAIN: i64 = 1;

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
            collision_change_interval: 0.1,
            rigid_body_handle: rigid_body,
            collision_handle: collision,
            detector_handle: detector,
        }
    }

    //checks for entity collision
    fn entity_contact(&mut self, context: &mut ScriptContext) {
        //gets all intersected colliders
        let intersections: Vec<_> = {
            let graph = &context.scene.graph;

            let Some(collider) = graph.try_get_of_type::<Collider>(self.detector_handle) else {
                return;
            };

            collider
                .intersects(&graph.physics2d)
                .filter(|i| i.has_any_active_contact)
                .collect()
        };

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

            //Log::info(format!(
            //    "COLLIDE {:?}, {:?}, {:?}, {:?}",
            //    intersection, self.detector_handle, self.collision_handle, collided
            //));

            if let Some(script) = context
                .scene
                .graph
                .try_get_script_of_mut::<Bugsters>(parent_rigid)
            {
                script.health_calculation(self.personality.clone());
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
            //use the direction apply a knockback force that knocks the two nodes away from eachother
            rigid_body.apply_impulse(Vector2::new(direction.x * -7.0, direction.y * -7.0));
        }
    }

    //calculates the health changes when contacting another bugster
    pub fn health_calculation(&mut self, contact_personality: PersonalityType) {
        Log::info(format!("Current Hp {}", self.healthpoints));
        let personality = &self.personality;
        match (personality, contact_personality) {
            (PersonalityType::Greedy, PersonalityType::Greedy) => {
                self.healthpoints += GREEDGREED_HEALTH_GAIN;
                Log::info(format!(
                    "Bugster gained {}, Current Hp {}",
                    GREEDGREED_HEALTH_GAIN, self.healthpoints
                ));
            }
            (PersonalityType::Greedy, PersonalityType::Cooperative) => {
                self.healthpoints += GREEDCOOP_HEALTH_GAIN;
                Log::info(format!(
                    "Bugster gained {}, Current Hp {}",
                    GREEDCOOP_HEALTH_GAIN, self.healthpoints
                ));
            }
            (PersonalityType::Cooperative, PersonalityType::Greedy) => {
                self.healthpoints += COOPGREED_HEALTH_GAIN;
                Log::info(format!(
                    "Bugster gained {}, Current Hp {}",
                    COOPGREED_HEALTH_GAIN, self.healthpoints
                ));
            }
            (PersonalityType::Cooperative, PersonalityType::Cooperative) => {
                self.healthpoints += COOPCOOP_HEALTH_GAIN;
                Log::info(format!(
                    "Bugster gained {}, Current Hp {}",
                    COOPCOOP_HEALTH_GAIN, self.healthpoints
                ));
            }
        }
    }

    //sets the texture of the bugster based on its personality type
    fn set_texture(
        &mut self,
        personality: PersonalityType,
        graph: &mut Graph,
        resource_manager: ResourceManager,
    ) -> Handle<Node> {
        let mut material = Material::standard_2d();
        match personality {
            PersonalityType::Cooperative => {
                material.bind(
                    "diffuseTexture",
                    Some(resource_manager.request::<Texture>(COOPERATIVE_SPRITE_PATH)),
                );
                Log::info("Set sprite to cooperative texture");
            }
            PersonalityType::Greedy => {
                material.bind(
                    "diffuseTexture",
                    Some(resource_manager.request::<Texture>(GREEDY_SPRITE_PATH)),
                );
                Log::info("Set sprite to greedy texture");
            }
        }

        let material_resource = MaterialResource::new_ok(
            uuid::Uuid::new_v4(), // Generate a random UUID for the resource
            ResourceKind::Embedded,
            material,
        );

        RectangleBuilder::new(
            BaseBuilder::new().with_local_transform(
                TransformBuilder::new()
                    // Size of the rectangle is defined only by scale.
                    .with_local_scale(Vector3::new(1.0, 1.0, 1.0))
                    .with_local_position(Vector3::new(0.0, 0.0, 0.0))
                    .build(),
            ),
        )
        .with_material(material_resource)
        .build(graph)
    }
}

impl ScriptTrait for Bugsters {
    fn on_init(&mut self, _context: &mut ScriptContext) {}

    fn on_start(&mut self, context: &mut ScriptContext) {
        //set the texture on start
        let parent_handle = context.handle;
        let child_handle = self.set_texture(
            self.personality.clone(),
            &mut context.scene.graph,
            context.resource_manager.clone(),
        );

        //link the sprite as a child of the bugster node
        context.scene.graph.link_nodes(child_handle, parent_handle);
    }

    fn on_update(&mut self, context: &mut ScriptContext) {
        //check for collision
        if self.collision_time_since_last_change >= self.collision_change_interval {
            self.entity_contact(context);
            self.collision_time_since_last_change = 0.0;

            //if hp drops to 0, remove this node
            if self.healthpoints <= 0 {
                context.scene.graph.remove_node(self.rigid_body_handle);
            }
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
            self.x_speed = rand::random_range(-self.speed..=self.speed);
            self.y_speed = rand::random_range(-1.0..=1.0) * (self.speed - self.x_speed.abs());
            //reset the timer
            self.move_time_since_last_change = 0.0;
            //set a new random change interval
            self.move_change_interval = rand::random_range(MIN_WAIT_TIME..=MAX_WAIT_TIME);

            //apply the new speeds as an impulse to the rigid body
            rigid_body.apply_impulse(Vector2::new(self.x_speed, self.y_speed));
            //log the new speeds
            //Log::info(format!("Bugster X_speed: {}", self.x_speed));
            //Log::info(format!("Bugster Y_speed: {}", self.y_speed));
        }
    }
}
