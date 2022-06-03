use super::super::services::dir_info;
use super::super::types::{DirectoryInfo, DirectoryInfoResponse};
use std::path::PathBuf;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Element, Event, MouseEvent};
use yew::prelude::*;

fn get_event_target_id(e: MouseEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: Element = event_target.dyn_into().unwrap_throw();
    target.id()
}

enum PathSelectType {
    File,
    Directory,
}

enum DirectoryInfoState {
    Fetching,
    Completed(DirectoryInfo),
    Error(String),
}

pub enum Msg {
    SetCurrentPath(PathBuf),
    GoToParentPath,
    SetSelection(PathBuf),
    OnCancel,
    OnSelect,
    SetDirInfo(DirectoryInfoResponse),
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or(PathBuf::from(""))]
    pub start_path: PathBuf,
    #[prop_or(false)]
    pub directory: bool,
    #[prop_or(false)]
    pub cancelable: bool,
    pub on_select: Callback<PathBuf>,
    #[prop_or(Callback::noop())]
    pub on_cancel: Callback<()>,
}

pub struct FileSelect {
    current_path: PathBuf,
    select_type: PathSelectType,
    selection: Option<PathBuf>,
    status: DirectoryInfoState,
}

impl Component for FileSelect {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let Props {
            start_path,
            directory,
            cancelable: _cancelable,
            on_select: _on_select,
            on_cancel: _on_cancel,
        } = ctx.props().clone();

        ctx.link().send_future(async move {
            let home_dir = dir_info::get_home_directory().await;

            match home_dir {
                Some(path) => Msg::SetCurrentPath(path),
                None => Msg::SetCurrentPath(PathBuf::from("")),
            }
        });

        Self {
            current_path: start_path,
            select_type: if !directory {
                PathSelectType::File
            } else {
                PathSelectType::Directory
            },
            selection: None,
            status: DirectoryInfoState::Fetching,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let current_path = self.current_path.clone();
        let on_set_current_path = ctx.link().callback(move |e| {
            Msg::SetCurrentPath(current_path.join(PathBuf::from(get_event_target_id(e))))
        });
        let on_go_to_parent_path = ctx.link().callback(|_| Msg::GoToParentPath);
        let on_set_selection = ctx
            .link()
            .callback(|e| Msg::SetSelection(PathBuf::from(get_event_target_id(e))));
        let on_cancel_click = ctx.link().callback(|_| Msg::OnCancel);
        let on_select_click = ctx.link().callback(|_| Msg::OnSelect);
        let selection = self.selection.clone();

        html! {
            <div class="file-select">
                <div class="file-select-header">
                    <div>{"Select a "}
                        {
                            match self.select_type {
                                PathSelectType::File => "file",
                                PathSelectType::Directory => "directory"
                            }
                        }
                    </div>
                    <div>
                        <button type="button" class="icon-button" disabled={self.current_path == PathBuf::from("") || self.current_path == PathBuf::from("/")} onclick={on_go_to_parent_path}>
                            <img src="arrow-up.svg" class="icon" />
                        </button>
                    </div>
                </div>
                <div class="file-select-body scrollbox">
                    {
                        match &self.status {
                            DirectoryInfoState::Fetching => html! {
                                <div class="dir-info-fetching">{"Fetching directory info..."}</div>
                            },
                            DirectoryInfoState::Completed(dir_info) => if dir_info.dirs.len() > 0 || dir_info.files.len() > 0 {
                                html! {
                                    <div class="dir-info">
                                        {
                                            dir_info.dirs.iter().map(|entry| match &selection {
                                                Some(path) if path.to_owned() == PathBuf::from(entry) => {
                                                    html! {
                                                        <div class="dir-info-dir file-selection" id={entry.clone()} onclick={on_set_current_path.clone()}>
                                                            <img src="folder.svg" class="icon folder-icon" />
                                                            <span>{entry}</span>
                                                        </div>
                                                    }
                                                },
                                                _ => {
                                                    html! {
                                                        <div class="dir-info-dir" id={entry.clone()} onclick={on_set_selection.clone()}>
                                                            <img src="folder.svg" class="icon folder-icon" />
                                                            <span>{entry}</span>
                                                        </div>
                                                    }
                                                }
                                            }).collect::<Html>()
                                        }
                                        {
                                            dir_info.files.iter().map(|entry| match &self.selection {
                                                Some(path) if path.to_owned() == PathBuf::from(entry) => {
                                                    html! {
                                                        <div class="dir-info-file file-selection" id={entry.clone()} onclick={on_set_selection.clone()}>
                                                            <img src="file.svg" class="icon file-icon" />
                                                            <span>{entry}</span>
                                                        </div>
                                                    }
                                                },
                                                _ => {
                                                    html! {
                                                        <div class="dir-info-file" id={entry.clone()} onclick={on_set_selection.clone()}>
                                                            <img src="file.svg" class="icon file-icon" />
                                                            <span>{entry}</span>
                                                        </div>
                                                    }
                                                }
                                            }).collect::<Html>()
                                        }
                                    </div>
                                }
                            } else {
                                html! {
                                    <div class="dir-info-info">{"Empty directory"}</div>
                                }
                            },
                            DirectoryInfoState::Error(err) => html! {
                                <div class="error dir-info-error">{"An error occurred while fetching directory info: "}{err}</div>
                            },
                        }
                    }
                </div>
                <div class="file-select-footer">
                    <div class="file-select-selection-container">
                        {
                            match &self.selection {
                                Some(path) => html! {
                                    <div class="file-select-selection">{"Current selection: "}{self.current_path.join(path).to_str().unwrap()}</div>
                                },
                                None => html! {
                                    <div class="file-select-selection">{"No path selected"}</div>
                                }
                            }
                        }
                    </div>
                    <div class="file-select-actions-container">
                        <div class="file-select-actions">
                            <button type="button" class="button secondary" onclick={on_cancel_click}>{"Cancel"}</button>
                            <button type="button" class="button primary" disabled={self.selection.is_none()} onclick={on_select_click}>{"Select"}</button>
                        </div>
                    </div>
                </div>
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetCurrentPath(path) => {
                self.current_path = path.clone();

                if path.to_str().unwrap() != "" && path.to_str().unwrap() != "/" {
                    self.selection = Some(PathBuf::from(""));
                } else {
                    self.selection = None;
                }

                ctx.link().send_future(async move {
                    let dir_info = dir_info::get_directory_info(&path).await;
                    Msg::SetDirInfo(dir_info)
                });
            }
            Msg::GoToParentPath => {
                if let Some(path) = self.current_path.parent() {
                    ctx.link()
                        .callback(Msg::SetCurrentPath)
                        .emit(path.to_path_buf());
                }
            }
            Msg::SetSelection(path) => match &self.status {
                DirectoryInfoState::Completed(dir_info) => match &self.select_type {
                    PathSelectType::Directory => {
                        if dir_info
                            .dirs
                            .iter()
                            .map(PathBuf::from)
                            .collect::<Vec<_>>()
                            .contains(&path)
                        {
                            self.selection = Some(path.clone());
                        }
                    }
                    PathSelectType::File => {
                        if dir_info
                            .files
                            .iter()
                            .map(PathBuf::from)
                            .collect::<Vec<_>>()
                            .contains(&path)
                        {
                            self.selection = Some(path.clone());
                        }
                    }
                },
                _ => {}
            },
            Msg::OnCancel => ctx.props().clone().on_cancel.emit(()),
            Msg::OnSelect => ctx
                .props()
                .clone()
                .on_select
                .emit(self.selection.clone().unwrap()),
            Msg::SetDirInfo(dir_info_response) => match dir_info_response {
                DirectoryInfoResponse::Ok(dir_info) => {
                    self.status = DirectoryInfoState::Completed(dir_info)
                }
                DirectoryInfoResponse::Error(err) => self.status = DirectoryInfoState::Error(err),
            },
        }

        true
    }
}
