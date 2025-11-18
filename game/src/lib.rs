//! Game project.
#[allow(unused_imports)]
use fyrox::graph::prelude::*;
use fyrox::{
    core::{
        algebra::{Vector2, Vector3},
        color::Color,
        log::Log,
        pool::Handle,
        reflect::prelude::*,
        visitor::prelude::*,
    },
    gui::UserInterface,
    plugin::{Plugin, PluginContext, PluginRegistrationContext},
    scene::{
        base::BaseBuilder,
        collider::{BitMask, InteractionGroups},
        dim2::{
            collider::{ColliderBuilder, ColliderShape},
            rigidbody::RigidBodyBuilder,
        },
        rigidbody::RigidBodyType,
        Scene,
    },
};

use std::path::Path;

// Re-export the engine.
pub use fyrox;

use crate::bugster::{Bugsters, PersonalityType};

//our scripts
pub mod bugster;

#[derive(Default, Visit, Reflect, Debug)]
#[reflect(non_cloneable)]
pub struct Game {
    scene: Handle<Scene>,
    ui: Handle<UserInterface>,
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
    }

    fn update(&mut self, context: &mut PluginContext) {}

    fn on_scene_begin_loading(&mut self, _path: &Path, ctx: &mut PluginContext) {}

    fn on_scene_loaded(
        &mut self,
        path: &Path,
        scene: Handle<Scene>,
        _data: &[u8],
        context: &mut PluginContext,
    ) {
        Log::info(format!("Path {}", path.display()).as_str());

        //set main scene
        if path == Path::new("data/scene.rgs") {
            self.scene = scene;
            //add our bugster to the scene
            add_bugster(context, scene, PersonalityType::Cooperative, 0.0, 0.0);
            add_bugster(context, scene, PersonalityType::Cooperative, 2.0, 3.0);
            add_bugster(context, scene, PersonalityType::Greedy, 2.0, 3.0);
        }

        if path == Path::new("data/Scenes/bugster.rgs") {}
    }
}

//creates the bugster
fn add_bugster(
    context: &mut PluginContext,
    scene: Handle<Scene>,
    personality: PersonalityType,
    x: f32,
    y: f32,
) {
    let graph = &mut context.scenes.try_get_mut(scene).unwrap().graph;

    let collision_body = ColliderBuilder::new(BaseBuilder::new())
        .with_shape(ColliderShape::Cuboid(
            fyrox::scene::dim2::collider::CuboidShape {
                half_extents: Vector2::new(0.5, 0.5),
            },
        ))
        .build(graph);
    let detector_body = ColliderBuilder::new(BaseBuilder::new())
        .with_shape(ColliderShape::Cuboid(
            fyrox::scene::dim2::collider::CuboidShape {
                half_extents: Vector2::new(0.75, 0.75),
            },
        ))
        .with_collision_groups(InteractionGroups::new(
            BitMask(0b0000_0000_0000_0000_0000_0000_0000_0010),
            BitMask(0b0000_0000_0000_0000_0000_0000_0000_0010),
        ))
        .with_sensor(true)
        .build(graph);

    //create our rigid body
    let node_handle =
        RigidBodyBuilder::new(BaseBuilder::new().with_children(&[collision_body, detector_body]))
            .with_mass(1.0)
            .with_lin_vel(Vector2::new(0.0, 0.0))
            .with_ang_damping(0.0)
            .with_gravity_scale(0.0)
            .with_rotation_locked(true)
            .with_can_sleep(false)
            .with_body_type(RigidBodyType::Dynamic)
            .build(graph);

    //then attach the script to it
    graph[node_handle].add_script(Bugsters::new(
        10,
        personality,
        collision_body,
        detector_body,
    ));

    //change the nodes position
    graph[node_handle]
        .local_transform_mut()
        .set_position(Vector3::new(x, y, 0.0));
}
