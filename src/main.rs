#![recursion_limit = "512"]

extern crate lazy_static;

use log::*;
use test::*;
use yew::prelude::*;

mod config;
pub mod hasura;
mod test;

#[derive(Debug)]
enum Message {}

struct Model {}

impl Component for Model {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        info!("{:?}", ctx.props());
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <TestView />
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    yew::start_app::<Model>();
}
