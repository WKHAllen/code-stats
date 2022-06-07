use super::{FileSelect, Stats};
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
}

pub struct App {
    state: AppState,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            state: AppState::Home,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_show_directory_selection = ctx.link().callback(|_| Msg::SelectDirectory);
        let on_show_stats = ctx.link().callback(Msg::ShowStats);
        let on_cancel = ctx.link().callback(|_| Msg::ShowHome);

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
                                <FileSelect directory={true} cancelable={true} on_select={on_show_stats} {on_cancel} />
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
            Msg::ShowStats(path) => self.state = AppState::DisplayingStats(path),
        };

        true
    }
}
