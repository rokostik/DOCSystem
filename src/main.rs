#![feature(plugin, custom_derive, custom_attribute)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate serde_json;
extern crate rocket;
extern crate rocket_contrib;
extern crate time;

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate dotenv;
#[macro_use] extern crate serde_derive;

mod static_files;
mod models;
mod utils;

use rocket::request::{Form};
use rocket::response::Redirect;
use rocket::response::NamedFile;
use rocket::http::{Cookie, Cookies};
use rocket_contrib::Template;
use std::io;

use utils::*;
use models::*;

#[get("/")]
fn show_login(cookies: &Cookies) -> io::Result<NamedFile> {
    NamedFile::open("static/login.html")
}

#[post("/", data = "<user_login>")]
fn login(cookies: &Cookies, user_login: Form<UserLogin>) -> Redirect {
    let user: User = user_login.into_inner().get();
    let mut cookie: Cookie = Cookie::new("session".to_string(), user.id.to_string());
    cookie.expires = Some(time::now() + time::Duration::minutes(10));
    cookies.add(cookie);
    Redirect::to("/")
}

#[get("/")]
fn logout(cookies: &Cookies) -> Redirect {
    if let Some(cookie) = cookies.find("session") {
        cookies.remove("session");
    }
    Redirect::to("/login")
}

#[get("/")]
fn show_register() -> io::Result<NamedFile> {
    NamedFile::open("static/register.html")
}

#[post("/", data = "<user_new>")]
fn register(user_new: Form<UserNew>) -> Redirect {
    let user = user_new.into_inner();
    //TODO:validation
    if user.insert() {
        Redirect::to("/login")
    }
    else {
        Redirect::to("/register")
    }
}

#[get("/")]
fn dashboard(cookies: &Cookies) -> Result<Template, Redirect> {
    if let Ok(user_id) = get_id_from_session(&cookies) {
        let user: User = User::get(user_id);
        Ok(Template::render("dashboard_base", &Context::folder_view(user_id)))
        //Ok(NamedFile::open("static/dashboard.html"))
    } else {
        Err(Redirect::to("/login"))
    }
}

fn main() {
    rocket::ignite().mount("/", routes![dashboard, static_files::all])
                    .mount("/login", routes![login, show_login])
                    .mount("/register", routes![register, show_register])
                    .mount("/logout", routes![logout])
                    .launch();
}