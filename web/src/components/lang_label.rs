use super::super::services::{format, lang};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub lang: String,
    pub count: usize,
    pub total: usize,
}

pub struct LangLabel {}

impl Component for LangLabel {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props { lang, count, total } = ctx.props().clone();
        let (lang_name, lang_color) = lang::get_lang(&lang);

        html! {
            <div class="lang-stats-lang">
                <div class="lang-stats-lang-color" style={format!("background-color: {};", lang_color.to_html())}></div>
                <div class="lang-stats-lang-label">{format!("{}: {} ({:.1}%)", lang_name, format::format_with_commas(count), (count as f64) / (total as f64) * 100.0)}</div>
            </div>
        }
    }
}
