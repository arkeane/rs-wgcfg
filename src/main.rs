pub mod fs_conf;
pub mod gen_keys;
pub mod ip_netmask;
pub mod peer_conf;
pub mod server_conf;

use base64::{engine::general_purpose, Engine as _};
use gen_keys::PrivKey;
use inquire::*;
use peer_conf::PeerConf;
use qrcode::{QrCode, render::unicode};
use server_conf::ServerConf;
use std::{collections::HashMap, fs, process::Command};

pub fn clear_terminal_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/c", "cls"])
            .spawn()
            .expect("cls command failed to start")
            .wait()
            .expect("failed to wait");
    } else {
        Command::new("clear")
            .spawn()
            .expect("clear command failed to start")
            .wait()
            .expect("failed to wait");
    };
}

fn main() {
    let server_cfg: ServerConf;
    loop {
        clear_terminal_screen();
        let menu = Select::new(
            "wg-cfg server config",
            vec![
                "Load Interface",
                "Create Interface",
                "Delete Interface",
                "Exit",
            ],
        )
        .prompt();

        match menu {
            Ok(choice) => match choice {
                "Exit" => return,
                "Create Interface" => {
                    let interface_name = Text::new("Insert wireguard interface name")
                        .with_default("wg0")
                        .prompt()
                        .unwrap();

                    if std::path::Path::new(&format!("{}.conf", interface_name)).exists() {
                        println!("{}.conf already exsists.", interface_name);
                        let overwrite = Confirm::new("Do you want to overwrite it?")
                            .with_default(false)
                            .prompt();

                        match overwrite {
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
                    } else {
                        server_cfg = server_conf::ServerConf::interactive_new(interface_name);
                        fs_conf::save_server_conf(&server_cfg);
                        break;
                    }
                }
                "Load Interface" => {
                    let interface_name = Text::new("Insert wireguard interface name")
                        .with_default("wg0")
                        .prompt()
                        .unwrap();

                    if std::path::Path::new(&format!("{}.conf", interface_name)).exists() {
                        println!("Loading {}.conf", interface_name);
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
                "Delete Interface" => {
                    let interface_name = Text::new("Insert wireguard interface name")
                        .with_help_message("This will delete the interface config file")
                        .prompt()
                        .unwrap();

                    if std::path::Path::new(&format!("{}.conf", interface_name)).exists() {
                        println!("Deleting {}.conf", interface_name);
                        let sure = Confirm::new("Are you sure?")
                            .with_default(false)
                            .prompt()
                            .unwrap();

                        if sure {
                            fs::remove_file(format!("{}.conf", interface_name)).unwrap();
                            let _ans = Text::new("Press Enter to go back to menu").prompt();
                            continue;
                        }

                        continue;
                    } else {
                        println!("{}.conf does not exsist.", interface_name);
                        let _ans = Text::new("Press Enter to go back to menu").prompt();
                        continue;
                    }
                }
                &_ => return,
            },

            Err(_) => {
                println!("Error. Closing");
                return;
            }
        }
    }

    let mut peers: HashMap<i32, PrivKey>;
    let mut peers_files: HashMap<i32, String>;
    loop {
        clear_terminal_screen();
        let menu = Select::new(
            &format!("{}.conf Loaded - Select Option", server_cfg.interface_name),
            vec!["Add Peer", "List Peers", "QrEncode Peer", "Delete Peer", "Exit"],
        )
        .prompt();

        match menu {
            Ok(choice) => match choice {
                "Exit" => return,
                "Add Peer" => {
                    let peer_cfg = PeerConf::interactive_new(&server_cfg);
                    fs_conf::save_peer_conf(&peer_cfg);
                    fs_conf::update_server_conf(&server_cfg, &peer_cfg);
                    let _ans = Text::new("Press Enter to go back to menu").prompt();
                }
                "Delete Peer" => {
                    peers = fs_conf::list_peers(&server_cfg);
                    let mut options = peers.iter().map(|(k, _)| k).collect::<Vec<&i32>>();

                    options.sort();

                    if options.len() == 0 {
                        println!("No peers to delete");
                        let _ans = Text::new("Press Enter to go back to menu").prompt();
                        continue;
                    }

                    let peer_id = Select::new("Select Peer to delete", options)
                        .prompt()
                        .unwrap();

                    let peer_to_delete =
                        general_purpose::STANDARD.encode(peers.get(peer_id).unwrap());

                    println!("Deleting peer {}", &peer_to_delete);

                    fs_conf::delete_peer_conf(&server_cfg, peer_to_delete);

                    let _ans = Text::new("Press Enter to go back to menu").prompt();
                }
                "List Peers" => {
                    fs_conf::list_peers(&server_cfg);
                    let _ans = Text::new("Press Enter to go back to menu").prompt();
                }

                "QrEncode Peer" => {
                    peers_files = fs_conf::list_peers_files();
                    let mut options = peers_files.iter().map(|(k, _)| k).collect::<Vec<&i32>>();

                    options.sort();

                    if options.len() == 0 {
                        println!("No peers to encode");
                        let _ans = Text::new("Press Enter to go back to menu").prompt();
                        continue;
                    }

                    let peer_id = Select::new("Select Peer to encode", options)
                        .prompt()
                        .unwrap();

                    let peer_to_encode = peers_files.get(peer_id).unwrap();

                    let data = fs::read_to_string(format!("./peers/{}", peer_to_encode)).unwrap();

                    
                    // print qr containing data 
                    let code = QrCode::new(data.as_bytes()).unwrap();
                    let image = code.render::<unicode::Dense1x2>()
                        .dark_color(unicode::Dense1x2::Light)
                        .light_color(unicode::Dense1x2::Dark)
                        .build();
                    println!("{}", image);

                    let _ans = Text::new("Press Enter to go back to menu").prompt();

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
