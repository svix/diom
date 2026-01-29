use crate::{
    State,
    entities::{MsgId, MsgIn, StreamId},
    tables::{MsgRow, msg_row_key},
};
use diom_error::Result;
use jiff::Timestamp;

pub struct AppendToStream {
    stream_id: StreamId,
    msgs: Vec<(MsgId, MsgRow)>,
}

pub struct AppendToStreamOutput {
    pub msg_ids: Vec<MsgId>,
}

impl AppendToStream {
    pub fn new(state: &State, stream_id: StreamId, msgs: Vec<MsgIn>) -> Result<Self> {
        let offset = MsgRow::get_next_msg_id_in_stream(state, stream_id)?;
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

        Ok(Self { stream_id, msgs })
    }

    // FIXME(@svix-gabriel) - I'm trying to adhere mostly to the API mentioned in the HA rfc (https://github.com/svix/rfc/pull/30/files#diff-1f8e708b840474d3072b1c965eb090a3e30b26e8b7036c6e0ae47ef36ffb09abR54)
    // However for expediency, I don't want to wait for the relevant traits to be added in order to have something working.
    // It should be straightforward (famous last words) to translate this method to the Operations trait once it's in place.
    pub fn apply_operation(self, state: &State) -> Result<AppendToStreamOutput> {
        let mut batch = state.db.batch();

        let msg_ids = self.msgs.iter().map(|(id, _msg)| *id).collect();

        for (id, msg) in self.msgs {
            let key = msg_row_key(self.stream_id, id);
            let val = msg.to_fjall_value()?;
            batch.insert(&state.msg_table, key, val);
        }

        batch.commit()?;

        Ok(AppendToStreamOutput { msg_ids })
    }
}
