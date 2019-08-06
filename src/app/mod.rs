use crate::render::WebRendererWrapper;
use camera_ss::{ButtonState, Camera, MouseButton};
use cgmath::InnerSpace;
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::console::log_1;

pub type AppWrapper = Rc<RefCell<App>>;

pub struct App {
    pub camera: Camera,
    renderer: Option<WebRendererWrapper>,
}

impl App {
    pub fn new_wrapper() -> AppWrapper {
        Rc::new(RefCell::new(App {
            camera: Camera::new(),
            renderer: None,
        }))
    }

    pub fn set_renderer(&mut self, renderer: WebRendererWrapper) {
        self.renderer = Some(renderer);
    }

    pub fn handle_message(&mut self, message: &Message) {
        match message {
            Message::MouseDown(x, y) => {
                self.camera
                    .handle_mouse_input(MouseButton::Left, ButtonState::Pressed);
                self.camera.handle_mouse_move(*x as f32, *y as f32);
            }
            Message::MouseUp => {
                self.camera
                    .handle_mouse_input(MouseButton::Left, ButtonState::Released);
            }
            Message::MouseMove(x, y) => {
                self.camera.handle_mouse_move(*x as f32, *y as f32);
            }
            Message::Zoom(delta) => {
                self.camera.handle_scroll(*delta);
            }
            Message::Update(time_delta, window_width, window_height) => {
                self.camera
                    .update(*time_delta, *window_width, *window_height);
            }
            Message::DrawGnomonCenter(draw_flag) => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.borrow_mut().set_draw_gnomon_center(*draw_flag);
                }
            }
            Message::DrawGnomonCorner(draw_flag) => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.borrow_mut().set_draw_gnomon_corner(*draw_flag);
                }
            }
            Message::DefaultCam => {
                self.camera.transition_to_default();
            }
            Message::Debug => {
                let up = self.camera.get_up();
                let right = self.camera.get_right();
                let pos = self.camera.get_position();

                log_1(&format!("up: {:?}, norm: {}", up, up.magnitude()).into());
                log_1(&format!("right: {:?}, norm: {}", right, right.magnitude()).into());
                log_1(&format!("Camera Pos: {:?}", pos).into());
            }
            Message::SetFov(fov) => {
                self.camera.set_field_of_view(*fov);
            }
        }
    }
}

pub enum Message {
    MouseDown(i32, i32),
    MouseUp,
    MouseMove(i32, i32),
    Zoom(f32),
    // TODO make this a struct
    Update(f32, f32, f32),
    DrawGnomonCenter(bool),
    DrawGnomonCorner(bool),
    DefaultCam,
    Debug,
    SetFov(f32),
}
