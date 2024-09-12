use crate::player::Player;
use fyrox::{
    core::{notify::EventKind, pool::Handle, reflect::prelude::*, uuid, visitor::prelude::*},
    engine::GraphicsContext,
    event::{ElementState, Event, WindowEvent},
    graph::SceneGraph,
    gui::{
        button::{ButtonBuilder, ButtonMessage},
        message::UiMessage,
        text::TextBuilder,
        widget::{WidgetBuilder, WidgetMessage},
        UiNode, UserInterface,
    },
    keyboard::{KeyCode, PhysicalKey},
    plugin::{Plugin, PluginContext, PluginRegistrationContext},
    scene::{node::Node, Scene},
    script::{constructor::ScriptConstructorContainer, ScriptTrait},
    window::CursorGrabMode,
};

use std::{path::Path, thread, time};

pub mod player;
// pub mod weapon;

enum Message {
    Forward,
}

#[derive(Default, Visit, Reflect, Debug)]
pub struct Game {
    scene: Handle<Scene>,

    #[visit(optional)]
    #[reflect(hidden)]
    cursor_released: bool,

    quit_button_handle: Handle<UiNode>,
    player: Handle<Node>,
}

fn create_quit_button(ui: &mut UserInterface) -> Handle<UiNode> {
    ButtonBuilder::new(WidgetBuilder::new().with_width(100.0).with_height(100.0))
        .with_content(
            TextBuilder::new(WidgetBuilder::new())
                .with_text("forward")
                .build(&mut ui.build_ctx()),
        )
        .build(&mut ui.build_ctx())
}

// impl Game {
//     fn init(&mut self, scene_path: Option<&str>, context: PluginContext) {
//         self.quit_button_handle =  create_quit_button(context.user_interfaces.first_mut());
//     }
// }

impl Plugin for Game {
    fn on_graphics_context_initialized(&mut self, context: PluginContext) {
        // At this stage it is safe to call `as_initialized_mut`, because graphics context is guaranteed
        // to be alive when this method is being called.
        let graphics_context = context.graphics_context.as_initialized_mut();
        let window = &graphics_context.window;
        window.set_title("My Cool Game");
        // window
        //     .set_cursor_grab(CursorGrabMode::Confined).unwrap();
        // window.set_cursor_visible(false);
    }

    fn register(&self, context: PluginRegistrationContext) {
        println!("Adding player");
        // Register your scripts here.
        let c = &context.serialization_context.script_constructors;

        c.add::<Player>("Player");

        let u = uuid!("62ae8f72-896e-412d-843c-3e24540e7f38");
        let Some(s) = c.try_create(&u) else {
            return;
        };
        println!("script: {:#?}", s);
        let Some(p) = s.cast::<Player>() else {
            return;
        };
        println!("Got player: {:#?}", p);
    }

    fn init(&mut self, scene_path: Option<&str>, context: PluginContext) {
        // println!("game init");
        self.quit_button_handle = create_quit_button(context.user_interfaces.first_mut());
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
                                println!("Cursor is currently {}", self.cursor_released);
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

    fn on_ui_message(&mut self, context: &mut PluginContext, message: &UiMessage) {
        println!("data {:#?}", message);
        if let Some(scene) = context.scenes.try_get_mut(self.scene) {
            let p = scene.graph.try_get_script_of_mut::<Player>(self.player).unwrap();
            // println!("{:#?}",p);
            println!("DT: {}",context.dt);
            // p.move_forward = true;

            // p.move_forward = false;
            if let Some(WidgetMessage::MouseDown { pos, button }) = message.data() {
                println!("MOUSE DOWN");
                p.move_forward = true;
            }
            if let Some(WidgetMessage::MouseUp { pos, button }) = message.data(){
                p.move_forward = false;
            }
        }
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
