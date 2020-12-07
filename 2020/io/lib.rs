use dotenv::from_filename;
use reqwest::{header::{COOKIE, HeaderMap, HeaderValue},
              blocking::{Response, Client, ClientBuilder}};
use std::env;
use std::fs;

pub fn init_env () {
    let _ = from_filename(".env.local");
    let _ = from_filename(".env");

    let session: String = env::var("SESSION").unwrap();
    let session_cookie: String = ["session=", session.as_str()].concat();

    env::set_var("SESSION_COOKIE", session_cookie.as_str());
}

fn compose_url (path: &str) -> String {
    let year = env::var("YEAR").unwrap();
    let base_url = env::var("BASE_URL").unwrap();
    let url = [base_url, year, path.to_string()].join("/");

    url
}

pub fn build_client () -> reqwest::Result<Client> {
    let mut headers = HeaderMap::new();

    let cookie: String = env::var("SESSION_COOKIE").unwrap();

    headers.insert(COOKIE, HeaderValue::from_str(cookie.as_str()).unwrap());

    ClientBuilder::new().default_headers(headers).build()
}

fn get (client: &Client, url: &str) -> reqwest::Result<Response> {
    client.get(url).send()
}

fn fetch_input_cache_miss (client: &Client, file: &str, day: &str) -> Result<String, String> {
    let path: String = ["day", day, "input"].join("/");
    let url: String = compose_url(path.as_str());
    let response: Response = get(client, url.as_str()).unwrap();

    match response.text() {
        Err(err) => Err(format!("{}", err)),
        Ok(text) => {
            let _ = fs::write(&file, &text);
            Ok(text)
        }
    }
}

pub fn fetch_input_text (client: &Client, day: &str) -> Result<String, String> {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let file = [dir.as_str(), "input.txt"].join("/");

    match fs::read_to_string(&file) {
        Ok(text) => Ok(text),
        Err(_err) => fetch_input_cache_miss(client, &file, day)
    }

}

fn post (client: &Client, url: &str, params: &Vec<(&str, &str)>) -> reqwest::Result<Response> {
    client.post(url).form(params).send()
}

pub fn post_result_text (client: &Client, day: &str, lvl: &str, result: &str) -> Result<String, String> {
    let path = ["day", day, "answer"].join("/");
    let url = compose_url(path.as_str());
    let params: Vec<(&str, &str)> = vec![("level", lvl), ("answer", result)];
    let response: Response = post(client, url.as_str(), &params).unwrap();

    match response.text() {
        Err(err) => Err(format!("{}", err)),
        Ok(text) => Ok(text)
    }
}
