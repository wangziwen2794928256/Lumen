//! HTTP serves only an in-memory public projection and bundled assets. Never a filesystem or database endpoint.
use crate::database::Result;
use std::{
    io::{Read, Write},
    net::TcpListener,
    sync::{Arc, Mutex, OnceLock},
    time::Duration,
};
type Preview = (u16, Arc<Mutex<Vec<u8>>>);
static PREVIEW: OnceLock<Mutex<Option<Preview>>> = OnceLock::new();
pub fn start(json: Vec<u8>) -> Result<String> {
    let mut guard = PREVIEW
        .get_or_init(|| Mutex::new(None))
        .lock()
        .map_err(|_| crate::database::DbError::invalid())?;
    if let Some((port, data)) = &*guard {
        *data
            .lock()
            .map_err(|_| crate::database::DbError::invalid())? = json;
        return Ok(format!("http://127.0.0.1:{port}/"));
    }
    let listener = TcpListener::bind(("127.0.0.1", 0))?;
    let port = listener.local_addr()?.port();
    let data = Arc::new(Mutex::new(json));
    let shared = data.clone();
    std::thread::spawn(move || {
        for mut stream in listener.incoming().flatten() {
            let _ = stream.set_read_timeout(Some(Duration::from_millis(500)));
            let _ = stream.set_write_timeout(Some(Duration::from_millis(500)));
            let mut buffer = [0; 4096];
            let Ok(n) = stream.read(&mut buffer) else {
                continue;
            };
            let Ok(request) = std::str::from_utf8(&buffer[..n]) else {
                continue;
            };
            let Some(path) = allowed_path(request, port) else {
                let _ = stream.write_all(
                    b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                );
                continue;
            };
            let Ok(json) = shared.lock() else {
                continue;
            };
            let (body, mime) = match path {
                "/" | "/index.html" => (
                    include_bytes!("../../../website/index.html").as_slice(),
                    "text/html; charset=utf-8",
                ),
                "/site.js" => (
                    include_bytes!("../../../website/site.js").as_slice(),
                    "text/javascript; charset=utf-8",
                ),
                "/site.css" => (
                    include_bytes!("../../../website/site.css").as_slice(),
                    "text/css; charset=utf-8",
                ),
                _ => (json.as_slice(), "application/json; charset=utf-8"),
            };
            let header = format!("HTTP/1.1 200 OK\r\nContent-Type: {mime}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nX-Content-Type-Options: nosniff\r\nContent-Security-Policy: default-src 'self'; script-src 'self'; style-src 'self'; connect-src 'self'; img-src 'none'; frame-ancestors 'none'; base-uri 'none'; form-action 'none'\r\nConnection: close\r\n\r\n",body.len());
            let _ = stream.write_all(header.as_bytes());
            let _ = stream.write_all(body);
        }
    });
    *guard = Some((port, data));
    Ok(format!("http://127.0.0.1:{port}/"))
}
fn allowed_path(request: &str, port: u16) -> Option<&str> {
    let mut lines = request.split("\r\n");
    let mut first = lines.next()?.split(' ');
    if first.next()? != "GET" {
        return None;
    }
    let path = first.next()?;
    if first.next()? != "HTTP/1.1" || first.next().is_some() || !request.ends_with("\r\n\r\n") {
        return None;
    }
    let hosts: Vec<_> = lines
        .filter_map(|s| s.split_once(':'))
        .filter(|(k, _)| k.eq_ignore_ascii_case("host"))
        .collect();
    if hosts.len() != 1 || hosts[0].1.trim() != format!("127.0.0.1:{port}") {
        return None;
    }
    [
        "/",
        "/index.html",
        "/site.js",
        "/site.css",
        "/public-profile.json",
    ]
    .contains(&path)
    .then_some(path)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn no_arbitrary_endpoints_or_dns_rebinding() {
        assert_eq!(
            allowed_path(
                "GET /public-profile.json HTTP/1.1\r\nHost: 127.0.0.1:1234\r\n\r\n",
                1234
            ),
            Some("/public-profile.json")
        );
        for (verb, path, host) in [
            ("GET", "/lumen.db", "127.0.0.1:1234"),
            ("GET", "/../attachments/a", "127.0.0.1:1234"),
            ("POST", "/", "127.0.0.1:1234"),
            ("GET", "/", "evil.example"),
            ("GET", "/public-profile.json?sql=x", "127.0.0.1:1234"),
        ] {
            assert!(allowed_path(
                &format!("{verb} {path} HTTP/1.1\r\nHost: {host}\r\n\r\n"),
                1234
            )
            .is_none());
        }
    }
}
