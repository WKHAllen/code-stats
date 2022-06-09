use super::super::services::{format, lang};
use super::LangLabel;
use std::collections::HashMap;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or_default]
    pub label: String,
    pub stats: HashMap<String, usize>,
}

pub struct LangStats {}

impl Component for LangStats {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props { label, stats } = ctx.props().clone();

        let filtered_stats: HashMap<&String, &usize> = stats
            .iter()
            .filter(|&(language, _count)| lang::known_language(language))
            .collect();
        let stats_total: usize = filtered_stats.iter().map(|(_language, count)| *count).sum();
        let mut ordered_stats: Vec<(&String, &usize)> = filtered_stats
            .into_iter()
            .map(|(language, count)| (language, count))
            .collect();
        ordered_stats.sort_by(|(_language1, count1), (_language2, count2)| count1.cmp(count2));
        ordered_stats.reverse();

        html! {
            <div class="lang-stats">
                <div class="lang-stats-label">{format::format_with_commas(stats_total)}{" "}{label}</div>
                <div class="lang-stats-bar">
                    {
                        ordered_stats.iter().map(|(language, count)| {
                            let (_lang_name, lang_color) = lang::get_lang(&language);

                            html! {
                                <div class="lang-stats-bar-item" style={format!("background-color: {}; flex-grow: {};", lang_color.to_html(), **count)}></div>
                            }
                        }).collect::<Html>()
                    }
                </div>
                <div class="lang-stats-langs">
                    {
                        ordered_stats.iter().map(|(language, count)| {
                            html! {
                                <LangLabel lang={language.clone().clone()} count={**count} total={stats_total} />
                            }
                        }).collect::<Html>()
                    }
                </div>
            </div>
        }
    }
}
