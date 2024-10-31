mod types;
mod providers;

pub mod prelude {
    pub use crate::providers::*;
}

#[cfg(test)]
mod tests {
    use providers::{windows::Network, NetworkBehavior};

    use super::*;

    #[test]
    fn it_works() {
        let binding = Network::new()
            .find_around()
            .unwrap();
        let networks = binding
            .get_around()
            .unwrap();
        let conn = Network::new();
        for i in networks {
            match conn.connect(i.get_ssid(), "Vovahat123") {
                Err(e) => println!("{:#?}", e),
                Ok(_) => {}
            };
            println!("SSID: {}", i.get_ssid());
            println!("Type: {:#?}", i.get_network_type());
            println!("Encryption: {:#?}", i.get_encryption_type());
            println!("");
        }
    }
}
