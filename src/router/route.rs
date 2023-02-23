use yew_router::prelude::*;


#[derive(Switch, Clone)]
pub enum AppRoute {
    //TEMPORARY
    #[to="/card"]
    CardTemp,
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
