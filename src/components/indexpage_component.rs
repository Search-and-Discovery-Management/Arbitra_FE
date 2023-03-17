use yew::{
    format::{ Json, Nothing },
    prelude::*,
    services::{
        fetch::{FetchService, FetchTask, Request, Response},
        ConsoleService,
    },
};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value, from_value, to_string_pretty,Map};

use crate::types::var;
use crate::types::var::EditModalData;
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
    ToggleEditRecord(String, String),
    ToggleDeleteRecord,
    ToggleDeleteCard(String),

    RequestRecordData,
    GetRecordData(Value),
    ResponseError(String),

    RequestIndexData,
    GetIndexData(Option<Vec<IndexData>>),

    SelectIndex(String),

    SendEditToParent(EditModalData),
    SendDeleteToParent(String),

    Ignore,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct IndexData{
    pub index: String
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
    pub edit_index: String,

    pub callback_edit_data: Callback<EditModalData>,

    #[prop_or(false)]
    pub display_delete_record: bool,

    #[prop_or(false)]
    pub display_delete_card: bool,
    #[prop_or_default]
    pub delete_index: String,

    pub callback_delete_window: Callback<String>,

    pub on_toggle_createapp:Callback<Msg>,
    pub on_toggle_createindex:Callback<Msg>,
    pub on_toggle_insertrecord:Callback<Msg>,
    pub on_toggle_editrecord:Callback<Msg>,
    pub on_toggle_deleterecord:Callback<Msg>,
    pub on_toggle_deletecard:Callback<Msg>,
    
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
    callback_toggle_deletecard: Callback<Msg>,

    fetch_task: Option<FetchTask>,
    record_data: Value,
    // more_data: Option<Vec<MoreData>>,
    index_data: Option<Vec<IndexData>>,
    error: Option<String>,
    index_name: String,

    callback_edit_data: Callback<EditModalData>,
    callback_delete_window: Callback<String>,
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
            callback_toggle_deletecard: props.on_toggle_deletecard.clone(),

            // DISPLAY WINDOWS / MODAL (STATE)
            fetch_task: None,
            record_data: serde_json::json!({"data": []}),
            error: None,
            // more_data: Some(vec![]),
            index_data: Some(vec![]),

            index_name: String::from("SELECT INDEX ..."),

            callback_edit_data: props.callback_edit_data.clone(),
            callback_delete_window: props.callback_delete_window.clone(),
            props,
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
            Msg::ToggleEditRecord (data, index)=> {

                ConsoleService::info(&format!("DEBUG : display_edit_record:{:?}", self.props.display_edit_record));
                ConsoleService::info(&format!("DEBUG : data INDEX PAGE CHILD:{:?}", data.clone()));
                ConsoleService::info(&format!("DEBUG : index INDEX PAGE CHILD:{:?}", index.clone()));
                
                self.callback_toggle_editrecord.emit(Msg::ToggleEditRecord(data, index));
                true
            }
            Msg::ToggleDeleteRecord => {
                self.callback_toggle_deleterecord.emit(Msg::ToggleDeleteRecord);
                ConsoleService::info(&format!("DEBUG : display_delete_record:{:?}", self.props.display_delete_record));
                true
            }
            Msg::ToggleDeleteCard (index) => {
                ConsoleService::info(&format!("DEBUG : delete_index EVENT :{:?}", index));
                ConsoleService::info(&format!("DEBUG : display_delete_card:{:?}", self.props.display_delete_card));
                self.callback_toggle_deletecard.emit(Msg::ToggleDeleteCard(index));
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

            Msg::SelectIndex(index) => {
                ConsoleService::info(&format!("Selected index: {:?}", index));
                self.index_name = index;
                self.link.send_message(Msg::RequestRecordData);
                true
            }

            Msg::GetIndexData(data) => {
                ConsoleService::info(&format!("data is {:?}", data));
                self.index_data = data;
                true
            }
            
            Msg::RequestRecordData => {
                //FETCHING...
                let request = Request::get(format!("https://search-discovery-api.dev-domain.site/api/search/{}", &self.index_name))
                    // .header("access_token", get_access_token{}.unwrap_or_default())
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<Value, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
        
                        match data { 
                            Ok(dataok) => {
                                ConsoleService::info(&format!("data response {:?}", &dataok));
                                Msg:: GetRecordData(dataok)
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

            Msg::GetRecordData(data) => {
                ConsoleService::info(&format!("data is {:?}", data.get("data").unwrap().as_array().unwrap()));
                self.record_data = data;
                true
            }

            //UNTUK NGIRIM DATA DI CARD KE EDIT MODAL!!!
            Msg::SendEditToParent(data) => {
                self.callback_edit_data.emit(data);
                true
            }
            //UNTUK NGIRIM DATA DI CARD KE DELETE MODAL (KE PARENT DULU)
            Msg::SendDeleteToParent(index) => {
                self.callback_delete_window.emit(index);
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
			self.link.send_message(Msg::RequestIndexData)
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
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
                                    <button class="mainmenubtnIndex">{ format!("{} \u{00a0} \u{00a0} \u{00a0} \u{00a0} \u{00a0} \u{00a0} \u{00a0} \u{00a0} \u{00a0} \u{23F7}", &self.index_name)}</button>
                                    <div class="dropdown-childIndex">
                                    
                                        { self.view_index_data() }
                                        
                                        <a 
                                            href="#"
                                            onclick=self.link.callback(|_| Msg::ToggleCreateIndex)
                                            style="background-color: #e3e8ed">
                                            { "Create New Index" }
                                        </a>
                                        <a 
                                            href="#"
                                            //Untuk sementara pakai yang delete record dlu
                                            onclick=self.link.callback(|_| Msg::ToggleDeleteRecord)
                                            style="color: white; background-color: #a73034">
                                            { "Remove Index" }
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
                                        // <a href="#" onclick=self.link.callback(|_| Msg::ToggleEditRecord)>{ "Edit Record" }</a>
                                        // <a href="#" onclick=self.link.callback(|_| Msg::ToggleDeleteRecord)>{ "Delete Record" }</a>
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
                                    { self.view_data() }

                                    // {
                                    //     if self.record_data.clone().unwrap_or_default.is_empty() {
                                    //         html!{
                                    //             <div class="alert alert-danger m-4" role="alert">
                                    //                 { "No Record in this Index...  " }
                                                    
                                    //                 <a href="#" onclick=self.link.callback(|_| Msg::ToggleInsertRecord)>
                                    //                     { "Insert New Record" }
                                    //                 </a>

                                    //             </div>
                                    //         }
                                    //     } else {
                                    //         html! {
                                    //             //NOTHING YET
                                    //         }
                                    //     }
                                    // }
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
                    let index_name = card_parse.index.clone();
                    html!{
                        <a class="index-name" onclick=self.link.callback(move |_| Msg::SelectIndex(index_name.clone()))>
                            // { serde_json::to_string_pretty(&card_parse.index).unwrap().trim_start().replace("\"", "")}
                            { card_parse.index.clone() }
                        </a>
                    }
                }).collect()
                
            }).collect()
    }

    fn view_data(&self) -> Vec<Html> {
        self.record_data.get("data").unwrap().as_array()
            .unwrap().iter().enumerate().map(|(i,card)|{
                
                let edit_text_data = serde_json::to_string(card).unwrap();

                let edit_index = serde_json::to_string_pretty(card.get("_id").unwrap()).unwrap();
                let delete_index = serde_json::to_string_pretty(card.get("_id").unwrap()).unwrap();

                let edit_modal_data = EditModalData{    
                    data: edit_text_data.clone(),
                    index: edit_index.clone(),
                        };


    // fn view_data(&self) -> Vec<Html> {
    //     self.record_data.get("data").unwrap().as_array()
    //                 .unwrap().iter().map(|card|{
                // card.iter().map(|card_parse|{
                //     html!{
                //         <div class="index-card">
                //             { serde_json::to_string(card_parse).unwrap() }
                //         </div>
                //     }
                // }).collect()

                // let x: Vec<String> = card.get("fields").unwrap().as_array().unwrap().iter().map(|f| f[0].to_string()).collect();
                
                // let x: Vec<Value> = card.get("fields").unwrap().as_array().unwrap().iter();

                ConsoleService::info(&format!("DEBUG : display_create_index:{:?}", self.props.display_create_index));
                
                html!{
                        <div class="index-card">
                            <div class="card-main">
                                <div class="card-sub">

                                    <div class="card-number">
                                        {"#"}{i+1}
                                    </div>
                                        

                                    // {for i in x.iter()}
                                    // {x.iter().flat_map(|s| s.iter()).for_each(|(key, value)| {
                                    //     html!{
                                    //         <h1> {value} </h1>
                                    //     }
                                            
                                        // })}

                                    <div class="card-json">    
                                        <pre>
                                            {"_id: "}{ serde_json::to_string_pretty(card.get("_id").unwrap()).unwrap() }{"\n"}
                                            {"_score: "}{ serde_json::to_string_pretty(card.get("_score").unwrap()).unwrap() }{"\n"}
                                                // {x}
                                                // { card.get("fields").unwrap().as_array().unwrap().iter().map(|f| f.as_array()).collect() }
                                                // { for x.iter().map( serde_json::to_string_pretty(i))  }
                                                // {serde_json::to_vec_pretty(&xfdas)}
                                                
                                            <p style="color: black; font-size:12px;font-weight: bold; line-height: 1.8;">
                                                { serde_json::to_string_pretty(card.get("fields").unwrap()).unwrap().replace(&['{', '}','"','_', '[', ']'], "") }
                                            </p>
                                        </pre>
                                                // {
                                                //     for card.get("fields").unwrap().as_array().iter().for_each(|val|
                                                
                                                //     html!{
                                                //         serde_json::to_string_pretty(val).unwrap()
                                                //     }
                                                // )
                                                // }
                                                // { serde_json::to_string_pretty(json!(x.clone()))}
                                                // { x.iter().map(|y| serde_json::to_string_pretty(y)) }
                                            
                                    </div>
                                </div>
                                // <div class="card-image">


                                    <img class="card-image-data" src="images/img-card/JeanGunnhildr.png"/>
                                    // <img class="card-image-placeholder" src="images/img-card/no-pictures.png"/>
                                // </div> 
                            </div>

                             
                            <div class="index-card-buttons">
                                <button
                                    type="button"
                                    class="card-button"
                                    onclick=self.link.batch_callback(move |_| vec![
                                        Msg::SendDeleteToParent(delete_index.clone()),
                                        Msg::ToggleDeleteCard(delete_index.clone())
                                    ]
                                    )
                                >
                                    <img class="card-icon" src="images/trash-can.png"/>
                                    
                                </button>

                                <button 
                                    type="button"
                                    class="card-button"
                                    // onclick=self.link.callback(move |_| Msg::SendEditToParent(edit_modal_data.clone()))
                                    // onclick=self.link.callback(move |_| Msg::ToggleEditRecord(edit_text_data.clone(), edit_index.clone()))

                                    onclick= self.link.batch_callback(move |_| vec![
                                        Msg::SendEditToParent(edit_modal_data.clone()),
                                        Msg::ToggleEditRecord(edit_text_data.clone(), edit_index.clone()),
                                    ]
                                    )
                                >
                                    <img class="card-icon" src="images/edit.png"/>
                                    
                                </button>
                            </div>           


                        </div>
                        }
                
            }).collect()
    }

    // fn view_more_data(&self) -> Vec<Html> {
    //     self.more_data.iter().map(|card|{
    //             card.iter().map(|card_parse|{
    //                 html!{
    //                     <div class="index-card">
    //                         { serde_json::to_string(card_parse).unwrap() }
    //                     </div>
    //                 }
    //             }).collect()
                
    //         }).collect()
    // }
}