use rocket::{get, post, launch, routes};
use ws;

#[get("/")]
fn get_index() -> &'static str {
    "Server is running"
}

#[post("/")]
fn post_index() -> &'static str {
    "Server is running"
}

#[get("/ws")]
fn websocket(ws: ws::WebSocket) -> ws::Stream!['static] {
    ws::Stream! { ws =>
        for await message in ws {
            match message {
                Ok(msg) => {
                    if let ws::Message::Text(text) = msg {
                        if text == "ping" {
                            yield ws::Message::Text("pong".to_string());
                        }
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    }
}

#[launch]
fn rocket() -> _ {
    let ssl_state = true; // Toogle for HTTPS and HTTP
    let port = if ssl_state { 443 } else { 80 };

    let mut config = rocket::config::Config::default();
    config.port = port;

    if ssl_state {
        config.tls = Some(rocket::config::TlsConfig::from_paths(
            "certs/cert.pem",
            "certs/key.pem",
        ));
    }

    rocket::custom(config)
      .mount("/", routes![get_index, post_index, websocket])
}


