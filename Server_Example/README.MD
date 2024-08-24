# TCP Server
## Example Server in Rust Using Rocket
### Features
- [X] HTTP(S)
- [X] WebSocket
- [ ] UDP(Enet)
- [ ] Database(MongoDB & sqlite3)

## Usage
Navigate to the this directory and
```cmd
cargo run
```
build Directory Also has HTTP and HTTPS builds for Windows which runs on localhost 80 and 443 respectively
<br>
### **Note** 
When Running HTTPS you need **certs** folder which has **key.pem** and **cert.pem** to be in the same folder as the exe

## DOC
* `/`
	+ **GET**: "Server is running"
	+ **POST**: "Server is running"
* `/ws` (WebSocket)
	+ **Messages**
		- **ping**: Server will return a **pong**


