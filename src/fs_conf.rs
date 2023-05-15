use crate::{gen_keys::PrivKey, peer_conf::*, server_conf::*};

use base64::{engine::general_purpose, Engine as _};

use std::{
    collections::HashMap,
    fs::{create_dir_all, File, OpenOptions, read_dir},
    io::{Read, Seek, Write},
};

pub fn save_peer_conf(peer_cfg: &PeerConf) {
    create_dir_all("./peers").unwrap();

    let name;
    if std::path::Path::new(&format!("./peers/{}.conf", peer_cfg.name)).exists() {
        name = format!("peer-{}", rand::random::<u32>());
    } else {
        name = peer_cfg.name.clone();
    }

    println!("Saving {}.conf", name);

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(format!("./peers/{}.conf", name))
        .unwrap();

    let peer_conf = format!("[Interface]\nPrivateKey = {}\nListenPort = {}\nAddress = {}\nDNS = {}, {}\n\n[Peer]\nPublicKey = {}\nPresharedKey = {}\nAllowedIPs = {}\nEndpoint = {}:{}\nPersistentKeepalive = {}\n",
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

    let peer_conf = format!(
        "\n[Peer]\nPublicKey = {}\nPresharedKey = {}\nAllowedIPs = {}\nPersistenKeepAlive = {}\n",
        general_purpose::STANDARD.encode(&peer_cfg.pub_key),
        general_purpose::STANDARD.encode(&peer_cfg.shared_key),
        peer_cfg.address,
        peer_cfg.keepalive,
    );

    file.seek(std::io::SeekFrom::End(0)).unwrap();
    file.write_all(peer_conf.as_bytes()).unwrap();
}

pub fn list_peers(server_cfg: &ServerConf) -> HashMap<i32, PrivKey> {
    println!("Peers list for {}.conf\n", server_cfg.interface_name);
    let mut file = File::open(format!("{}.conf", server_cfg.interface_name)).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut lines = contents.lines();
    let mut peers: HashMap<i32, PrivKey> = HashMap::new();
    let mut peer_num = 0;
    let mut shared_key = String::new();
    while let Some(line) = lines.next() {
        if line == "[Peer]" {
            let mut peer = String::new();
            peer.push_str(line);
            peer.push('\n');
            while let Some(line) = lines.next() {
                if line == "" {
                    break;
                }
                if line.starts_with("PresharedKey = ") {
                    shared_key = line[15..].to_string();
                }
                peer.push_str(line);
                peer.push('\n');
            }
            println!("{}", peer);
            peers.insert(
                peer_num,
                PrivKey::from(
                    general_purpose::STANDARD
                        .decode(&shared_key.as_bytes())
                        .unwrap(),
                ),
            );
            peer_num += 1;
        }
    }
    peers
}

pub fn delete_peer_conf(server_cfg: &ServerConf, peer_shared_key: String) {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(format!("{}.conf", server_cfg.interface_name))
        .unwrap();

    let mut contents = String::new();
    let mut new_contents = String::new();

    file.read_to_string(&mut contents).unwrap();
    let mut lines = contents.lines();

    // put first 4 lines in new_contents
    for _ in 0..4 {
        if let Some(line) = lines.next() {
            new_contents.push_str(line);
            new_contents.push('\n');
        }
    }

    while let Some(line) = lines.next() {
        let mut delete = false;
        if line == "[Peer]" {
            let mut peer = String::new();
            peer.push_str(line);
            peer.push('\n');
            while let Some(line) = lines.next() {
                if line == "" || line == "[Peer]" {
                    break;
                }
                if line.starts_with("PresharedKey = ") {
                    if peer_shared_key == line[15..].to_string() {
                        delete = true;
                    };
                }
                peer.push_str(line);
                peer.push('\n');
                
            }
            if !delete {
                peer.push('\n');
                new_contents.push_str(&peer);
            }
        }
    }

    new_contents.pop();

    file.set_len(0).unwrap();

    file.seek(std::io::SeekFrom::Start(0)).unwrap();
    file.write_all(new_contents.as_bytes()).unwrap();

}

pub fn list_peers_files() -> HashMap<i32, String>{
    let mut files: HashMap<i32, String> = HashMap::new();
    let mut file_num = 0;
    for entry in read_dir("./peers").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
            if file_name.ends_with(".conf") {
                println!("{}: ./peers/{}", file_num, file_name);
                files.insert(file_num, file_name);
                file_num += 1;
            }
        }
    }
    files
}