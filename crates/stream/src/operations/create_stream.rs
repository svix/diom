use crate::{
    State,
    entities::StreamId,
    tables::{NameToStreamRow, StreamRow},
};
use diom_error::Result;
use fjall_utils::TableRow as _;
use jiff::Timestamp;
use std::num::NonZeroU64;
use uuid::Uuid;

pub struct CreateStream {
    stream: StreamRow,
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
        state: &State,
        name: String,
        retention_period_seconds: Option<NonZeroU64>,
        max_byte_size: Option<NonZeroU64>,
    ) -> Result<Self> {
        let now = Timestamp::now();

        let mut stream = NameToStreamRow::fetch(&state.metadata_tables, &name)?
            .and_then(|row| StreamRow::fetch(&state.metadata_tables, &row.id).transpose())
            .transpose()?
            .unwrap_or_else(|| {
                let id = Uuid::new_v4().to_string();
                StreamRow {
                    id,
                    name,
                    retention_period_seconds,
                    max_byte_size,
                    created_at: now,
                    updated_at: now,
                }
            });

        // If we're doing an update, this fields will have to change.
        stream.retention_period_seconds = retention_period_seconds;
        stream.max_byte_size = max_byte_size;
        stream.updated_at = now;

        Ok(Self { stream })
    }

    // FIXME(@svix-gabriel) - I'm trying to adhere mostly to the API mentioned in the HA rfc (https://github.com/svix/rfc/pull/30/files#diff-1f8e708b840474d3072b1c965eb090a3e30b26e8b7036c6e0ae47ef36ffb09abR54)
    // However for expediency, I don't want to wait for the relevant traits to be added in order to have something working.
    // It should be straightforward (famous last words) to translate this method to the Operations trait once it's in place.
    pub fn apply_operation(self, state: &State) -> Result<CreateStreamOutput> {
        let (k1, v1) = self.stream.to_fjall_entry()?;

        let name_entry = NameToStreamRow {
            name: self.stream.name,
            id: self.stream.id,
        };

        let (k2, v2) = name_entry.to_fjall_entry()?;

        {
            let mut batch = state.db.batch();
            batch.insert(&state.metadata_tables, k1, v1);
            batch.insert(&state.metadata_tables, k2, v2);
            batch.commit()?;
        }

        Ok(CreateStreamOutput {
            id: name_entry.id,
            name: name_entry.name,
            retention_period_seconds: self.stream.retention_period_seconds,
            max_byte_size: self.stream.max_byte_size,
            created_at: self.stream.created_at,
            updated_at: self.stream.updated_at,
        })
    }
}
