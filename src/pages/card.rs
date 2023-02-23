use yew::{
    format::{ Json, Nothing },
    prelude::*,
    services::{
        fetch::{FetchService, FetchTask, Request, Response},
        ConsoleService,
    },
};

use serde_json::{from_str, Value};


use crate::types::var::{
    Record, 
    RecordData,
};

pub enum Msg {
    RequestData,
    GetCardData(Option<Value>),
    ResponseError(String),
}

pub struct CardTemp {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
    card_data: Option<Value>,
    error: Option<String>,
}

impl Component for CardTemp {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            fetch_task: None,
            link,
            card_data: None,
            error: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RequestData => {
                //FETCHING...
                let request = Request::get("http://localhost:3000/index_card_data")
                    // .header("access_token", get_access_token{}.unwrap_or_default())
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<serde_json::Value, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
                        // let status_number = meta.status.as_u16();
        
                        match data { 
                            Ok(dataok) => {
                                ConsoleService::info(&format!("data response {:?}", &dataok));
                                Msg:: GetCardData(Some(dataok))
                            }
                            Err(error) => {
                                Msg::ResponseError(error.to_string())
                            }
                        }
                    });
        
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                
                self.fetch_task = Some(task);
                true
            }

            Msg::GetCardData(data) => {
                ConsoleService::info(&format!("data is {:?}", data));
                self.card_data = data;
                true
            }

            Msg::ResponseError(text) => {
                ConsoleService::info(&format!("error is {:?}", text));
                true
            }


        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
			self.link.send_message(Msg::RequestData);
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                { self.view_data() }
            </div>
            //BODY END
        }
    }
}

// impl fmt::Display for CardTemp {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "({:?})", self.card_data)
//     }
// }

impl CardTemp {
    fn view_data(&self) -> Html{
        let card_display= serde_json::to_string(&self.card_data).unwrap();
            html!{
                
                <div class="index-card">
                    { card_display }
                </div>
            }
        }
}