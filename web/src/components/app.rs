use super::{FileSelect, Stats};
use std::path::PathBuf;
use yew::prelude::*;

pub enum Msg {
    ShowCodeStats(bool),
    SetCodeStatsPath(PathBuf),
}

pub struct App {
    show_code_stats: bool,
    code_stats_path: PathBuf,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            show_code_stats: false,
            code_stats_path: PathBuf::new(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_set_code_stats_path = ctx.link().callback(Msg::SetCodeStatsPath);
        let on_hide_code_stats_click = ctx.link().callback(|_| Msg::ShowCodeStats(false));

        html! {
            <div class="app">
                <div class="content">
                    {
                        if self.show_code_stats {
                            html! {
                                <>
                                    <Stats path={self.code_stats_path.to_owned()} />
                                    <button type="button" onclick={on_hide_code_stats_click}>{"Back"}</button>
                                </>
                            }
                        } else {
                            html! {
                                <>
                                    <FileSelect directory={true} on_select={on_set_code_stats_path} />
                                </>
                            }
                        }
                    }
                </div>
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ShowCodeStats(show) => self.show_code_stats = show,
            Msg::SetCodeStatsPath(path) => self.code_stats_path = path,
        };

        true
    }
}
