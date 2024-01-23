//! Basic language statistics display.

use super::LanguageLabel;
use crate::services::*;
use dioxus::prelude::*;
use std::collections::HashMap;

/// Language statistics properties.
#[derive(Props)]
pub struct LanguageStatsProps<'a, F>
where
    F: Fn(&DirCounts) -> usize,
{
    /// A label for the statistics.
    label: &'a str,
    /// The statistics.
    stats: &'a HashMap<String, DirCounts>,
    /// An function to extract the desired count from the statistics.
    extractor: F,
}

/// Language statistics display component.
pub fn LanguageStats<'a, F>(cx: Scope<'a, LanguageStatsProps<'a, F>>) -> Element
where
    F: Fn(&DirCounts) -> usize,
{
    let filtered_stats = cx
        .props
        .stats
        .iter()
        .filter_map(|(language, count)| {
            known_language(language).then_some((language, (cx.props.extractor)(count)))
        })
        .collect::<HashMap<_, _>>();
    let stats_total = filtered_stats.values().sum::<usize>();
    let mut ordered_stats = filtered_stats.into_iter().collect::<Vec<_>>();
    ordered_stats.sort_by_key(|(_, count)| *count);
    ordered_stats.reverse();

    let stats_label = format!("{} {}", format_with_commas(stats_total), cx.props.label);

    render! {
        div {
            "class": "lang-stats",

            div {
                class: "lang-stats-label",
                stats_label
            }

            div {
                class: "lang-stats-bar",

                ordered_stats.iter().map(|(language, count)| {
                    let (_, language_color) = get_language(language);
                    let bar_style = format!("background-color: {}; flex-grow: {};", language_color, *count);

                    render! {
                        div {
                            class: "lang-stats-bar-item",
                            style: "{bar_style}"
                        }
                    }
                })
            }

            div {
                class: "lang-stats-langs",

                for (language, count) in &ordered_stats {
                    LanguageLabel {
                        language: language,
                        count: *count,
                        total: stats_total
                    }
                }
            }
        }
    }
}
