use super::super::services::code_stats;
use super::super::types::{CodeStatsResponse, DirStats};
use super::{LangStats, LangStatsTraversal};
use std::path::{Path, PathBuf};
use yew::prelude::*;

fn remove_trailing_slash(path: &PathBuf) -> PathBuf {
    let path_str = path.display().to_string();

    if path_str.ends_with("/") {
        let mut chars = path_str.chars();
        chars.next_back();
        PathBuf::from(chars.as_str().clone())
    } else {
        path.clone()
    }
}

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
    pub on_close: Callback<()>,
}

pub struct Stats {
    status: CodeStatsState,
    subpath: PathBuf,
}

impl Component for Stats {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let Props {
            path,
            on_close: _on_close,
        } = ctx.props().clone();

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
        let Props { path, on_close } = ctx.props().clone();
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
                            <div class="stats-header">
                                <div>
                                    <div>{"Language breakdown for: "}</div>
                                    <div>
                                        <span class="stats-path">{remove_trailing_slash(&path).display()}</span>
                                        <span class="stats-subpath">{"/"}{self.subpath.display()}</span>
                                    </div>
                                </div>
                                <div>
                                    <button type="button" class="icon-button" onclick={move |_| on_close.emit(())}>
                                        <img src="xmark.svg" class="icon" />
                                    </button>
                                </div>
                            </div>
                            <LangStats label="files" stats={substats.file_counts.clone()} />
                            <LangStats label="lines" stats={substats.line_counts.clone()} />
                            <LangStats label="characters" stats={substats.char_counts.clone()} />
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
