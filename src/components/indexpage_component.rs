use yew::{
    format::{ Json, Nothing },
    prelude::*,
    services::{
        fetch::{FetchService, FetchTask, Request, Response},
        ConsoleService,
    },
};
use serde_json::{from_str, Value, from_value, to_string_pretty};

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
    ToggleEditRecord(String, usize),
    ToggleDeleteRecord,

    RequestData,
    GetCardData(Option<Vec<Value>>),
    ResponseError(String),

    Ignore,
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
    #[prop_or_default]
    pub edit_data: String,
    #[prop_or_default]
    pub edit_index: usize,
    
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


    props: IndexPageCompProps,
    callback_toggle_createapp: Callback<Msg>,
    callback_toggle_createindex: Callback<Msg>,
    callback_toggle_insertrecord: Callback<Msg>,
    callback_toggle_editrecord: Callback<Msg>,
    callback_toggle_deleterecord: Callback<Msg>,

    fetch_task: Option<FetchTask>,
    card_data: Option<Vec<Value>>,
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
            Msg::ToggleEditRecord (data, index) => {
                
                // self.props.edit_data.emit("Hello World".to_string());
                ConsoleService::info(&format!("DEBUG : display_edit_record:{:?}", self.props.display_edit_record));
                ConsoleService::info(&format!("DEBUG : data INDEX PAGE CHILD:{:?}", data.clone()));
                ConsoleService::info(&format!("DEBUG : index INDEX PAGE CHILD:{:?}", index.clone()));

                self.callback_toggle_editrecord.emit(Msg::ToggleEditRecord("Hello world".to_string(), index));
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
            
            Msg::RequestData => {
                //FETCHING...
                let request = Request::get("http://localhost:3000/index_card_data")
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
            // ConsoleService::info(&format!("DEBUG : self.edit_data INDEX PAGE COMPONENT:{:?}", self.props.edit_data));
			self.link.send_message(Msg::RequestData);
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.edit_data != props.edit_data || self.props.edit_index != props.edit_index {
            self.props.edit_data = props.edit_data;
            self.props.edit_index = props.edit_index;
            // self.role_permissions = props.role_permissions;
            true
        }else {
            false
        }
        
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
                                        <a 
                                            href="#"
                                            onclick=self.link.callback(|_| Msg::ToggleCreateIndex)>
                                            { "Create New Index" }
                                        </a>
                                        // <a href="#">{ "Link 2" }</a>
                                        // <a href="#">{ "Link 3" }</a>
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
                                        // <a href="#" onclick=self.link.callback(|_| Msg::ToggleEditRecord)>{ "Edit Record" }</a>
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
                                    <input class="search" type="text" placeholder="Search..." />
                                </div>

                                <div>
                                    { self.view_data() }

                                    {
                                        if self.card_data.clone().unwrap().is_empty() {
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
    fn view_data(&self) -> Vec<Html> {
        self.card_data.iter().enumerate().map(|(i,cards)|{
                cards.iter().enumerate().map(|(i,card_parse)|{
                    // //
                    // let card_value = to_string_pretty(&card_parse).unwrap();
                    // let card_trim = card_value.trim_start().trim_end().to_string();
                    // //

                    let edit_text_data = serde_json::to_string(card_parse).unwrap();
                    let edit_index = i.clone();
                   
                    
                    html!{
                        <div class="index-card">
                            <div class="card-main">
                                <div class="card-number">
                                    {"#"}{i+1}
                                </div>

                                <pre class="card-json">
                                    <code style="font-size:12px;font-weight: bold; line-height: 1.8;">
                                        { serde_json::to_string_pretty(card_parse).unwrap().replace(&['{', '}','"'], "") }
                                    </code>
                                </pre>
                            
                            </div>


                            <div class="index-card-buttons">
                                <button 
                                    type="button"
                                    class="card-button"
                                    onclick=self.link.callback(|_| Msg::ToggleDeleteRecord)
                                >
                                    <img class="card-icon" src="images/trash-can.png"/>
                                    
                                </button>

                                <button 
                                    type="button"
                                    class="card-button"
                                    onclick=self.link.callback(move |_| Msg::ToggleEditRecord(edit_text_data.clone(), edit_index.clone()))
                                >
                                    <img class="card-icon" src="images/edit.png"/>
                                    
                                </button>
                            </div>
                        </div>
                    }
                }).collect()
                
            }).collect()
        }
}