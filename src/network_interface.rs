pub struct NetworkInterface {
    pub name: String,
}

impl NetworkInterface {
    pub fn new(name: &str) -> Self {
        NetworkInterface {
            name: String::from(name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_interface_new() {
        let name = "en0";
        let interface = NetworkInterface::new(name);

        // Assert the name property
        assert_eq!(interface.name, name);
    }
}
