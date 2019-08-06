use wasm_bindgen::JsValue;
use web_sys::console::log_1;
use web_sys::WebGlRenderingContext as GL;
use web_sys::WebGlRenderingContext;

use crate::shader::ShaderSystem;
use camera_ss::Camera;
use std::cell::RefCell;
use std::rc::Rc;

mod buffers;
mod color;
mod fade_background;
mod gnomon;
mod point_system;

pub type WebRendererWrapper = Rc<RefCell<WebRenderer>>;

pub struct WebRenderer {
    shader_sys: ShaderSystem,
    pub gl_context: WebGlRenderingContext,
    draw_gnomon_center: bool,
    draw_gnomon_corner: bool,
    gnomon: gnomon::Gnomon,
    fade_background: fade_background::FadeBackground,
}

impl WebRenderer {
    pub fn new_wrapper(gl_context: WebGlRenderingContext) -> Result<WebRendererWrapper, JsValue> {
        let shader_sys = ShaderSystem::new(&gl_context);
        let gnomon = gnomon::Gnomon::new(&gl_context)?;
        let fade_background = fade_background::FadeBackground::new(&gl_context)?;

        Ok(Rc::new(RefCell::new(WebRenderer {
            shader_sys,
            gl_context,
            draw_gnomon_center: crate::DRAW_GNOMON_CENTER_START,
            draw_gnomon_corner: crate::DRAW_GNOMON_CORNER_START,
            gnomon,
            fade_background,
        })))
    }

    pub fn set_draw_gnomon_center(&mut self, draw_flag: bool) {
        self.draw_gnomon_center = draw_flag;
    }

    pub fn set_draw_gnomon_corner(&mut self, draw_flag: bool) {
        self.draw_gnomon_corner = draw_flag;
    }

   pub fn render(&mut self, camera: &Camera) {
        // Parchment color?
        self.gl_context.clear_color(0.13, 0.11, 0.21, 1.);
        self.gl_context
            .clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        let width = self.gl_context.drawing_buffer_width();
        let height = self.gl_context.drawing_buffer_height();
        self.gl_context.viewport(0, 0, width, height);

        self.gl_context.disable(GL::DEPTH_TEST);
        self.fade_background
            .render(&self.gl_context, &self.shader_sys);
        self.gl_context.enable(GL::DEPTH_TEST);

       if self.draw_gnomon_center {
            self.gnomon
                .render(&self.gl_context, &self.shader_sys, camera, false, 20.0);
        }

        if self.draw_gnomon_corner {
            self.gnomon
                .render(&self.gl_context, &self.shader_sys, camera, true, 1.0);
        }
    }
}
