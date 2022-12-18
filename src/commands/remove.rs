use spinners::*;
#[tracing::instrument]
pub fn remove() {
    let token = super::expect_token();
    match super::ask_for_app_id(token.clone(), "delete", false) {
        Ok(app_id) => {
            let mut spinner = Spinner::new(Spinners::Flip, "Deleting your app".into());
            match crate::entities::app::App::delete(token, app_id) {
                Ok(()) => {
                    spinner
                        .stop_with_message(super::format_log("Your app was successfully nuked!"));
                }
                Err(err) => {
                    super::err(&format!("Couldn't delete your app: {}", err));
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
