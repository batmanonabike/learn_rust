// Network Programming in Rust,  Abhishek Chanda
// Page 90
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct ServerConfig {
    workers: u64,
    ignore: bool,
    auth_server: Option<String>,
}

fn main() {
    let config = ServerConfig {
        workers: 100,
        ignore: false,
        auth_server: Some("auth.server.io".to_string()),
    };

    println!("To and from JSON");
    let json = serde_json::to_string(&config).unwrap();
    println!("{}", json);

    let obj: ServerConfig = serde_json::from_str(&json).unwrap();
    println!("{:?}", obj);
}
