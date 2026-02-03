use diom::cfg::DatabaseConfig;
use diom_configgroup::{BothDatabases, entities::StreamConfig};
use test_utils::TestResult;

use crate::TestContext;

#[tokio::test]
async fn test_configgroup_fetch() -> TestResult {
    let TestContext {
        cfg,
        handle: _handle,
        ..
    } = super::start_server().await;

    let persistent_db = DatabaseConfig::persistent(&cfg.persistent_db).expect("persistent db");
    let ephemeral_db = DatabaseConfig::ephemeral(&cfg.ephemeral_db).expect("ephemeral db");

    let configgroup_state = diom_configgroup::State::init(BothDatabases {
        persistent_db: persistent_db.clone(),
        ephemeral_db: ephemeral_db.clone(),
    })
    .expect("initializing configgroup state");

    // Random-name group should resolve to "default" group
    let random_group =
        configgroup_state.fetch_group::<StreamConfig>("bloopety-blorp".to_string())?;
    assert!(random_group.is_some());
    let random_group = random_group.unwrap();
    assert_eq!(&random_group.name, "default");

    Ok(())
}
