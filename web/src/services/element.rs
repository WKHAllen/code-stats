use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Element, Event, HtmlInputElement, InputEvent, MouseEvent};

pub fn get_event_target_id(e: MouseEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: Element = event_target.dyn_into().unwrap_throw();
    target.id()
}

pub fn get_event_target_html(e: MouseEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: Element = event_target.dyn_into().unwrap_throw();
    target.inner_html()
}

pub fn get_value_from_input_event(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    target.value()
}
