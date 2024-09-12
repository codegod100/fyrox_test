use crate::player::Player;
use fyrox::{
    core::{
        pool::Handle, reflect::prelude::*,
        visitor::prelude::*,
    },
    engine::GraphicsContext,
    event::{Event, WindowEvent},
    gui::{
        message::UiMessage,
        UiNode,
    },
    keyboard::{KeyCode, PhysicalKey},
    plugin::{Plugin, PluginContext, PluginRegistrationContext},
    scene::{node::Node, Scene},
    window::CursorGrabMode,
};

use std::{
    path::Path,
    time::{self},
};

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

    forward_button: Handle<UiNode>,
    backward_button: Handle<UiNode>,

    player: Handle<Node>,

    secs: f64,
}

#[derive(Debug)]
enum Direction {
    Forward,
    Back,
    Empty,
}

// fn timer() -> f64 {
//     SystemTime::now()
//         .duration_since(UNIX_EPOCH)
//         .unwrap()
//         .as_secs_f64()
// }

impl Game {
    fn direction(&self, dest: Handle<UiNode>) -> Direction {
        if dest == self.forward_button {
            return Direction::Forward;
        }
        if dest == self.backward_button {
            return Direction::Back;
        }
        return Direction::Empty;
    }
}

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
    }

    fn init(&mut self, scene_path: Option<&str>, context: PluginContext) {
        // println!("game init");
        // self.quit_button_handle = create_quit_button(context.user_interfaces.first_mut());
        let ui = context.user_interfaces.first_mut();
        let ctx = &mut ui.build_ctx();
        // self.forward_button = ButtonBuilder::new(
        //     WidgetBuilder::new()
        //         .with_width(400.0)
        //         .with_height(400.0)
        //         .with_desired_position(Vector2::new(50.0, 300.0)),
        // )
        // .with_repeat_clicks_on_hold(true)
        // .with_content(
        //     TextBuilder::new(WidgetBuilder::new())
        //         .with_text("forward")
        //         .build(ctx),
        // )
        // .build(ctx);

        // self.backward_button = ButtonBuilder::new(
        //     WidgetBuilder::new()
        //         .with_width(400.0)
        //         .with_height(400.0)
        //         .with_desired_position(Vector2::new(50.0, 700.0)),
        // )
        // .with_repeat_clicks_on_hold(true)
        // .with_content(
        //     TextBuilder::new(WidgetBuilder::new())
        //         .with_text("backwards")
        //         .build(ctx),
        // )
        // .build(ctx);

        context
            .async_scene_loader
            .request(scene_path.unwrap_or("data/scene.rgs"));
    }

    fn on_deinit(&mut self, _context: PluginContext) {
        // Do a cleanup here.
    }

    fn update(&mut self, context: &mut PluginContext) {
        // Add your global update code here.
        if let Some(scene) = context.scenes.try_get_mut(self.scene) {
            let p = scene
                .graph
                .try_get_script_of_mut::<Player>(self.player);
            if p.is_none(){
            return
            }
            let p = p.unwrap();
            // let now = timer();
            // let diff = now - self.secs;
            // println!("diff {}", diff);
            // if diff > 0.12 {
            //     p.move_backward = false;
            //     p.move_forward = false;
            // }
        }
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
        if let Some(scene) = context.scenes.try_get_mut(self.scene) {
            let p = scene
                .graph
                .try_get_script_of_mut::<Player>(self.player)
                .unwrap();

            let dest = message.destination();
            let d = self.direction(dest);

            // println!("DT: {:#?}", context.dt);
            match d {
                Direction::Forward => p.move_forward = true,
                Direction::Back => p.move_backward = true,
                _ => return,
            };
            println!("DIRECTION: {:#?}; {:#?}", d, message);
            // let now = timer();
            // let diff = now-self.secs;

            // let mut sleeping = false;
            // self.secs = timer();
            context.task_pool.spawn_plugin_task(
                async move {
                    // p.move_forward = false;
                    // p.move_backward = false;
                    let ten_millis = time::Duration::from_millis(10);
                    // sleeping = true;
                    // thread::sleep(ten_millis);
                },
                move |_, s: &mut Game, c| {
                    let scene = c.scenes.try_get_mut(s.scene).unwrap();
                    let p = scene
                        .graph
                        .try_get_script_of_mut::<Player>(s.player)
                        .unwrap();
                    // let now = timer();
                    // println!("dur: {}",now-s.secs);
                    // if now - s.secs >= 0.1 {
                    //     println!("stopping");
                    //     match d {
                    //         Direction::Forward => p.move_forward = false,
                    //         Direction::Back => p.move_backward = false,
                    //         _ => return,
                    //     };
                    //     // p.move_backward = false;
                    //     // p.move_forward = false;

                    // }
                },
            );
            // p.move_forward = false;
            // p.move_backward = false;
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

    fn register_property_editors(
        &self,
    ) -> fyrox::gui::inspector::editors::PropertyEditorDefinitionContainer {
        fyrox::gui::inspector::editors::PropertyEditorDefinitionContainer::empty()
    }

    fn on_loaded(&mut self, #[allow(unused_variables)] context: PluginContext) {}

    fn before_rendering(&mut self, #[allow(unused_variables)] context: PluginContext) {}

    fn on_graphics_context_destroyed(&mut self, #[allow(unused_variables)] context: PluginContext) {
    }

    fn on_scene_loading_failed(
        &mut self,
        #[allow(unused_variables)] path: &Path,
        #[allow(unused_variables)] error: &VisitError,
        #[allow(unused_variables)] context: &mut PluginContext,
    ) {
    }
}
