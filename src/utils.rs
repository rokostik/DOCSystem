use rocket::http::{Cookie, Cookies};
use rand::{self, Rng};
use std::env;
use redis::{self, Commands};

use models::*;

const BASE62: &'static [u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const SESSION_LENGTH: i32 = 60 * 10;

pub fn generate_api_key() -> String {
    let size = 32;
    let mut api_key = String::with_capacity(size);
    let mut rng = rand::thread_rng();
    for _ in 0..size {
        api_key.push(BASE62[rng.gen::<usize>() % 62] as char);
    }
    api_key
}

pub fn save_to_redis(api_key: String, user_id: i32) {
    let con = get_redis_connection();
    let _ : () = con.set(&api_key, user_id).unwrap();
    //TODO: change session length
    let _ : () = con.expire(api_key, 10).unwrap();
}

pub fn remove_from_redis(api_key: String) {
    let con = get_redis_connection();
    let _: () = con.del(api_key).unwrap();
}

pub fn get_id_from_session(cookies: &Cookies) -> Option<i32> {
    if let Some(cookie) =  cookies.find("doc-session") {
        let api_key = cookie.value;
        let user_id = get_user_id_from_api_key(api_key);
        Some(user_id)
    }
    else {
        None
    }
}

// metoda tudi podaljsa session expiration
fn get_user_id_from_api_key(api_key: String) -> i32 {
    let con = get_redis_connection();
    //TODO: change session length
    let _ : () = con.expire(&api_key, 10).unwrap();
    con.get(api_key).unwrap()
}

fn get_redis_connection() -> redis::Connection {
    let redis_url = env::var("REDIS_URL")
        .expect("REDIS_URL must be set");
    let client = redis::Client::open(&redis_url[..]).unwrap();
    client.get_connection().unwrap()
}

#[derive(Debug, Serialize)]
pub struct Context{ user: User,
                    folders: Vec<Folder>,
                    folder_name: String,
                    documents: Option<Vec<Document>>,
                    document: Option<Document>, }

impl Context {
    pub fn folder_view(user_id: i32, folder_name: String) -> Context {
        let user: User = User::get(user_id);
        let folders: Vec<Folder> = (&user).get_folders();
        let folder = folders.clone().into_iter().filter(|folder| folder.name == folder_name).next().unwrap();
        let documents: Vec<Document> = folder.get_documents();
        Context{ user: user,
                 folders: folders,
                 folder_name: folder_name,
                 documents: Some(documents),
                 document: None }
    }

    pub fn document_view(user_id: i32, folder_name: String, document_name: String) -> Context {
        let user: User = User::get(user_id);
        let folders: Vec<Folder> = (&user).get_folders();
        let folder = folders.clone().into_iter().filter(|folder| folder.name == folder_name).next().unwrap();
        let document = folder.get_document_by_name(document_name);
        Context{ user: user,
                 folders: folders,
                 folder_name: folder_name,
                 documents: None,
                 document: Some(document) }
    }
}