// Openraft 0.10 removes the ability to do streaming snapshots; the openraft_legacy
// helper crate implements a wrapper for it, but we don't want to use that wrapper because it also
// wraps all of the other Network APIs. This code is copy-pasted from openraft_legacy and is
// licensed under the openraft license (dual MIT/Apache-2.0)

use futures_util::FutureExt;
use openraft::{
    ErrorSubject, ErrorVerb, OptionalSend, Raft, RaftTypeConfig, SnapshotId, SnapshotSegmentId,
    StorageError, ToStorageResult,
    async_runtime::{Mutex, WatchReceiver},
    errors::{
        ErrorSource as _, InstallSnapshotError, RPCError, RaftError, ReplicationClosed,
        SnapshotMismatch, StreamingError,
    },
    network::RPCOption,
    raft::{InstallSnapshotRequest, InstallSnapshotResponse, SnapshotResponse},
    type_config::{
        TypeConfigExt,
        alias::{MutexOf, SnapshotOf, VoteOf},
    },
};
use std::{io::SeekFrom, sync::Arc, time::Duration};
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};

use super::{TypeConfig, network::NetworkClient};

type SnapshotData = <TypeConfig as RaftTypeConfig>::SnapshotData;
type ErrorSource = <TypeConfig as RaftTypeConfig>::ErrorSource;

pub(super) struct Streaming {
    /// The offset of the last byte written to the snapshot.
    offset: u64,

    /// The ID of the snapshot being written.
    snapshot_id: SnapshotId,

    /// A handle to the snapshot writer.
    snapshot_data: SnapshotData,
}

impl Streaming {
    pub(super) fn new(snapshot_id: SnapshotId, snapshot_data: SnapshotData) -> Self {
        Self {
            offset: 0,
            snapshot_id,
            snapshot_data,
        }
    }

    /// Get the snapshot ID for this streaming snapshot.
    pub(super) fn snapshot_id(&self) -> &SnapshotId {
        &self.snapshot_id
    }

    /// Consumes the `Streaming` and returns the snapshot data.
    pub(super) fn into_snapshot_data(self) -> SnapshotData {
        self.snapshot_data
    }

    /// Receive a single chunk of snapshot data.
    ///
    /// Writes the chunk data to the snapshot at the specified offset.
    /// Returns `true` if this was the final chunk.
    pub(super) async fn receive_chunk(
        &mut self,
        req: &InstallSnapshotRequest<TypeConfig>,
    ) -> Result<bool, StorageError<TypeConfig>> {
        // Seek to the target offset if not an exact match.
        if req.offset != self.offset {
            if let Err(err) = self.snapshot_data.seek(SeekFrom::Start(req.offset)).await {
                return Err(StorageError::from_io_error(
                    ErrorSubject::Snapshot(Some(req.meta.signature())),
                    ErrorVerb::Seek,
                    err,
                ));
            }
            self.offset = req.offset;
        }

        // Write the chunk data.
        if let Err(err) = self.snapshot_data.write_all(&req.data).await {
            return Err(StorageError::from_io_error(
                ErrorSubject::Snapshot(Some(req.meta.signature())),
                ErrorVerb::Write,
                err,
            ));
        }
        self.offset += req.data.len() as u64;

        Ok(req.done)
    }
}

/// Shared state for receiving snapshot chunks, stored via [`Raft::extension()`].
///
/// This wrapper holds the ongoing snapshot reception state and is stored
/// via [`Raft::extension()`] to track chunk-based snapshot transfers.
///
/// [`Raft::extension()`]: openraft::Raft::extension
#[derive(Clone)]
pub(super) struct StreamingState {
    pub streaming: Arc<MutexOf<TypeConfig, Option<Streaming>>>,
}

impl StreamingState {
    /// Create a new empty streaming state.
    pub(super) fn new() -> Self {
        Self {
            streaming: Arc::new(TypeConfig::mutex(None)),
        }
    }
}

impl Default for StreamingState {
    fn default() -> Self {
        Self::new()
    }
}

pub(super) trait ChunkedSnapshotReceiver {
    /// Receive a snapshot chunk and assemble it into a complete snapshot.
    ///
    /// This method should be called from your RPC handler when receiving an
    /// `InstallSnapshotRequest`. It handles:
    ///
    /// 1. Getting or creating the streaming state via `Raft::extension()`
    /// 2. Receiving chunks via `Streaming::receive_chunk()`
    /// 3. When all chunks are received, calling `Raft::install_full_snapshot()`
    ///
    /// # Returns
    ///
    /// - `Ok(response)` with the current vote on success
    /// - `Err(RaftError::APIError(InstallSnapshotError::SnapshotMismatch(...)))` if chunks arrive
    ///   out of order
    /// - `Err(RaftError::Fatal(...))` on fatal errors
    fn install_snapshot(
        &self,
        req: InstallSnapshotRequest<TypeConfig>,
    ) -> impl Future<
        Output = Result<
            InstallSnapshotResponse<TypeConfig>,
            RaftError<TypeConfig, InstallSnapshotError>,
        >,
    >;
}

impl<SM> ChunkedSnapshotReceiver for Raft<TypeConfig, SM> {
    #[tracing::instrument(skip_all, fields(
        snapshot_id = %req.meta.snapshot_id,
        offset = req.offset,
        done = req.done
    ))]
    async fn install_snapshot(
        &self,
        req: InstallSnapshotRequest<TypeConfig>,
    ) -> Result<InstallSnapshotResponse<TypeConfig>, RaftError<TypeConfig, InstallSnapshotError>>
    {
        let vote = req.vote;
        let snapshot_id = &req.meta.snapshot_id;
        let snapshot_meta = req.meta.clone();
        let done = req.done;

        // Get or create streaming state via extension()
        let state: StreamingState = self.extension();
        let mut streaming = state.streaming.lock().await;

        // Check if this is a new snapshot or continuation
        let curr_id = streaming.as_ref().map(|s| s.snapshot_id());

        if curr_id != Some(snapshot_id) {
            // New snapshot - must start at offset 0
            if req.offset != 0 {
                let mismatch = InstallSnapshotError::SnapshotMismatch(SnapshotMismatch {
                    expect: SnapshotSegmentId {
                        id: snapshot_id.clone(),
                        offset: 0,
                    },
                    got: SnapshotSegmentId {
                        id: snapshot_id.clone(),
                        offset: req.offset,
                    },
                });
                return Err(RaftError::APIError(mismatch));
            }

            // Initialize new streaming state
            let snapshot_data = self
                .begin_receiving_snapshot()
                .await
                .map_err(|e| RaftError::Fatal(e.unwrap_fatal()))?;

            *streaming = Some(Streaming::new(snapshot_id.clone(), snapshot_data));
        }

        let chunk_size = req.data.len();

        // Write the chunk
        streaming.as_mut().unwrap().receive_chunk(&req).await?;

        tracing::debug!(?snapshot_id, chunk_size, "Received snapshot chunk");

        // If done, finalize the snapshot
        if done {
            let streaming = streaming.take().unwrap();
            let mut data = streaming.into_snapshot_data();

            data.shutdown().await.map_err(|e| {
                RaftError::Fatal(openraft::errors::Fatal::from(StorageError::write_snapshot(
                    Some(snapshot_meta.signature()),
                    ErrorSource::from_error(&e),
                )))
            })?;

            tracing::debug!(
                snapshot_meta = debug(&snapshot_meta),
                "Finished streaming snapshot"
            );

            let snapshot = SnapshotOf::<TypeConfig> {
                meta: snapshot_meta,
                snapshot: data,
            };

            self.install_full_snapshot(vote, snapshot)
                .await
                .map_err(RaftError::Fatal)?;
        }

        // Return response with current vote from metrics
        let my_vote = self.metrics().borrow_watched().vote;

        Ok(InstallSnapshotResponse { vote: my_vote })
    }
}

/// Sends snapshots in chunks via `RaftNetwork::install_snapshot()`.
///
/// This is the client-side (Leader) component for chunk-based snapshot transport.
/// It splits a snapshot into chunks and sends them incrementally to a follower.
pub(super) struct Sender(std::marker::PhantomData<TypeConfig>);

impl Sender {
    /// Send a snapshot to a target node via `Net`.
    ///
    /// This function provides a default implementation for `RaftNetworkV2::full_snapshot()`
    /// using `RaftNetwork::install_snapshot()`.
    ///
    /// The argument `vote` is the leader's (the caller's) vote,
    /// which is used to check if the leader is still valid by a follower.
    ///
    /// `cancel` is a future that is polled by this function to check if the caller decides to
    /// cancel. It returns `Ready` if the caller decides to cancel this snapshot transmission.
    pub(super) async fn send_snapshot(
        net: &mut NetworkClient,
        vote: VoteOf<TypeConfig>,
        mut snapshot: SnapshotOf<TypeConfig>,
        cancel: impl Future<Output = ReplicationClosed> + OptionalSend + 'static,
        option: RPCOption,
    ) -> Result<SnapshotResponse<TypeConfig>, StreamingError<TypeConfig>> {
        let subject_verb = || {
            (
                ErrorSubject::Snapshot(Some(snapshot.meta.signature())),
                ErrorVerb::Read,
            )
        };

        let mut offset = 0;
        let end = snapshot
            .snapshot
            .seek(SeekFrom::End(0))
            .await
            .sto_res(subject_verb)?;

        let mut c = std::pin::pin!(cancel);
        loop {
            // If canceled, return at once
            if let Some(err) = c.as_mut().now_or_never() {
                return Err(err.into());
            }

            // Sleep a short time otherwise in test environment it is a dead-loop that never
            // yields. Because network implementation does not yield.
            TypeConfig::sleep(Duration::from_millis(1)).await;

            snapshot
                .snapshot
                .seek(SeekFrom::Start(offset))
                .await
                .sto_res(subject_verb)?;

            // Safe unwrap(): this function is called only by default implementation of
            // `RaftNetwork::full_snapshot()` and it is always set.
            let chunk_size = option.snapshot_chunk_size().unwrap();
            let mut buf = Vec::with_capacity(chunk_size);
            while buf.capacity() > buf.len() {
                let n = snapshot
                    .snapshot
                    .read_buf(&mut buf)
                    .await
                    .sto_res(subject_verb)?;
                if n == 0 {
                    break;
                }
            }

            let n_read = buf.len();

            let done = (offset + n_read as u64) == end;
            let req = InstallSnapshotRequest {
                vote,
                meta: snapshot.meta.clone(),
                offset,
                data: buf,
                done,
            };

            // Send the RPC over to the target.
            tracing::debug!(
                "sending snapshot chunk: snapshot_size: {}, offset: {}, end: {}, done: {}",
                req.data.len(),
                req.offset,
                end,
                req.done
            );

            #[allow(deprecated)]
            let res =
                TypeConfig::timeout(option.hard_ttl(), net.install_snapshot(req, option.clone()))
                    .await;

            let resp = match res {
                Ok(outer_res) => match outer_res {
                    Ok(res) => res,
                    Err(err) => {
                        let err: RPCError<TypeConfig, RaftError<TypeConfig, InstallSnapshotError>> =
                            err;

                        tracing::warn!("failed to send InstallSnapshot RPC: {err}");

                        match err {
                            RPCError::Timeout(_)
                            | RPCError::Unreachable(_)
                            | RPCError::Network(_) => {}
                            RPCError::RemoteError(remote_err) => match remote_err.source {
                                RaftError::Fatal(_) => {}
                                RaftError::APIError(snapshot_err) => match snapshot_err {
                                    InstallSnapshotError::SnapshotMismatch(mismatch) => {
                                        tracing::warn!(
                                            "snapshot mismatch, reset offset and retry: mismatch: {mismatch}",
                                        );
                                        offset = 0;
                                    }
                                },
                            },
                        }
                        continue;
                    }
                },
                Err(err) => {
                    tracing::warn!("timeout sending InstallSnapshot RPC: {err}");
                    continue;
                }
            };

            if resp.vote != vote {
                // Unfinished, return a response with a higher vote.
                // The caller checks the vote and return a HigherVote error.
                return Ok(SnapshotResponse::new(resp.vote));
            }

            if done {
                return Ok(SnapshotResponse::new(resp.vote));
            }

            offset += n_read as u64;
        }
    }
}
