use fyrox::{
    asset::{manager::ResourceManager, untyped::ResourceKind},
    core::{
        algebra::{Vector2, Vector3},
        log::Log,
        pool::Handle,
        reflect::prelude::*,
        type_traits::prelude::*,
        uuid,
        visitor::prelude::*,
    },
    gui::{texture::Texture, UiNode},
    material::{Material, MaterialResource},
    scene::{
        base::BaseBuilder,
        dim2::{
            rectangle::{Rectangle, RectangleBuilder},
            rigidbody::RigidBody,
        },
        graph::Graph,
        node::Node,
        transform::TransformBuilder,
    },
    script::{ScriptContext, ScriptTrait},
};
const COOPERATIVE_SPRITE_PATH: &str = "data/sprites/bugster_cooperative.png";
const GREEDY_SPRITE_PATH: &str = "data/sprites/bugster_greedy.png";
const MAX_SPEED: f32 = 3.0;
const MAX_WAIT_TIME: f32 = 5.0;
const MIN_WAIT_TIME: f32 = 3.0;

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
    pub ui_handle: Handle<UiNode>, //keeps track of the UI health element
    speed: f32,
    x_speed: f32,
    y_speed: f32,
    time_since_last_change: f32,
    change_interval: f32,
}

impl Bugsters {
    //create a new bugster with the passed in args
    pub fn new(healthpoints: i64, personality: PersonalityType, ui_handle: Handle<UiNode>) -> Self {
        Self {
            healthpoints,
            personality,
            ui_handle,
            speed: MAX_SPEED,
            x_speed: 0.0,
            y_speed: 0.0,
            time_since_last_change: 2.0,
            change_interval: 1.0,
        }
    }
}

impl ScriptTrait for Bugsters {
    fn on_init(&mut self, _context: &mut ScriptContext) {}

    fn on_start(&mut self, context: &mut ScriptContext) {
        //set the texture on start
        let parent_handle = context.handle;
        let child_handle = set_texture(
            self.personality.clone(),
            &mut context.scene.graph,
            context.resource_manager.clone(),
        );
        //link the sprite as a child of the bugster node
        //for some reason link_nodes is not accessible here, so using link_nodes_keep_global_transform instead
        context
            .scene
            .graph
            .link_nodes_keep_global_transform(child_handle, parent_handle);

        //set the sprite position to (0,0,0) relative to the bugster node
        if let Some(rectangle) = context.scene.graph[child_handle].cast_mut::<Rectangle>() {
            rectangle
                .local_transform_mut()
                .set_position(Vector3::new(0.0, 0.0, 0.0));
        }
    }

    fn on_update(&mut self, context: &mut ScriptContext) {
        self.time_since_last_change += context.dt;

        if let Some(rigid_body) = context.scene.graph[context.handle].cast_mut::<RigidBody>() {
            //when the time since last change exceeds the change interval, change direction and apply impulse
            if self.time_since_last_change >= self.change_interval {
                //randomly generate new x and y speeds within the speed limit
                self.x_speed = rand::random_range(-self.speed..=self.speed);
                self.y_speed = rand::random_range(-1.0..=1.0) * (self.speed - self.x_speed.abs());
                //reset the timer
                self.time_since_last_change = 0.0;
                //set a new random change interval
                self.change_interval = rand::random_range(MIN_WAIT_TIME..=MAX_WAIT_TIME);
                //log the new speeds
                Log::info(format!("Bugster X_speed: {}", self.x_speed).as_str());
                Log::info(format!("Bugster Y_speed: {}", self.y_speed).as_str());

                //apply the new speeds as an impulse to the rigid body
                rigid_body.apply_impulse(Vector2::new(self.x_speed, self.y_speed));
            }
        }
    }
}

//sets the texture of the bugster based on its personality type
fn set_texture(
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
