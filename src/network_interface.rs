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
