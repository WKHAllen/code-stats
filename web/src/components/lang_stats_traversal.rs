use super::super::services::{code_stats, element};
use super::super::types::{DirStats, FileStats};
use super::LangLabel;
use std::path::PathBuf;
use yew::prelude::*;

pub enum Msg {
    TraverseDown(String),
    TraverseUp,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub subpath: PathBuf,
    pub dir_stats: DirStats,
    pub on_traverse_down: Callback<String>,
    pub on_traverse_up: Callback<()>,
}

pub struct LangStatsTraversal {}

impl Component for LangStatsTraversal {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props {
            subpath,
            dir_stats,
            on_traverse_down: _on_traverse_down,
            on_traverse_up: _on_traverse_up,
        } = ctx.props().clone();
        let on_traverse_down = ctx
            .link()
            .callback(move |e| Msg::TraverseDown(element::get_event_target_id(e)));
        let on_traverse_up = ctx.link().callback(|_| Msg::TraverseUp);

        let mut sorted_dirs: Vec<(&String, &DirStats)> =
            dir_stats.dirs.iter().map(|entry| entry).collect();
        sorted_dirs.sort_by(|(name1, _stats1), (name2, _stats2)| name1.cmp(name2));

        let mut sorted_files: Vec<(&String, &FileStats)> =
            dir_stats.files.iter().map(|entry| entry).collect();
        sorted_files.sort_by(|(name1, _stats1), (name2, _stats2)| name1.cmp(name2));

        html! {
            <div class="lang-stats-traversal">
                <div class="lang-stats-traversal-header">
                    <div>{"/"}{subpath.display()}</div>
                    <div>
                        <button type="button" class="icon-button" disabled={subpath == PathBuf::from("") || subpath == PathBuf::from("/")} onclick={on_traverse_up}>
                            <img src="arrow-up.svg" class="icon" />
                        </button>
                    </div>
                </div>
                <div class="lang-stats-traversal-body">
                    {
                        if dir_stats.dirs.len() > 0 || dir_stats.files.len() > 0 {
                            html! {
                                <div class="lang-stats-traversal-dir-info">
                                    {
                                        sorted_dirs.iter().map(|(name, stats)| {
                                            let primary_lang_ext = code_stats::get_dir_primary_lang(&stats);

                                            html! {
                                                <div class="lang-stats-traversal-dir-info-dir" id={name.clone().clone()} onclick={on_traverse_down.clone()}>
                                                    <div class="lang-stats-traversal-dir-info-label">
                                                        <img src="folder.svg" class="icon folder-icon" />
                                                        <span>{name}</span>
                                                    </div>
                                                    <div>
                                                        {
                                                            match primary_lang_ext {
                                                                Some(lang_ext) => html! {
                                                                    <LangLabel lang={lang_ext} />
                                                                },
                                                                None => html! {},
                                                            }
                                                        }
                                                    </div>
                                                </div>
                                            }
                                        }).collect::<Html>()
                                    }
                                    {
                                        sorted_files.iter().map(|(name, _stats)| {
                                            let lang_ext = PathBuf::from(name).extension().unwrap_or_default().to_str().unwrap().to_owned();

                                            html! {
                                                <div class="lang-stats-traversal-dir-info-file" id={name.clone().clone()}>
                                                    <div class="lang-stats-traversal-dir-info-label">
                                                        <img src="file.svg" class="icon file-icon" />
                                                        <span>{name}</span>
                                                    </div>
                                                    <div>
                                                        <LangLabel lang={lang_ext} />
                                                    </div>
                                                </div>
                                            }
                                        }).collect::<Html>()
                                    }
                                </div>
                            }
                        } else {
                            html! {
                                <div class="lang-stats-traversal-dir-info-info">{"Empty directory"}</div>
                            }
                        }
                    }
                </div>
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::TraverseDown(dir) => ctx.props().on_traverse_down.emit(dir),
            Msg::TraverseUp => ctx.props().on_traverse_up.emit(()),
        }

        true
    }
}
