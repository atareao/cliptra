use cli_clipboard::{ClipboardContext, ClipboardProvider};
use std::{env, collections::HashMap};
use dotenv::dotenv;
use serde_json::Value;

use gtk4 as gtk;
use gtk::{prelude::*, Orientation, ScrolledWindow, WrapMode};
use gtk::{Application, ApplicationWindow, TextView, TextBuffer, Box};
use dirs::config_dir;

const APP_ID: &str = "es.atareao.instrl";

fn main() {
    let mut config_path = config_dir().unwrap();
    config_path.push("cliptra");
    config_path.push(".env");
    println!("{:?}", config_path);

    dotenv::from_path(config_path).ok();
    //dotenv().ok();
    let auth_key = env::var("DEEPL_AUTH_KEY").expect("Authentication not found");

    let app = Application::builder().application_id(APP_ID).build();
    //let from_text = "Desde".to_string();
    //let to_text = "To".to_string();
    let mut ctx = ClipboardContext::new().unwrap();
    let from_text  = ctx.get_contents().unwrap();
    let to_text = translate(&auth_key, &from_text);

    app.connect_activate(move |app|{
        build_ui(app, &from_text, &to_text);
    });

    app.run();
    // println!("{}", auth_key);
    // println!("Hello, world!");
    // let mut ctx = ClipboardContext::new().unwrap();
    // let contents  = ctx.get_contents().unwrap();
    // println!("Clipboard content: {}", &contents);
    // translate(&auth_key, &contents);

}

fn build_ui(app: &Application, from_text: &str, to_text: &str) {
    // Create a window and set the title
    let vbox = Box::new(Orientation::Vertical, 10);
    let scrolled_window_from = ScrolledWindow::new();
    scrolled_window_from.set_hexpand(true);
    scrolled_window_from.set_vexpand(true);
    let text_box_from = TextView::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    text_box_from.set_wrap_mode(WrapMode::Char);
    scrolled_window_from.set_child(Some(&text_box_from));
    vbox.append(&scrolled_window_from);
    let scrolled_window_to = ScrolledWindow::new();
    scrolled_window_to.set_hexpand(true);
    scrolled_window_to.set_vexpand(true);
    let text_box_to = TextView::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    text_box_to.set_wrap_mode(WrapMode::Char);
    scrolled_window_to.set_child(Some(&text_box_to));
    vbox.append(&scrolled_window_to);
    let buffer_from = TextBuffer::new(None);
    buffer_from.set_text(from_text);
    text_box_from.set_buffer(Some(&buffer_from));
    let buffer_to = TextBuffer::new(None);
    buffer_to.set_text(to_text);
    text_box_to.set_buffer(Some(&buffer_to));
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&vbox)
        .build();

    // Present window
    window.present();
}


fn translate(auth_key: &str, texto: &str) -> String{
    let url = "https://api-free.deepl.com/v2/translate";
    let mut map = HashMap::new();
    map.insert("auth_key", auth_key);
    map.insert("text", texto);
    map.insert("source_lang", "EN");
    map.insert("target_lang", "ES");
    let client = reqwest::blocking::Client::new();
    let res = client.post(url)
        .form(&map)
        .send()
        .unwrap();
    let content = res.text().unwrap();
    println!("{:?}", &content);
    let v: Value = serde_json::from_str(&content).unwrap();
    println!("{}", v["translations"][0]["text"]);
    v["translations"][0]["text"].to_string()
}
