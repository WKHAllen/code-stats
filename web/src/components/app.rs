use super::super::services::{config, recent_paths};
use super::super::types::AppConfig;
use super::{FileSelect, RecentPaths, Stats};
use std::path::PathBuf;
use yew::prelude::*;

enum AppState {
    Home,
    DirectorySelection,
    DisplayingStats(PathBuf),
}

pub enum Msg {
    ShowHome,
    SelectDirectory,
    ShowStats(PathBuf),
    RemoveRecentPath(PathBuf),
}

pub struct App {
    state: AppState,
    app_config: AppConfig,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            state: AppState::Home,
            app_config: config::load_config(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_show_directory_selection = ctx.link().callback(|_| Msg::SelectDirectory);
        let on_show_stats = ctx.link().callback(Msg::ShowStats);
        let on_cancel = ctx.link().callback(|_| Msg::ShowHome);
        let on_remove_recent_path = ctx.link().callback(Msg::RemoveRecentPath);

        html! {
            <div class="app">
                <div class="content">
                    {
                        match &self.state {
                            AppState::Home => html! {
                                <div class="home">
                                    <h1 class="home-title">{"Code Statistics"}</h1>
                                    <div class="home-description">{"View detailed language statistics for any codebase."}</div>
                                    <button type="button" class="button primary" onclick={on_show_directory_selection}>{"Get started"}</button>
                                </div>
                            },
                            AppState::DirectorySelection => html! {
                                <div class="path-select">
                                    <FileSelect directory={true} cancelable={true} on_select={on_show_stats.clone()} {on_cancel} />
                                    <RecentPaths recent_paths={self.app_config.recent_paths.clone()} on_select_path={on_show_stats} on_remove_path={on_remove_recent_path} />
                                </div>
                            },
                            AppState::DisplayingStats(path) => html! {
                                <Stats path={path.clone()} on_close={on_cancel} />
                            }
                        }
                    }
                </div>
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ShowHome => self.state = AppState::Home,
            Msg::SelectDirectory => self.state = AppState::DirectorySelection,
            Msg::ShowStats(path) => {
                self.state = AppState::DisplayingStats(path.clone());
                self.app_config = recent_paths::add_recent_path(&self.app_config, &path);
            }
            Msg::RemoveRecentPath(path) => {
                web_sys::console::log_1(&path.clone().to_str().unwrap().into());
                self.app_config = recent_paths::remove_recent_path(&self.app_config, &path)
            }
        }

        true
    }
}
