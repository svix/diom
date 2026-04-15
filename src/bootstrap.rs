use std::{fs, str::FromStr, time::Instant};

use crate::{
    cfg::Configuration as AppConfig,
    core::{INTERNAL_NAMESPACE, cluster::RaftState},
    v1::endpoints::{
        admin::auth::{policy::AdminAccessPolicyConfigureIn, role::AdminRoleConfigureIn},
        auth_token::AuthTokenConfigureNamespaceIn,
        cache::CacheConfigureNamespaceIn,
        idempotency::IdempotencyConfigureNamespaceIn,
        kv::KvConfigureNamespaceIn,
        msgs::MsgNamespaceConfigureIn,
        rate_limit::RateLimitConfigureNamespaceIn,
    },
};
use anyhow::{Context, bail};
use diom_msgs::entities::Retention;
use diom_namespace::{
    DEFAULT_NAMESPACE_NAME,
    entities::{EvictionPolicy, NamespaceName},
};

#[derive(Debug)]
enum BootstrapCommand {
    Kv(KvConfigureNamespaceIn),
    Cache(CacheConfigureNamespaceIn),
    Idempotency(IdempotencyConfigureNamespaceIn),
    RateLimit(RateLimitConfigureNamespaceIn),
    Msgs(MsgNamespaceConfigureIn),
    AuthToken(AuthTokenConfigureNamespaceIn),
    AdminAuthPolicy(AdminAccessPolicyConfigureIn),
    AdminAuthRole(AdminRoleConfigureIn),
}

impl BootstrapCommand {
    /// Shared logic for matching module/resource/action and deserializing JSON.
    fn from_parts(
        module: &str,
        resource: &str,
        action: &str,
        json_str: &str,
    ) -> anyhow::Result<Self> {
        match (module, resource, action) {
            ("kv", "namespace", "configure") => Ok(BootstrapCommand::Kv(
                serde_json::from_str(json_str)
                    .with_context(|| format!("invalid JSON for kv namespace: {json_str:?}"))?,
            )),
            ("cache", "namespace", "configure") => Ok(BootstrapCommand::Cache(
                serde_json::from_str(json_str)
                    .with_context(|| format!("invalid JSON for cache namespace: {json_str:?}"))?,
            )),
            ("idempotency", "namespace", "configure") => Ok(BootstrapCommand::Idempotency(
                serde_json::from_str(json_str).with_context(|| {
                    format!("invalid JSON for idempotency namespace: {json_str:?}")
                })?,
            )),
            ("rate-limit", "namespace", "configure") => Ok(BootstrapCommand::RateLimit(
                serde_json::from_str(json_str).with_context(|| {
                    format!("invalid JSON for rate-limit namespace: {json_str:?}")
                })?,
            )),
            ("msgs", "namespace", "configure") => Ok(BootstrapCommand::Msgs(
                serde_json::from_str(json_str)
                    .with_context(|| format!("invalid JSON for msgs namespace: {json_str:?}"))?,
            )),
            ("admin", "auth-policy", "configure") => Ok(BootstrapCommand::AdminAuthPolicy(
                serde_json::from_str(json_str)
                    .with_context(|| format!("invalid JSON for admin auth-policy: {json_str:?}"))?,
            )),
            ("admin", "auth-role", "configure") => Ok(BootstrapCommand::AdminAuthRole(
                serde_json::from_str(json_str)
                    .with_context(|| format!("invalid JSON for admin auth-role: {json_str:?}"))?,
            )),
            _ => bail!("unknown command: {module} {resource} {action}"),
        }
    }

    fn split_header(line: &str) -> anyhow::Result<(&str, &str, &str, &str)> {
        let (module, rest) = line.split_once(char::is_whitespace).with_context(|| {
            format!("expected '<module> <resource> <action> <json>', got: {line:?}")
        })?;
        let rest = rest.trim_start();
        let (resource, rest) = rest
            .split_once(char::is_whitespace)
            .with_context(|| format!("expected '<resource> <action> <json>', got: {rest:?}"))?;
        let rest = rest.trim_start();
        let (action, json_str) = rest
            .split_once(char::is_whitespace)
            .with_context(|| format!("expected '<action> <json>', got: {rest:?}"))?;
        Ok((module, resource, action, json_str))
    }

    async fn apply(self, raft_state: &RaftState) -> anyhow::Result<()> {
        match self {
            BootstrapCommand::Kv(v) => {
                tracing::debug!(name = v.name.as_str(), "bootstrapping kv");
                raft_state
                    .client_write(diom_kv::operations::ConfigureKvOperation::from(v))
                    .await?;
            }
            BootstrapCommand::Cache(v) => {
                tracing::debug!(name = v.name.as_str(), "bootstrapping cache");
                raft_state
                    .client_write(diom_cache::operations::ConfigureCacheOperation::from(v))
                    .await?;
            }
            BootstrapCommand::Idempotency(v) => {
                tracing::debug!(name = v.name.as_str(), "bootstrapping idempotency");
                raft_state
                    .client_write(
                        diom_idempotency::operations::ConfigureIdempotencyOperation::from(v),
                    )
                    .await?;
            }
            BootstrapCommand::RateLimit(v) => {
                tracing::debug!(name = v.name.as_str(), "bootstrapping rate-limit");
                raft_state
                    .client_write(diom_rate_limit::operations::ConfigureRateLimitOperation::from(v))
                    .await?;
            }
            BootstrapCommand::Msgs(v) => {
                tracing::debug!(name = v.name.as_str(), "bootstrapping msgs");
                raft_state
                    .client_write(diom_msgs::operations::ConfigureNamespaceOperation::from(v))
                    .await?;
            }
            BootstrapCommand::AuthToken(v) => {
                tracing::debug!(name = v.name.as_str(), "bootstrapping auth_token");
                raft_state
                    .client_write(
                        diom_auth_token::operations::ConfigureAuthTokenNamespaceOperation::from(v),
                    )
                    .await?;
            }
            BootstrapCommand::AdminAuthPolicy(v) => {
                tracing::debug!(id = v.id.as_str(), "bootstrapping auth-policy");
                raft_state
                    .client_write(
                        diom_admin_auth::operations::ConfigureAccessPolicyOperation::new(
                            v.id,
                            v.description,
                            v.rules,
                        ),
                    )
                    .await?;
            }
            BootstrapCommand::AdminAuthRole(v) => {
                tracing::debug!(id = v.id.as_str(), "bootstrapping auth-role");
                raft_state
                    .client_write(diom_admin_auth::operations::ConfigureRoleOperation::new(
                        v.id,
                        v.description,
                        v.rules,
                        v.policies,
                        v.context,
                    ))
                    .await?;
            }
        }
        Ok(())
    }

    fn namespace(&self) -> Option<&NamespaceName> {
        Some(match self {
            BootstrapCommand::Kv(v) => &v.name,
            BootstrapCommand::Cache(v) => &v.name,
            BootstrapCommand::Idempotency(v) => &v.name,
            BootstrapCommand::RateLimit(v) => &v.name,
            BootstrapCommand::Msgs(v) => &v.name,
            BootstrapCommand::AuthToken(v) => &v.name,
            BootstrapCommand::AdminAuthPolicy(_) | BootstrapCommand::AdminAuthRole(_) => {
                return None;
            }
        })
    }
}

impl FromStr for BootstrapCommand {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> anyhow::Result<Self> {
        let (module, resource, action, json_str) = Self::split_header(line)?;
        Self::from_parts(module, resource, action, json_str.trim())
    }
}

#[allow(clippy::disallowed_types)]
fn parse_bootstrap(content: &str) -> anyhow::Result<Vec<BootstrapCommand>> {
    let mut commands = Vec::new();
    let mut lines = content.lines().enumerate().peekable();

    while let Some((i, line)) = lines.next() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let (module, resource, action, json_start) = BootstrapCommand::split_header(trimmed)
            .with_context(|| format!("on line {}", i + 1))?;

        // Accumulate lines until serde can parse a complete JSON value
        let mut json_buf = json_start.trim().to_string();
        let json_str = loop {
            let mut stream =
                serde_json::Deserializer::from_str(&json_buf).into_iter::<serde_json::Value>();
            match stream.next() {
                Some(Ok(_)) => {
                    let consumed = stream.byte_offset();
                    json_buf.truncate(consumed);
                    break json_buf;
                }
                Some(Err(e)) if e.classify() == serde_json::error::Category::Eof => {
                    match lines.next() {
                        Some((_, next_line)) => {
                            json_buf.push('\n');
                            json_buf.push_str(next_line);
                        }
                        None => {
                            return Err(e).with_context(|| {
                                format!("unterminated JSON starting at line {}", i + 1)
                            });
                        }
                    }
                }
                Some(Err(e)) => {
                    return Err(e).with_context(|| format!("invalid JSON on line {}", i + 1));
                }
                None => {
                    anyhow::bail!("expected JSON on line {}", i + 1);
                }
            }
        };

        let cmd = BootstrapCommand::from_parts(module, resource, action, &json_str)
            .with_context(|| format!("error on line {}", i + 1))?;
        commands.push(cmd);
    }

    Ok(commands)
}

fn ensure_defaults(commands: &mut Vec<BootstrapCommand>) {
    macro_rules! ensure_default {
        ($variant:ident, $constructor:expr) => {
            if !commands.iter().any(|c| {
                matches!(c, BootstrapCommand::$variant(_))
                    && c.namespace() == Some(&*DEFAULT_NAMESPACE_NAME)
            }) {
                commands.insert(0, $constructor);
            }
        };
    }

    ensure_default!(
        AuthToken,
        BootstrapCommand::AuthToken(AuthTokenConfigureNamespaceIn {
            name: (*DEFAULT_NAMESPACE_NAME).clone(),
        })
    );
    commands.insert(
        0,
        BootstrapCommand::AuthToken(AuthTokenConfigureNamespaceIn {
            name: (*INTERNAL_NAMESPACE).clone(),
        }),
    );
    ensure_default!(
        Msgs,
        BootstrapCommand::Msgs(MsgNamespaceConfigureIn {
            name: (*DEFAULT_NAMESPACE_NAME).clone(),
            retention: Retention::default(),
        })
    );
    ensure_default!(
        RateLimit,
        BootstrapCommand::RateLimit(RateLimitConfigureNamespaceIn {
            name: (*DEFAULT_NAMESPACE_NAME).clone(),
        })
    );
    ensure_default!(
        Idempotency,
        BootstrapCommand::Idempotency(IdempotencyConfigureNamespaceIn {
            name: (*DEFAULT_NAMESPACE_NAME).clone(),
        })
    );
    ensure_default!(
        Cache,
        BootstrapCommand::Cache(CacheConfigureNamespaceIn {
            name: (*DEFAULT_NAMESPACE_NAME).clone(),
            eviction_policy: EvictionPolicy::NoEviction,
        })
    );
    ensure_default!(
        Kv,
        BootstrapCommand::Kv(KvConfigureNamespaceIn {
            name: (*DEFAULT_NAMESPACE_NAME).clone(),
        })
    );
}

fn load_commands(
    config_path: Option<&str>,
    config_content: Option<&str>,
) -> anyhow::Result<Vec<BootstrapCommand>> {
    let content = match (config_content, config_path) {
        (Some(content), _) => content.to_owned(),
        (None, Some(path)) => fs::read_to_string(path)
            .with_context(|| format!("opening bootstrap config {path:?}"))?,
        (None, None) => String::new(),
    };
    let mut commands = parse_bootstrap(&content).context("parsing bootstrap config")?;
    ensure_defaults(&mut commands);
    Ok(commands)
}

async fn wait_for_up(config: &AppConfig, raft_state: &RaftState) -> anyhow::Result<()> {
    let mut deadline: std::pin::Pin<Box<dyn Future<Output = ()> + Send>> =
        if let Some(time) = config.bootstrap_max_wait_time {
            Box::pin(tokio::time::sleep(time.into()))
        } else {
            Box::pin(futures_util::future::pending())
        };
    let shutdown = crate::shutting_down_token();
    let mut interval = tokio::time::interval(std::time::Duration::from_millis(100));

    loop {
        tokio::select! {
            _ = shutdown.cancelled() => {
                anyhow::bail!("process shut down before bootstrap finished");
            }
            _ = deadline.as_mut() => {
                anyhow::bail!("bootstrap timeout exceeded");
            },
            _ = interval.tick() => {
                if raft_state.is_up().await {
                    tracing::trace!("cluster is up");
                    return Ok(())
                }
            }
        }
    }
}

pub async fn run(app_config: AppConfig, raft_state: RaftState) -> anyhow::Result<()> {
    let t = Instant::now();

    wait_for_up(&app_config, &raft_state).await?;

    let commands = load_commands(
        app_config.bootstrap_cfg_path.as_deref(),
        app_config.bootstrap_cfg.as_deref(),
    )?;

    tracing::debug!(
        num_commands = commands.len(),
        persistent_db = ?app_config.persistent_db,
        ephemeral_db = ?app_config.ephemeral_db,
        "Starting bootstrapping."
    );

    for cmd in commands {
        cmd.apply(&raft_state).await?;
    }

    tracing::info!(
        duration_ms = (Instant::now() - t).as_millis(),
        "Finished bootstrapping."
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kv_defaults() {
        let cmd: BootstrapCommand = r#"kv namespace configure {"name":"myns"}"#.parse().unwrap();
        let BootstrapCommand::Kv(v) = cmd else {
            panic!()
        };
        assert_eq!(v.name.as_str(), "myns");
    }

    #[test]
    fn cache_defaults() {
        let cmd: BootstrapCommand = r#"cache namespace configure {"name":"myns"}"#.parse().unwrap();
        let BootstrapCommand::Cache(v) = cmd else {
            panic!()
        };
        assert_eq!(v.eviction_policy, EvictionPolicy::NoEviction);
    }

    #[test]
    fn idempotency() {
        let cmd: BootstrapCommand = r#"idempotency namespace configure {"name":"myns"}"#
            .parse()
            .unwrap();
        let BootstrapCommand::Idempotency(v) = cmd else {
            panic!()
        };
        assert_eq!(v.name.as_str(), "myns");
    }

    #[test]
    fn rate_limit() {
        let cmd: BootstrapCommand =
            r#"rate-limit namespace configure {"name":"myns"}"#.parse().unwrap();
        assert!(matches!(cmd, BootstrapCommand::RateLimit(v) if v.name.as_str() == "myns"));
    }

    #[test]
    fn msgs_defaults() {
        let cmd: BootstrapCommand = r#"msgs namespace configure {"name":"myns"}"#.parse().unwrap();
        let BootstrapCommand::Msgs(v) = cmd else {
            panic!()
        };
        assert_eq!(v.name.as_str(), "myns");
        assert_eq!(v.retention, Retention::default());
    }

    #[test]
    fn msgs_with_options() {
        let cmd: BootstrapCommand =
            r#"msgs namespace configure {"name":"myns","retention":{"period_ms":60000,"size_bytes":500}}"#
                .parse()
                .unwrap();
        let BootstrapCommand::Msgs(v) = cmd else {
            panic!()
        };
        assert_eq!(v.retention.period.unwrap().as_millis(), 60000);
        assert_eq!(v.retention.size_bytes.unwrap().get(), 500);
    }

    #[test]
    fn too_few_tokens() {
        assert!(
            "kv namespace configure"
                .parse::<BootstrapCommand>()
                .is_err()
        );
    }

    #[test]
    fn wrong_resource() {
        assert!(r#"kv config configure {"name":"myns"}"#.parse::<BootstrapCommand>().is_err());
    }

    #[test]
    fn wrong_action() {
        assert!(r#"kv namespace update {"name":"myns"}"#.parse::<BootstrapCommand>().is_err());
    }

    #[test]
    fn unknown_module() {
        assert!(r#"blob namespace configure {"name":"myns"}"#.parse::<BootstrapCommand>().is_err());
    }

    #[test]
    fn auth_policy_configure() {
        let cmd: BootstrapCommand =
            r#"admin auth-policy configure {"id":"mypolicy","description":"test","rules":[]}"#
                .parse()
                .unwrap();
        let BootstrapCommand::AdminAuthPolicy(v) = cmd else {
            panic!()
        };
        assert_eq!(v.id.as_str(), "mypolicy");
        assert_eq!(v.description, "test");
        assert!(v.rules.is_empty());
    }

    #[test]
    fn auth_role_configure() {
        let cmd: BootstrapCommand =
            r#"admin auth-role configure {"id":"myrole","description":"test","rules":[]}"#
                .parse()
                .unwrap();
        let BootstrapCommand::AdminAuthRole(v) = cmd else {
            panic!()
        };
        assert_eq!(v.id.as_str(), "myrole");
        assert_eq!(v.description, "test");
        assert!(v.rules.is_empty());
        assert!(v.policies.is_empty());
    }

    #[test]
    fn unknown_admin_subcommand() {
        assert!(r#"admin unknown configure {"id":"x"}"#.parse::<BootstrapCommand>().is_err());
    }

    #[test]
    fn invalid_json() {
        assert!(
            "kv namespace configure not-json"
                .parse::<BootstrapCommand>()
                .is_err()
        );
    }

    #[test]
    fn missing_name_field() {
        assert!(r#"kv namespace configure {}"#.parse::<BootstrapCommand>().is_err());
    }

    #[test]
    fn invalid_eviction_policy() {
        assert!(
            r#"cache namespace configure {"name":"myns","eviction_policy":"random"}"#
                .parse::<BootstrapCommand>()
                .is_err()
        );
    }

    #[test]
    fn skips_blank_lines_and_comments() {
        let input = r#"
            # this is a comment
            kv namespace configure {"name":"foo"}

            # another comment
            cache namespace configure {"name":"bar"}
        "#;
        let cmds = parse_bootstrap(input).unwrap();
        assert_eq!(cmds.len(), 2);
        assert!(matches!(&cmds[0], BootstrapCommand::Kv(v) if v.name.as_str() == "foo"));
        assert!(matches!(&cmds[1], BootstrapCommand::Cache(v) if v.name.as_str() == "bar"));
    }

    #[test]
    fn parse_error_includes_line_number() {
        let input = "kv namespace configure {\"name\":\"foo\"}\nkv namespace configure not-json\n";
        let err = parse_bootstrap(input).unwrap_err();
        assert!(err.to_string().contains("line 2"), "error was: {err}");
    }

    #[test]
    fn empty_input_produces_no_commands() {
        assert!(parse_bootstrap("").unwrap().is_empty());
    }

    #[test]
    fn ensure_defaults_injects_all_five_when_empty() {
        let mut cmds = vec![];
        ensure_defaults(&mut cmds);
        // Make sure at least the original five are there
        assert!(cmds.len() >= 5);
        assert!(
            cmds.iter()
                .any(|c| matches!(c, BootstrapCommand::Kv(v) if v.name == *DEFAULT_NAMESPACE_NAME))
        );
        assert!(
            cmds.iter().any(
                |c| matches!(c, BootstrapCommand::Cache(v) if v.name == *DEFAULT_NAMESPACE_NAME)
            )
        );
        assert!(cmds.iter().any(
            |c| matches!(c, BootstrapCommand::Idempotency(v) if v.name == *DEFAULT_NAMESPACE_NAME)
        ));
        assert!(cmds.iter().any(
            |c| matches!(c, BootstrapCommand::RateLimit(v) if v.name == *DEFAULT_NAMESPACE_NAME)
        ));
        assert!(
            cmds.iter().any(
                |c| matches!(c, BootstrapCommand::Msgs(v) if v.name == *DEFAULT_NAMESPACE_NAME)
            )
        );
    }

    #[test]
    fn ensure_defaults_does_not_duplicate_existing_default() {
        let mut cmds = vec![BootstrapCommand::Kv(KvConfigureNamespaceIn {
            name: DEFAULT_NAMESPACE_NAME.clone(),
        })];
        ensure_defaults(&mut cmds);
        let kv_defaults: Vec<_> = cmds
            .iter()
            .filter(|c| matches!(c, BootstrapCommand::Kv(v) if v.name == *DEFAULT_NAMESPACE_NAME))
            .collect();
        assert_eq!(kv_defaults.len(), 1);
    }

    #[test]
    fn ensure_defaults_does_not_suppress_non_default_namespaces() {
        let mut cmds = vec![BootstrapCommand::Kv(KvConfigureNamespaceIn {
            name: NamespaceName("other".to_owned()),
        })];
        ensure_defaults(&mut cmds);
        let kv_count = cmds
            .iter()
            .filter(|c| matches!(c, BootstrapCommand::Kv(_)))
            .count();
        assert_eq!(kv_count, 2);
    }

    #[test]
    fn auth_policy_configure_single_line() {
        let input = r#"admin auth-policy configure {"id":"pol","description":"full access","rules":[{"effect":"allow","resource":"*:*:*","actions":["*"]}]}"#;
        let cmds = parse_bootstrap(input).unwrap();
        assert_eq!(cmds.len(), 1);
        let BootstrapCommand::AdminAuthPolicy(v) = &cmds[0] else {
            panic!()
        };
        assert_eq!(v.id.as_str(), "pol");
        assert_eq!(v.rules.len(), 1);
    }

    #[test]
    fn auth_policy_configure_multiline() {
        let input = r#"admin auth-policy configure {
            "id": "pol",
            "description": "full access",
            "rules": [
                {
                    "effect": "allow",
                    "resource": "*:*:*",
                    "actions": ["*"]
                }
            ]
        }"#;
        let cmds = parse_bootstrap(input).unwrap();
        assert_eq!(cmds.len(), 1);
        let BootstrapCommand::AdminAuthPolicy(v) = &cmds[0] else {
            panic!()
        };
        assert_eq!(v.id.as_str(), "pol");
        assert_eq!(v.rules.len(), 1);
    }
    #[test]
    fn parse_multiple_commands_all_modules() {
        let input = r#"
            kv namespace configure {"name":"my-kv"}
            cache namespace configure {"name":"my-cache","eviction_policy":"no-eviction"}
            idempotency namespace configure {"name":"my-idemp"}
            rate-limit namespace configure {"name":"my-rl"}
            msgs namespace configure {"name":"my-msgs","retention":{"period_ms":60000,"size_bytes":500}}
            admin auth-policy configure {"id":"pol","description":"full access","rules":[{"effect":"allow","resource":"*:*:*","actions":["*"]}]}
            admin auth-role configure {"id":"role","description":"admin","rules":[],"policies":["pol"]}
        "#;
        let cmds = parse_bootstrap(input).unwrap();
        assert_eq!(cmds.len(), 7);
        assert!(matches!(&cmds[0], BootstrapCommand::Kv(v) if v.name.as_str() == "my-kv"));
        assert!(matches!(&cmds[1], BootstrapCommand::Cache(v) if v.name.as_str() == "my-cache"));
        assert!(
            matches!(&cmds[2], BootstrapCommand::Idempotency(v) if v.name.as_str() == "my-idemp")
        );
        assert!(matches!(&cmds[3], BootstrapCommand::RateLimit(v) if v.name.as_str() == "my-rl"));
        assert!(matches!(&cmds[4], BootstrapCommand::Msgs(v) if v.name.as_str() == "my-msgs"));
        assert!(matches!(&cmds[5], BootstrapCommand::AdminAuthPolicy(v) if v.id.as_str() == "pol"));
        assert!(matches!(&cmds[6], BootstrapCommand::AdminAuthRole(v) if v.id.as_str() == "role"));
    }

    #[test]
    fn multiline_auth_policy_and_role() {
        let input = r#"
            kv namespace configure {"name":"my-kv"}
            admin auth-policy configure {
                "id": "pol",
                "description": "full access",
                "rules": [
                    {
                        "effect": "allow",
                        "resource": "*:*:*",
                        "actions": ["*"]
                    }
                ]
            }
            admin auth-role configure {
                "id": "role",
                "description": "admin",
                "rules": [],
                "policies": ["pol"]
            }
        "#;
        let cmds = parse_bootstrap(input).unwrap();
        assert_eq!(cmds.len(), 3);
        assert!(matches!(&cmds[0], BootstrapCommand::Kv(v) if v.name.as_str() == "my-kv"));
        assert!(matches!(&cmds[1], BootstrapCommand::AdminAuthPolicy(v) if v.id.as_str() == "pol"));
        assert!(matches!(&cmds[2], BootstrapCommand::AdminAuthRole(v) if v.id.as_str() == "role"));
    }
}
