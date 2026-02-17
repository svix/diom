use super::{AppendResponse, StreamRaftState, StreamRequest};
use crate::{
    State,
    entities::{MsgId, MsgIn},
    tables::{MsgRow, msg_row_key},
};
use coyote_configgroup::entities::ConfigGroupId;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

pub struct AppendToStream {
    group_id: ConfigGroupId,
    msgs: Vec<(MsgId, MsgRow)>,
}

pub struct AppendToStreamOutput {
    pub msg_ids: Vec<MsgId>,
}

impl AppendToStream {
    pub fn new(
        state: &State,
        group_id: ConfigGroupId,
        msgs: Vec<MsgIn>,
    ) -> coyote_error::Result<Self> {
        let offset = MsgRow::get_next_msg_id_in_stream(state, group_id)?;
        let created_at = Timestamp::now();

        let msgs: Vec<_> = msgs
            .into_iter()
            .enumerate()
            .map(|(i, msg)| {
                let i =
                    MsgId::try_from(i).expect("usize should trivially be convertible to a msg-id");
                let msg_id = offset + i;
                (msg_id, msg)
            })
            .map(|(id, msg)| {
                let msg = MsgRow {
                    payload: msg.payload,
                    headers: msg.headers,
                    created_at,
                };

                (id, msg)
            })
            .collect();

        Ok(Self { group_id, msgs })
    }

    pub fn apply_operation(self, state: &State) -> coyote_error::Result<AppendToStreamOutput> {
        let mut batch = state.db.batch();

        let msg_ids = self.msgs.iter().map(|(id, _msg)| *id).collect();

        for (id, msg) in self.msgs {
            let key = msg_row_key(self.group_id, id);
            let val = msg.to_fjall_value()?;
            batch.insert(&state.msg_table, key, val);
        }

        batch.commit()?;

        Ok(AppendToStreamOutput { msg_ids })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppendOperation {
    pub(crate) group_id: ConfigGroupId,
    pub(crate) msgs: Vec<MsgIn>,
}

impl AppendOperation {
    pub fn new(group_id: ConfigGroupId, msgs: Vec<MsgIn>) -> Self {
        Self { group_id, msgs }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppendResponseData {
    pub msg_ids: Vec<MsgId>,
}

impl AppendOperation {
    fn apply_real(self, state: &State) -> coyote_operations::Result<AppendResponseData> {
        let op = AppendToStream::new(state, self.group_id, self.msgs)?;
        let out = op.apply_operation(state)?;
        Ok(AppendResponseData {
            msg_ids: out.msg_ids,
        })
    }
}

impl StreamRequest for AppendOperation {
    fn apply(self, state: StreamRaftState<'_>) -> AppendResponse {
        AppendResponse(self.apply_real(state.stream))
    }
}
