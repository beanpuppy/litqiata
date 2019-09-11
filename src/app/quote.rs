use failure::{format_err, Error};
use serde_derive::{Deserialize, Serialize};
use yew::callback::Callback;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

#[derive(Serialize, Deserialize)]
pub struct Quote{
    pub quote: String,
}

#[derive(Default)]
pub struct QuoteService {
    web: FetchService,
}

impl QuoteService {
    pub fn new() -> Self {
        Self {
            web: FetchService::new(),
        }
    }

    pub fn get(&mut self, callback: Callback<Result<Quote, Error>>) -> FetchTask {
        let url = "https://blog.justinduch.com/api/quote";

        let handler = move |response: Response<Json<Result<Quote, Error>>>| {
            let (meta, Json(data)) = response.into_parts();
            if meta.status.is_success() {
                callback.emit(data)
            } else {
                // format_err! is a macro in crate `failure`
                callback.emit(Err(format_err!(
                    "{}: error getting quote",
                    meta.status
                )))
            }
        };

        let request = Request::get(url).body(Nothing).unwrap();
        self.web.fetch(request, handler.into())
    }
}
