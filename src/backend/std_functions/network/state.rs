use std::{
    collections::HashMap,
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

pub static LISTENERS: Mutex<Option<HashMap<i64, TcpListener>>> = Mutex::new(None);
pub static STREAMS: Mutex<Option<HashMap<i64, TcpStream>>> = Mutex::new(None);
static NEXT_HANDLE: Mutex<i64> = Mutex::new(0);

pub fn next_handle() -> i64 {
    let mut handle = NEXT_HANDLE.lock().unwrap();
    *handle += 1;
    *handle
}
