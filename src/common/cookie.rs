use chrono::{prelude::*, Duration};
use wasm_bindgen::{JsCast};

pub fn create_cookie(name: &str, value: &str, days: i32) {
    let mut expires: String = "".to_string();

    if days > 0 {
        let date: DateTime<Utc> = Utc::now() + Duration::days(days.into());
        expires = format!("; expires={}", date.to_rfc3339());
    }

    let cookie_value = format!("{}={}{}; path=/", name, value, expires);

    let window = web_sys::window().expect("global window does not exists");
    let document = window.document().expect("expecting a document on window");
    let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();
    html_document
        .set_cookie(&cookie_value)
        .expect("Failed to set the cookie.");
}

pub fn read_cookie(name: &str) -> String {
    let window = web_sys::window().expect("Global window does not exists");
    let document = window.document().expect("Expecting a document on window");
    let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();
    let cookies = html_document.cookie().expect("Cannot get cookie.");

    let cookie_list = cookies.split(";");
    let mut valids = cookie_list.filter(|&x| x.to_string().contains(name));

    let cookie = valids.next();
    match cookie {
        Some(_) => {
            let values = cookie.expect("Cannot get cookie value").split("=");
            let cookie_value = values.last().expect("No cookie value found.").to_string();
            return cookie_value;
        },
        None => {return "".to_string()}
    }   
}

#[allow(dead_code)]
pub fn erase_cookie(name: &str) {
    create_cookie(name, "", -1)
}
