use yew::{
    format::{ Json, Nothing },
    prelude::*,
    services::{
        fetch::{FetchService, FetchTask, Request, Response},
        ConsoleService,
    },
};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value, from_value, Map};
use crate::types::var;

// use crate::components::{
//     indexpage_window_createapp::AppCreate,
//     indexpage_window_createindex::IndexCreate,
//     indexpage_window_deleterecord::DeleteRecord,
//     indexpage_window_editrecord::EditRecord,
//     indexpage_window_insertrecord::InsertRecord,
// };

pub enum Msg {
    //EVENT TOGGLE (MERGE CLOSE DAN OPEN)
    ToggleCreateApp,
    ToggleCreateIndex,
    ToggleInsertRecord,
    ToggleEditRecord,
    ToggleDeleteRecord,

    RequestData,
    RequestMoreData,
    GetCardData(Option<Vec<Value>>),
    GetMoreData(Option<Vec<MoreData>>),
    ResponseError(String),

    RequestIndexData,
    GetIndexData(Option<Vec<IndexData>>),

    Ignore,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct IndexData{
    pub index: Option<String>
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MoreData {
    name : Option<String>,
    element: Option<String>,
    level: Option<u32>,
    attack: Option<u32>,
    defense: Option<u32>,
    em : Option<u32>,
    nation: Option<String>,
}


#[derive(Properties, Clone, Debug, PartialEq)]
pub struct IndexPageCompProps {
    // #[prop_or(String::from("this is value"))]
    #[prop_or(false)]
    pub display_create_app: bool,
    #[prop_or(false)]
    pub display_create_index: bool,
    #[prop_or(false)]
    pub display_insert_record: bool,
    #[prop_or(false)]
    pub display_edit_record: bool,
    #[prop_or(false)]
    pub display_delete_record: bool,

    pub on_toggle_createapp:Callback<Msg>,
    pub on_toggle_createindex:Callback<Msg>,
    pub on_toggle_insertrecord:Callback<Msg>,
    pub on_toggle_editrecord:Callback<Msg>,
    pub on_toggle_deleterecord:Callback<Msg>,
    
}


pub struct IndexPageComp {
    link: ComponentLink<Self>,
    //DISPLAY WINDOWS / MODAL (STATE)
    // display_create_app: bool,
    // display_create_index: bool,
    // display_insert_record: bool,
    // display_edit_record: bool,
    // display_delete_record: bool,

    props: IndexPageCompProps,
    callback_toggle_createapp: Callback<Msg>,
    callback_toggle_createindex: Callback<Msg>,
    callback_toggle_insertrecord: Callback<Msg>,
    callback_toggle_editrecord: Callback<Msg>,
    callback_toggle_deleterecord: Callback<Msg>,

    fetch_task: Option<FetchTask>,
    card_data: Option<Vec<Value>>,
    more_data: Option<Vec<MoreData>>,
    index_data: Option<Vec<IndexData>>,
    error: Option<String>,
}

impl Component for IndexPageComp {
    type Message = Msg;
    type Properties = IndexPageCompProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            callback_toggle_createapp: props.on_toggle_createapp.clone(),
            callback_toggle_createindex: props.on_toggle_createindex.clone(),
            callback_toggle_insertrecord: props.on_toggle_insertrecord.clone(),
            callback_toggle_editrecord: props.on_toggle_editrecord.clone(),
            callback_toggle_deleterecord: props.on_toggle_deleterecord.clone(),
            props,
            // DISPLAY WINDOWS / MODAL (STATE)
            fetch_task: None,
            card_data: Some(vec![]),
            error: None,
            more_data: Some(vec![]),
            index_data: Some(vec![]),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            //EVENT BUAT OPEN MODAL
            Msg::ToggleCreateIndex => {
                self.callback_toggle_createindex.emit(Msg::ToggleCreateIndex);
                ConsoleService::info(&format!("DEBUG : display_create_index:{:?}", self.props.display_create_index));
                true
            }
            Msg::ToggleCreateApp => {
                self.callback_toggle_createapp.emit(Msg::ToggleCreateApp);
                ConsoleService::info(&format!("DEBUG : display_create_app:{:?}", self.props.display_create_app));
                true
            }
            Msg::ToggleInsertRecord => {
                self.callback_toggle_insertrecord.emit(Msg::ToggleInsertRecord);
                ConsoleService::info(&format!("DEBUG : display_insert_record:{:?}", self.props.display_insert_record));
                true
            }
            Msg::ToggleEditRecord => {
                self.callback_toggle_editrecord.emit(Msg::ToggleEditRecord);
                ConsoleService::info(&format!("DEBUG : display_edit_record:{:?}", self.props.display_edit_record));
                true
            }
            Msg::ToggleDeleteRecord => {
                self.callback_toggle_deleterecord.emit(Msg::ToggleDeleteRecord);
                ConsoleService::info(&format!("DEBUG : display_delete_record:{:?}", self.props.display_delete_record));
                true
            }
            Msg::Ignore => {
                ConsoleService::info(&format!("DEBUG : Event Ignore", ));
                true
            }

            Msg::RequestIndexData => {
                //FETCHING...
                let request = Request::get("https://search-discovery-api.dev-domain.site/api/index")
                    // .header("access_token", get_access_token{}.unwrap_or_default())
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<Vec<IndexData>, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
                        // let status_number = meta.status.as_u16();
        
                        match data { 
                            Ok(dataok) => {
                                ConsoleService::info(&format!("data response {:?}", &dataok));
                                Msg:: GetIndexData(Some(dataok))
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
            
            Msg::RequestData => {
                //FETCHING...
                let request = Request::get("https://search-discovery-api.dev-domain.site/api/search")
                    // .header("access_token", get_access_token{}.unwrap_or_default())
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<Vec<Value>, anyhow::Error>>>| {
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

            Msg::RequestMoreData => {
                //POST FETCHING...

                let villain = MoreData {
                    name: Some(String::from("Yelan")),
                    element: None,
                    level: None,
                    attack: None,
                    defense: None,
                    em : None,
                    nation: None,
                };

                let request = Request::post("http://localhost:3000/attack")
                    .header("Content-Type", "application/json")
                    // .header(Json(&villain))
                    .body(Json(&villain))
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<Vec<MoreData>, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
                        // let status_number = meta.status.as_u16();
        
                        match data { 
                            Ok(dataok) => {
                                ConsoleService::info(&format!("data response {:?}", &dataok));
                                Msg:: GetMoreData(Some(dataok))
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

            Msg::GetIndexData(data) => {
                ConsoleService::info(&format!("data is {:?}", data));
                self.index_data = data;
                true
            }


            Msg::GetMoreData(data) => {
                ConsoleService::info(&format!("data is {:?}", data));
                self.card_data = None;
                self.more_data = data;
                true
            }

            Msg::GetCardData(data) => {
                ConsoleService::info(&format!("data is {:?}", data));
                self.card_data = data;
                self.more_data = None;
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
			self.link.send_message(Msg::RequestIndexData);
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        //CONDITIONAL DEFAULT CASE (NO MODAL)
            html! {
                <div> 
                        <div>
                            <div class="leftbox index-sidebar-small">
                                <img class="index-logo" src="images/Arbitra_LogoOnly.png"/> 
                            </div>

                            <div class="rightSideBar">
                                <p style="color: #bd3143; font-size: 2rem">{"S E A R C H"}</p>
                                <p style="margin-top: -8px">{ "Application" }</p>

                                <div class="dropdown">
                                    <button class="mainmenubtn"><img class="applicationIcon" src="images/APP.png"/>{ "Scara \u{00a0} \u{23F7}"}</button>
                                    <div class="dropdown-child">
                                        <a 
                                            href="#" 
                                            onclick=self.link.callback(|_| Msg::ToggleCreateApp)>
                                            { "Create New Application" }
                                        </a>
                                        // <a href="#">{ "Link 2" }</a>
                                        // <a href="#">{ "Link 3" }</a>
                                    </div>
                                </div>
                                
                                <br/><br/>

                                <p class="index-directry">{ "\u{007C}\u{00a0} Index" }</p>
                                <p class="index-directry">{ "\u{007C}\u{00a0} Dictionary" }</p>
                                <p class="index-directry">{ "\u{007C}\u{00a0} Lorem Ipsum" }</p>
                                <p class="index-directry">{ "\u{007C}\u{00a0} Lorem Ipsum" }</p>
                            </div>
                        </div>

                        <div>
                            <div class="top-index-dashboard">

                                <div class="dropdownIndex">
                                    <button class="mainmenubtnIndex">{ "INDEX NAME \u{00a0} \u{00a0} \u{00a0} \u{00a0} \u{00a0} \u{00a0} \u{00a0} \u{00a0} \u{00a0} \u{23F7}"}</button>
                                    <div class="dropdown-childIndex">
                                        
                                        { self.view_index_data() }
                                        
                                        <a 
                                            href="#"
                                            onclick=self.link.callback(|_| Msg::ToggleCreateIndex)>
                                            { "Create New Index" }
                                        </a>
                                    </div>
                                </div>

                                <div class="recordData">
                                    <p class="recordNum">{ "No. of Records \u{00a0} \u{00a0} \u{00a0} \u{00a0} 1.000.000.000" }</p>
                                    <p style="float: left;">{ "\u{00a0} \u{00a0} \u{00a0}" }</p>
                                    <p class="recordSize">{ "Average Record Size\u{00a0} \u{00a0} \u{00a0} \u{00a0} 1.000.000.000B" }</p>
                                </div>

                                <br/><br/><br/>

                                <div class="dropdownRecord">
                                    <button class="mainmenubtnRecord">{ "New Record \u{00a0} \u{00a0} \u{00a0} \u{00a0} \u{23F7}"}</button>
                                    <div class="dropdown-childRecord">
                                        <a href="#" onclick=self.link.callback(|_| Msg::ToggleInsertRecord)>{ "Insert New Record" }</a>
                                        <a href="#" onclick=self.link.callback(|_| Msg::ToggleEditRecord)>{ "Edit Record" }</a>
                                        <a href="#" onclick=self.link.callback(|_| Msg::ToggleDeleteRecord)>{ "Delete Record" }</a>
                                    </div>
                                </div>

                                <div class="dropdownRecord">
                                    <button class="mainmenubtnRecord">{ "Add Records \u{00a0} \u{00a0} \u{00a0} \u{23F7}"}</button>
                                    <div class="dropdown-childRecord">
                                        <a href="#">{ "Upload File" }</a>
                                        <a href="#">{ "Use the API" }</a>
                                        <a href="#">{ "Add Manually" }</a>
                                    </div>
                                </div>

                                <div class="dropdownRecord">
                                    <button class="mainmenubtnRecord">{ "Manage Index \u{00a0} \u{00a0} \u{00a0} \u{23F7}"}</button>
                                    <div class="dropdown-childRecord">
                                        <a href="#">{ "Rename" }</a>
                                        <a href="#">{ "Duplicate" }</a>
                                        <a href="#">{ "Copy Settings" }</a>
                                        <a href="#">{ "Clear" }</a>
                                        <a href="#">{ "Delete" }</a>
                                    </div>
                                </div>

                                <img class="copyIcon" src="images/Copy Icon.png"/>
                                <img class="copyIcon" src="images/Refresh.png"/>

                            </div>
                        </div>

                        <div class="bottom-index-dashboard">
                            <div class="flex-container">
                                <button class="subtab-p">{ "Browse" }</button>
                                <button class="subtab-p">{ "Configuration" }</button>
                                <button class="subtab-p">{ "Replicas" }</button>
                                <button class="subtab-p">{ "Search API Records" }</button>
                                <button class="subtab-p">{ "Stats" }</button>
                                <button class="subtab-p">{ "UI Demos" }</button>
                            </div>

                            <div class="card">
                                <div class="search-bar">
                                    <input class="search" type="text" placeholder="Search..."/>
                                </div>

                                <div>
                                    <button style="margin-left: 45%; margin-top: 1%" onclick=self.link.callback(|_| Msg::RequestData)>{ "Get Data" }</button>
                                </div>

                                <div>
                                    <button style="margin-left: 45%; margin-top: 1%" onclick=self.link.callback(|_| Msg::RequestMoreData)>{ "Get More Data (Post)" }</button>
                                </div>

                                <div>
                                    { self.view_data() }
                                    { self.view_more_data() }

                                    {
                                        if self.card_data.clone().unwrap_or_default().is_empty() && self.more_data.clone().unwrap_or_default().is_empty() {
                                            html!{
                                                <div class="alert alert-danger m-4" role="alert">
                                                    { "No Record in this Index...  " }
                                                    
                                                    <a href="#" onclick=self.link.callback(|_| Msg::ToggleInsertRecord)>
                                                        { "Insert New Record" }
                                                    </a>

                                                </div>
                                            }
                                        } else {
                                            html! {
                                                //NOTHING YET
                                            }
                                        }
                                    }
                                </div>

                            </div>      

                        </div>

                    </div>
               
        }
            //BODY END
        }
    }

impl IndexPageComp {
    fn view_index_data(&self) -> Vec<Html> {
        self.index_data.iter().map(|card|{
                card.iter().map(|card_parse|{
                    html!{
                        <a class="index-name">
                            { serde_json::to_string(card_parse).unwrap() }
                        </a>
                    }
                }).collect()
                
            }).collect()
    }

    fn view_data(&self) -> Vec<Html> {
        self.card_data.iter().map(|card|{
                card.iter().map(|card_parse|{
                    html!{
                        <div class="index-card">
                            { serde_json::to_string(card_parse).unwrap() }
                        </div>
                    }
                }).collect()
                
            }).collect()
    }

    fn view_more_data(&self) -> Vec<Html> {
        self.more_data.iter().map(|card|{
                card.iter().map(|card_parse|{
                    html!{
                        <div class="index-card">
                            { serde_json::to_string(card_parse).unwrap() }
                        </div>
                    }
                }).collect()
                
            }).collect()
    }
}