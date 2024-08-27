# Example Server
## Example Server in Rust Using Rocket
### Features
- [X] HTTP(S)
- [X] WebSocket
- [X] Database(MongoDB)
- [X] UDP(Enet)


## Usage
Navigate to the this directory and
```cmd
cargo run --bin Example_Server
```
```cmd
cargo run --bin enet_client
```
for the server and the enet client. To build
```cmd
cargo build --release --bin Example_Server
```
```cmd
cargo build --release --bin enet_client
```
build Directory Also has **HTTP and HTTPS builds** for **Windows** which runs on **localhost 80 and 443** respectively **(need manual setup for /db endpoint else you will get [ ])**
**enet_client.exe** is also provided to checkout **UDP client** which **connects to the Server, sends a Client Hello msg and then Disconnect**
<br>
### **Note** 
- Make Sure The **MongoDB** is Setup Correctly for **Localhost** and you Have a database Named **DatabaseName** and a collection named **CollectionName** and in documnets you have **id (int64) and content (string)**
- You can change them and rebuild as you need, these are just placeholder and only requred if you wanna use the prebuilt binaries
- Run The Prebuilds in [**Terminal**](https://www.microsoft.com/store/productId/9N0DX20HK701) for Better View
- Also All the **Config of Server is in Rocket.toml** and Dependencies in Cargo.toml
- The **log_level** is **normal** by default, if you **don't want logs** make **log_level = "off" in Rocket.toml** and build it
## DOC
* `/`
	+ **GET**: "Server is running"
	+ **POST**: "Server is running"
* `/ws` (WebSocket)
	+ **Messages**
		- **ping**: Server will return a **pong**
* `/db` (Database)
	+ **GET**: Returns the MongoDB Database Documents (need to setup first)
* `/shutdown` (UDP Server)
	+ **POST**: **Shuts Down the Enet Server** which is running in a **Different Thread**


