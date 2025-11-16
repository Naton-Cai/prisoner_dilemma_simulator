//! Game project.
#[allow(unused_imports)]
use fyrox::graph::prelude::*;
use fyrox::{
    core::{
        algebra::Vector2, color::Color, log::Log, pool::Handle, reflect::prelude::*,
        visitor::prelude::*,
    },
    event::Event,
    gui::{
        brush::Brush,
        button::{ButtonBuilder, ButtonMessage},
        message::UiMessage,
        text::TextBuilder,
        text_box::TextBoxBuilder,
        widget::WidgetBuilder,
        UiNode, UserInterface,
    },
    plugin::{Plugin, PluginContext, PluginRegistrationContext},
    scene::{
        base::BaseBuilder,
        dim2::{
            collider::{ColliderBuilder, ColliderShape},
            rigidbody::RigidBodyBuilder,
        },
        graph::Graph,
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
pub mod bugster_sprite;

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

        context
            .serialization_context
            .script_constructors
            .add::<bugster_sprite::BugsterSprite>("BugsterSprite");
    }

    fn init(&mut self, scene_path: Option<&str>, context: PluginContext) {
        //loads the main scene
        context
            .async_scene_loader
            .request(scene_path.unwrap_or("data/scene.rgs"));

        //load the bugster
        context
            .async_scene_loader
            .request(scene_path.unwrap_or("data/Scenes/busters.rgs"));
    }

    fn on_deinit(&mut self, _context: PluginContext) {
        // Do a cleanup here.
    }

    fn update(&mut self, context: &mut PluginContext) {}

    fn on_os_event(&mut self, _event: &Event<()>, _context: PluginContext) {
        // Do something on OS event here.
    }

    fn on_ui_message(
        &mut self,
        _context: &mut PluginContext,
        _message: &UiMessage,
        _ui_handle: Handle<UserInterface>,
    ) {
        // Handle UI events here.
    }

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
            add_bugster(context, scene, PersonalityType::Cooperative);
        }

        if path == Path::new("data/Scenes/bugster.rgs") {}
    }
}

//adds a textbox to the UI, returns the UI handle to be used later
fn add_textbox(context: &mut PluginContext) -> Handle<UiNode> {
    let mut ui = UserInterface::new(Vector2::new(1024.0, 768.0));
    let textbox =
        TextBoxBuilder::new(WidgetBuilder::new().with_foreground(Brush::Solid(Color::RED).into()))
            .with_text("10")
            .build(&mut ui.build_ctx());
    context.user_interfaces.add(ui);
    textbox
}

//creates the bugster
fn add_bugster(context: &mut PluginContext, scene: Handle<Scene>, personality: PersonalityType) {
    //create our rigid body
    let node_handle = {
        let graph = &mut context.scenes.try_get_mut(scene).unwrap().graph;

        RigidBodyBuilder::new(
            BaseBuilder::new().with_children(&[ColliderBuilder::new(BaseBuilder::new())
                .with_shape(ColliderShape::Cuboid(
                    fyrox::scene::dim2::collider::CuboidShape {
                        half_extents: Vector2::new(0.5, 0.5),
                    },
                ))
                .build(graph)]),
        )
        .with_mass(1.0)
        .with_lin_vel(Vector2::new(0.0, 0.0))
        .with_ang_damping(0.0)
        .with_gravity_scale(0.0)
        .with_rotation_locked(true)
        .with_can_sleep(false)
        .with_body_type(RigidBodyType::Dynamic)
        .build(graph)
    };

    //add out text box for each of our bugster
    let textbox = add_textbox(context);
    let graph = &mut context.scenes.try_get_mut(scene).unwrap().graph;
    graph[node_handle].add_script(Bugsters::new(10, personality, textbox));
}
