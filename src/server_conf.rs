/*

Generate a server.conf file that looks like this

[Interface]
PrivateKey = redacted
ListenPort = 443

*/

static WG_SERVER_PORT: i32 = 51820;

use crate::gen_keys::*;

use inquire::*;
use rand_core::OsRng;
use std::{fs::File, io::Read};
use base64::{engine::general_purpose, Engine as _};

pub struct ServerConf {
    pub interface_name: String,
    pub priv_key: PrivKey,
    pub pub_key: PubKey,
    pub port: i32,
}

impl ServerConf {
    pub fn new(interface_name: String, port: i32) -> ServerConf {
        let priv_key = PrivKey::new(OsRng);
        let pub_key = PubKey::from(&priv_key);
        ServerConf {
            interface_name,
            priv_key,
            pub_key,
            port,
        }
    }

    pub fn load(interface_name: String, port: i32, priv_key: PrivKey) -> ServerConf {
        let pub_key = PubKey::from(&priv_key);
        ServerConf {
            interface_name,
            priv_key,
            pub_key,
            port,
        }
    }

    pub fn interactive_new() -> ServerConf {
        println!("--------------------------------------------------");
        println!("Configure Server: ");
        let interface_name = Text::new("Insert interface name").with_default("wg0").prompt().unwrap();
        let port = CustomType::<i32>::new("Insert port number")
            .with_formatter(&|i| format!("{}", i))
            .with_error_message("please insert an integer number")
            .with_default(WG_SERVER_PORT)
            .prompt()
            .unwrap();
        println!("--------------------------------------------------");
        ServerConf::new(interface_name, port)
    }
}

pub fn load_server_conf(interface: String) -> ServerConf{
    let mut file = File::open(format!("{}.conf", interface)).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut lines = contents.lines();
    lines.next();
    let priv_key_raw = lines.next().unwrap().split(" = ").collect::<Vec<&str>>()[1];
    let port = lines.next().unwrap().split(" = ").collect::<Vec<&str>>()[1];

    let priv_key = PrivKey::from(general_purpose::STANDARD.decode(priv_key_raw.as_bytes()).unwrap());

    ServerConf::load(
        String::from(interface),
        port.parse::<i32>().unwrap(),
        priv_key
    )
} 