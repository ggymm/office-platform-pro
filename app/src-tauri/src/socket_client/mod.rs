use std::io::{ErrorKind, Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

use tauri::{Invoke, Runtime, Window};
use tauri::plugin::Plugin;

pub mod proto;

pub struct TauriLibSocketClient<R: Runtime> {
    invoke_handler: Box<dyn Fn(Invoke<R>) + Send + Sync>,
}

impl<R: Runtime> TauriLibSocketClient<R> {
    pub fn new() -> Self {
        Self {
            invoke_handler: Box::new(tauri::generate_handler![send_message])
        }
    }
}

#[tauri::command]
fn send_message() {}


impl<R: Runtime> Plugin<R> for TauriLibSocketClient<R> {
    fn name(&self) -> &'static str {
        "lib_socket_client"
    }

    fn created(&mut self, _window: Window<R>) {
        tauri::async_runtime::spawn(async move {
            println!("not complete!")
        });
    }

    fn extend_api(&mut self, message: Invoke<R>) {
        (self.invoke_handler)(message)
    }
}