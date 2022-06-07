use super::super::services::code_stats;
use super::super::types::{CodeStatsResponse, DirStats};
use super::{LangStats, LangStatsTraversal};
use std::path::{Path, PathBuf};
use yew::prelude::*;

enum CodeStatsState {
    Fetching,
    Completed(DirStats),
    Error(String),
}

pub enum Msg {
    SetStats(CodeStatsResponse),
    TraverseDown(String),
    TraverseUp,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub path: PathBuf,
}

pub struct Stats {
    status: CodeStatsState,
    subpath: PathBuf,
}

impl Component for Stats {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let Props { path } = ctx.props().clone();

        ctx.link().send_future(async move {
            let stats = code_stats::get_code_stats(&path).await;
            Msg::SetStats(stats)
        });

        Self {
            status: CodeStatsState::Fetching,
            subpath: PathBuf::new(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props { path } = ctx.props().clone();
        let on_traverse_down = ctx.link().callback(Msg::TraverseDown);
        let on_traverse_up = ctx.link().callback(|_| Msg::TraverseUp);

        match &self.status {
            CodeStatsState::Fetching => html! {
                <div class="stats-fetching">{"Fetching code stats..."}</div>
            },
            CodeStatsState::Completed(stats) => {
                let substats = code_stats::get_stats_subpath(&stats, &self.subpath).unwrap();

                html! {
                    <div class="stats-container">
                        <div class="stats">
                            <div>
                                <div>{"Language breakdown for: "}</div>
                                <div>
                                    <span class="stats-path">{path.clone().display()}</span>
                                    <span class="stats-subpath">{"/"}{self.subpath.display()}</span>
                                </div>
                            </div>
                            <LangStats label="Number of files" stats={substats.file_counts.clone()} />
                            <LangStats label="Number of lines" stats={substats.line_counts.clone()} />
                            <LangStats label="Number of characters" stats={substats.char_counts.clone()} />
                            <LangStatsTraversal subpath={self.subpath.clone()} dir_stats={substats.clone()} {on_traverse_down} {on_traverse_up} />
                        </div>
                    </div>
                }
            }
            CodeStatsState::Error(err) => html! {
                <div class="error stats-error">{"An error occurred while fetching code stats: "}{err}</div>
            },
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetStats(stats_response) => match stats_response {
                CodeStatsResponse::Ok(stats) => self.status = CodeStatsState::Completed(stats),
                CodeStatsResponse::Error(err) => self.status = CodeStatsState::Error(err),
            },
            Msg::TraverseDown(subpath) => self.subpath = self.subpath.join(subpath),
            Msg::TraverseUp => {
                self.subpath = self.subpath.parent().unwrap_or(Path::new("")).to_path_buf()
            }
        }

        true
    }
}
