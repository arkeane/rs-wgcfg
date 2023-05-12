use crate::{peer_conf::*, server_conf::*};

use base64::{engine::general_purpose, Engine as _};

use std::{
    fs::{create_dir_all, File, OpenOptions},
    io::{Seek, Write},
};

pub fn save_peer_conf(peer_cfg: &PeerConf) {
    println!("Saving {}.conf", peer_cfg.name);
    create_dir_all("./peers").unwrap();

    let name;
    if std::path::Path::new(&format!("./peers/{}.conf", peer_cfg.name)).exists() {
        name = format!("peer-{}", rand::random::<u32>());
    } else {
        name = peer_cfg.name.clone();
    }

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(format!("./peers/{}.conf", name))
        .unwrap();

    let peer_conf = format!("[Interface]\nPrivateKey = {}\nListenPort = {}\nAddress = {}/32\nDNS = {}, {}\n\n[Peer]\nPublicKey = {}\nPresharedKey = {}\nAllowedIPs = {}/0\nEndpoint = {}:{}\nPersistentKeepalive = {}\n",
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

pub fn save_server_conf(server_cfg: &ServerConf) {
    println!("Saving {}.conf", server_cfg.interface_name);
    let mut file = File::create(format!("{}.conf", server_cfg.interface_name)).unwrap();
    let server_conf = format!(
        "[Interface]\nPrivateKey = {}\nListenPort = {}\n",
        general_purpose::STANDARD.encode(&server_cfg.priv_key),
        server_cfg.port,
    );

    file.write_all(server_conf.as_bytes()).unwrap();
}

pub fn update_server_conf(server_cfg: &ServerConf, peer_cfg: &PeerConf) {
    println!("Updating {}.conf", server_cfg.interface_name);
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(format!("{}.conf", server_cfg.interface_name))
        .unwrap();

    let peer_conf = format!("\n[Peer]\nPublicKey = {}\nPresharedKey = {}\nAllowedIPs = {}/32\nPersistenKeepAlive = {}\n", 
        general_purpose::STANDARD.encode(&peer_cfg.pub_key),
        general_purpose::STANDARD.encode(&peer_cfg.shared_key),
        peer_cfg.address,
        peer_cfg.keepalive,
    );

    file.seek(std::io::SeekFrom::End(0)).unwrap();
    file.write_all(peer_conf.as_bytes()).unwrap();
}
