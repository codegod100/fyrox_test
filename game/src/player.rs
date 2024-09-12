use fyrox::core::reflect::GetField;
use fyrox::generic_animation::machine::Parameter;
use fyrox::graph::SceneGraph;
use fyrox::gui::button::{ButtonBuilder, ButtonMessage};
use fyrox::gui::message::UiMessage;
use fyrox::gui::text::TextBuilder;
use fyrox::gui::widget::WidgetBuilder;
use fyrox::gui::{UiNode, UserInterface};
use fyrox::plugin::{Plugin, PluginContext};
use fyrox::scene::animation::absm::AnimationBlendingStateMachine;
use fyrox::window::Window;
use fyrox::{
    core::{
        algebra::{UnitQuaternion, UnitVector3, Vector3},
        pool::Handle,
        reflect::prelude::*,
        type_traits::prelude::*,
        variable::InheritableVariable,
        visitor::prelude::*,
    },
    event::{DeviceEvent, ElementState, Event, MouseButton, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
    scene::{node::Node, rigidbody::RigidBody},
    script::{ScriptContext, ScriptDeinitContext, ScriptTrait},
};

use crate::Game;

#[derive(Visit, Reflect, Default, Debug, Clone, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "62ae8f72-896e-412d-843c-3e24540e7f38")]
#[visit(optional)]
pub struct Player {
    // Add fields here.
    #[visit(optional)]
    #[reflect(hidden)]
    pub move_forward: bool,

    #[visit(optional)]
    #[reflect(hidden)]
    move_backward: bool,

    #[visit(optional)]
    #[reflect(hidden)]
    move_left: bool,

    #[visit(optional)]
    #[reflect(hidden)]
    move_right: bool,

    #[visit(optional)]
    #[reflect(hidden)]
    jump: bool,

    #[visit(optional)]
    #[reflect(hidden)]
    yaw: f32,

    #[visit(optional)]
    #[reflect(hidden)]
    pitch: f32,

    absm: InheritableVariable<Handle<Node>>,
    model_root: InheritableVariable<Handle<Node>>,

    #[visit(optional)]
    camera: Handle<Node>,

    quit_button_handle: Handle<UiNode>,
}
fn create_quit_button(ui: &mut UserInterface) -> Handle<UiNode> {
    ButtonBuilder::new(WidgetBuilder::new())
        .with_content(
            TextBuilder::new(WidgetBuilder::new())
                .with_text("forward")
                .build(&mut ui.build_ctx()),
        )
        .build(&mut ui.build_ctx())
}



impl ScriptTrait for Player {

    fn on_init(&mut self, context: &mut ScriptContext) {
        // Put initialization logic here.
        self.yaw = 180.0;
        // self.quit_button_handle =  create_quit_button(context.user_interfaces.first_mut());

    }

    fn on_start(&mut self, context: &mut ScriptContext) {
        // There should be a logic that depends on other scripts in scene.
        // It is called right after **all** scripts were initialized.
        println!("We are starting");
        context.plugins.get_mut::<Game>().player = context.handle;
    }

    fn on_deinit(&mut self, context: &mut ScriptDeinitContext) {
        // Put de-initialization logic here.
    }

    fn on_os_event(&mut self, event: &Event<()>, context: &mut ScriptContext) {
        // Respond to OS events here.
        match event {
            // Raw mouse input is responsible for camera rotation.
            Event::DeviceEvent {
                event:
                    DeviceEvent::MouseMotion {
                        delta: (dx, dy), ..
                    },
                ..
            } => {
                // Pitch is responsible for vertical camera rotation. It has -89.9..89.0 degree limits,
                // to prevent infinite rotation.
                let mouse_speed = 0.35;
                self.pitch = (self.pitch + *dy as f32 * mouse_speed).clamp(-89.9, 89.9);
                self.yaw -= *dx as f32 * mouse_speed;
            }
            // Keyboard input is responsible for player's movement.
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { event, .. },
                ..
            } => {
                if let PhysicalKey::Code(code) = event.physical_key {
                    let is_pressed = event.state == ElementState::Pressed;
                    match code {
                        KeyCode::KeyW => {
                            println!("got W");
                            self.move_forward = is_pressed;
                        }
                        KeyCode::KeyS => {
                            self.move_backward = is_pressed;
                        }
                        KeyCode::KeyA => {
                            self.move_left = is_pressed;
                        }
                        KeyCode::KeyD => {
                            self.move_right = is_pressed;
                        }
                        KeyCode::Space => {
                            self.jump = is_pressed;
                        }
                        _ => (),
                    }
                }
            }
            _ => {}
        }
    }



    fn on_update(&mut self, ctx: &mut ScriptContext) {
        // Put object logic here.
        let mut look_vector = Vector3::default();
        let mut side_vector = Vector3::default();
        let mut jump_vector = Vector3::default();
        if let Some(camera) = ctx.scene.graph.try_get_mut(self.camera) {
            look_vector = camera.look_vector();
            side_vector = camera.side_vector();
            jump_vector = camera.up_vector();

            let yaw = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), self.yaw.to_radians());
            let transform = camera.local_transform_mut();
            // transform.set_rotation(
            //     UnitQuaternion::from_axis_angle(
            //         &UnitVector3::new_normalize(yaw * Vector3::x()),
            //         self.pitch.to_radians(),
            //     ) * yaw,
            // );
        }

        if let Some(rigid_body) = ctx.scene.graph.try_get_mut_of_type::<RigidBody>(ctx.handle) {
            // Form a new velocity vector that corresponds to the pressed buttons.
            let mut velocity = Vector3::new(0.0, 0.0, 0.0);
            let yaw = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), self.yaw.to_radians());
            // let transform = rigid_body.local_transform_mut();
            // transform.set_rotation(UnitQuaternion::from_axis_angle(
            //     &UnitVector3::new_normalize(yaw * Vector3::x()),
            //     self.pitch.to_radians(),
            // ) * yaw,);
            if self.move_forward {
                println!("forward is pressed");
                velocity += look_vector;
            }
            if self.move_backward {
                velocity -= look_vector;
            }
            if self.move_left {
                velocity += side_vector;
            }
            if self.move_right {
                velocity -= side_vector;
            }
            if self.jump {
                velocity += jump_vector;
            }

            let y_vel = rigid_body.lin_vel().y;
            if let Some(normalized_velocity) = velocity.try_normalize(f32::EPSILON) {
                let movement_speed = 240.0 * ctx.dt;
                println!("dt: {}", ctx.dt);
                rigid_body.set_lin_vel(Vector3::new(
                    normalized_velocity.x * movement_speed,
                    normalized_velocity.y * 3.0,
                    normalized_velocity.z * movement_speed,
                ));
            } else {
                // Hold player in-place in XZ plane when no button is pressed.
                rigid_body.set_lin_vel(Vector3::new(0.0, y_vel, 0.0));
            }
        }

        let model_transform = ctx
            .scene
            .graph
            .try_get_mut(*self.model_root)
            .map(|model| model.global_transform())
            .unwrap_or_default();

        // let mut velocity = Vector3::default();
        if let Some(state_machine) = ctx
            .scene
            .graph
            .try_get_mut(*self.absm)
            .and_then(|node| node.query_component_mut::<AnimationBlendingStateMachine>())
        {
            let mut moving =
                self.move_left || self.move_right || self.move_forward || self.move_backward;
            if  self.jump {
                moving = false
            }
            // println!("Moving: {}", moving);

            let machine = state_machine.machine_mut().get_value_mut_silent();
            // let val = state_machine.get;
            // // val.get_field(name, func)
            machine.set_parameter("walk", Parameter::Rule(moving));
            machine.set_parameter("idle", Parameter::Rule(!moving));
            // machine.set_parameter("attack", Parameter::Rule(self.jump));

            // // val.set_parameter("idle",Parameter::Rule(!moving));
        }
    }
}
