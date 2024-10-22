use dotenv::dotenv;
use reqwest::blocking::Client;
use serde_json::{self, json};
use std::env;
// use std::thread::sleep;
// use std::time::Duration;
use x_win::{get_active_window, WindowInfo};

fn send_event(client: &Client, endpoint: &str, active_window: &WindowInfo) {
    let response = client
        .post(endpoint)
        .json(&json!({
            "app_name": active_window.info.name,
            "title": active_window.title,
            "url": active_window.info.path,
            "type": "app",
        }))
        .send();

    match response {
        Ok(response) => {
            println!("reqwest response: {:?}", response);
        }
        Err(e) => {
            println!("reqwest error: {:?}", e);
        }
    }
}

fn main() {
    dotenv().ok();

    let client = Client::new();
    let api_url = env::var("API_URL").expect("API_URL not provided");
    let endpoint = api_url + "/new-event";

    // loop {
    match get_active_window() {
        Ok(active_window) => {
            println!("active window: {:?}", active_window);

            send_event(&client, &endpoint, &active_window)
        }
        Err(e) => {
            println!("x-win error: {:?}", e);
        }
    }

    // sleep(Duration::from_secs(5));
    // sleep(Duration::from_secs(10 * 60));
    // }
}
