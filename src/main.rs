pub mod gen_keys;
pub mod peer_conf;
pub mod server_conf;
pub mod write_conf;
pub mod ip_netmask;

use inquire::*;
use write_conf::update_server_conf;

fn main() {
    let server_cfg;

    let interface_name = Text::new("Insert interface name")
        .with_default("wg0")
        .prompt()
        .unwrap();

    if std::path::Path::new(&format!("{}.conf", interface_name)).exists() {
        println!("{}.conf exists, loading it", interface_name);
        server_cfg = server_conf::load_server_conf(interface_name);
    } else {
        println!("{}.conf does not exist, creating it", interface_name);
        server_cfg = server_conf::ServerConf::interactive_new();
        write_conf::save_server_conf(&server_cfg);
    }

    let peer_cfg = peer_conf::PeerConf::interactive_new(&server_cfg);
    write_conf::save_peer_conf(&peer_cfg);
    update_server_conf(&server_cfg, &peer_cfg);
}
