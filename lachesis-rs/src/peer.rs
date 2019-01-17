use crate::event::event_hash::EventHash;
use crate::lachesis::opera::Opera;

pub type PeerId = Vec<u8>;

pub struct Peer(PeerId);

impl Peer {
    //    pub fn get_sync(&self, pk: PeerId, opera: &Opera) -> EventHash {
    //
    //    }
    pub fn id(&self) -> PeerId {
        self.0.clone()
    }
}
