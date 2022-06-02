use super::super::services::dir_info;
use super::super::types::{DirectoryInfo, DirectoryInfoResponse};
use std::path::PathBuf;
use yew::prelude::*;

enum PathSelectType {
    File,
    Directory,
}

enum DirectoryInfoState {
    Fetching,
    Completed(DirectoryInfo),
    Error,
}

pub enum Msg {
    SetCurrentPath(PathBuf),
    SetSelection(PathBuf),
    OnCancel,
    OnSelect,
    SetDirInfo(DirectoryInfoResponse),
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or_default]
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

        let start_path_clone = start_path.clone();

        ctx.link().send_future(async move {
            let dir_info = dir_info::get_directory_info(&start_path_clone).await;
            Msg::SetDirInfo(dir_info)
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
        let on_set_current_path = ctx.link().callback(Msg::SetCurrentPath);
        let on_cancel_click = ctx.link().callback(|_| Msg::OnCancel);
        let on_select_click = ctx.link().callback(|_| Msg::OnSelect);

        html! {
            <div class="file-select">
                <div class="file-select-header">{"Select a "}
                    {
                        match self.select_type {
                            PathSelectType::File => "file",
                            PathSelectType::Directory => "directory"
                        }
                    }
                </div>
                <div class="file-select-body scrollbox">
                    {
                        match &self.status {
                            DirectoryInfoState::Fetching => html! {
                                <div class="dir-info-fetching">{"Fetching directory info..."}</div>
                            },
                            DirectoryInfoState::Completed(dir_info) => html! {
                            },
                            DirectoryInfoState::Error => html! {
                                <div class="error">{"An error occurred while fetching code stats. This may be because the path was invalid or because you do not have permission to access the specified files and directories."}</div>
                            },
                        }
                    }
                </div>
                <div class="file-select-footer">
                    <div>
                        <button type="button" class="secondary" onclick={on_cancel_click}>{"Cancel"}</button>
                        <button type="button" class="primary" disabled={self.selection.is_none()} onclick={on_select_click}>{"Select"}</button>
                    </div>
                </div>
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetCurrentPath(path) => {
                self.current_path = path.clone();
                self.selection = Some(path.clone());

                ctx.link().send_future(async move {
                    let dir_info = dir_info::get_directory_info(&path).await;
                    Msg::SetDirInfo(dir_info)
                });
            }
            Msg::SetSelection(path) => self.selection = Some(path),
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
                DirectoryInfoResponse::Error => self.status = DirectoryInfoState::Error,
            },
        }

        true
    }
}
