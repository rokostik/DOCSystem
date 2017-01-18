#![feature(plugin, custom_derive, custom_attribute)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate serde;
extern crate serde_json;
extern crate rocket;
extern crate rocket_contrib;
extern crate time;
extern crate chrono;
extern crate rand;
extern crate redis;
extern crate bcrypt;

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate dotenv;
#[macro_use] extern crate serde_derive;

mod static_files;
mod models;
mod session;
mod context;

use rocket::request::{Form};
use rocket::response::Redirect;
use rocket::response::NamedFile;
use rocket::http::{Cookie, Cookies};
use rocket_contrib::Template;
use std::io;

use session::*;
use models::*;
use context::*;

#[get("/")]
fn show_login(cookies: &Cookies) -> io::Result<NamedFile> {
    NamedFile::open("static/login.html")
}

#[post("/", data = "<user_login>")]
fn login(cookies: &Cookies, user_login: Form<UserLogin>) -> Redirect {
    let user: User = user_login.into_inner().get();
    let api_key = session::generate_api_key();
    save_to_redis(api_key.clone(), user.id);
    let mut cookie: Cookie = Cookie::new("doc-session".to_string(), api_key);
    cookies.add(cookie);
    Redirect::to("/")
}

#[get("/")]
fn logout(cookies: &Cookies) -> Redirect {
    if let Some(cookie) = cookies.find("doc-session") {
        remove_from_redis(cookie.value);
        cookies.remove("doc-session");
    }
    Redirect::to("/login")
}

#[get("/")]
fn show_register() -> io::Result<NamedFile> {
    NamedFile::open("static/register.html")
}

#[post("/", data = "<user_new>")]
fn register(user_new: Form<UserNew>) -> Redirect {
    let mut user = user_new.into_inner();
    //TODO:validation
    if user.insert() {
        Redirect::to("/login")
    }
    else {
        Redirect::to("/register")
    }
}

#[get("/")]
fn dashboard_redirect(cookies: &Cookies) -> Redirect {
    if let Some(user_id) = get_id_from_session(&cookies) {
        let user: User = User::get(user_id);
        let folders: Vec<Folder> = (&user).get_folders();
        let ref folder_name = folders[0].name;
        Redirect::to(&format!("/home/{}", folder_name))
    } else {
        Redirect::to("/login")
    }
}

#[get("/<folder_name>")]
fn show_folder(cookies: &Cookies, folder_name: &str) -> Result<Template, Redirect> {
    if let Some(user_id) = get_id_from_session(&cookies) {
        Ok(Template::render("dashboard_folder", &Context::folder_view(user_id, folder_name.to_string())))
    } else {
        Err(Redirect::to("/login"))
    }
}

#[get("/<folder_name>/<document_name>")]
fn show_document(cookies: &Cookies, folder_name: &str, document_name: &str) -> Result<Template, Redirect> {
    if let Some(user_id) = get_id_from_session(&cookies) {
        let user: User = User::get(user_id);
        Ok(Template::render("dashboard_document", &Context::document_view(user_id, folder_name.to_string(), document_name.to_string())))
    } else {
        Err(Redirect::to("/login"))
    }
}

/*#[post("/", data="<folder_new>")]
fn new_folder(cookies: &Cookies, folder_new: Form<FolderNew>) -> Redirect {
    if let Ok(user_id) = get_id_from_session(&cookies) {
        let user: User = User::get(user_id);
        if user.new_folder(folder_new) {
            Redirect::to("/")
        }
        else {
            Redirect::to("/")
        }

    } else {
        Redirect::to("/login")
    }
}*/

#[get("/")]
fn show_profile(cookies: &Cookies) -> Result<Template, Redirect> {
    if let Some(user_id) = get_id_from_session(&cookies) {
        Ok(Template::render("dashboard_user", &Context::profile_view(user_id)))
    } else {
        Err(Redirect::to("/login"))
    }
}

#[post("/", data = "<user_updated>")]
fn update_profile(cookies: &Cookies, user_updated: Form<UserNew>) -> Redirect {
    if let Some(user_id) = get_id_from_session(&cookies) {
        let user_updated = user_updated.into_inner();
        let mut user = User::get(user_id);
        //TODO:validation
        println!("router user_name: {:?}", user_updated.name);
        user.update_profile(user_updated);
        Redirect::to("/")
    } else {
        Redirect::to("/login")
    }
}

#[error(404)]
fn not_found(req: &rocket::Request) -> String {
    format!("<p>'{}' was not found</p>", req.uri())
}

fn main() {
    rocket::ignite().mount("/",         routes![dashboard_redirect, static_files::all])
                    .mount("/home",     routes![show_folder, show_document])
                    //.mount("/new",    routes![new_folder])
                    .mount("/login",    routes![login, show_login])
                    .mount("/logout",   routes![logout])
                    .mount("/register", routes![register, show_register])
                    .mount("/user",     routes![update_profile, show_profile])
                    .catch(errors![not_found])
                    .launch();
}