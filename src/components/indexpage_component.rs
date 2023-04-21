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
    pub index: String,
    pub primary_size: String
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SearchRecord {
    pub index: String,
    pub search_term: String,
    pub from: u32,
    pub count: u32,
    pub wildcards: bool
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Pagination {
    pub from: i64,
    pub count: i64,
}

pub enum Msg {
    //EVENT TOGGLE (MERGE CLOSE DAN OPEN)
    ToggleCreateApp,
    ToggleDeleteApp,
    ToggleCreateIndex(String),
    ToggleInsertRecord(String, String),
    ToggleEditRecord(String, String, String, String),
    ToggleDeleteRecord(String),
    ToggleDeleteCard(String, String, String),

    RequestRecordData,
    RequestRecordDataPage(i64),
    GetRecordDataFirst(Value), //UNTUK MENGETAHUI JUUMLAH RECORD DAN SIZE NYA UNTUK DISPLAY
    GetRecordData(Value), // UNTUK SEARCH
    ResponseError(String),

    RequestIndexData,
    GetIndexData(Option<Vec<IndexData>>),

    RequestAppData,
    GetAppData(Option<Vec<AppData>>),
    
    RequestDeleteApp,

    SelectApp(String, String),

    SelectIndex(String, String),

    SendEditToParent(EditModalData),
    SendDeleteToParent(String),
    SendIndexNameToParent(String),
    SendAppIdToParent(String),

    InputSearch(String),
    RequestSearch(String),

    ToggleSidebar,
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
    search_input: bool,
    app_data: Option<Vec<AppData>>,
    app_id: String,
    app_name: String,

    record_count: i32,
    index_size: String,

    total_page: i64,
    current_page: i64,

    loading_record: bool,
    loading_first: bool,
    toggle_sidebar: bool,
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
            app_name: String::from("UNSELECTED"),
            app_data: Some(vec![]),

            search_input: false,

            callback_edit_data: props.callback_edit_data.clone(),
            callback_delete_window: props.callback_delete_window.clone(),
            callback_card_index: props.callback_card_index.clone(),
            callback_app_id: props.callback_app_id.clone(),
            props,
            
            record_count: 0,
            index_size: String::from(""),

            total_page: 0,
            current_page: 0,

            loading_record : false,
            loading_first : true,
            toggle_sidebar: true,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            //EVENT BUAT OPEN MODAL
            Msg::ToggleCreateIndex(app_id) => {
                self.callback_toggle_createindex.emit(Msg::ToggleCreateIndex(app_id));
                // ConsoleService::info(&format!("DEBUG : display_create_index:{:?}", self.props.display_create_index));
                // ConsoleService::info(&format!("DEBUG : modal_open COMPONENT:{:?}", self.props.modal_open_index));
                true
            }

            Msg::ToggleCreateApp => {
                self.callback_toggle_createapp.emit(Msg::ToggleCreateApp);
                // ConsoleService::info(&format!("DEBUG : display_create_app:{:?}", self.props.display_create_app));
                // ConsoleService::info(&format!("DEBUG : modal_open COMPONENT:{:?}", self.props.modal_open_app));
                true
            }

            Msg::ToggleDeleteApp => {
                self.callback_toggle_deleteapp.emit(Msg::ToggleDeleteApp);
                // ConsoleService::info(&format!("DEBUG : display_create_app:{:?}", self.props.display_delete_app));
                // ConsoleService::info(&format!("DEBUG : modal_open COMPONENT:{:?}", self.props.modal_open_app));
                true
            }

            Msg::ToggleInsertRecord(app_id, card_index) => {
                self.callback_toggle_insertrecord.emit(Msg::ToggleInsertRecord(app_id, card_index));
                // ConsoleService::info(&format!("DEBUG : display_insert_record:{:?}", self.props.display_insert_record));
                // ConsoleService::info(&format!("DEBUG : modal_open COMPONENT:{:?}", self.props.modal_open_record));
                true
            }

            Msg::ToggleEditRecord (data, index, app_id, card_index)=> {

                // ConsoleService::info(&format!("DEBUG : display_edit_record:{:?}", self.props.display_edit_record));
                // ConsoleService::info(&format!("DEBUG : data INDEX PAGE CHILD:{:?}", data.clone()));
                // ConsoleService::info(&format!("DEBUG : index INDEX PAGE CHILD:{:?}", index.clone()));
                // ConsoleService::info(&format!("DEBUG : card_index EVENT :{:?}", card_index));
                
                self.callback_toggle_editrecord.emit(Msg::ToggleEditRecord(data, index, app_id, card_index));
                // ConsoleService::info(&format!("DEBUG : modal_open COMPONENT:{:?}", self.props.modal_open_record));
                true
            }

            Msg::ToggleDeleteRecord(app_id) => {
                self.callback_toggle_deleterecord.emit(Msg::ToggleDeleteRecord(app_id));
                // ConsoleService::info(&format!("DEBUG : display_delete_record:{:?}", self.props.display_delete_record));
                // ConsoleService::info(&format!("DEBUG : modal_open COMPONENT:{:?}", self.props.modal_open_index));
                true
            }

            Msg::ToggleDeleteCard (index, app_id, card_index) => {
                // ConsoleService::info(&format!("DEBUG : delete_index EVENT :{:?}", index));
                // ConsoleService::info(&format!("DEBUG : card_index EVENT :{:?}", card_index));
                // ConsoleService::info(&format!("DEBUG : display_delete_card:{:?}", self.props.display_delete_card));
                self.callback_toggle_deletecard.emit(Msg::ToggleDeleteCard(index, app_id, card_index));
                // ConsoleService::info(&format!("DEBUG : modal_open COMPONENT:{:?}", self.props.modal_open_record));
                true
            }

            Msg::RequestSearch(data) => {
                let mut search_term = SearchRecord{
                    index: self.index_name.clone(),
                    search_term: String::from(""),
                    from: 0,
                    count: 20,
                    wildcards: true
                };
                if data.is_empty() {
                    search_term.search_term = String::from("*");
                }else {
                    search_term.search_term = data;
                }
                let request = Request::post(format!("https://test-dps-api.dev-domain.site/api/search/{}/{}", &self.app_id, &self.index_name))
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
                self.search_input = !data.is_empty();
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

            Msg::SelectIndex(index, index_size) => {
                // ConsoleService::info(&format!("Selected index: {:?}", index));
                self.index_name = index;
                self.index_size = index_size;
                self.record_data =  serde_json::json!({"data": []});
                // ConsoleService::info(&format!("self.index_size: {:?}", self.index_size));
                self.link.send_message(Msg::RequestRecordData);
                true
            }

            Msg::GetIndexData(data) => {
                // ConsoleService::info(&format!("data is {:?}", data));

                {
                    if self.index_name != String::from("SELECT INDEX ...") {
                        if let Some(data_vec) = &data {
                            let found_index_name = data_vec.iter().find(
                                |index_find| index_find.index.clone().split('.').next_back().unwrap().to_string() == self.index_name
                            );

                            match found_index_name {
                                Some(app) => {
                                    // ConsoleService::info(&format!("Matched index name!"));
                                },
                                None => {
                                    // ConsoleService::info(&format!("index Name not found!"));
                                    self.index_name = String::from("SELECT INDEX ...");
                                }
                            }
                        }
                    }
     
                }


                self.index_data = data;
                // ConsoleService::info(&format!("self.index_data is {:?}", self.index_data));
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
                                // ConsoleService::info(&format!("data response {:?}", &dataok));
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
                // ConsoleService::info(&format!("data response {:?}", &data));
                
                {
                    if self.app_name != String::from("UNSELECTED") {
                        if let Some(data_vec) = &data {
                            let found_app_name = data_vec.iter().find(
                                |app| app._source.get("name").unwrap().to_string().replace("\"", "") == self.app_name
                            );

                            match found_app_name {
                                Some(app) => {
                                    // ConsoleService::info(&format!("Matched App name!"));
                                },
                                None => {
                                    // ConsoleService::info(&format!("App Name not found!"));
                                    self.app_name = String::from("UNSELECTED");
                                }
                            }
                        }
                    }
                }
                
                self.loading_first = false;
                self.app_data = data;
                // ConsoleService::info(&format!("DEBUG : self.loading_first : {:?}", self.loading_first ));
                true
            }

            Msg::SelectApp(app, name) => {
                // ConsoleService::info(&format!("Selected index: {:?}", index));
                self.index_name = String::from("SELECT INDEX ...");
                self.record_data = serde_json::json!({"data": []});
                self.app_name = name;
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

            Msg::RequestRecordDataPage (data) => {
                let mut pages = Pagination{
                    from: data * 20,
                    count: 20
                };

                let request = Request::post(format!("https://test-dps-api.dev-domain.site/api/search/{}/{}", &self.app_id, &self.index_name))
                    .header("Content-Type", "application/json")
                    .body(Json(&pages))
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
            
            Msg::RequestRecordData => {
                self.loading_record = true;
                let request = Request::get(format!("https://test-dps-api.dev-domain.site/api/search/{}/{}", &self.app_id, &self.index_name))
                    // .header("access_token", get_access_token{}.unwrap_or_default())
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<Value, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
        
                        match data { 
                            Ok(dataok) => {
                                ConsoleService::info(&format!("data response {:?}", &dataok));
                                Msg:: GetRecordDataFirst(dataok)
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
                self.loading_record = false;
                self.record_data = data;
                true
            }

            Msg::GetRecordDataFirst(data) => {
                // ConsoleService::info(&format!("data is {:?}", data.get("data").unwrap().as_array().unwrap()));
                let from = data.get("from").unwrap().as_i64().unwrap_or(0);
                let count = data.get("count").unwrap().as_i64().unwrap_or(0);
                let total_data = data.get("total_data").unwrap().as_i64().unwrap_or(0);

                self.total_page = (total_data as f64 / count as f64).ceil() as i64;
                ConsoleService::info(&format!("Total page:  {:?}", self.total_page));
                self.current_page = (from / count) + 1;
                ConsoleService::info(&format!("Current page:  {:?}", self.current_page));
                
                self.loading_record = false;
                self.record_data = data;

                self.record_count = self.record_data.get("total_data").unwrap().to_string().parse().unwrap();
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
                self.loading_first = false;
                self.loading_record = false;
                ConsoleService::info(&format!("error is {:?}", text));
                // ConsoleService::info(&format!("DEBUG : self.loading_first : {:?}", self.loading_first ));
                true
            }

            Msg::ToggleSidebar => {
                self.toggle_sidebar = !self.toggle_sidebar;
                ConsoleService::info(&format!("DEBUG : self.toggle_sidebar {:?}", self.toggle_sidebar));
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
            self.loading_first = true;
            // ConsoleService::info(&format!("DEBUG : self.loading_first : {:?}", self.loading_first ));
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
            self.link.send_message(Msg::RequestAppData);
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        //CONDITIONAL DEFAULT CASE (NO MODAL)
        let app_id_view = self.app_id.clone();
        let app_id_view2 = self.app_id.clone();
        let app_id_view3 = self.app_id.clone();

        let index_name_view = self.index_name.clone();

            html! {
                <div> 
                        <div>
                            <div class="leftbox index-sidebar-small">
                                <img class="index-logo" src="images/Arbitra_LogoOnly2.png"/> 
                            
                                <label for="sidebar-check" class="sidebar-toggle-label">
                                    <img class="index-logo" id="sidebar-toggle-img" src="images/menu-burger.png"/>
                                </label>
                            </div>

                            <input type="checkbox" id="sidebar-check"
                                onchange=self.link.callback(|_| Msg::ToggleSidebar)
                            />

                            {
                                if self.toggle_sidebar {
                                    html!{
                                        <div class="rightSideBar">
                                            <p style="color: #bd3143; font-size: 2rem">{"S E A R C H"}</p>
                                            <p style="margin-top: -8px">{ "Application" }</p>
            
                                            <div class="dropdown">
                                                {
                                                    if self.app_name == "UNSELECTED"{
                                                        html!{
                                                            <button class="mainmenubtn-warn"><img class="applicationIcon" src="images/APP_WARN.png"/>{ format!("{} \u{00a0} \u{23F7}", &self.app_name)}</button>
                                                        }
                                                    } else{
                                                        html!{
                                                            <button class="mainmenubtn"><img class="applicationIcon" src="images/APP.png"/>{ format!("{} \u{00a0} \u{23F7}", &self.app_name)}</button>
                                                    }
                                                    }
                                                }
                                                <div class="dropdown-child">
            
                                                    { self.view_app_data() }
                                                    <a 
                                                        href="#" 
                                                        onclick=self.link.callback(|_| Msg::ToggleCreateApp)
                                                        style="background-color: #e3e8ed"
                                                        >
                                                        { "Create New Application" }
                                                    </a>
            
                                                    <a 
                                                        href="#" 
                                                        onclick=self.link.callback(|_| Msg::ToggleDeleteApp)
                                                        style="color: white; background-color: #a73034"
                                                        >
                                                        { "Remove Application" }
                                                    </a>
                                                </div>
                                            </div>
                                            
                                            <br/><br/>
            
                                            <p class="index-directry">{ "\u{007C}\u{00a0} Index" }</p>
                                            <p class="index-directry">{ "\u{007C}\u{00a0} Dictionary" }</p>
                                        </div>
                                    }
                                        
                                } else {
                                    html!{}
                                }
                            }
                            
                        </div>

                        <div>
                            <div class="top-index-dashboard">

                                
                                    {
                                        if self.app_name == "UNSELECTED" {
                                            html!{
                                                <div class="dropdownIndex">
                                                    <button class="mainmenubtnIndex-warn">{"NO APPLICATION SELECTED"}</button>
                                                </div>
                                            }

                                        } else {
                                            html!{
                                                <div class="dropdownIndex">
                                                    <button class="mainmenubtnIndex">
                                                        <span>{&self.index_name}</span>
                                                        <span>{format!("\u{00a0} \u{23F7}")}</span>
                                                    </button>

                                                    // { format!("{} \u{00a0} \u{00a0} \u{00a0} \u{00a0} \u{00a0} \u{00a0} \u{00a0} \u{00a0} \u{00a0} \u{23F7}", &self.index_name)}
                                                    <div class="dropdown-childIndex">
                                                    
                                                        { self.view_index_data() }
                                                        
                                                        <a 
                                                            href="#"
                                                            // onclick=self.link.callback(|_| Msg::ToggleCreateIndex)
                                                            style="background-color: #e3e8ed"
                                                            onclick=self.link.batch_callback(move |_| vec![
                                                                Msg::SendAppIdToParent(app_id_view.clone()),
                                                                Msg::ToggleCreateIndex(app_id_view.clone()),
                                                            ]) 
                                                            >
                                                            { "Create New Index" }
                                                        </a>
                                                        <a 
                                                            href="#"
                                                            //Untuk sementara pakai yang delete record dlu
                                                            // onclick=self.link.callback(|_| Msg::ToggleDeleteRecord)
                                                            style="color: white; background-color: #a73034"
                                                            onclick=self.link.batch_callback(move |_| vec![
                                                                Msg::SendAppIdToParent(app_id_view2.clone()),
                                                                Msg::ToggleDeleteRecord(app_id_view2.clone()),
                                                            ])
                                                            >
                                                            { "Remove Index" }
                                                        </a>
                                                    </div>
                                                </div>
                                            }                       
                                        }
                                    }      
                                
                                    {
                                        if self.index_name == "SELECT INDEX ..." {
                                            html!{}
                                        } else {
                                            html!{
                                                <div class="recordData">
                                                    <p class="recordNum">{ "No. of Records \u{00a0} \u{00a0} \u{00a0} \u{00a0}" }{self.record_count}</p>
                                                    <p style="float: left;">{ "\u{00a0} \u{00a0} \u{00a0}" }</p>
                                                    <p class="recordSize">{ "Index Size\u{00a0} \u{00a0} \u{00a0} \u{00a0}" }{&self.index_size}</p>
                                                </div>
                                            }
                                        }
                                    }
                               

                                <br/><br/><br/>

                                {
                                        if &self.index_name == "SELECT INDEX ..." {
                                        html!{<p style="margin-bottom: -50px">{ "" }</p>}
                                    } else {
                                        html!{
                                            <div style="margin-bottom: -25px">
                                                <div class="dropdownRecord">
                                                <button class="mainmenubtnRecord">{ "New Record \u{00a0} \u{00a0} \u{00a0} \u{00a0} \u{23F7}"}</button>
                                                <div class="dropdown-childRecord">
                                                    <a 
                                                        href="#" 
                                                        // onclick=self.link.callback(move |_| Msg::ToggleInsertRecord(app_id_view3.clone(), index_name_view.clone()))
                                                        onclick=self.link.batch_callback(move |_| vec![
                                                            Msg::SendAppIdToParent(app_id_view3.clone()),
                                                            Msg::SendIndexNameToParent(index_name_view.clone()),
                                                            Msg::ToggleInsertRecord(app_id_view3.clone(), index_name_view.clone()),
                                                        ])
                                                    >
                                                        { "Insert New Record" }
                                                    </a>
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
                                        }
                                    }
                                }
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
                            
                                <div class="search-bar">
                                    <div class="search">

                                        {
                                            if self.index_name == "SELECT INDEX ..." {
                                                html!{
                                                    <input
                                                        class= "search-input"
                                                        disabled = true
                                                        type="text"
                                                        placeholder="Please Select An Application & Index first!"
                                                        oninput = self.link.callback(|data: InputData| Msg::InputSearch(data.value))
                                                    />
                                                }

                                            } else {
                                                html!{
                                                    <input
                                                        class= "search-input"
                                                        type="text"
                                                        placeholder="Search..."
                                                        oninput = self.link.callback(|data: InputData| Msg::InputSearch(data.value))
                                                    />
                                                }   
                                            }
                                        }
                                        
                                        
                                        // <div >
                                            {
                                                if self.index_name == "SELECT INDEX ..." {
                                                    html!{
                                                        <div class= "search-statistics-disabled"></div>
                                                    }
                                                } else if self.record_data.get("total_took").is_some() && self.record_data.get("total_data").is_some() {
                                                    html!{
                                                        <div class= "search-statistics">
                                                            <strong>{ self.record_data.get("total_data").unwrap()  }</strong> { " hits in " }
                                                            { self.record_data.get("total_took").unwrap()  }{ "ms" }       
                                                        </div>
                                                    }
                                                } else {
                                                    html!{
                                                        <div class= "search-statistics"></div>
                                                    }
                                                }
                                            }
                                    </div>  

                                    <div class = "pagination">
                                        // { for records.iter().skip(start_record as usize - 1).take(self.records_per_page as usize).map(|record| {
                                        //     html! {
                                        //         <button></button>
                                        //     }
                                        // })}
                                        <button> { "1" } </button>
                                        {
                                            if self.current_page == self.total_page {
                                                html!{
                                                    <div>
                                                        <button>{self.current_page - 3}</button>
                                                        <button>{self.current_page - 2}</button>
                                                        <button>{self.current_page - 1}</button>
                                                        <button>{self.current_page}</button>
                                                    </div>
                                                }
                                            } else if self.current_page > 5 && self.current_page != self.total_page {
                                                html!{
                                                    <div>
                                                        <button>{self.current_page - 2}</button>
                                                        <button>{self.current_page - 1}</button>
                                                        <button>{self.current_page}</button>
                                                        <button>{self.current_page + 1}</button>
                                                        { "..." }
                                                        <button >{self.total_page}</button>
                                                    </div>
                                                }
                                            } else {
                                                let mut pages: Vec<_> = (2..=5).into_iter().map(|i| if self.current_page == i {
                                                    html!{
                                                        <button onclick=self.link.callback(move |_| Msg::RequestRecordDataPage(i))>{i}</button>
                                                    }
                                                } else {
                                                    html!{
                                                        <button onclick=self.link.callback(move |_| Msg::RequestRecordDataPage(i))>{i}</button>
                                                    }
                                                }).collect();

                                                pages.push(html!{"\u{00a0} ... \u{00a0}"});
                                                pages.push(html!{<button>{self.total_page}</button>});

                                                html!{
                                                    <div>
                                                        {pages}
                                                    </div>
                                                } 
                                            }
                                        }
                                    </div>
                                </div>



                            <div class="card">
                                <div>
                                    
                                    { self.view_data() }
                                    {
                                        if  self.view_data().is_empty() && 
                                            self.app_name != "UNSELECTED" && 
                                            self.index_name != "SELECT INDEX ..." &&
                                            !self.loading_record 
                                                {
                                                    html!{
                                                        <button disabled=true class="window-delete-warning-main" >
                                                            {"NO RECORD!"}
                                                        </button> 
                                                    }
                                        } else if   self.index_name == "SELECT INDEX ..." && 
                                                    self.app_name != "UNSELECTED" && 
                                                    !self.loading_record 
                                                        {
                                                            html!{
                                                                <button disabled=true class="window-delete-warning-main" >
                                                                    {"SELECT INDEX!"}
                                                                </button> 
                                                            }
                                        } else if   self.app_name == "UNSELECTED" && 
                                                    !self.loading_record 
                                                        {
                                                            html!{
                                                                <button disabled=true class="window-delete-warning-main" >
                                                                    {"SELECT APP!"}
                                                                </button> 
                                                            }
                                        } else {
                                            html!{}
                                        }
                                    }
                                    {
                                        if self.loading_record {
                                            html!{ 
                                                <button 
                                                class="loading-button-main"
                                                >
                                                    <span class="loader">
                                                        <span class="loader-inner">
                                                        </span>
                                                    </span>
                                                    {"       LOADING RECORDS, PLEASE WAIT..."}
                                                </button>

                                            }
                                        } else {
                                            html!{}
                                        }
                                    }
                                    
                                </div>

                            </div>      

                        </div>

                        {
                            if self.loading_first {
                                html!{
                                    <div class = "pre-loader-wrapper-loop"> 
                                    
                                        <span class="loader">
                                            <span class="loader-inner">
                                            </span>
                                        </span>
                                    </div>
                                }
                            } else {
                                html!{
                                    <div class = "pre-loader-wrapper-finish"> 
                                        <span class="loader-finish">
                                            <span class="loader-inner-finish">
                                            </span>
                                        </span>
                                    </div>
                                }
                            }
                        }



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
                    let app_name_2 = card_parse._source.get("name").unwrap().to_string().replace("\"", "");
                    html!(
                        <a onclick=self.link.callback(move |_| Msg::SelectApp(app_id.clone(), app_name_2.clone()))>
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
                    // ConsoleService::info(&format!("vec indexData is {:?}", card));
                    // ConsoleService::info(&format!("indexData is {:?}", card_parse));
                    
                    let index_name = card_parse.index.clone().split('.').next_back().unwrap().to_string();
                    let index_size_impl = card_parse.primary_size.clone();
                    // ConsoleService::info(&format!("indexSize is {:?}", index_size_impl));
                    html!{
                        <a class="index-name" onclick=self.link.callback(move |_| Msg::SelectIndex(index_name.clone(), index_size_impl.clone()))>
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

                            let edit_text_data = serde_json::to_string(card.get("_source").unwrap()).unwrap();

                            let edit_index = serde_json::to_string_pretty(card.get("_id").unwrap()).unwrap();
                            let delete_index = serde_json::to_string_pretty(card.get("_id").unwrap()).unwrap().replace("\"", "");

                            let edit_modal_data = EditModalData{    
                                data: edit_text_data.clone(),
                                index: edit_index.clone(),
                                };

                            // let card_index = serde_json::to_string(card.get("_index").unwrap()).unwrap().replace("\"", "");
                            // let card_index_2 = serde_json::to_string(card.get("_index").unwrap()).unwrap().replace("\"", "");

                            let card_index = self.index_name.clone();
                            let card_index_2 = self.index_name.clone();

                            let app_id_impl = self.app_id.clone();
                            let app_id_impl_2 = self.app_id.clone();
                            
                            
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
                                                    match card.get("_source"){ 
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
                                                Msg::SendAppIdToParent(app_id_impl.clone()),
                                                Msg::SendIndexNameToParent(card_index.clone()),
                                                Msg::ToggleDeleteCard(delete_index.clone(), app_id_impl.clone(), card_index.clone())
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
                                                Msg::SendAppIdToParent(app_id_impl_2.clone()),
                                                Msg::SendIndexNameToParent(card_index_2.clone()),
                                                Msg::ToggleEditRecord(edit_text_data.clone(), edit_index.clone(), app_id_impl_2.clone(),  card_index_2.clone()),
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
                            if key.eq("_source") {
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



