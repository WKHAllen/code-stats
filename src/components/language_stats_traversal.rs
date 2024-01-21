//! Traversal for viewing levels of code statistics.

use super::LanguageLabel;
use crate::services::*;
use dioxus::prelude::*;
use std::path::Path;

/// Language statistics traversal properties.
#[derive(Props)]
pub struct LanguageStatsTraversalProps<'a> {
    /// The subpath within the traversal.
    subpath: &'a Path,
    /// The statistics at the current level of the traversal.
    dir_stats: &'a DirStats,
    /// The callback to trigger traversal down the directory structure.
    on_traverse_down: EventHandler<'a, &'a str>,
    /// The callback to trigger traversal up the directory structure.
    on_traverse_up: EventHandler<'a, ()>,
}

/// Language statistics traversal component.
pub fn LanguageStatsTraversal<'a>(cx: Scope<'a, LanguageStatsTraversalProps<'a>>) -> Element {
    render! {
        div {}
    }
}
