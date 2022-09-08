use spinners::*;
pub fn start() {
    let token = super::expect_token();
    match super::ask_for_app(token.clone(), "restart") {
        Ok(app_id) => {
            let mut spinner = Spinner::new(Spinners::Earth, "Restarting your app".into());
            match crate::entities::app::App::restart(token.clone(), app_id) {
                Ok(()) => {
                    spinner.stop_with_message(super::format_log("Your app is up!"));
                }
                Err(err) => {
                    super::err(&format!("Couldn't restart your app: {}", err));
                    std::process::exit(1);
                }
            }
            
        }
        Err(err) => {
            super::err(&format!("Couldn't fetch apps from api: {}", err));
            std::process::exit(1);
        }
    }
    
}
