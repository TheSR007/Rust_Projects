# TCP Server
## Example Server in Rust Using Rocket
### Features
- [X] HTTP(S)
- [X] WebSocket
- [X] Database(MongoDB)
- [ ] UDP(Enet)


## Usage
Navigate to the this directory and
```cmd
cargo run
```
build Directory Also has **HTTP and HTTPS builds** for **Windows** which runs on **localhost 80 and 443** respectively **(need manual setup for /db endpoint else you will get [ ])**
<br>
### **Note** 
- Make Sure The **MongoDB** is Setup Correctly for **Localhost** and you Have a database Named **DatabaseName** and a collection named **CollectionName** and in documnets you have **id (int64) and content (string)**
- You can change them and rebuild as you need, these are just placeholder and only requred if you wanna use the prebuilt binaries
- Run The Prebuilds in [**Terminal**](https://www.microsoft.com/store/productId/9N0DX20HK701) for Better View
- Also All the **Config of Server is in Rocket.toml** and Dependencies in Cargo.toml
## DOC
* `/`
	+ **GET**: "Server is running"
	+ **POST**: "Server is running"
* `/ws` (WebSocket)
	+ **Messages**
		- **ping**: Server will return a **pong**
* `/db` (Database)
	+ **GET**: Returns the MongoDB Database Documents (need to setup first)


