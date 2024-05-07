
use std::ptr;
use std::cmp::Ordering;

const URL_PROTOCOL_MAX_LENGTH: usize = 20;

static URL_SCHEMES: [&str; 4] = [
    "aaa", 
    "javascript", 
    "jdbc", 
    "doi"
];

fn url_is_protocol(str: &str) -> bool {
    for scheme in URL_SCHEMES.iter() {
        if str.eq_ignore_ascii_case(scheme) {
            return true;
        }
    }
    false
}

fn url_is_ssh(str: &str) -> bool {
    str.eq_ignore_ascii_case("ssh") || str.eq_ignore_ascii_case("git")
}

fn get_part(url: &str, format: &str, l: usize) -> Option<String> {
    let mut has = false;
    let mut tmp = String::new();
    let mut tmp_url = url.to_string();
    let mut fmt_url = url.to_string();
    
    fmt_url = &fmt_url[..l];

    if let Ok(extracted) = fmt_url.parse::<String>() {
        tmp = extracted;
        has = url != &tmp_url;
    }

    fmt_url = &fmt_url[l..];

    if has {
        Some(tmp)
    } else {
        None
    }
}

fn url_get_protocol(url: &str) -> Option<String> {
    let mut protocol = String::with_capacity(URL_PROTOCOL_MAX_LENGTH);
    if let Ok(_) = url.parse::<String>(&format!("%[^://]s", &mut protocol)) {
        if url_is_protocol(&protocol) {
            return Some(protocol);
        }
    }
    None
}

fn url_get_auth(url: &str) -> Option<String> {
    let protocol = url_get_protocol(url)?;
    let l = protocol.len() + 3;
    get_part(url, "%[^@]s", l)
}

fn url_get_hostname(url: &str) -> Option<String> {
    let mut l = 3;
    let protocol = url_get_protocol(url)?;
    if let Some(auth) = url_get_auth(url) {
        l += auth.len() + 1; // Add 1 for @ symbol
    }
    l += protocol.len();
    if url_is_ssh(&protocol) {
        get_part(url, "%[^:]s", l)
    } else {
        get_part(url, "%[^/]s", l)
    }
}

fn url_get_path(url: &str) -> Option<String> {
    let mut l = 3;
    let protocol = url_get_protocol(url)?;
    let auth = url_get_auth(url);
    let hostname = url_get_hostname(url)?;
    let is_ssh = url_is_ssh(&protocol);
    l += protocol.len() + hostname.len();
    if let Some(auth) = auth {
        l += auth.len() + 1; // Add 1 for @ symbol
    }
    let fmt = if is_ssh { "%s" } else { "/%s" };
    let tmp_path = get_part(url, fmt, l)?;
    Some(format!(fmt, tmp_path))
}

fn main() {
    let url = "http://user:pass@subdomain.host.com:8080/p/a/t/h?query=string#hash";
    assert_eq!("/p/a/t/h?query=string#hash", url_get_path(url).unwrap());
}
