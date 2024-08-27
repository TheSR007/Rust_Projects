use rocket::{get, post, routes, tokio, State};
use rocket_ws as ws;
use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};
use rocket_db_pools::Database;
use rocket_db_pools::mongodb::Client;
use rocket::futures::stream::TryStreamExt;
use tokio::sync::mpsc as tokio_mpsc;
use tokio::task;
use tokio::runtime::Runtime;

mod enet_server;

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

#[post("/shutdown")]
async fn shutdown_enetserver(tx: &State<tokio_mpsc::Sender<enet_server::EnetCommand>>) -> &'static str {
    if let Err(e) = tx.send(enet_server::EnetCommand::Shutdown).await {
        eprintln!("Failed to send shutdown command: {:?}", e);
    }
    "ENet server shutting down"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Config has been moved to Rocket.toml
    // To Disable HTTPS just comment out default.tls and change The Port in Rocket.toml
    // Database Config is also in Rocket.toml
    // Enet Server is as enet_server.rs and example Client is in enet_client.rs

    let (tx, rx) = tokio_mpsc::channel(10);

    let enet_server_handle = task::spawn_blocking(move || {
        // Creating a runtime to run async code synchronously
        let rt = Runtime::new().expect("Failed to create Tokio runtime");
        rt.block_on(async {
            if let Err(e) = enet_server::run_enet_server(rx).await {
                eprintln!("ENet server error: {:?}", e);
            }
        })
    });

    let rocket = rocket::build()
        .attach(MongoDb::init())
        .manage(tx)
        .mount("/", routes![get_index, post_index, websocket, get_db, shutdown_enetserver])
        .ignite()
        .await?;

    rocket.launch().await?;

    enet_server_handle.await.expect("Failed to join ENet server task");

    Ok(())
}