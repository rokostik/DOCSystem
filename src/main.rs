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

use rocket::request::{Form};
use rocket::response::Redirect;
use rocket::http::{Cookie, Cookies};
use std::io;

use rocket::response::NamedFile;
use models::{User, UserLogin, UserNew};

#[get("/")]
fn show_login(cookies: &Cookies) -> io::Result<NamedFile> {

    /*if let Some(cookie) = cookies.find("session") {
        let mut cook = cookie;
        println!("{:?}", cook);
        cook.expires = Some(time::now() + time::Duration::minutes(15));
        cookies.add(cook);

        cookies.remove("session");
    }
    */


    NamedFile::open("static/login.html")
}

#[post("/", data = "<user_login>")]
fn login(cookies: &Cookies, user_login: Form<UserLogin>) -> Redirect {
    let user: Vec<User> = user_login.into_inner().get();

    let mut cookie: Cookie = Cookie::new("session".to_string(), "1".to_string());
    cookie.expires = Some(time::now() + time::Duration::minutes(15));
    cookies.add(cookie);
    println!("{:?}", user);
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
fn register(user_new: Form<UserNew>) -> io::Result<NamedFile> {
    let user = user_new.into_inner();
    //TODO:validation
    if user.insert() {
        NamedFile::open("static/dashboard.html")
    }
    else {
        NamedFile::open("static/login.html")
    }
}

#[get("/")]
fn dashboard(cookies: &Cookies) -> Result<io::Result<NamedFile>, Redirect> {
    for cookie in cookies.iter() {
        println!("{:?}", cookie);
    }

    if let Some(cookie) = cookies.find("session") {
        Ok(NamedFile::open("static/dashboard.html"))
    }
    else {
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