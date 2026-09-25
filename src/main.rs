use std::env;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

const PAGE: &str = r#"<!doctype html>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>Live on velixir</title>
<style>
  :root { color-scheme: dark; }
  body { margin:0; min-height:100vh; display:grid; place-items:center;
         background:#0b0b10; color:#e8e8ef;
         font:16px/1.6 ui-sans-serif,system-ui,-apple-system,Segoe UI,sans-serif; }
  main { max-width:34rem; padding:2.5rem 1.5rem; text-align:center; }
  h1 { font-size:1.6rem; margin:0 0 .5rem; letter-spacing:-.02em; }
  p { color:#a0a0b0; margin:.5rem 0; }
  code { background:#16161f; border:1px solid #26263a; border-radius:6px;
         padding:.15rem .4rem; font-size:.875em; color:#c7d2fe; }
  .dot { display:inline-block; width:.5rem; height:.5rem; border-radius:50%;
         background:#34d399; margin-right:.5rem; vertical-align:middle; }
</style>
<main>
  <p><span class="dot"></span>Rust is running on velixir</p>
  <h1>Your first deploy worked.</h1>
  <p>This page is served by your own container, built from source. No Dockerfile was involved.</p>
  <p>Next: edit <code>src/main.rs</code>, then run <code>velixir deploy</code>.</p>
</main>"#;

fn main() {
    // velixir injects PORT. 0.0.0.0 so the listener is reachable from outside the
    // container, which is where the health check comes from.
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).expect("could not bind the port");
    println!("listening on {port}");

    for stream in listener.incoming() {
        let Ok(mut stream) = stream else { continue };

        let mut first = String::new();
        let _ = BufReader::new(&stream).read_line(&mut first);

        let (ctype, body) = if first.starts_with("GET /healthz") {
            ("text/plain", "ok")
        } else {
            ("text/html; charset=utf-8", PAGE)
        };

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: {ctype}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        );
        let _ = stream.write_all(response.as_bytes());
    }
}
