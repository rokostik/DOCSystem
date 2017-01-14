#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

mod static_files;

use std::io;
use std::path::{Path, PathBuf};

use rocket::response::NamedFile;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[post("/")]
fn dashboard() -> io::Result<NamedFile> {
    print!("what");
    NamedFile::open("static/dashboard.html")
}

fn main() {
    rocket::ignite().mount("/", routes![index, static_files::all])
                    .mount("/dashboard", routes![dashboard])
                    .launch();
}