#![feature(plugin, custom_derive, custom_attribute)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate dotenv;

extern crate rocket;

mod static_files;
mod models;

use rocket::request::{Form};
use std::io;

use rocket::response::NamedFile;
use models::{User, UserLogin, UserNew};

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/login.html")
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

#[post("/", data = "<user_login>")]
fn dashboard(user_login: Form<UserLogin>) -> io::Result<NamedFile> {
    let user: Vec<User> = user_login.into_inner().get();
    println!("{:?}", user);
    NamedFile::open("static/dashboard.html")
}

fn main() {
    rocket::ignite().mount("/", routes![index, static_files::all])
                    .mount("/dashboard", routes![dashboard])
                    .mount("/register", routes![register])
                    .launch();
}