pub mod gen_keys;
pub mod ip_netmask;
pub mod peer_conf;
pub mod server_conf;
pub mod fs_conf;

use inquire::*;
use peer_conf::PeerConf;
use server_conf::ServerConf;

fn main() {
    let server_cfg: ServerConf;
    loop {
        let menu = Select::new("wg-cfg server config", vec!["Load Interface", "Exit"]).prompt();

        match menu {
            Ok(choice) => {
                if choice == "Exit" {
                    return;
                } else {
                    let interface_name = Text::new("Insert wireguard interface name")
                        .with_default("wg0")
                        .prompt()
                        .unwrap();

                    if std::path::Path::new(&format!("{}.conf", interface_name)).exists() {
                        println!("{}.conf exists, loading it", interface_name);
                        server_cfg = server_conf::load_server_conf(interface_name);
                        break;
                    } else {
                        println!("{}.conf does not exsist.", interface_name);
                        let create = Confirm::new("Do you want to create it?")
                            .with_default(false)
                            .prompt();

                        match create {
                            Ok(true) => {
                                server_cfg =
                                    server_conf::ServerConf::interactive_new(interface_name);
                                fs_conf::save_server_conf(&server_cfg);
                                break;
                            }

                            Ok(false) => continue,

                            Err(_) => {
                                println!("Error. Closing");
                                return;
                            }
                        }
                    }
                }
            }

            Err(_) => {
                println!("Error. Closing");
                return;
            }
        }
    }

    loop {
        let menu = Select::new(
            &format!("{}.conf Loaded - Select Option", server_cfg.interface_name),
            vec!["Add Peer", "List Peers", "Exit"],
        )
        .prompt();

        match menu {
            Ok(choice) => match choice {
                "Exit" => return,
                "Add Peer" => {
                    let peer_cfg = PeerConf::interactive_new(&server_cfg);
                    fs_conf::save_peer_conf(&peer_cfg);
                    fs_conf::update_server_conf(&server_cfg, &peer_cfg);
                }
                "List Peers" => {
                    fs_conf::list_peers(&server_cfg);
                }
                &_ => return,
            },

            Err(_) => {
                println!("Error. Closing");
                return;
            }
        }
    }
}
