use super::raft::{NodeId, Raft};
use coyote_kv::operations::KvRequest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Request {
    Kv(coyote_kv::operations::Operation),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Response {
    Blank,
    Kv(coyote_kv::operations::Response),
}

#[derive(Clone)]
pub struct RaftState {
    pub raft: Raft,
    pub node_id: NodeId,
}

impl RaftState {
    pub async fn client_write_kv<O: KvRequest>(&self, op: O) -> O::Response {
        let request = Request::Kv(op.into());
        // TODO: don't unwrap here!
        let response = self.raft.client_write(request).await.unwrap();
        let Response::Kv(response) = response.data else {
            panic!("got back incorrect response");
        };
        let Ok(resp) = O::Response::try_from(response) else {
            panic!("got back incorrect response");
        };
        resp
    }
}
