use serde:: {
    Deserialize, Serialize
};
use serde_json::{Value};

//DASHBOARD PAGE
// 1. Data untuk Dashboard (ping dan searches)
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DashboardData {
    pub request_amount: Option<u32>,
    pub ping : Option<u32>,
}

// pub struct DashboardData {
//     pub request_amount: Vec<SerdeJSONValue>,
//     pub ping : Option<u32>,
// }

//STRUKTUR UNTUK INDEX (TEMP)
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Application {
    pub app_name: String, 
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct IndexList {
    pub app_name: String,
    pub index_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
// pub let mut Record:Option<Value>;
pub struct Record { 
    // pub app_name: String,
    // pub index_name: String,
    // pub record_id: String,
    pub record_data: Option<Value>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RecordData{ 
    pub list: Option<Vec<Record>>,
    pub error_description: Option<String>,
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct EditModalData{
    pub data: String,
    // pub index: usize,
    pub index: String,
}