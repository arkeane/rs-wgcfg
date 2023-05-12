pub mod gen_keys;
pub mod peer_conf;
pub mod server_conf;

fn main() {
    let server_cfg = server_conf::ServerConf::interactive_new();
    let _peer_cfg = peer_conf::PeerConf::interactive_new(server_cfg);
}
