use crate::{peer_conf::*, server_conf::*};

use base64::{engine::general_purpose, Engine as _};

use std::{fs::File, io::Write};

pub fn save_peer_conf(peer_cfg: &PeerConf) {
    let mut file = File::create(format!("{}.conf", peer_cfg.name)).unwrap();
    let peer_conf = format!("[Interface]\nPrivateKey = {}\nListenPort = {}\nAddress = {}\nDNS = {}, {}\n\n[Peer]\nPublicKey = {}\nPresharedKey = {}\nAllowedIPs = {}/0\nEndpoint = {}:{}\nPersistentKeepalive = {}\n",
        general_purpose::STANDARD.encode(&peer_cfg.priv_key),
        peer_cfg.port,
        peer_cfg.address,
        peer_cfg.dns_primary,
        peer_cfg.dns_secondary,
        general_purpose::STANDARD.encode(&peer_cfg.server_pub_key),
        general_purpose::STANDARD.encode(&peer_cfg.shared_key),
        peer_cfg.allowed_ips,
        peer_cfg.endpoint,
        peer_cfg.port,
        peer_cfg.keepalive,
    );

    file.write_all(peer_conf.as_bytes()).unwrap();
}

pub fn save_server_conf(server_cfg: &ServerConf){
    let mut file = File::create(format!("server.conf")).unwrap();
    let server_conf = format!("[Interface]\nPrivateKey = {}\nListenPort = {}\n\n",
        general_purpose::STANDARD.encode(&server_cfg.priv_key),
        server_cfg.port,
    );

    file.write_all(server_conf.as_bytes()).unwrap();
}
