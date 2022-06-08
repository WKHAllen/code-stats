use super::super::services::element;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub value: String,
    pub on_change: Callback<String>,
    #[prop_or(false)]
    pub disabled: bool,
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let Props {
        value,
        on_change,
        disabled,
    } = props.clone();

    let oninput = Callback::from(move |input_event: InputEvent| {
        on_change.emit(element::get_value_from_input_event(input_event));
    });

    html! {
        <div class="input-container">
            <input type="text" {value} {oninput} {disabled} />
        </div>
    }
}
