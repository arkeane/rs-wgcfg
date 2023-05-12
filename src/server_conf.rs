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

pub struct ServerConf {
    pub priv_key: PrivKey,
    pub pub_key: PubKey,
    pub port: i32,
}

impl ServerConf {
    pub fn new(port: i32) -> ServerConf {
        let priv_key = PrivKey::new(OsRng);
        let pub_key = PubKey::from(&priv_key);
        ServerConf {
            priv_key,
            pub_key,
            port,
        }
    }

    pub fn interactive_new() -> ServerConf {
        println!("--------------------------------------------------");
        println!("Configure Server: ");

        let port = CustomType::<i32>::new("Insert port number")
            .with_formatter(&|i| format!("{}", i))
            .with_error_message("please insert an integer number")
            .with_default(WG_SERVER_PORT)
            .prompt()
            .unwrap();
        println!("--------------------------------------------------");
        ServerConf::new(port)
    }
}
