/*

Generate peer.conf file that looks like this

[Interface]
PrivateKey = redacted
ListenPort = 443
Address = 10.0.0.3/32
DNS = 10.10.10.5, 10.0.0.3

[Peer]
PublicKey = redacted
PresharedKey = redacted
AllowedIPs = 0.0.0.0/0
Endpoint = 84.33.127.241:443
PersistentKeepalive = 25

*/
static WG_SERVER_PORT: i32 = 51820;

use crate::gen_keys::*;
use crate::server_conf::*;

use inquire::*;
use rand::Rng;
use rand_core::OsRng;
use std::net::Ipv4Addr;

pub struct PeerConf {
    pub name: String,
    pub priv_key: PrivKey,
    pub pub_key: PubKey,
    pub server_pub_key: PubKey,
    pub shared_key: PrivKey,
    pub dns_primary: Ipv4Addr,
    pub dns_secondary: Ipv4Addr,
    pub allowed_ips: Ipv4Addr,
    pub address: Ipv4Addr,
    pub endpoint: Ipv4Addr,
    pub port: i32,
    pub keepalive: i32,
}

impl PeerConf {
    pub fn new(
        name: String,
        server_cfg: &ServerConf,
        dns_primary: Ipv4Addr,
        dns_secondary: Ipv4Addr,
        allowed_ips: Ipv4Addr,
        address: Ipv4Addr,
        endpoint: Ipv4Addr,
        port: i32,
        keepalive: i32,
    ) -> PeerConf {
        let priv_key = PrivKey::new(OsRng);
        let pub_key = PubKey::from(&priv_key);
        let shared_key = PrivKey::new(OsRng);
        let server_pub_key = server_cfg.pub_key.clone();
        PeerConf {
            name,
            priv_key,
            pub_key,
            server_pub_key,
            shared_key,
            dns_primary,
            dns_secondary,
            allowed_ips,
            address,
            endpoint,
            port,
            keepalive,
        }
    }

    pub fn interactive_new(server_cfg: &ServerConf) -> PeerConf {
        println!("--------------------------------------------------");
        println!("Configure Peer: ");
        let mut rng = rand::thread_rng();
        let name = Text::new("Insert peer name")
            .with_default("peer")
            .prompt()
            .unwrap();
        let endpoint = CustomType::<Ipv4Addr>::new("Insert Endpoint IpV4 Address")
            .with_formatter(&|i| format!("{}", i))
            .with_error_message("Please insert a valid IP")
            .prompt()
            .unwrap();
        let port = CustomType::<i32>::new("Insert port number")
            .with_formatter(&|i| format!("{}", i))
            .with_error_message("please insert an integer number")
            .with_default(WG_SERVER_PORT)
            .prompt()
            .unwrap();
        let allowed_ips = CustomType::<Ipv4Addr>::new("Insert Allowed IPs")
            .with_formatter(&|i| format!("{}/0", i))
            .with_error_message("Please insert a valid IP")
            .with_default(Ipv4Addr::new(0, 0, 0, 0))
            .prompt()
            .unwrap();
        let address = CustomType::<Ipv4Addr>::new("Insert Peer Address")
            .with_formatter(&|i| format!("{}/32", i))
            .with_error_message("Please insert a valid IP")
            .with_default(Ipv4Addr::new(10, 0, 0, rng.gen_range(1..128)))
            .prompt()
            .unwrap();
        let dns_primary = CustomType::<Ipv4Addr>::new("Insert primary DNS IpV4 Address")
            .with_formatter(&|i| format!("{}", i))
            .with_error_message("Please insert a valid IP")
            .with_default(Ipv4Addr::new(1, 1, 1, 1))
            .prompt()
            .unwrap();
        let dns_secondary = CustomType::<Ipv4Addr>::new("Insert secondary DNS IpV4 Address")
            .with_formatter(&|i| format!("{}", i))
            .with_error_message("Please insert a valid IP")
            .with_default(Ipv4Addr::new(8, 8, 8, 8))
            .prompt()
            .unwrap();
        let keepalive = CustomType::<i32>::new("Insert PersistentKeepalive time (seconds)")
            .with_formatter(&|i| format!("{}", i))
            .with_error_message("please insert an integer number")
            .with_default(25)
            .prompt()
            .unwrap();
        println!("--------------------------------------------------");

        PeerConf::new(
            name,
            server_cfg,
            dns_primary,
            dns_secondary,
            allowed_ips,
            address,
            endpoint,
            port,
            keepalive,
        )
    }
}
