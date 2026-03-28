use diom_client::{
    DiomClient, DiomOptions,
    models::{MsgIn, MsgPublishIn},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = DiomClient::new(
        "admin_abcdefghijlmnopqrstuvwxyz012345".to_owned(),
        Some(DiomOptions {
            server_url: Some("http://localhost:8050".to_string()),
            ..Default::default()
        }),
    );

    let msgs = vec![
        MsgIn::new(b"hello from diom".to_vec()),
        MsgIn::new(b"second message".to_vec()).with_key(Some("my-key".to_string())),
    ];

    let publish_in = MsgPublishIn::new(msgs);

    let result = client.msgs().publish("test".to_owned(), publish_in).await?;

    for topic in result.topics {
        println!(
            "Published to topic '{}': offsets {}..{}",
            topic.topic, topic.start_offset, topic.offset
        );
    }

    Ok(())
}
