extern crate openssl;
extern crate rust-base58;
extern crate protobuf;

mod crypto;

use openssl::crypto::pkey::PKey;

struct IPFSNode {
    id: Vec<u8>,
    keys: PKey,
}

fn main() {
    let mut node = IPFSNode{keys: PKey::new()};
    print!("Generating keys for a new node... ");
    node.keys.gen(4096);
    println!("Done!");



    node.keys.write_pem(&mut std::io::stdout());
}
