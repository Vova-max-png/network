mod types;
mod providers;

#[cfg(test)]
mod tests {
    use providers::{windows::Network, NetworkBehavior};

    use super::*;

    #[test]
    fn it_works() {
        let binding = Network::new().get_around().unwrap();
        let networks = binding.print_around().unwrap();
        for i in networks {
            println!("SSID: {}", i.get_ssid());
            println!("Type: {:#?}", i.get_network_type());
            println!("Encryption: {:#?}", i.get_encryption_type());
            println!("");
        }
    }
}
