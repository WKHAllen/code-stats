//! Code stats component.

use crate::services::*;
use dioxus::prelude::*;
use std::io;
use std::path::Path;

/// The current state of the code statistics.
enum CodeStatsState {
    /// Currently fetching code statistics information.
    Fetching,
    /// Done fetching code statistics information.
    Complete(Box<CodeStats>),
    /// An error occurred while fetching code statistics information.
    Error(io::Error),
}

/// Code stats properties.
#[derive(Props)]
pub struct StatsProps<'a> {
    /// The path to display code statistics for.
    path: &'a Path,
    /// The callback to signal that the user is done viewing the statistics.
    on_done: EventHandler<'a, ()>,
}

/// Code stats component.
pub fn Stats<'a>(cx: Scope<'a, StatsProps<'a>>) -> Element {
    let status = use_state(cx, || CodeStatsState::Fetching);

    use_on_create(cx, || {
        let path = cx.props.path.to_owned();
        async move {
            let now = std::time::Instant::now();
            let stats = collect_stats(path).await.unwrap();
            let elapsed = now.elapsed();
            let total_lines = stats
                .stats
                .counts
                .values()
                .fold(0usize, |total, current| total + current.lines);
            dbg!(total_lines);
            dbg!(elapsed);
        }
    });

    render! {
        div {
            "Placeholder for code statistics: "
            {cx.props.path.display().to_string()}
        }
    }
}
