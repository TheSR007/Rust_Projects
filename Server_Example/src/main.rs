use rocket::{get, post, routes};
use rocket_ws as ws;
use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};
use rocket_db_pools::Database;
use rocket_db_pools::mongodb::Client;
use rocket::futures::stream::TryStreamExt;

#[derive(Database)]
#[database("mongodb")]
struct MongoDb(Client);

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Log {
    id: i64,
    content: String,
}

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

#[get("/db")]
async fn get_db(db: &MongoDb) -> Json<Vec<Log>> {
    let database = db.database("DatabaseName");
    let collection = database.collection::<Log>("CollectionName"); // <Log> here is a structure of CollectionName in Rust

    match collection.find(None, None).await {
        Ok(cursor) => {
            match cursor.try_collect::<Vec<Log>>().await { // Make sure Document exist in the database Based on <Log> Struct
                Ok(logs) => {
                    Json(logs)
                },
                Err(e) => {
                    eprintln!("Error collecting logs: {:?}", e);
                    Json(vec![])
                }
            }
        },
        Err(e) => {
            eprintln!("Error querying MongoDB: {:?}", e);
            Json(vec![])
        }
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Config has been moved to Rocket.toml
    // To Disable HTTPS just comment out default.tls and change The Port in Rocket.toml
    // Database Config is also in Rocket.toml

    let rocket = rocket::build()
        .attach(MongoDb::init())
        .mount("/", routes![get_index, post_index, websocket, get_db])
        .ignite()
        .await?;

    rocket.launch().await?;
    Ok(())
}