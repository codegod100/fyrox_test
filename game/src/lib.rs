use crate::player::Player;
use fyrox::{
    core::{pool::Handle, reflect::prelude::*, visitor::prelude::*},
    engine::GraphicsContext,
    event::{ElementState, Event, WindowEvent},
    gui::message::UiMessage,
    keyboard::{KeyCode, PhysicalKey},
    plugin::{Plugin, PluginContext, PluginRegistrationContext},
    scene::Scene,
    window::CursorGrabMode,
};

use std::path::Path;

pub mod player;
// pub mod weapon;

#[derive(Default, Visit, Reflect, Debug)]
pub struct Game {
    scene: Handle<Scene>,

    #[visit(optional)]
    #[reflect(hidden)]
    cursor_released: bool,
}

impl Plugin for Game {
    fn on_graphics_context_initialized(&mut self, context: PluginContext) {
        // At this stage it is safe to call `as_initialized_mut`, because graphics context is guaranteed
        // to be alive when this method is being called.
        let graphics_context = context.graphics_context.as_initialized_mut();
        let window = &graphics_context.window;
        window.set_title("My Cool Game");
        window
            .set_cursor_grab(CursorGrabMode::Confined).unwrap();
        window.set_cursor_visible(false);
    }

    fn register(&self, context: PluginRegistrationContext) {
        println!("Adding player");
        // Register your scripts here.
        context
            .serialization_context
            .script_constructors
            .add::<Player>("Player");
    }

    fn init(&mut self, scene_path: Option<&str>, context: PluginContext) {
        context
            .async_scene_loader
            .request(scene_path.unwrap_or("data/scene.rgs"));
    }

    fn on_deinit(&mut self, _context: PluginContext) {
        // Do a cleanup here.
    }

    fn update(&mut self, _context: &mut PluginContext) {
        // Add your global update code here.
    }

    fn on_os_event(&mut self, event: &Event<()>, context: PluginContext) {
        // Do something on OS event here.
        let graphics_context = context.graphics_context;
        // println!("OS EVENT!");
        if let GraphicsContext::Initialized(ctx) = graphics_context {
            // println!("GC INITIALIZAED");
            let window = &ctx.window;

            match event {
                // Raw mouse input is responsible for camera rotation.

                // Keyboard input is responsible for player's movement.
                Event::WindowEvent {
                    event: WindowEvent::KeyboardInput { event, .. },
                    ..
                } => {
                    if let PhysicalKey::Code(code) = event.physical_key {
                        match code {
                            KeyCode::KeyG => {
                                println!("Cursor is currently {}",self.cursor_released);
                                window.set_cursor_grab(CursorGrabMode::None).unwrap();
                                if self.cursor_released {
                                    // window.set_cursor_grab(CursorGrabMode::Confined).unwrap();
                                } else {
                                    // window.set_cursor_grab(CursorGrabMode::None).unwrap();
                                }
                                self.cursor_released = !self.cursor_released;
                            }

                            _ => (),
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn on_ui_message(&mut self, _context: &mut PluginContext, _message: &UiMessage) {
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
        _context: &mut PluginContext,
    ) {
        self.scene = scene;
    }
}
