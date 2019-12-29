#[derive(Clone)]
pub struct Peers {
    pub local: String,
    pub remote: String,
}

impl Peers {
    pub fn new(local: String, remote: String) -> Self {
        Peers { local, remote }
    }
}
