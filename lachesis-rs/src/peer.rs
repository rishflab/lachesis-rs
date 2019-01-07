use crate::event::EventHash;

pub type PeerId = Vec<u8>;

pub trait Peer<H>: Send + Sync {
    fn get_sync(&self, pk: PeerId, known: Option<&H>) -> (EventHash, H);
    fn id(&self) -> &PeerId;
}
