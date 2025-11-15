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
    scene::Scene,
};

use std::path::Path;

// Re-export the engine.
pub use fyrox;

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

    fn on_scene_begin_loading(&mut self, _path: &Path, ctx: &mut PluginContext) {
        if self.scene.is_some() {
            ctx.scenes.remove(self.scene);
        }
    }

    fn on_scene_loaded(
        &mut self,
        _path: &Path,
        scene: Handle<Scene>,
        _data: &[u8],
        context: &mut PluginContext,
    ) {
        self.scene = scene;
    }
}

//adds a textbox to the UI, returns the UI handle to be used later
fn add_textbox(context: &mut PluginContext) -> Handle<UserInterface> {
    let mut ui = UserInterface::new(Vector2::new(1024.0, 768.0));
    let textbox =
        TextBoxBuilder::new(WidgetBuilder::new().with_foreground(Brush::Solid(Color::RED).into()))
            .with_text("10")
            .build(&mut ui.build_ctx());
    context.user_interfaces.add(ui)
}
