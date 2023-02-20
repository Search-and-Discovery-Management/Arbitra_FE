use yew_router::prelude::*;


#[derive(Switch, Clone)]
pub enum AppRoute {
    //TEMPORARY
    // #[to="/delete_record"]
    // DeleteRecord,
    // #[to="/edit_record"]
    // EditRecord,
    // #[to="/insert_record"]
    // InsertRecord,
    // #[to="/app_create"]
    // AppCreate,
    // #[to="/index_create"]
    // IndexCreate,






    
    //
    #[to="/index"]
    IndexPage,
    #[to="/dashboard"]
    DashboardPage,
    #[to="/login"]
    LoginPage,
    #[to="/signup"]
    SignupPage,
    #[to="/"]
    HomePage,
}
