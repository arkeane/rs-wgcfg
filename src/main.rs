pub mod gen_keys;
pub mod peer_conf;
pub mod server_conf;
pub mod write_conf;

fn main() {
    let server_cfg = server_conf::ServerConf::interactive_new();
    println!("Saving server.conf");
    write_conf::save_server_conf(&server_cfg);

    let peer_cfg = peer_conf::PeerConf::interactive_new(server_cfg);
    println!("Saving {}", peer_cfg.name);
    write_conf::save_peer_conf(&peer_cfg);
}
