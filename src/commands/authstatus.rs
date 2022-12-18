use crate::auth;
use std::io::ErrorKind;
#[tracing::instrument]
pub fn authstatus() -> std::io::Result<()> {
    match auth::get_token() {
        Ok(token) => {
            super::log("You're already logged in!\n");
            let mut stars = String::new();
            stars.push_str(&"*".repeat(token.len() - 5));
            super::log(&format!("Token: {}{}", &token[..5], stars));
            super::check_token();
        }
        Err(err) => match err.kind() {
            ErrorKind::NotFound => {
                super::err("You're not logged in yet!");
            }
            err => {
                super::err(&format!("Couldn't open token file: {}", err));
            }
        },
    }
    Ok(())
}
