use rocket::http::{Cookie, Cookies};

pub fn get_id_from_session(cookies: &Cookies) -> Result<i32, String>{
    if let Some(cookie) = cookies.find("session") {
        Ok(cookie.value.parse::<i32>().unwrap())
    }
    else {
        Err(format!("User is not loged in"))
    }
}