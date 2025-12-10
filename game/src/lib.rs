//! Game project.
#[allow(unused_imports)]
use fyrox::graph::prelude::*;
use fyrox::{
    asset::{manager::ResourceManager, untyped::ResourceKind},
    core::{
        algebra::{Vector2, Vector3},
        log::Log,
        pool::Handle,
        reflect::prelude::*,
        uuid,
        visitor::prelude::*,
    },
    graph::SceneGraph,
    gui::{
        button::ButtonMessage, message::MessageDirection, numeric::NumericUpDown,
        text::TextMessage, texture::Texture, widget::WidgetMessage, UiNode, UserInterface,
    },
    material::{Material, MaterialResource},
    plugin::{Plugin, PluginContext, PluginRegistrationContext},
    renderer::QualitySettings,
    scene::{
        base::BaseBuilder,
        collider::{BitMask, InteractionGroups},
        dim2::{
            collider::{ColliderBuilder, ColliderShape},
            rectangle::RectangleBuilder,
            rigidbody::RigidBodyBuilder,
        },
        graph::Graph,
        node::Node,
        rigidbody::RigidBodyType,
        transform::TransformBuilder,
        Scene,
    },
};

use rand::random_range;
use std::path::Path;
// Re-export the engine.
pub use fyrox;

use crate::bugster::{Bugsters, PersonalityType};

//our scripts
pub mod bugster;

const MAX_X: f32 = 8.0;
const MAX_Y: f32 = 3.5;
const BASE_HEALTH: i64 = 10;
const BASE_SIZE: f32 = 0.5;

const COOPERATIVE_SPRITE_PATH: &str = "data/Sprites/bugster_cooperative.png";
const GREEDY_SPRITE_PATH: &str = "data/Sprites/bugster_greedy.png";

#[derive(Default, Visit, Reflect, Debug)]
#[reflect(non_cloneable)]
pub struct Game {
    pub coop_hp: i64,
    pub greed_hp: i64,
    pub coop_counter: Handle<UiNode>,
    pub greed_counter: Handle<UiNode>,
    scene: Handle<Scene>,
    start: Handle<UiNode>,
    exit: Handle<UiNode>,
    coop_field: Handle<UiNode>,
    greed_field: Handle<UiNode>,
}

impl Game {
    fn game_start(&mut self, context: &mut PluginContext, coop_count: i64, greed_count: i64) {
        //add our bugsters to the scene with random positions
        for _ in 0..coop_count {
            self.add_bugster(
                context,
                self.scene,
                PersonalityType::Cooperative,
                random_range(-MAX_X..=MAX_X),
                random_range(-MAX_Y..=MAX_Y),
            );
            //add the health of the bugster to the counter
            self.change_coop_hp(BASE_HEALTH);
            context
                .user_interfaces
                .first()
                .send_message(TextMessage::text(
                    self.coop_counter,
                    MessageDirection::ToWidget,
                    format!("Coop Total: {}", self.coop_hp).to_owned(),
                ));
        }
        for _ in 0..greed_count {
            self.add_bugster(
                context,
                self.scene,
                PersonalityType::Greedy,
                random_range(-MAX_X..=MAX_X),
                random_range(-MAX_Y..=MAX_Y),
            );
            self.change_greed_hp(BASE_HEALTH);
            context
                .user_interfaces
                .first()
                .send_message(TextMessage::text(
                    self.greed_counter,
                    MessageDirection::ToWidget,
                    format!("Greed Total: {}", self.greed_hp).to_owned(),
                ));
        }
    }

    pub fn change_coop_hp(&mut self, value: i64) {
        self.coop_hp += value
    }
    pub fn change_greed_hp(&mut self, value: i64) {
        self.greed_hp += value
    }

    //creates the bugster at a given position
    pub fn add_bugster(
        &mut self,
        context: &mut PluginContext,
        scene_handle: Handle<Scene>,
        personality: PersonalityType,
        x: f32,
        y: f32,
    ) {
        let scene = context
            .scenes
            .try_get_mut(scene_handle)
            .expect("Invalid scene handle");

        let graph = &mut scene.graph;

        //create the colliders for both collision and the hitbox detection
        let collision_body = ColliderBuilder::new(BaseBuilder::new())
            .with_shape(ColliderShape::Cuboid(
                fyrox::scene::dim2::collider::CuboidShape {
                    half_extents: Vector2::new(BASE_SIZE / 2.0, BASE_SIZE / 2.0),
                },
            ))
            .build(graph);

        let detector_body = ColliderBuilder::new(BaseBuilder::new())
            .with_shape(ColliderShape::Cuboid(
                fyrox::scene::dim2::collider::CuboidShape {
                    half_extents: Vector2::new(BASE_SIZE / 2.0, BASE_SIZE / 2.0),
                },
            ))
            .with_collision_groups(InteractionGroups::new(
                BitMask(0b0000_0000_0000_0000_0000_0000_0000_0001),
                BitMask(0b0000_0000_0000_0000_0000_0000_0000_0010),
            ))
            .with_sensor(true)
            .build(graph);

        let sprite = self.get_texture(&personality, graph, context.resource_manager);

        //create our rigid body and attach our colliders
        let node_handle = RigidBodyBuilder::new(BaseBuilder::new().with_children(&[
            collision_body,
            detector_body,
            sprite,
        ]))
        .with_mass(1.0)
        .with_lin_vel(Vector2::new(0.0, 0.0))
        .with_ang_damping(0.0)
        .with_gravity_scale(0.0)
        .with_rotation_locked(true)
        .with_can_sleep(false)
        .with_body_type(RigidBodyType::Dynamic)
        .build(graph);

        //then attach the script to it

        if let Some(node) = graph.try_get_mut(node_handle) {
            node.add_script(Bugsters::new(
                BASE_HEALTH,
                personality,
                node_handle,
                collision_body,
                detector_body,
            ));
            node.local_transform_mut()
                .set_position(Vector3::new(x, y, 0.0));
        }
    }

    //gets the texture of the bugster based on its personality type
    fn get_texture(
        &mut self,
        personality: &PersonalityType,
        graph: &mut Graph,
        resource_manager: &ResourceManager,
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
                    .with_local_position(Vector3::new(0.0, 0.0, 1.0))
                    .build(),
            ),
        )
        .with_material(material_resource)
        .build(graph)
    }
}

impl Plugin for Game {
    fn register(&self, context: PluginRegistrationContext) {
        context
            .serialization_context
            .script_constructors
            .add::<bugster::Bugsters>("Bugsters");
    }

    fn init(&mut self, scene_path: Option<&str>, context: PluginContext) {
        //loads the main scene
        context
            .async_scene_loader
            .request(scene_path.unwrap_or("data/scene.rgs"));

        context
            .user_interfaces
            .add(UserInterface::new(Vector2::new(1024.0, 768.0)));

        context.task_pool.spawn_plugin_task(
            UserInterface::load_from_file("data/UI/start.ui", context.resource_manager.clone()),
            |result, game: &mut Game, ctx| {
                *ctx.user_interfaces.first_mut() = result.unwrap();
                game.start = ctx
                    .user_interfaces
                    .first()
                    .find_handle_by_name_from_root("Start");
                game.exit = ctx
                    .user_interfaces
                    .first()
                    .find_handle_by_name_from_root("Exit");
                game.coop_field = ctx
                    .user_interfaces
                    .first()
                    .find_handle_by_name_from_root("Coop");
                game.greed_field = ctx
                    .user_interfaces
                    .first()
                    .find_handle_by_name_from_root("Greed");
                game.coop_counter = ctx
                    .user_interfaces
                    .first()
                    .find_handle_by_name_from_root("CoopCount");
                game.greed_counter = ctx
                    .user_interfaces
                    .first()
                    .find_handle_by_name_from_root("GreedyCount");
            },
        );
    }

    fn on_graphics_context_initialized(&mut self, context: PluginContext) {
        let graphics_context = context.graphics_context.as_initialized_mut();
        let mut settings = QualitySettings::low();

        settings.use_ssao = false;
        settings.fxaa = false;
        Log::verify(graphics_context.renderer.set_quality_settings(&settings));
        graphics_context.window.set_title("Prisoner Dilemma");
    }

    fn on_ui_message(
        &mut self,
        context: &mut PluginContext,
        message: &fyrox::gui::message::UiMessage,
        _ui_handle: Handle<UserInterface>,
    ) {
        if let Some(ButtonMessage::Click) = message.data() {
            if message.destination() == self.start {
                let coop_count = *context
                    .user_interfaces
                    .first_mut()
                    .try_get_mut_of_type::<NumericUpDown<i64>>(self.coop_field)
                    .unwrap()
                    .value;
                let greed_count = *context
                    .user_interfaces
                    .first_mut()
                    .try_get_mut_of_type::<NumericUpDown<i64>>(self.greed_field)
                    .unwrap()
                    .value;

                self.game_start(context, coop_count, greed_count);
                context
                    .user_interfaces
                    .first()
                    .send_message(WidgetMessage::visibility(
                        self.start,
                        MessageDirection::ToWidget,
                        false,
                    ));
            } else if message.destination() == self.exit {
                context.loop_controller.exit();
            }
        }
    }

    fn on_scene_loaded(
        &mut self,
        path: &Path,
        scene: Handle<Scene>,
        _data: &[u8],
        _context: &mut PluginContext,
    ) {
        Log::info(format!("Path {}", path.display()).as_str());

        //set main scene
        if path == Path::new("data/scene.rgs") {
            self.scene = scene;
        }

        if path == Path::new("data/Scenes/bugster.rgs") {}
    }
}
