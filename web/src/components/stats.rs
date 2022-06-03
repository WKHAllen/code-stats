use super::super::services::code_stats;
use super::super::types::{CodeStatsResponse, DirStats};
use std::path::PathBuf;
use yew::prelude::*;

enum CodeStatsState {
    Fetching,
    Completed(DirStats),
    Error(String),
}

pub enum Msg {
    SetStats(CodeStatsResponse),
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub path: PathBuf,
}

pub struct Stats {
    status: CodeStatsState,
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
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.status {
            CodeStatsState::Fetching => html! {
                <div class="stats-fetching">{"Fetching code stats..."}</div>
            },
            CodeStatsState::Completed(stats) => html! {
                <div class="stats">{"Stats here"}</div>
            },
            CodeStatsState::Error(err) => html! {
                <div class="error stats-error">{"An error occurred while fetching code stats: "}{err}</div>
            },
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetStats(stats_response) => match stats_response {
                CodeStatsResponse::Ok(stats) => self.status = CodeStatsState::Completed(stats),
                CodeStatsResponse::Error(err) => self.status = CodeStatsState::Error(err),
            },
        }

        true
    }
}
