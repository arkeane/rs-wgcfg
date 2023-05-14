pub mod gen_keys;
pub mod ip_netmask;
pub mod peer_conf;
pub mod server_conf;
pub mod write_conf;

use inquire::*;
use write_conf::update_server_conf;

fn main() {
    let server_cfg;

    let interface_name = Text::new("Insert wireguard interface name")
        .with_default("wg0")
        .prompt()
        .unwrap();

    if std::path::Path::new(&format!("{}.conf", interface_name)).exists() {
        println!("{}.conf exists, loading it", interface_name);
        server_cfg = server_conf::load_server_conf(interface_name);
    } else {
        println!("{}.conf does not exsist.", interface_name);
        let create = Confirm::new("Do you want to create it?")
            .with_default(false)
            .prompt();

        match create {
            Ok(true) => {
                server_cfg = server_conf::ServerConf::interactive_new(interface_name);
                write_conf::save_server_conf(&server_cfg)
            }

            Ok(false) => {
                println!("{}.conf will not be created. Closing", interface_name);
                return;
            }

            Err(_) => {
                println!("Error. Closing");
                return;
            }
        }
    }

    loop {
        let create = Confirm::new("Do you want to add a new peer?")
            .with_default(false)
            .prompt();

        match create {
            Ok(true) => {
                let peer_cfg = peer_conf::PeerConf::interactive_new(&server_cfg);
                write_conf::save_peer_conf(&peer_cfg);
                update_server_conf(&server_cfg, &peer_cfg);
            }

            Ok(false) => {
                println!("All peers created. Closing");
                return;
            }

            Err(_) => {
                println!("Error. Closing");
                return;
            }
        }
    }
}
