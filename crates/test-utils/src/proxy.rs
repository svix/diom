use std::net::SocketAddr;

use tokio::{
    net::{TcpListener, TcpStream},
    task::{JoinHandle, JoinSet},
};

use crate::TestResult;

/// Start a simple TCP proxy that can be disconnected/aborted from its JoinHandle.
/// This is useful to fault-inject cluster replication for correctness checks.
#[tracing::instrument]
pub async fn start_replication_proxy(
    target: SocketAddr,
) -> TestResult<(SocketAddr, JoinHandle<()>)> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let address = listener.local_addr()?;
    let handle = tokio::spawn(async move {
        // TODO this joinset will technically just keep ballooning and leak, if not disconnected...
        let mut connections = JoinSet::new();
        loop {
            let (mut incoming, _) = listener.accept().await.unwrap();
            connections.spawn(async move {
                let mut outgoing = TcpStream::connect(target).await?;
                tokio::io::copy_bidirectional(&mut incoming, &mut outgoing).await
            });
        }
    });
    Ok((address, handle))
}
