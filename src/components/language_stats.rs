//! Basic language statistics display.

use super::LanguageLabel;
use crate::services::*;
use dioxus::prelude::*;
use std::collections::HashMap;

/// Language statistics properties.
#[derive(Props)]
pub struct LanguageStatsProps<'a, F>
where
    F: FnOnce(&DirCounts) -> usize,
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
    F: FnOnce(&DirCounts) -> usize,
{
    render! {
        div {}
    }
}
