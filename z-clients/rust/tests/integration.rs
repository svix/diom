use std::time::Duration;

use diom_client::{
    DiomClient, DiomOptions,
    models::{CacheDeleteIn, CacheGetIn, CacheSetIn, KvDeleteIn, KvGetIn, KvSetIn},
};

fn client() -> DiomClient {
    let token = std::env::var("DIOM_TOKEN").expect("DIOM_TOKEN must be set");
    let server_url = std::env::var("DIOM_SERVER_URL").expect("DIOM_SERVER_URL must be set");
    let options = DiomOptions {
        server_url: Some(server_url),
        ..Default::default()
    };
    DiomClient::new(token, Some(options))
}

#[tokio::test]
#[ignore]
async fn test_health_ping() {
    let client = client();
    client.health().ping().await.unwrap();
}

#[tokio::test]
#[ignore]
async fn test_kv_set_get_delete() {
    let client = client();
    let key = "rust-integration-kv-key".to_string();
    let value = b"rust-integration-kv-value".to_vec();

    // Set
    let set_resp = client
        .kv()
        .set(key.clone(), value.clone(), KvSetIn::new())
        .await
        .unwrap();
    assert!(set_resp.success);

    // Get
    let get_resp = client.kv().get(key.clone(), KvGetIn::new()).await.unwrap();
    assert_eq!(get_resp.value, Some(value));

    // Delete
    let del_resp = client
        .kv()
        .delete(key.clone(), KvDeleteIn::new())
        .await
        .unwrap();
    assert!(del_resp.success);

    // Verify deleted
    let get_resp = client.kv().get(key, KvGetIn::new()).await.unwrap();
    assert_eq!(get_resp.value, None);
}

#[tokio::test]
#[ignore]
async fn test_cache_set_get_delete() {
    let client = client();
    let key = "rust-integration-cache-key".to_string();
    let value = b"rust-integration-cache-value".to_vec();

    // Set (ttl_ms = 60000)
    client
        .cache()
        .set(
            key.clone(),
            value.clone(),
            CacheSetIn::new(Duration::from_secs(60)),
        )
        .await
        .unwrap();

    // Get
    let get_resp = client
        .cache()
        .get(key.clone(), CacheGetIn::new())
        .await
        .unwrap();
    assert_eq!(get_resp.value, Some(value));

    // Delete
    let del_resp = client
        .cache()
        .delete(key.clone(), CacheDeleteIn::new())
        .await
        .unwrap();
    assert!(del_resp.success);

    // Verify deleted
    let get_resp = client.cache().get(key, CacheGetIn::new()).await.unwrap();
    assert_eq!(get_resp.value, None);
}
