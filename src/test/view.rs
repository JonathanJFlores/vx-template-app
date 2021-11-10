use super::model::*;
use crate::hasura::{
    GraphQLService, GraphQLTask, Request, RequestTask, Subscribe, SubscriptionTask,
};
use log::*;
use yew::prelude::*;

#[derive(Debug)]
pub enum TestViewMessage {
    TimeAdd,
    TimeAdded(Option<time_add::ResponseData>),
    OnTimeAdded(Option<on_time_added::ResponseData>),
    SubscriptionEnable,
    SubscriptionDisable,
    Subscribe,
    Unsubscribe,
}

pub struct TestView {
    link: ComponentLink<Self>,
    graphql_task: Option<GraphQLTask>,
    req_task: Option<RequestTask>,
    sub_task: Option<SubscriptionTask>,
}

impl Component for TestView {
    type Message = TestViewMessage;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            graphql_task: None,
            req_task: None,
            sub_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // info!("{:?}", msg);
        match msg {
            Self::Message::TimeAdd => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = time_add::Variables {
                        time: wasm_utc_now().to_rfc3339(),
                    };
                    let task = TimeAdd::request(graphql_task, &self.link, vars, |data| {
                        Self::Message::TimeAdded(data)
                    });
                    self.req_task = Some(task);
                }
            }
            Self::Message::TimeAdded(data) => {
                info!("TimeAdded: {:?}", data)
            }
            Self::Message::OnTimeAdded(data) => {
                info!("OnTimeAdded: {:?}", data)
            }
            Self::Message::SubscriptionEnable => {
                self.graphql_task = Some(GraphQLService::connect())
            }
            Self::Message::SubscriptionDisable => self.graphql_task = None,
            Self::Message::Subscribe => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = on_time_added::Variables {};
                    let sub_task = OnTimeAdded::subscribe(graphql_task, &self.link, vars, |data| {
                        Self::Message::OnTimeAdded(data)
                    });
                    self.sub_task = Some(sub_task);
                }
            }
            Self::Message::Unsubscribe => {
                self.sub_task = None;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // info!("{:?}", props);
        true
    }

    fn view(&self) -> Html {
        let graphql_task = if self.graphql_task.is_some() {
            html! { <button class="button is-danger" onclick=self.link.callback(|_| Self::Message::SubscriptionDisable)>{"Subscription Disable"}</button> }
        } else {
            html! { <button class="button is-primary" onclick=self.link.callback(|_| Self::Message::SubscriptionEnable)>{"Subscription Enable"}</button> }
        };

        let subscribed = if self.sub_task.is_some() {
            html! { <button class="button is-warning" onclick=self.link.callback(|_| Self::Message::Unsubscribe)>{"Unsubscribe"}</button> }
        } else {
            html! { <button class="button is-success" onclick=self.link.callback(|_| Self::Message::Subscribe)>{"Subscribe"}</button> }
        };
        let position_add = self.link.callback(|_| Self::Message::TimeAdd);
        html! {
            <>
                <span class="is-block is-title has-text-weight-bold is-size-2 has-text-centered mb-4">{"Test"}</span>
                <div class="columns is-centered">
                    <div class="column is-flex is-justify-content-center">
                        <button class="button is-link" onclick=position_add>{"Send Location"}</button>
                    </div>
                    <div class="column is-flex is-justify-content-center">
                        { graphql_task }
                    </div>
                    <div class="column is-flex is-justify-content-center">
                        { subscribed }
                    </div>
                </div>
            </>
        }
    }
}
