use rocket::http::{Cookie, Cookies};
use rand::{self, Rng};
use std::env;
use redis::{self, Commands};

const BASE62: &'static [u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const SESSION_LENGTH: usize = 60 * 10;

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
    let _ : () = con.expire(api_key, SESSION_LENGTH).unwrap();
}

pub fn remove_from_redis(api_key: String) {
    let con = get_redis_connection();
    let _: () = con.del(api_key).unwrap();
}

pub fn get_id_from_session(cookies: &Cookies) -> Option<i32> {
    if let Some(cookie) =  cookies.find("doc-session") {
        let api_key = cookie.value;
        let user_id = get_user_id_from_api_key(api_key);
        user_id
    }
    else {
        None
    }
}

// ko dobimo veljaven session, ga tudi podaljsamo
fn get_user_id_from_api_key(api_key: String) -> Option<i32> {
    let con = get_redis_connection();
    let user_id = con.get(&api_key);
    println!("{:?}", user_id);
    match user_id {
        Ok(_) => {
            let _ : () = con.expire(&api_key, SESSION_LENGTH).unwrap();
            user_id.unwrap()
        },
        Err(_)  => None
    }
}

fn get_redis_connection() -> redis::Connection {
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let client = redis::Client::open(&redis_url[..]).unwrap();
    client.get_connection().unwrap()
}