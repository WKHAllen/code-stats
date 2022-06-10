use super::super::services::{format, lang};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub lang: String,
    pub count: Option<usize>,
    pub total: Option<usize>,
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

        let mut lang_label = format!("{}", lang_name);

        match count {
            Some(count_value) => {
                lang_label.push_str(&format!(": {}", format::format_with_commas(count_value)));

                match total {
                    Some(total_value) => lang_label.push_str(&format!(
                        " ({:.1}%)",
                        (count_value as f64) / (total_value as f64) * 100.0
                    )),
                    None => (),
                }
            }
            None => (),
        }

        html! {
            <div class="lang-stats-lang">
                <div class="lang-stats-lang-color" style={format!("background-color: {};", lang_color.to_html())}></div>
                <div class="lang-stats-lang-label">{lang_label}</div>
            </div>
        }
    }
}
