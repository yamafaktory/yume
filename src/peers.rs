use crossterm::{execute, style};
use std::io::{stdout, Write};

#[derive(Clone, Debug, PartialEq)]
pub struct Peers {
    pub local: String,
    pub remote: String,
}

impl Peers {
    pub fn new(local: String, remote: String) -> Self {
        Peers { local, remote }
    }

    pub fn display_remote(&self) {
        execute!(
            stdout(),
            style::SetForegroundColor(style::Color::DarkMagenta),
            style::Print(format!("{} ", self.local.clone())),
            style::SetForegroundColor(style::Color::White)
        )
        .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_peers() {
        let local_peer = String::from("2001:3984:3989::10");
        let remote_peer = String::from("2001:3984:3989::20");
        let local_peer_clone = local_peer.clone();
        let remote_peer_clone = remote_peer.clone();
        let peers = Peers::new(local_peer, remote_peer);

        assert_eq!(
            peers,
            Peers {
                local: local_peer_clone,
                remote: remote_peer_clone
            }
        );

        assert_eq!(peers.display_remote(), ());
    }
}
