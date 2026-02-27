use std::{collections::HashMap, ops::RangeInclusive};

use diom_error::{Error, Result};
use diom_namespace::entities::NamespaceId;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::{
    State,
    entities::{Offset, PartitionIndex},
};

#[derive(Serialize, Deserialize)]
pub(crate) struct MsgRow {
    pub value: Vec<u8>,
    pub headers: HashMap<String, String>,
    pub created_at: Timestamp,
}

const MSG_KEY_LEN: usize = size_of::<NamespaceId>() + size_of::<u16>() + size_of::<Offset>();

type MsgRowKey = [u8; MSG_KEY_LEN];

pub(crate) fn msg_row_key(
    ns_id: NamespaceId,
    partition: PartitionIndex,
    offset: Offset,
) -> MsgRowKey {
    let mut key = [0u8; MSG_KEY_LEN];
    let ns_bytes = ns_id.as_u128().to_be_bytes();
    key[..16].copy_from_slice(&ns_bytes);
    key[16..18].copy_from_slice(&partition.get().to_be_bytes());
    key[18..26].copy_from_slice(&offset.to_be_bytes());
    key
}

fn parse_msg_offset(key: MsgRowKey) -> Result<Offset> {
    let bytes: [u8; 8] = key[18..26].try_into().map_err(Error::generic)?;
    Ok(Offset::from_be_bytes(bytes))
}

pub(crate) fn msg_row_key_range(
    ns_id: NamespaceId,
    partition: PartitionIndex,
    offsets: RangeInclusive<Offset>,
) -> RangeInclusive<MsgRowKey> {
    msg_row_key(ns_id, partition, *offsets.start())..=msg_row_key(ns_id, partition, *offsets.end())
}

impl MsgRow {
    pub(crate) fn max_offset(
        state: &State,
        ns_id: NamespaceId,
        partition: PartitionIndex,
    ) -> Result<Option<Offset>> {
        let range = msg_row_key_range(ns_id, partition, Offset::MIN..=Offset::MAX);

        let Some(max_entry) = state.msg_table.range(range).next_back() else {
            return Ok(None);
        };

        let key: MsgRowKey = max_entry
            .key()?
            .as_ref()
            .try_into()
            .map_err(Error::generic)?;

        let offset = parse_msg_offset(key)?;
        Ok(Some(offset))
    }

    pub(crate) fn next_offset(
        state: &State,
        ns_id: NamespaceId,
        partition: PartitionIndex,
    ) -> Result<Offset> {
        match Self::max_offset(state, ns_id, partition)? {
            None => Ok(Offset::MIN),
            Some(id) => Ok(id + 1),
        }
    }

    pub(crate) fn to_fjall_value(&self) -> Result<fjall::UserValue> {
        rmp_serde::to_vec(&self)
            .map(fjall::UserValue::from)
            .map_err(Error::generic)
    }
}
