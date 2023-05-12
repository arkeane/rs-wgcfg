mod gen_keys;

use base64::{engine::general_purpose, Engine as _};
use rand_core::OsRng;
fn main() {
    let priv_key = gen_keys::PrivKey::new(OsRng);
    let shared_key = gen_keys::PrivKey::new(OsRng);
    let pub_key = gen_keys::PubKey::from(&priv_key);

    println!("PrivKey: {}", general_purpose::STANDARD.encode(&priv_key));
    println!("SharedKey: {}", general_purpose::STANDARD.encode(&shared_key));
    println!("PubKey: {}", general_purpose::STANDARD.encode(&pub_key));

}
