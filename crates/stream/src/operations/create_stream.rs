use crate::{
    State,
    entities::StreamId,
    tables::{NameToStreamRow, StreamRow},
};
use coyote_error::{Error, Result};
use fjall_utils::TableRow as _;
use jiff::Timestamp;
use std::num::NonZeroU64;
use uuid::Uuid;

pub struct CreateStream {
    timestamp: Timestamp,
    name: String,
    retention_period_seconds: Option<NonZeroU64>,
    max_byte_size: Option<NonZeroU64>,
}

pub struct CreateStreamOutput {
    pub id: StreamId,
    pub name: String,
    pub retention_period_seconds: Option<NonZeroU64>,
    pub max_byte_size: Option<NonZeroU64>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl CreateStream {
    pub fn new(
        name: String,
        retention_period_seconds: Option<NonZeroU64>,
        max_byte_size: Option<NonZeroU64>,
    ) -> Self {
        Self {
            timestamp: Timestamp::now(),
            name,
            retention_period_seconds,
            max_byte_size,
        }
    }

    // FIXME(@svix-gabriel) - I'm trying to adhere mostly to the API mentioned in the HA rfc (https://github.com/svix/rfc/pull/30/files#diff-1f8e708b840474d3072b1c965eb090a3e30b26e8b7036c6e0ae47ef36ffb09abR54)
    // However for expediency, I don't want to wait for the relevant traits to be added in order to have something working.
    // It should be straightforward (famous last words) to translate this method to the Operations trait once it's in place.
    pub fn apply_operation(self, state: &State) -> Result<CreateStreamOutput> {
        let mut stream = NameToStreamRow::fetch(state.metadata_tables.as_ref(), &self.name)?
            .and_then(|row| StreamRow::fetch(state.metadata_tables.as_ref(), &row.id).transpose())
            .transpose()?
            .unwrap_or_else(|| {
                let id = Uuid::new_v4();
                StreamRow {
                    id: id.to_string(),
                    name: self.name,
                    retention_period_seconds: self.retention_period_seconds,
                    max_byte_size: self.max_byte_size,
                    created_at: self.timestamp,
                    updated_at: self.timestamp,
                }
            });

        stream.retention_period_seconds = self.retention_period_seconds;
        stream.max_byte_size = self.max_byte_size;
        stream.updated_at = self.timestamp;

        {
            let mut tx = state.metadata_tables.write_tx()?;
            let (k1, v1) = stream.to_fjall_entry()?;
            let (k2, v2) = NameToStreamRow {
                name: stream.name.clone(),
                id: stream.id.clone(),
            }
            .to_fjall_entry()?;

            tx.insert(k1, v1);
            tx.insert(k2, v2);

            // FIXME(@svix-gabriel) - it's not clear to me what would actually cause a transaction to fail with a conflict.
            // Maybe someone knows? I'm not sure what the best way to handle this would be. For now, just propogate the error
            // since it means we failed to create the stream.
            tx.commit()?.map_err(Error::generic)?;
        }

        let StreamRow {
            id,
            name,
            retention_period_seconds,
            max_byte_size,
            created_at,
            updated_at,
        } = stream;

        Ok(CreateStreamOutput {
            id,
            name,
            retention_period_seconds,
            max_byte_size,
            created_at,
            updated_at,
        })
    }
}
