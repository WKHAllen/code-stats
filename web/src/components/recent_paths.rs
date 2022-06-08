use super::super::services::element;
use std::path::PathBuf;
use yew::prelude::*;

pub enum Msg {
    SelectPath(PathBuf),
    RemovePath(PathBuf),
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub recent_paths: Vec<PathBuf>,
    pub on_select_path: Callback<PathBuf>,
    pub on_remove_path: Callback<PathBuf>,
}

pub struct RecentPaths {}

impl Component for RecentPaths {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props {
            recent_paths,
            on_select_path: _on_select_path,
            on_remove_path: _on_remove_path,
        } = ctx.props().clone();
        let on_select_path_click = ctx
            .link()
            .callback(|e| Msg::SelectPath(PathBuf::from(element::get_event_target_html(e))));
        let on_remove_path_click = ctx
            .link()
            .callback(|e| Msg::RemovePath(PathBuf::from(element::get_event_target_id(e))));

        if recent_paths.len() > 0 {
            html! {
                <div class="recent-paths-container">
                    <div>{"Recent selections"}</div>
                    <div class="recent-paths">
                        {
                            recent_paths.iter().map(|path| html! {
                                <div class="recent-paths-path">
                                    <div class="recent-paths-path-name" onclick={on_select_path_click.clone()}>{path.display()}</div>
                                    <div class="recent-paths-path-close">
                                        <button type="button" class="icon-button" id={path.clone().to_str().unwrap().to_owned()} onclick={on_remove_path_click.clone()}>
                                            <img src="xmark.svg" class="icon" />
                                        </button>
                                    </div>
                                </div>
                            }).collect::<Html>()
                        }
                    </div>
                </div>
            }
        } else {
            html! {}
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SelectPath(path) => ctx.props().on_select_path.emit(path),
            Msg::RemovePath(path) => ctx.props().on_remove_path.emit(path),
        }

        true
    }
}
