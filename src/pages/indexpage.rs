use yew::{prelude::*, services::ConsoleService};


use crate::components::{
    indexpage_window_createapp::AppCreate,
    indexpage_window_createindex::IndexCreate,
    indexpage_window_deleterecord::DeleteRecord,
    indexpage_window_editrecord::EditRecord,
    indexpage_window_insertrecord::InsertRecord,
    indexpage_component::IndexPageComp,
};
use crate::types::var::EditModalData;

pub enum Msg {
    //EVENT TOGGLE (MERGE CLOSE DAN OPEN)
    // State(Rc<State>),
    // Output(CounterOutput),

    ToggleCreateApp,
    ToggleCreateIndex,
    ToggleInsertRecord,
    ToggleEditRecord,
    ToggleDeleteRecord,

    RecvEditData(EditModalData),
    RecvDeleteData(usize),
}

pub struct IndexPage {
    link: ComponentLink<Self>,
    //DISPLAY WINDOWS / MODAL (STATE)
    display_create_app: bool,
    display_create_index: bool,
    display_insert_record: bool,
    display_edit_record: bool,
    display_delete_record: bool,

    edit_data : String,
    edit_index: usize,

    delete_index : usize,

    // dispatch: Dispatch<CounterStore>,
    // recv_edit_data : Callback<IndexPageCompMsg>,
    
}

impl Component for IndexPage {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // let dispatch = {
        //     let on_state = link.callback(Msg::State);
        //     let on_output = link.callback(Msg::Output);

        //     Dispatch::bridge(on_state, on_output)
        // };

        Self {
            
            //DISPLAY WINDOWS / MODAL (STATE)
            display_create_index: false,
            display_create_app: false,
            display_insert_record: false,
            display_edit_record: false,
            display_delete_record: false,

            //UNTUK EDIT DATA
            edit_data : String::from("JSON Goes Here"),
            edit_index : 47,

            delete_index: 118,
            //RECEIVE EDIT DATA
            // recv_edit_data: link.callback(Msg::RecvDataEdit),
            link,
            // dispatch,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            //EVENT BUAT OPEN MODAL
            Msg::ToggleCreateIndex => {
                self.display_create_index = !self.display_create_index;
                ConsoleService::info(&format!("DEBUG : display_create_index:{:?}", self.display_create_index));
                true
            }
            Msg::ToggleCreateApp => {
                self.display_create_app = !self.display_create_app;
                ConsoleService::info(&format!("DEBUG : display_create_app:{:?}", self.display_create_app));
                true
            }
            Msg::ToggleInsertRecord => {
                self.display_insert_record = !self.display_insert_record;
                ConsoleService::info(&format!("DEBUG : display_insert_record:{:?}", self.display_insert_record));
                true
            }
            //EVENT PARENT KIRIM KE CHILD
            Msg::ToggleEditRecord =>  {
                ConsoleService::info(&format!("DEBUG : display_edit_record:{:?}", self.display_edit_record));
                // ConsoleService::info(&format!("DEBUG : data PARENT:{:?}", data.clone()));
                // ConsoleService::info(&format!("DEBUG : idx PARENT:{:?}", index.clone()));
                ConsoleService::info(&format!("DEBUG : self.edit_data:{:?}", self.edit_data.clone()));
                ConsoleService::info(&format!("DEBUG : self.edit_index:{:?}", self.edit_index.clone()));

                self.display_edit_record = !self.display_edit_record;
                // self.dispatch.send(CounterInput::ToggleEditRecord(data, index));
                // self.edit_data = data;
                // self.edit_index = index;

                true
            }
            Msg::ToggleDeleteRecord => {
                self.display_delete_record = !self.display_delete_record;
                ConsoleService::info(&format!("DEBUG : display_delete_record:{:?}", self.display_delete_record));
                true
            }
            Msg::RecvEditData(data_recv) => {
                ConsoleService::info(&format!("data in parent is {:?}", data_recv));
                // ConsoleService::info(("SUPERMAN"));
                self.edit_data = data_recv.data.clone();
                // self.edit_data = String::from("SUPERMAN");
                self.edit_index = data_recv.index.clone();
                true
            }
            Msg::RecvDeleteData(data_recv) => {
                ConsoleService::info(&format!("data in parent is (DELETE MODAL INDEX) {:?}", data_recv));
                self.delete_index = data_recv.clone();
                true
            }
            
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {

        let mut ToggleCreateApp = self.display_create_app;
        let mut ToggleCreateIndex = self.display_create_index;
        let mut ToggleInsertRecord = self.display_insert_record;
        let mut ToggleEditRecord = self.display_edit_record;
        let mut ToggleDeleteRecord = self.display_delete_record;

        ConsoleService::info(&format!("DEBUG : FUNCITON VIEW self.edit_data:{:?}", self.edit_data.clone()));
        
        let mut edit_data_event = self.edit_data.clone();
        let mut edit_index_event = self.edit_index.clone();
        
        //CONDITIONAL KALAU BUKA CREATE APP
        if ToggleCreateApp { 
            html! {
                <div> 
                   
                    <IndexPageComp
                        display_create_app=self.display_create_app.clone()
                        display_create_index=self.display_create_index.clone()
                        display_insert_record=self.display_insert_record.clone()
                        display_edit_record=self.display_edit_record.clone()
                        display_delete_record=self.display_delete_record.clone()
                        on_toggle_createapp = self.link.callback(|_| Msg::ToggleCreateApp)
                        on_toggle_createindex = self.link.callback(|_| Msg::ToggleCreateIndex)
                        on_toggle_insertrecord = self.link.callback(|_| Msg::ToggleInsertRecord)
                        on_toggle_editrecord = self.link.callback(move |_| Msg::ToggleEditRecord)
                        on_toggle_deleterecord = self.link.callback(|_| Msg::ToggleDeleteRecord)
                        
                        edit_data = self.edit_data.clone()
                        edit_index = self.edit_index.clone()
                        callback_edit_data = self.link.callback(Msg::RecvEditData)
                    
                        delete_index = self.delete_index.clone()
                        callback_delete_window = self.link.callback(Msg::RecvDeleteData)
                    />
                    //DISPLAY WINDOW DISINI         
                    <AppCreate 
                        display_create_app=self.display_create_app.clone()
                        on_toggle_createapp = self.link.callback(|_| Msg::ToggleCreateApp) />

                </div>
                
            }
        //CONDITIONAL BUKA MODAL CREATE INDEX
        } else if ToggleCreateIndex {
            html! {
                <div> 
                    <IndexPageComp
                        display_create_app=self.display_create_app.clone()
                        display_create_index=self.display_create_index.clone()
                        display_insert_record=self.display_insert_record.clone()
                        display_edit_record=self.display_edit_record.clone()
                        display_delete_record=self.display_delete_record.clone()
                        on_toggle_createapp = self.link.callback(|_| Msg::ToggleCreateApp)
                        on_toggle_createindex = self.link.callback(|_| Msg::ToggleCreateIndex)
                        on_toggle_insertrecord = self.link.callback(|_| Msg::ToggleInsertRecord)
                        on_toggle_editrecord = self.link.callback(move |_| Msg::ToggleEditRecord)
                        on_toggle_deleterecord = self.link.callback(|_| Msg::ToggleDeleteRecord)
                    
                        edit_data = self.edit_data.clone()
                        edit_index = self.edit_index.clone()
                        callback_edit_data = self.link.callback(Msg::RecvEditData)
                    
                        delete_index = self.delete_index.clone()
                        callback_delete_window = self.link.callback(Msg::RecvDeleteData)
                    />
                    //DISPLAY WINDOW DISINI      
                    <IndexCreate 
                        display_create_index=self.display_create_index.clone()
                        on_toggle_createindex = self.link.callback(|_| Msg::ToggleCreateIndex) />

                </div>
                
            }
        //CONDITIONAL BUKA MODAL INSERT RECORD
        } else if ToggleInsertRecord {
            html! {
                <div> 
                    <IndexPageComp
                        display_create_app=self.display_create_app.clone()
                        display_create_index=self.display_create_index.clone()
                        display_insert_record=self.display_insert_record.clone()
                        display_edit_record=self.display_edit_record.clone()
                        display_delete_record=self.display_delete_record.clone()
                        on_toggle_createapp = self.link.callback(|_| Msg::ToggleCreateApp)
                        on_toggle_createindex = self.link.callback(|_| Msg::ToggleCreateIndex)
                        on_toggle_insertrecord = self.link.callback(|_| Msg::ToggleInsertRecord)
                        on_toggle_editrecord = self.link.callback(move |_| Msg::ToggleEditRecord)
                        on_toggle_deleterecord = self.link.callback(|_| Msg::ToggleDeleteRecord)
                    
                        edit_data = self.edit_data.clone()
                        edit_index = self.edit_index.clone()
                        callback_edit_data = self.link.callback(Msg::RecvEditData)
                
                        delete_index = self.delete_index.clone()
                        callback_delete_window = self.link.callback(Msg::RecvDeleteData)
                    />
                    //DISPLAY WINDOW DISINI         
                    <InsertRecord
                        display_insert_record=self.display_insert_record.clone()
                        on_toggle_insertrecord = self.link.callback(|_| Msg::ToggleInsertRecord) />

                </div>
            }
        //CONDITIONAL BUKA MODAL EDIT RECORD
        } else if ToggleEditRecord {
            // ConsoleService::info(&format!("DEBUG : FUNCITON VIEW EDIT CONDITIONAL self.edit_data:{:?}", self.edit_data.clone()));
            let edit_data_event_again = edit_data_event.clone();
            let edit_index_event_again = edit_index_event.clone();

            html!{  
                <div> 
                    // <div>
                    // {"self.edit_data ="}
                    // {self.edit_data.clone()}
                    // </div>
                    <IndexPageComp
                        display_create_app=self.display_create_app.clone()
                        display_create_index=self.display_create_index.clone()
                        display_insert_record=self.display_insert_record.clone()
                        display_edit_record=self.display_edit_record.clone()
                        display_delete_record=self.display_delete_record.clone()
                        on_toggle_createapp = self.link.callback(|_| Msg::ToggleCreateApp)
                        on_toggle_createindex = self.link.callback(|_| Msg::ToggleCreateIndex)
                        on_toggle_insertrecord = self.link.callback(|_| Msg::ToggleInsertRecord)
                        on_toggle_editrecord = self.link.callback(move |_| Msg::ToggleEditRecord)
                        on_toggle_deleterecord = self.link.callback(|_| Msg::ToggleDeleteRecord)

                        edit_data = self.edit_data.clone()
                        edit_index = self.edit_index.clone()
                        callback_edit_data = self.link.callback(Msg::RecvEditData)
                    
                        delete_index = self.delete_index.clone()
                        callback_delete_window = self.link.callback(Msg::RecvDeleteData)
                    />

                    //DISPLAY WINDOW DISINI         
                    <EditRecord
                        display_edit_record=self.display_edit_record.clone()
                        edit_data = self.edit_data.clone()
                        edit_index = self.edit_index.clone()
                        on_toggle_editrecord = self.link.callback(move |_| Msg::ToggleEditRecord)
                        
                        />

                </div>

            }
        //CONDITIONAL BUKA MODAL DELETE RECORD
        } else if ToggleDeleteRecord {
            html!{
                <div> 
                    <IndexPageComp
                        display_create_app=self.display_create_app.clone()
                        display_create_index=self.display_create_index.clone()
                        display_insert_record=self.display_insert_record.clone()
                        display_edit_record=self.display_edit_record.clone()
                        display_delete_record=self.display_delete_record.clone()
                        on_toggle_createapp = self.link.callback(|_| Msg::ToggleCreateApp)
                        on_toggle_createindex = self.link.callback(|_| Msg::ToggleCreateIndex)
                        on_toggle_insertrecord = self.link.callback(|_| Msg::ToggleInsertRecord)
                        on_toggle_editrecord = self.link.callback(move |_| Msg::ToggleEditRecord)
                        on_toggle_deleterecord = self.link.callback(|_| Msg::ToggleDeleteRecord)
                    
                        edit_data = self.edit_data.clone()
                        edit_index = self.edit_index.clone()
                        callback_edit_data = self.link.callback(Msg::RecvEditData)
                    
                        delete_index = self.delete_index.clone()
                        callback_delete_window = self.link.callback(Msg::RecvDeleteData)
                    />
                    //DISPLAY WINDOW DISINI         
                    <DeleteRecord
                        display_delete_record=self.display_delete_record.clone()
                        on_toggle_deleterecord = self.link.callback(|_| Msg::ToggleDeleteRecord) 
                        
                        delete_index = self.delete_index.clone()

                        />

                </div>
            }
        //CONDITIONAL DEFAULT CASE (NO MODAL)
        } else {
            html! {
                <div> 
                    <IndexPageComp
                        display_create_app=self.display_create_app.clone()
                        display_create_index=self.display_create_index.clone()
                        display_insert_record=self.display_insert_record.clone()
                        display_edit_record=self.display_edit_record.clone()
                        display_delete_record=self.display_delete_record.clone()
                        on_toggle_createapp = self.link.callback(|_| Msg::ToggleCreateApp)
                        on_toggle_createindex = self.link.callback(|_| Msg::ToggleCreateIndex)
                        on_toggle_insertrecord = self.link.callback(|_| Msg::ToggleInsertRecord)
                        on_toggle_editrecord = self.link.callback(move |_| Msg::ToggleEditRecord)
                        on_toggle_deleterecord = self.link.callback(|_| Msg::ToggleDeleteRecord)
                    
                        edit_data = self.edit_data.clone()
                        edit_index = self.edit_index.clone()
                        callback_edit_data = self.link.callback(Msg::RecvEditData)

                        delete_index = self.delete_index.clone()
                        callback_delete_window = self.link.callback(Msg::RecvDeleteData)
                    />
                    </div>
               
        }
            //BODY END
        }
    }
}