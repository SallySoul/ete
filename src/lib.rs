use console_error_panic_hook;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::console::log_1;

mod app;
mod canvas;
mod controls;
mod render;
mod shader;
use crate::app::{App, AppWrapper, Message};
use crate::render::WebRendererWrapper;

pub static CANVAS_WIDTH: i32 = 820;
pub static CANVAS_HEIGHT: i32 = 820;
pub static APP_DIV_ID: &'static str = "ete-app";
pub static FOV_START_VALUE: f32 = std::f32::consts::PI / 2.0;
pub static DRAW_GNOMON_CENTER_START: bool = false;
pub static DRAW_GNOMON_CORNER_START: bool = true;

#[wasm_bindgen]
pub struct WebClient {
    app: AppWrapper,
    renderer: WebRendererWrapper,
}

#[wasm_bindgen]
impl WebClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WebClient {
        console_error_panic_hook::set_once();

        let app = App::new_wrapper();

        let gl_context = canvas::create_webgl_context(Rc::clone(&app)).unwrap();

        let renderer = render::WebRenderer::new_wrapper(gl_context).expect("Renderer");

        app.borrow_mut().set_renderer(Rc::clone(&renderer));

        controls::append_controls(Rc::clone(&app)).expect("append_controls");

        WebClient { app, renderer }
    }

    pub fn start(&self) {
        log_1(&"Start_1".into());
    }

    pub fn update(&self, time_delta: f32) {
        let width = self.renderer.borrow().gl_context.drawing_buffer_width() as f32;
        let height = self.renderer.borrow().gl_context.drawing_buffer_height() as f32;

        self.app
            .borrow_mut()
            .handle_message(&Message::Update(time_delta, width, height));
    }

    pub fn render(&self) {
        self.renderer.borrow_mut().render(&self.app.borrow().camera);
    }
}
