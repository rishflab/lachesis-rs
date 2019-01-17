use crate::errors::ParentsError;
use crate::event::event_hash::EventHash;
use failure::Error;

#[derive(Clone, Eq, PartialEq, Deserialize, Serialize, Debug)]
pub struct ParentsList(pub Vec<EventHash>);

impl ParentsList {
    pub fn self_parent(&self) -> Result<EventHash, Error> {
        Ok(self.0.first().ok_or(ParentsError::EmptyParents)?.clone())
    }
}
