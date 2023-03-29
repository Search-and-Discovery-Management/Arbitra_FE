use yew::{prelude::*, services::ConsoleService};

use crate::components::{
    indexpage_window_createapp::AppCreate,
    indexpage_window_createindex::IndexCreate,
    indexpage_window_deleterecord::DeleteRecord,
    indexpage_window_editrecord::EditRecord,
    indexpage_window_insertrecord::InsertRecord,
    indexpage_window_delete_card::DeleteCard,
    indexpage_component::IndexPageComp,
};
use crate::types::var::EditModalData;

pub enum Msg {
    //EVENT TOGGLE (MERGE CLOSE DAN OPEN)
    ToggleCreateApp,
    ToggleCreateIndex,
    ToggleInsertRecord,
    ToggleEditRecord,
    ToggleDeleteRecord,
    ToggleDeleteCard,
    
    RecvEditData(EditModalData),
    RecvDeleteData(String),
    RecvIndexName(String),
}

pub struct IndexPage {
    link: ComponentLink<Self>,
    //DISPLAY WINDOWS / MODAL (STATE)
    display_create_app: bool,
    display_create_index: bool,
    display_insert_record: bool,
    display_edit_record: bool,
    display_delete_record: bool,
    display_delete_card: bool,

    edit_data : String,
    edit_index: String,

    delete_index : String,

    card_index : String,
}

impl Component for IndexPage {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {

            //DISPLAY WINDOWS / MODAL (STATE)
            display_create_index: false,
            display_create_app: false,
            display_insert_record: false,
            display_edit_record: false,
            display_delete_record: false,
            display_delete_card: false,

            edit_data : String::from("JSON Goes Here"),
            edit_index : String::from("47"),

            delete_index: String::from("118"),

            card_index: String::from("Index Name here"),
            link,
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
            Msg::ToggleEditRecord => {
                ConsoleService::info(&format!("DEBUG : display_edit_record:{:?}", self.display_edit_record));
                ConsoleService::info(&format!("DEBUG : self.edit_data:{:?}", self.edit_data.clone()));
                ConsoleService::info(&format!("DEBUG : self.edit_index:{:?}", self.edit_index.clone()));

                self.display_edit_record = !self.display_edit_record;

                true
            }
            Msg::ToggleDeleteRecord => {
                self.display_delete_record = !self.display_delete_record;
                ConsoleService::info(&format!("DEBUG : display_delete_record:{:?}", self.display_delete_record));
                true
            }
            Msg::ToggleDeleteCard => {
                self.display_delete_card = !self.display_delete_card;
                ConsoleService::info(&format!("DEBUG : display_delete_card:{:?}", self.display_delete_card));
                true
            }
            Msg::RecvEditData(data_recv) => {
                ConsoleService::info(&format!("data in parent is {:?}", data_recv));

                self.edit_data = data_recv.data.clone();
                self.edit_index = data_recv.index.clone();
                true
            }
            Msg::RecvDeleteData(data_recv) => {
                ConsoleService::info(&format!("data in parent is (DELETE MODAL INDEX) {:?}", data_recv));
                self.delete_index = data_recv.clone();
                true
            }
            Msg::RecvIndexName(data_recv) => {
                ConsoleService::info(&format!("data in parent is (INDEX NAME) {:?}", data_recv));
                self.card_index = data_recv.clone();
                ConsoleService::info(&format!("data in parent STATE is (INDEX NAME) {:?}", self.card_index));
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

        let ToggleCreateApp = self.display_create_app;
        let ToggleCreateIndex = self.display_create_index;
        let ToggleInsertRecord = self.display_insert_record;
        let ToggleEditRecord = self.display_edit_record;
        let ToggleDeleteRecord = self.display_delete_record;
        let ToggleDeleteCard = self.display_delete_card;
        

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
                        display_delete_card=self.display_delete_card.clone()
                        on_toggle_createapp = self.link.callback(|_| Msg::ToggleCreateApp)
                        on_toggle_createindex = self.link.callback(|_| Msg::ToggleCreateIndex)
                        on_toggle_insertrecord = self.link.callback(|_| Msg::ToggleInsertRecord)
                        on_toggle_editrecord = self.link.callback(|_| Msg::ToggleEditRecord)
                        on_toggle_deleterecord = self.link.callback(|_| Msg::ToggleDeleteRecord)
                        on_toggle_deletecard = self.link.callback(|_| Msg::ToggleDeleteCard)
                        
                        edit_data = self.edit_data.clone()
                        edit_index = self.edit_index.clone()
                        callback_edit_data = self.link.callback(Msg::RecvEditData)

                        delete_index = self.delete_index.clone()
                        callback_delete_window = self.link.callback(Msg::RecvDeleteData)

                        callback_card_index = self.link.callback(Msg::RecvIndexName)
                        card_index = self.card_index.clone()
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
                        display_delete_card=self.display_delete_card.clone()
                        on_toggle_createapp = self.link.callback(|_| Msg::ToggleCreateApp)
                        on_toggle_createindex = self.link.callback(|_| Msg::ToggleCreateIndex)
                        on_toggle_insertrecord = self.link.callback(|_| Msg::ToggleInsertRecord)
                        on_toggle_editrecord = self.link.callback(|_| Msg::ToggleEditRecord)
                        on_toggle_deleterecord = self.link.callback(|_| Msg::ToggleDeleteRecord)
                        on_toggle_deletecard = self.link.callback(|_| Msg::ToggleDeleteCard)


                        edit_data = self.edit_data.clone()
                        edit_index = self.edit_index.clone()
                        callback_edit_data = self.link.callback(Msg::RecvEditData)
                        
                        delete_index = self.delete_index.clone()
                        callback_delete_window = self.link.callback(Msg::RecvDeleteData)

                        callback_card_index = self.link.callback(Msg::RecvIndexName)
                        card_index = self.card_index.clone()
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
                        display_delete_card=self.display_delete_card.clone()
                        on_toggle_createapp = self.link.callback(|_| Msg::ToggleCreateApp)
                        on_toggle_createindex = self.link.callback(|_| Msg::ToggleCreateIndex)
                        on_toggle_insertrecord = self.link.callback(|_| Msg::ToggleInsertRecord)
                        on_toggle_editrecord = self.link.callback(|_| Msg::ToggleEditRecord)
                        on_toggle_deleterecord = self.link.callback(|_| Msg::ToggleDeleteRecord)
                        on_toggle_deletecard = self.link.callback(|_| Msg::ToggleDeleteCard)

                        edit_data = self.edit_data.clone()
                        edit_index = self.edit_index.clone()
                        callback_edit_data = self.link.callback(Msg::RecvEditData)
                        
                        delete_index = self.delete_index.clone()
                        callback_delete_window = self.link.callback(Msg::RecvDeleteData)

                        callback_card_index = self.link.callback(Msg::RecvIndexName)
                        card_index = self.card_index.clone()
                    />
                    //DISPLAY WINDOW DISINI         
                    <InsertRecord
                        display_insert_record=self.display_insert_record.clone()
                        on_toggle_insertrecord = self.link.callback(|_| Msg::ToggleInsertRecord) />

                </div>
            }
        //CONDITIONAL BUKA MODAL EDIT RECORD
        } else if ToggleEditRecord {
            html!{  
                <div> 
                    <IndexPageComp
                        display_create_app=self.display_create_app.clone()
                        display_create_index=self.display_create_index.clone()
                        display_insert_record=self.display_insert_record.clone()
                        display_edit_record=self.display_edit_record.clone()
                        display_delete_record=self.display_delete_record.clone()
                        display_delete_card=self.display_delete_card.clone()
                        on_toggle_createapp = self.link.callback(|_| Msg::ToggleCreateApp)
                        on_toggle_createindex = self.link.callback(|_| Msg::ToggleCreateIndex)
                        on_toggle_insertrecord = self.link.callback(|_| Msg::ToggleInsertRecord)
                        on_toggle_editrecord = self.link.callback(|_| Msg::ToggleEditRecord)
                        on_toggle_deleterecord = self.link.callback(|_| Msg::ToggleDeleteRecord)
                        on_toggle_deletecard = self.link.callback(|_| Msg::ToggleDeleteCard)
                   
                        edit_data = self.edit_data.clone()
                        edit_index = self.edit_index.clone()
                        callback_edit_data = self.link.callback(Msg::RecvEditData)

                        delete_index = self.delete_index.clone()
                        callback_delete_window = self.link.callback(Msg::RecvDeleteData)

                        callback_card_index = self.link.callback(Msg::RecvIndexName)
                        card_index = self.card_index.clone()
                    />

                    //DISPLAY WINDOW DISINI         
                    <EditRecord
                        display_edit_record=self.display_edit_record.clone()
                        edit_data = self.edit_data.clone()
                        edit_index = self.edit_index.clone()
                        on_toggle_editrecord = self.link.callback(|_| Msg::ToggleEditRecord) 
                        card_index = self.card_index.clone()
                        />

                </div>

            }
        //CONDITIONAL BUKA MODAL DELETE INDEX
        } else if ToggleDeleteRecord {
            html!{
                <div> 
                    <IndexPageComp
                        display_create_app=self.display_create_app.clone()
                        display_create_index=self.display_create_index.clone()
                        display_insert_record=self.display_insert_record.clone()
                        display_edit_record=self.display_edit_record.clone()
                        display_delete_record=self.display_delete_record.clone()
                        display_delete_card=self.display_delete_card.clone()
                        on_toggle_createapp = self.link.callback(|_| Msg::ToggleCreateApp)
                        on_toggle_createindex = self.link.callback(|_| Msg::ToggleCreateIndex)
                        on_toggle_insertrecord = self.link.callback(|_| Msg::ToggleInsertRecord)
                        on_toggle_editrecord = self.link.callback(|_| Msg::ToggleEditRecord)
                        on_toggle_deleterecord = self.link.callback(|_| Msg::ToggleDeleteRecord)
                        on_toggle_deletecard = self.link.callback(|_| Msg::ToggleDeleteCard)
                   
                        edit_data = self.edit_data.clone()
                        edit_index = self.edit_index.clone()
                        callback_edit_data = self.link.callback(Msg::RecvEditData)

                        delete_index = self.delete_index.clone()
                        callback_delete_window = self.link.callback(Msg::RecvDeleteData)   

                        callback_card_index = self.link.callback(Msg::RecvIndexName)    
                        card_index = self.card_index.clone()         
                    />
                    //DISPLAY WINDOW DISINI         
                    <DeleteRecord
                        display_delete_record=self.display_delete_record.clone()
                        on_toggle_deleterecord = self.link.callback(|_| Msg::ToggleDeleteRecord) />

                </div>
            }
             //CONDITIONAL BUKA MODAL DELETE INDEX
        } else if ToggleDeleteCard {
            html!{
                <div> 
                    <IndexPageComp
                        display_create_app=self.display_create_app.clone()
                        display_create_index=self.display_create_index.clone()
                        display_insert_record=self.display_insert_record.clone()
                        display_edit_record=self.display_edit_record.clone()
                        display_delete_record=self.display_delete_record.clone()
                        display_delete_card=self.display_delete_card.clone()
                        on_toggle_createapp = self.link.callback(|_| Msg::ToggleCreateApp)
                        on_toggle_createindex = self.link.callback(|_| Msg::ToggleCreateIndex)
                        on_toggle_insertrecord = self.link.callback(|_| Msg::ToggleInsertRecord)
                        on_toggle_editrecord = self.link.callback(|_| Msg::ToggleEditRecord)
                        on_toggle_deleterecord = self.link.callback(|_| Msg::ToggleDeleteRecord)
                        on_toggle_deletecard = self.link.callback(|_| Msg::ToggleDeleteCard)
                   
                        edit_data = self.edit_data.clone()
                        edit_index = self.edit_index.clone()
                        callback_edit_data = self.link.callback(Msg::RecvEditData)

                        delete_index = self.delete_index.clone()
                        callback_delete_window = self.link.callback(Msg::RecvDeleteData)   

                        callback_card_index = self.link.callback(Msg::RecvIndexName)  
                        card_index = self.card_index.clone()           
                    />
                    //DISPLAY WINDOW DISINI         
                    <DeleteCard
                        display_delete_card=self.display_delete_card.clone()
                        on_toggle_deletecard = self.link.callback(|_| Msg::ToggleDeleteCard)
                        delete_index = self.delete_index.clone()
                        card_index = self.card_index.clone()
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
                        display_delete_card=self.display_delete_card.clone()
                        on_toggle_createapp = self.link.callback(|_| Msg::ToggleCreateApp)
                        on_toggle_createindex = self.link.callback(|_| Msg::ToggleCreateIndex)
                        on_toggle_insertrecord = self.link.callback(|_| Msg::ToggleInsertRecord)
                        on_toggle_editrecord = self.link.callback(|_| Msg::ToggleEditRecord)
                        on_toggle_deleterecord = self.link.callback(|_| Msg::ToggleDeleteRecord)
                        on_toggle_deletecard = self.link.callback(|_| Msg::ToggleDeleteCard)

                        edit_data = self.edit_data.clone()
                        edit_index = self.edit_index.clone()
                        callback_edit_data = self.link.callback(Msg::RecvEditData)
                        
                        delete_index = self.delete_index.clone()
                        callback_delete_window = self.link.callback(Msg::RecvDeleteData) 

                        callback_card_index = self.link.callback(Msg::RecvIndexName)
                        card_index = self.card_index.clone()
                    />
                    </div>
               
        }
            //BODY END
        }
    }
}
