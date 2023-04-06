use yew::{
    format::{ Json, Nothing },
    prelude::*,
    services::{
        fetch::{FetchService, FetchTask, Request, Response},
        ConsoleService,
    },
};
use serde::{Deserialize, Serialize};
use serde_json::{Value};
use crate::types::var::EditModalData;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AppData{
    pub _id: String,
    pub _source: Value
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct IndexData{
    pub index: String
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SearchRecord {
    index: String,
    search_term: String,
    from: u32,
    count: u32
}

pub enum Msg {
    //EVENT TOGGLE (MERGE CLOSE DAN OPEN)
    ToggleCreateApp,
    ToggleDeleteApp,
    ToggleCreateIndex,
    ToggleInsertRecord,
    ToggleEditRecord(String, String, String),
    ToggleDeleteRecord,
    ToggleDeleteCard(String, String),

    RequestRecordData,
    GetRecordData(Value),
    ResponseError(String),

    RequestIndexData,
    GetIndexData(Option<Vec<IndexData>>),

    RequestAppData,
    GetAppData(Option<Vec<AppData>>),
    
    RequestDeleteApp,

    SelectApp(String),

    SelectIndex(String),

    SendEditToParent(EditModalData),
    SendDeleteToParent(String),
    SendIndexNameToParent(String),
    SendAppIdToParent(String),

    InputSearch(String),
    RequestSearch(String),

    Ignore,
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct IndexPageCompProps {
    #[prop_or(false)]
    pub display_create_app: bool,

    #[prop_or(false)]
    pub display_delete_app: bool,

    #[prop_or_default]
    pub app_id: String,
    pub callback_app_id: Callback<String>,

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


    #[prop_or_default]
    pub card_index: String,
    
    pub callback_card_index: Callback<String>,

    pub on_toggle_createapp:Callback<Msg>,
    pub on_toggle_deleteapp:Callback<Msg>,
    pub on_toggle_createindex:Callback<Msg>,
    pub on_toggle_insertrecord:Callback<Msg>,
    pub on_toggle_editrecord:Callback<Msg>,
    pub on_toggle_deleterecord:Callback<Msg>,
    pub on_toggle_deletecard:Callback<Msg>,
    

    #[prop_or(false)]
    pub modal_open_index: bool,
    #[prop_or(false)]
    pub modal_open_record: bool,
    //BUAT MODAL CREATEAPP MSIH BLM SKRG
    #[prop_or(false)]
    pub modal_open_app: bool,
    
}


pub struct IndexPageComp {
    link: ComponentLink<Self>,
    props: IndexPageCompProps,

    callback_toggle_createapp: Callback<Msg>,
    callback_toggle_deleteapp: Callback<Msg>,
    callback_toggle_createindex: Callback<Msg>,
    callback_toggle_insertrecord: Callback<Msg>,
    callback_toggle_editrecord: Callback<Msg>,
    callback_toggle_deleterecord: Callback<Msg>,
    callback_toggle_deletecard: Callback<Msg>,
    
    callback_edit_data: Callback<EditModalData>,
    callback_delete_window: Callback<String>,
    callback_card_index: Callback<String>,
    callback_app_id: Callback<String>,

    fetch_task: Option<FetchTask>,
    record_data: Value,
    index_data: Option<Vec<IndexData>>,
    index_name: String,
    error: Option<String>,
    search_input: String,
    app_data: Option<Vec<AppData>>,
    app_id: String
}

impl Component for IndexPageComp {
    type Message = Msg;
    type Properties = IndexPageCompProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            callback_toggle_createapp: props.on_toggle_createapp.clone(),
            callback_toggle_deleteapp: props.on_toggle_deleteapp.clone(),
            callback_toggle_createindex: props.on_toggle_createindex.clone(),
            callback_toggle_insertrecord: props.on_toggle_insertrecord.clone(),
            callback_toggle_editrecord: props.on_toggle_editrecord.clone(),
            callback_toggle_deleterecord: props.on_toggle_deleterecord.clone(),
            callback_toggle_deletecard: props.on_toggle_deletecard.clone(),

            // DISPLAY WINDOWS / MODAL (STATE)
            fetch_task: None,
            record_data: serde_json::json!({"data": []}),
            error: None,

            index_name: String::from("SELECT INDEX ..."),
            index_data: Some(vec![]),

            app_id: String::from(""),
            app_data: Some(vec![]),

            search_input: String::from(""),

            callback_edit_data: props.callback_edit_data.clone(),
            callback_delete_window: props.callback_delete_window.clone(),
            callback_card_index: props.callback_card_index.clone(),
            callback_app_id: props.callback_app_id.clone(),
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            //EVENT BUAT OPEN MODAL
            Msg::ToggleCreateIndex => {
                self.callback_toggle_createindex.emit(Msg::ToggleCreateIndex);
                // ConsoleService::info(&format!("DEBUG : display_create_index:{:?}", self.props.display_create_index));
                ConsoleService::info(&format!("DEBUG : modal_open COMPONENT:{:?}", self.props.modal_open_index));
                true
            }

            Msg::ToggleCreateApp => {
                self.callback_toggle_createapp.emit(Msg::ToggleCreateApp);
                // ConsoleService::info(&format!("DEBUG : display_create_app:{:?}", self.props.display_create_app));
                ConsoleService::info(&format!("DEBUG : modal_open COMPONENT:{:?}", self.props.modal_open_app));
                true
            }

            Msg::ToggleDeleteApp => {
                self.callback_toggle_deleteapp.emit(Msg::ToggleDeleteApp);
                // ConsoleService::info(&format!("DEBUG : display_create_app:{:?}", self.props.display_delete_app));
                ConsoleService::info(&format!("DEBUG : modal_open COMPONENT:{:?}", self.props.modal_open_app));
                true
            }

            Msg::ToggleInsertRecord => {
                self.callback_toggle_insertrecord.emit(Msg::ToggleInsertRecord);
                // ConsoleService::info(&format!("DEBUG : display_insert_record:{:?}", self.props.display_insert_record));
                ConsoleService::info(&format!("DEBUG : modal_open COMPONENT:{:?}", self.props.modal_open_record));
                true
            }

            Msg::ToggleEditRecord (data, index, card_index)=> {

                // ConsoleService::info(&format!("DEBUG : display_edit_record:{:?}", self.props.display_edit_record));
                // ConsoleService::info(&format!("DEBUG : data INDEX PAGE CHILD:{:?}", data.clone()));
                // ConsoleService::info(&format!("DEBUG : index INDEX PAGE CHILD:{:?}", index.clone()));
                // ConsoleService::info(&format!("DEBUG : card_index EVENT :{:?}", card_index));
                
                self.callback_toggle_editrecord.emit(Msg::ToggleEditRecord(data, index, card_index));
                ConsoleService::info(&format!("DEBUG : modal_open COMPONENT:{:?}", self.props.modal_open_record));
                true
            }

            Msg::ToggleDeleteRecord => {
                self.callback_toggle_deleterecord.emit(Msg::ToggleDeleteRecord);
                // ConsoleService::info(&format!("DEBUG : display_delete_record:{:?}", self.props.display_delete_record));
                ConsoleService::info(&format!("DEBUG : modal_open COMPONENT:{:?}", self.props.modal_open_index));
                true
            }

            Msg::ToggleDeleteCard (index, card_index) => {
                // ConsoleService::info(&format!("DEBUG : delete_index EVENT :{:?}", index));
                // ConsoleService::info(&format!("DEBUG : card_index EVENT :{:?}", card_index));
                // ConsoleService::info(&format!("DEBUG : display_delete_card:{:?}", self.props.display_delete_card));
                self.callback_toggle_deletecard.emit(Msg::ToggleDeleteCard(index, card_index));
                ConsoleService::info(&format!("DEBUG : modal_open COMPONENT:{:?}", self.props.modal_open_record));
                true
            }

            Msg::RequestSearch(data) => {
                let mut search_term = SearchRecord{
                    index: self.index_name.clone(),
                    search_term: String::from(""),
                    from: 0,
                    count: 50
                };
                if data.is_empty() {
                    search_term.search_term = String::from("*");
                }else {
                    search_term.search_term = data;
                }
                let request = Request::post("https://search-discovery-api.dev-domain.site/api/search")
                    .header("Content-Type", "application/json")
                    .body(Json(&search_term))
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<Value, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
        
                        match data { 
                            Ok(dataok) => {
                                // ConsoleService::info(&format!("data response {:?}", &dataok));
                                Msg::GetRecordData(dataok)
                            }
                            Err(error) => {
                                Msg::Ignore
                            }
                        }
                    });
        
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                
                self.fetch_task = Some(task);
                true
            }

            Msg::InputSearch(data) => {
                // ConsoleService::info(&format!("Input Data for Search: {:?}", data));
                self.link.send_message(Msg::RequestSearch(data));
                true
            }

            Msg::RequestIndexData => {
                //FETCHING...
                let url = format!("https://test-dps-api.dev-domain.site/api/index/{}", &self.app_id);
                let request = Request::get(url)
                    // .header("access_token", get_access_token{}.unwrap_or_default())
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<Vec<IndexData>, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
        
                        match data { 
                            Ok(dataok) => {
                                // ConsoleService::info(&format!("data response {:?}", &dataok));
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
                // ConsoleService::info(&format!("data is {:?}", data));
                self.index_data = data;
                true
            }

            Msg::RequestAppData => {
                let request = Request::get("https://test-dps-api.dev-domain.site/api/apps")
                    // .header("access_token", get_access_token{}.unwrap_or_default())
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<Vec<AppData>, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
        
                        match data { 
                            Ok(dataok) => {
                                ConsoleService::info(&format!("data response {:?}", &dataok));
                                Msg:: GetAppData(Some(dataok))
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

            Msg::GetAppData(data) => {
                self.app_data = data;
                true
            }

            Msg::SelectApp(app) => {
                // ConsoleService::info(&format!("Selected index: {:?}", index));
                self.app_id = app;
                self.link.send_message(Msg::RequestIndexData);
                true
            }

            Msg::RequestDeleteApp => {
                let url = format!("https://test-dps-api.dev-domain.site/api/app/:app_id", );
                // ConsoleService::info(&format!("RecordID: {:?}", self.props.delete_index));
                let request = Request::delete(url)
                    // .header("Content-Type", "application/json")
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<String, anyhow::Error>>>| {
                        Msg::Ignore
                    });
        
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                
                self.fetch_task = Some(task);
                true
            }
            
            Msg::RequestRecordData => {
                //FETCHING...
                let request = Request::get(format!("https://test-dps-api.dev-domain.site/api/search/{}/{}", &self.app_id, &self.index_name))
                    // .header("access_token", get_access_token{}.unwrap_or_default())
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<Value, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
        
                        match data { 
                            Ok(dataok) => {
                                // ConsoleService::info(&format!("data response {:?}", &dataok));
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
                // ConsoleService::info(&format!("data is {:?}", data.get("data").unwrap().as_array().unwrap()));
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

            //UNTUK NGIRIM DATA INDEX KE PARENT
            Msg::SendIndexNameToParent(data) => {
                self.callback_card_index.emit(data);
                true
            }
            
            //UNTUK NGIRIM DATA APP ID KE PARENT
            Msg::SendAppIdToParent(data) => {
                self.callback_app_id.emit(data);
                true
            }
 
            Msg::ResponseError(text) => {
                ConsoleService::info(&format!("error is {:?}", text));
                true
            }

            Msg::Ignore => {
                ConsoleService::info(&format!("DEBUG : Event Ignore", ));
                true
            }
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
			self.link.send_message(Msg::RequestAppData)
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if  self.props.modal_open_index != props.modal_open_index {

            self.props.modal_open_index = props.modal_open_index;
            self.link.send_message(Msg::RequestIndexData);
            true
        } else if self.props.modal_open_record != props.modal_open_record {

            self.props.modal_open_record = props.modal_open_record;
            self.link.send_message(Msg::RequestRecordData);
            // ConsoleService::info(&format!("DEBUG : modal_open COMPONENT Fn change:{:?}", self.props.modal_open_record));
            true
        } else if self.props.modal_open_app != props.modal_open_app {
            
            self.props.modal_open_app = props.modal_open_app;
            true
        } else {
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
                                    <button class="mainmenubtn"><img class="applicationIcon" src="images/APP.png"/>{ format!("{} \u{00a0} \u{23F7}", &self.app_id)}</button>
                                    <div class="dropdown-child">

                                        { self.view_app_data() }
                                        <a 
                                            href="#" 
                                            onclick=self.link.callback(|_| Msg::ToggleCreateApp)
                                            style="background-color: #e3e8ed">
                                            { "Create New Application" }
                                        </a>

                                        <a 
                                            href="#" 
                                            onclick=self.link.callback(|_| Msg::ToggleDeleteApp)
                                            style="color: white; background-color: #a73034">
                                            { "Remove Application" }
                                        </a>
                                    </div>
                                </div>
                                
                                <br/><br/>

                                <p class="index-directry">{ "\u{007C}\u{00a0} Index" }</p>
                                <p class="index-directry">{ "\u{007C}\u{00a0} Dictionary" }</p>
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

                                //Add Record Dropdown
                                // <div class="dropdownRecord">
                                //     <button class="mainmenubtnRecord">{ "Add Records \u{00a0} \u{00a0} \u{00a0} \u{23F7}"}</button>
                                //     <div class="dropdown-childRecord">
                                //         <a href="#">{ "Upload File" }</a>
                                //         <a href="#">{ "Use the API" }</a>
                                //         <a href="#">{ "Add Manually" }</a>
                                //     </div>
                                // </div>

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
                                <a onclick=self.link.callback(|_| Msg::RequestRecordData)><img class="copyIcon" src="images/Refresh.png"/></a>

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
                                    <input
                                    class="search"
                                    type="text"
                                    placeholder="Search..."
                                    oninput = self.link.callback(|data: InputData| Msg::InputSearch(data.value))
                                    />
                                </div>

                                <div>
                                    
                                    { self.view_data() }
                                    {
                                        if self.view_data().is_empty(){
                                            html!{
                                                <button disabled=true class="window-delete-warning" style="padding: 8px">
                                                    {"NO RECORD!"}
                                                </button> 
                                            }
                                        } else {
                                            html!{}
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
    fn view_app_data(&self) -> Vec<Html> {
        self.app_data.iter().map(|card|{
                card.iter().map(|card_parse|{
                    let app_id = card_parse._id.clone();
                    let app_name = card_parse._source.clone();
                    html!(
                        <a onclick=self.link.callback(move |_| Msg::SelectApp(app_id.clone()))>
                            { app_name.get("name").unwrap().as_str().unwrap() }
                        </a>
                    )
                }).collect()
                
            }).collect()
    }

    fn view_app_data_id(&self) -> Vec<Html> {
        self.app_data.iter().map(|card|{
                card.iter().map(|card_parse|{
                    let app_id = card_parse._id.clone();
                    let app_name = card_parse._source.clone();
                    app_name.as_object().unwrap().iter().map(| (appstring, appvalue) |{
                        if appstring.eq("name") {
                            html!{
                                <a>
                                { format!{"{} - {}", appvalue.as_str().unwrap(), app_id} }
                                </a>
                            }
                        } else {
                            html!{
                                
                            }
                        }
                    }).collect::<Html>()
                }).collect()
                
            }).collect()
    }

    fn view_index_data(&self) -> Vec<Html> {
        self.index_data.iter().map(|card|{
                card.iter().map(|card_parse|{
                    let index_name = card_parse.index.clone().split('.').next_back().unwrap().to_string();
                    html!{
                        <a class="index-name" onclick=self.link.callback(move |_| Msg::SelectIndex(index_name.clone()))>
                            // { serde_json::to_string_pretty(&card_parse.index).unwrap().trim_start().replace("\"", "")}
                            { card_parse.index.clone().split('.').next_back().unwrap() }
                        </a>
                    }
                }).collect()
                
            }).collect()
    }

    fn view_data(&self) -> Vec<Html> {

        match self.record_data.get("data") {
            Some(data_get) => {
                match data_get.as_array() {
                    Some(data_as_array) => {
                        data_as_array.iter().enumerate().map(|(i, card)|{

                            let edit_text_data = serde_json::to_string(card.get("fields").unwrap()).unwrap();

                            let edit_index = serde_json::to_string_pretty(card.get("_id").unwrap()).unwrap();
                            let delete_index = serde_json::to_string_pretty(card.get("_id").unwrap()).unwrap().replace("\"", "");
                            
                            let edit_modal_data = EditModalData{    
                                data: edit_text_data.clone(),
                                index: edit_index.clone(),
                                };

                            let card_index = serde_json::to_string(card.get("_index").unwrap()).unwrap().replace("\"", "");
                            let card_index_2 = serde_json::to_string(card.get("_index").unwrap()).unwrap().replace("\"", "");
                            
                            html!{
                                <div class="index-card">
                                    <div class="card-main">
                                        <div class="card-sub">
        
                                            <div class="card-number">
                                                {"#"}{i+1}
                                            </div>
                                                
        
        
                                            <div class="card-json">    
                                                //DISPLAY DATA NEW
                                                
                                                { self.view_card(card) }
                                                        
                                                    
                                            </div>
                                        </div>
                                                {
                                                    match card.get("fields"){ 
                                                       Some(card_fields) => {
                                                            match card_fields.get("image"){
                                                                Some(card_image) => {
                                                                    html!{ 
                                                                        <img class="card-image-data" src={
                                                                            match serde_json::to_string(card_image){
                                                                                Ok(image) => {
                                                                                    image.replace(&['[', ']','"','_'], "")
                                                                                }
                                                                                Err(error) => {
                                                                                    "images/img-card/no-pictures.png".to_string()
                                                                                }
                                                                            }}/>
                                                                    }
                                                                }
                                                                None => {
                                                                    html!{ 
                                                                        <img class="card-image-placeholder" src="images/img-card/no-pictures.png"/>
                                                                    }
                                                                }
                                                            }
                                                       }
                                                       None => {
                                                            html!{ 
                                                                <img class="card-image-placeholder" src="images/img-card/no-pictures.png"/>
                                                            }
                                                       }
                                                    }
                                                }
                                    </div>
        
                                     
                                    <div class="index-card-buttons">
                                        <button
                                            type="button"
                                            class="card-button"
                                            onclick=self.link.batch_callback(move |_| vec![
                                                Msg::SendDeleteToParent(delete_index.clone()),
                                                Msg::SendIndexNameToParent(card_index.clone()),
                                                Msg::ToggleDeleteCard(delete_index.clone(), card_index.clone())
                                            ]
                                            )
                                        >
                                            <img class="card-icon" src="images/trash-can.png"/>
                                            
                                        </button>
        
                                        <button 
                                            type="button"
                                            class="card-button"
        
        
                                            onclick= self.link.batch_callback(move |_| vec![
                                                Msg::SendEditToParent(edit_modal_data.clone()),
                                                Msg::SendIndexNameToParent(card_index_2.clone()),
                                                Msg::ToggleEditRecord(edit_text_data.clone(), edit_index.clone(), card_index_2.clone()),
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

                    None => vec![html! {}],
                }
            }

            None => vec![html! {}],
        }
    }

    ///////////////////////
    fn view_card(&self, card:&Value) -> Vec<Html> {

        match card.as_object() {
            Some(data_parse_3) => data_parse_3.iter().map(|(key, value)|{
                // ConsoleService::info(&format!("DEBUG DATAPARSE3  :{:?}", data_parse_3));
                // ConsoleService::info(&format!("DEBUG :{:?}, {:?}", key, value.to_string()));
                html! {
                    <div class="card-json-line"> 
                    
                        {
                            if key.eq("fields") {
                                match value.as_object() {
                                        Some (data) => data.iter().map(|(key, value)|{
                                            html!{
                                                <div class="data-fields"> 
                                                    <b>{ key }</b>
                                                    // {" : "}
                                                    <p>{ serde_json::to_string_pretty(value).unwrap().replace(&['{', '}','"','_', '[', ']'], "") }</p>
                                                </div> 
                                                // <p class="card-json-key"><b>{ key }</b>{" : "}{ serde_json::to_string_pretty(value).unwrap().replace(&['{', '}','"','_', '[', ']'], "") }</p>
                                            }
                                        }).collect(),
                                    
                                        None => html!{}
                                    }
                            } else {
                                html!{
                                    <div class="data-header">
                                            <b>{ key }</b>
                                            // {" : "}
                                            <p>{ serde_json::to_string_pretty(value).unwrap().replace(&['{', '}','"','_', '[', ']'], "") }</p>
                                    </div>
                                }
                            }
                            
                        }
                    </div>            

                    
                }  
            }).collect(),

            None => vec![html! {}],
            }
    }

}




   // fn view_data(&self) -> Vec<Html> {
    //     self.card_data.get("data").unwrap().as_array()
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


                                    // {for i in x.iter()}
                                    // {x.iter().flat_map(|s| s.iter()).for_each(|(key, value)| {
                                    //     html!{
                                    //         <h1> {value} </h1>
                                    //     }
                                            
                                        // })}


              // {x}
                                                // { card.get("fields").unwrap().as_array().unwrap().iter().map(|f| f.as_array()).collect() }
                                                // { for x.iter().map( serde_json::to_string_pretty(i))  }
                                                // {serde_json::to_vec_pretty(&xfdas)}




                                                // {
                                                //     for card.get("fields").unwrap().as_array().iter().for_each(|val|
                                                
                                                //     html!{
                                                //         serde_json::to_string_pretty(val).unwrap()
                                                //     }
                                                // )
                                                // }
                                                // { serde_json::to_string_pretty(json!(x.clone()))}
                                                // { x.iter().map(|y| serde_json::to_string_pretty(y)) }