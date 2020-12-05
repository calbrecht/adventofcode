use dotenv::from_filename;
use reqwest::{header::{COOKIE, HeaderMap, HeaderValue},
              blocking::{Response, Client, ClientBuilder}};
use std::env;

pub fn init_env () {
    let _ = from_filename(".env.local");
    let _ = from_filename(".env");

    let session: String = env::var("SESSION").unwrap();
    let session_cookie: String = ["session=", session.as_str()].concat();

    env::set_var("SESSION_COOKIE", session_cookie.as_str());
}

fn compose_url (path: &str) -> String {
    let base_url: String = env::var("BASE_URL").unwrap();
    let url: String = base_url + path;

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

pub fn fetch_input_text (client: &Client, day: &str) -> Result<String, String> {
    let path: String = ["/day/", day, "/input"].concat();
    let url: String = compose_url(path.as_str());
    let response: Response = get(client, url.as_str()).unwrap();

    match response.text() {
        Err(err) => Err(format!("{}", err)),
        Ok(text) => Ok(text)
    }
}
