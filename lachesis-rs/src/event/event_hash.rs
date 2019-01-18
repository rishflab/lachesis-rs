#[derive(Clone, Debug, Deserialize, Ord, Eq, Hash, PartialEq, PartialOrd, Serialize)]
pub struct EventHash(pub Vec<u8>);

impl AsRef<[u8]> for EventHash {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}
