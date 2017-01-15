#![feature(plugin, custom_derive, custom_attribute)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate dotenv;

extern crate rocket;
extern crate time;

mod static_files;
mod models;
mod utils;

use rocket::request::{Form};
use rocket::response::Redirect;
use rocket::response::NamedFile;
use rocket::http::{Cookie, Cookies};
use std::io;

use utils::*;
use models::{User, UserLogin, UserNew};

#[get("/")]
fn show_login(cookies: &Cookies) -> io::Result<NamedFile> {
    NamedFile::open("static/login.html")
}

#[post("/", data = "<user_login>")]
fn login(cookies: &Cookies, user_login: Form<UserLogin>) -> Redirect {
    let user: User = user_login.into_inner().get();
    println!("{:?}", user);
    let mut cookie: Cookie = Cookie::new("session".to_string(), "1".to_string());
    cookie.expires = Some(time::now() + time::Duration::minutes(15));
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
fn dashboard(cookies: &Cookies) -> Result<io::Result<NamedFile>, Redirect> {
    if let Ok(user_id) = utils::get_id_from_session(&cookies) {
        Ok(NamedFile::open("static/dashboard.html"))
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