use crate::app::{AppWrapper, Message};
use crate::APP_DIV_ID;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::console::log_1;
use web_sys::window;
use web_sys::Element;
use web_sys::HtmlElement;
use web_sys::HtmlInputElement;

pub fn append_controls(app: AppWrapper) -> Result<(), JsValue> {
    let window = window().unwrap();
    let document = window.document().unwrap();

    let container: HtmlElement = match document.get_element_by_id(APP_DIV_ID) {
        Some(container) => container.dyn_into().expect("Html element"),
        None => document.body().expect("Document body"),
    };

    let controls = document.create_element("div")?;
    container.append_child(&controls)?;
    let controls: HtmlElement = controls.dyn_into()?;
    controls.style().set_property("padding-left", "5px")?;
    let controls: Element = controls.dyn_into()?;

    {
        let app = Rc::clone(&app);
        let element = create_draw_gnomon_center_checkbox(app)?;
        controls.append_child(&element)?;
    }

    {
        let app = Rc::clone(&app);
        let element = create_draw_gnomon_corner_checkbox(app)?;
        controls.append_child(&element)?;
    }

    {
        let app = Rc::clone(&app);
        let element = create_default_cam_button(app)?;
        controls.append_child(&element)?;
    }

    {
        let app = Rc::clone(&app);
        let element = create_fov_slider(app)?;
        controls.append_child(&element)?;
    }

    {
        let app = Rc::clone(&app);
        let element = create_debug_button(app)?;
        controls.append_child(&element)?;
    }

    Ok(())
}

fn create_default_cam_button(app: AppWrapper) -> Result<HtmlElement, JsValue> {
    let handler = move |_event: web_sys::Event| {
        app.borrow_mut().handle_message(&Message::DefaultCam);
    };
    let closure = Closure::wrap(Box::new(handler) as Box<FnMut(_)>);

    let window = window().unwrap();
    let document = window.document().unwrap();

    let button: HtmlInputElement = document.create_element("input")?.dyn_into()?;
    button.set_type("button");
    button.set_value("Default Cam Pos");
    button.set_onclick(Some(closure.as_ref().unchecked_ref()));
    closure.forget();

    Ok(button.dyn_into()?)
}

fn create_debug_button(app: AppWrapper) -> Result<HtmlElement, JsValue> {
    let handler = move |event: web_sys::Event| {
        log_1(&format!("Event: {:?}", event).into());
        app.borrow_mut().handle_message(&Message::Debug);
    };
    let closure = Closure::wrap(Box::new(handler) as Box<FnMut(_)>);

    let window = window().unwrap();
    let document = window.document().unwrap();

    let button: HtmlInputElement = document.create_element("input")?.dyn_into()?;
    button.set_type("button");
    button.set_value("Debug Message");
    button.set_onclick(Some(closure.as_ref().unchecked_ref()));
    closure.forget();

    Ok(button.dyn_into()?)
}

fn create_draw_gnomon_center_checkbox(app: AppWrapper) -> Result<HtmlElement, JsValue> {
    let handler = move |event: web_sys::Event| {
        let input_elem: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
        let draw_flag = input_elem.checked();

        app.borrow_mut()
            .handle_message(&Message::DrawGnomonCenter(draw_flag));
    };
    let closure = Closure::wrap(Box::new(handler) as Box<FnMut(_)>);

    let draw_control = Checkbox {
        start_checked: crate::DRAW_GNOMON_CENTER_START,
        label: "Draw Gnomon Center",
        closure,
    }
    .create_element()?;

    Ok(draw_control)
}

fn create_draw_gnomon_corner_checkbox(app: AppWrapper) -> Result<HtmlElement, JsValue> {
    let handler = move |event: web_sys::Event| {
        let input_elem: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
        let draw_flag = input_elem.checked();

        app.borrow_mut()
            .handle_message(&Message::DrawGnomonCorner(draw_flag));
    };
    let closure = Closure::wrap(Box::new(handler) as Box<FnMut(_)>);

    let draw_control = Checkbox {
        start_checked: crate::DRAW_GNOMON_CORNER_START,
        label: "Draw Gnomon Corner",
        closure,
    }
    .create_element()?;

    Ok(draw_control)
}

fn create_fov_slider(app: AppWrapper) -> Result<HtmlElement, JsValue> {
    let handler = move |event: web_sys::Event| {
        let input_elem: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
        let field_of_view = input_elem.value().parse().unwrap();

        app.borrow_mut()
            .handle_message(&Message::SetFov(field_of_view));
    };
    let closure = Closure::wrap(Box::new(handler) as Box<FnMut(_)>);

    let fov_control = Slider {
        min: 0.0,
        max: std::f32::consts::PI,
        step: 0.1,
        start: crate::FOV_START_VALUE,
        label: "Field of View",
        closure,
    }
    .create_element()?;

    Ok(fov_control)
}

struct Slider {
    min: f32,
    max: f32,
    step: f32,
    start: f32,
    label: &'static str,
    closure: Closure<FnMut(web_sys::Event)>,
}

impl Slider {
    fn create_element(self) -> Result<HtmlElement, JsValue> {
        let window = window().unwrap();
        let document = window.document().unwrap();

        let slider: HtmlInputElement = document.create_element("input")?.dyn_into()?;
        slider.set_type("range");
        slider.set_min(&format!("{}", self.min));
        slider.set_max(&format!("{}", self.max));
        slider.set_step(&format!("{}", self.step));
        slider.set_value(&format!("{}", self.start));

        let closure = self.closure;
        slider.set_oninput(Some(closure.as_ref().unchecked_ref()));
        closure.forget();

        let label = document.create_element("div")?;
        label.set_inner_html(self.label);

        let container = document.create_element("div")?;
        container.append_child(&label)?;
        container.append_child(&slider)?;

        let container: HtmlElement = container.dyn_into()?;
        container.style().set_property("margin-bottom", "15px")?;

        Ok(container)
    }
}

struct Checkbox {
    start_checked: bool,
    label: &'static str,
    closure: Closure<FnMut(web_sys::Event)>,
}

impl Checkbox {
    fn create_element(self) -> Result<HtmlElement, JsValue> {
        let window = window().unwrap();
        let document = window.document().unwrap();

        let checkbox: HtmlInputElement = document.create_element("input")?.dyn_into()?;
        checkbox.set_type("checkbox");
        checkbox.set_checked(self.start_checked);

        let closure = self.closure;
        checkbox.set_onclick(Some(closure.as_ref().unchecked_ref()));
        closure.forget();

        let label = document.create_element("label")?;
        label.set_inner_html(self.label);
        label.append_child(&checkbox)?;

        let container = document.create_element("div")?;
        container.append_child(&label)?;

        let container: HtmlElement = container.dyn_into()?;
        container.style().set_property("margin-bottom", "15px")?;
        container.style().set_property("display", "flex")?;
        container.style().set_property("align-items", "center")?;
        container.style().set_property("cursor", "pointer")?;

        Ok(container)
    }
}
