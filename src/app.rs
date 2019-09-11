mod quote;
use quote::{QuoteService, Quote};

use failure::Error;

use stdweb::js;
use stdweb::unstable::TryFrom;
use stdweb::web::Node;

use yew::virtual_dom::VNode;
use yew::callback::Callback;
use yew::services::fetch::FetchTask;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct App {
    quote_service: QuoteService,
    quote: Option<Quote>,
    callback: Callback<Result<Quote, Error>>,
    task: Option<FetchTask>,
}

pub enum Msg {
    Quote,
    QuoteReady(Result<Quote, Error>),
}


impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let mut app = App {
            callback: link.send_back(Msg::QuoteReady),
            task: None,
            quote_service: QuoteService::new(),
            quote: None,
        };

        app.update(Msg::Quote);

        app
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Quote => {
                self.quote = None;

                let task = self
                    .quote_service
                    .get(self.callback.clone());

                self.task = Some(task);
            }
            Msg::QuoteReady(Ok(quote)) => self.quote = Some(quote),
            Msg::QuoteReady(Err(_)) => { /* Can't load quote */ }
        }
        true
    }
}

impl Renderable<App> for App {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="container">
                <h1>{ "Life Is the Question, I Am the Answer" }</h1>
                <small>
                    { "random quotes and images from " }
                    <a href="https://blog.justinduch.com" target="_blank">{ "https://blog.justinduch.com" }</a>
                    { " using /api/quote." }
                </small>
                <p>{ self.view_quote() }</p>
                <div class="footer">
                    <button onclick=|_| Msg::Quote>{ "Refresh" }</button>
                    <small>
                        <a href="https://github.com/beanpuppy/litqiata" target="_blank">
                            { "view the source code" }
                        </a>
                    </small>
                </div>
            </div>
        }
    }
}

impl App {
    fn view_quote(&self) -> Html<App> {
        let quote = self.quote.as_ref();

        match quote {
            Some(_) => {
                let tag = js! {
                    var div = document.createElement("div");
                    div.innerHTML = @{&self.quote.as_ref().unwrap().quote};
                    return div;
                };

                let node = Node::try_from(tag).expect("convert tag");

                html! { VNode::VRef(node) }
            },
            None => html! { <p>{ "Loading..." }</p> }
        }
    }
}
