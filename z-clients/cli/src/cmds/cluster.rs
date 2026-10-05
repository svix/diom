use clap::{Args, Subcommand};
use comfy_table::{Attribute, Cell, Table};
use diom::{
    DiomClient,
    models::{
        ClusterForceNodeDowngradeIn, ClusterForceNodeUpgradeIn, ClusterInitializeIn,
        ClusterInitializeOut, ClusterRemoveNodeIn,
    },
};
use itertools::Itertools;
use yansi::Paint;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub(crate) struct ClusterAdminArgs {
    #[command(subcommand)]
    pub command: ClusterCommands,
}

#[derive(Args)]
pub(crate) struct RemoveNodeArgs {
    node_id: String,
    /// This command is dangerous
    #[arg(long)]
    yes_i_know_what_im_doing: bool,
}

#[derive(Args)]
pub(crate) struct ChangeNodeArgs {
    node_id: String,
    /// This command is dangerous
    #[arg(long)]
    yes_i_know_what_im_doing: bool,
}

#[derive(Subcommand)]
pub(crate) enum ClusterCommands {
    /// Print information about the cluster and its nodes
    Status {
        /// Output results as JSON instead of a table
        #[arg(long)]
        json: bool,
    },
    /// Remove a node immediately from the cluster
    RemoveNode(RemoveNodeArgs),
    /// Attempt to upgrade a node from Learner to Voter
    UpgradeNode(ChangeNodeArgs),
    /// Attempt to downgrade a node from Voter to Learner
    DowngradeNode(ChangeNodeArgs),
    /// Initialize a new single-node cluster
    Initialize,
}

impl ClusterCommands {
    pub(crate) async fn exec(self, client: &DiomClient) -> anyhow::Result<()> {
        match self {
            Self::Status { json } => print_status(json, client).await,
            Self::RemoveNode(args) => remove_node(args, client).await,
            Self::UpgradeNode(args) => upgrade_node(args, client).await,
            Self::DowngradeNode(args) => downgrade_node(args, client).await,
            Self::Initialize => initialize(client).await,
        }
    }
}

async fn print_status(json: bool, client: &DiomClient) -> anyhow::Result<()> {
    let raw_status = client.cluster_admin().status().await?;
    if json {
        return crate::json::print_json_output(&raw_status);
    }

    let header = "Cluster Information".underline();
    println!("{header}\n");

    let mut table = Table::new();
    table.load_preset(comfy_table::presets::NOTHING).add_rows([
        [
            Cell::new("Cluster ID").add_attribute(Attribute::Bold),
            Cell::new(raw_status.cluster_id.as_deref().unwrap_or("")),
        ],
        [
            Cell::new("Cluster Name").add_attribute(Attribute::Bold),
            Cell::new(raw_status.cluster_name.as_deref().unwrap_or("(unset)")),
        ],
    ]);
    println!("{table}");

    let header = "Responding Server".underline();
    println!("\n{header}\n");

    let mut table = Table::new();
    table.load_preset(comfy_table::presets::NOTHING).add_rows([
        [
            Cell::new("Server ID").add_attribute(Attribute::Bold),
            Cell::new(raw_status.this_node_id),
        ],
        [
            Cell::new("Server State").add_attribute(Attribute::Bold),
            Cell::new(raw_status.this_node_state.to_string()),
        ],
        [
            Cell::new("Last Committed Timestamp").add_attribute(Attribute::Bold),
            Cell::new({
                let ts = raw_status.this_node_last_committed_timestamp;
                #[allow(clippy::disallowed_methods)]
                let now = jiff::Timestamp::now();
                if now > ts {
                    let ago = now - ts;
                    let ago = ago
                        .round(jiff::SpanRound::new().smallest(jiff::Unit::Millisecond))
                        .expect("rounding time should succeed");
                    format!("{ts} ({ago:#} ago)")
                } else {
                    format!("{ts} (in the future!)")
                }
            }),
        ],
        [
            Cell::new("Last Snapshot ID").add_attribute(Attribute::Bold),
            Cell::new({
                raw_status
                    .this_node_last_snapshot_id
                    .unwrap_or_else(|| "(none)".to_owned())
            }),
        ],
    ]);
    if let Some(last_purged) = raw_status.this_node_last_purged_log_index {
        table.add_row([Cell::new("Last Purged Log"), Cell::new(last_purged)]);
    }
    println!("{table}");
    let header = "Nodes".underline();
    println!("\n{header}\n");

    let mut table = Table::new();
    let rows = raw_status
        .nodes
        .iter()
        .sorted_unstable_by_key(|r| &r.node_id)
        .map(|row| {
            let term = if let Some(term) = row.last_committed_term
                && let Some(index) = row.last_committed_log_index
            {
                format!("{term}-{index}")
            } else {
                "".to_string()
            };
            [
                Cell::new(&row.node_id),
                Cell::new(&row.address),
                Cell::new(row.state),
                Cell::new(term),
            ]
        })
        .collect::<Vec<_>>();
    table
        .load_preset(comfy_table::presets::UTF8_FULL)
        .set_header(["Node ID", "Address", "State", "Last Transaction"])
        .add_rows(rows);
    println!("{table}");
    Ok(())
}

async fn remove_node(args: RemoveNodeArgs, client: &DiomClient) -> anyhow::Result<()> {
    let status = client.cluster_admin().status().await?;
    let Some(node) = status.nodes.iter().find(|n| n.node_id == args.node_id) else {
        anyhow::bail!("unable to find node {}", args.node_id);
    };
    if !args.yes_i_know_what_im_doing
        && !crate::utils::prompt(format!(
            "Are you sure you want to remove node {} ({})",
            args.node_id, node.address
        ))?
    {
        anyhow::bail!("aborting");
    }
    tracing::info!(
        node_id = args.node_id,
        address = node.address,
        "removing node"
    );
    client
        .cluster_admin()
        .remove_node(ClusterRemoveNodeIn {
            node_id: args.node_id,
        })
        .await?;
    Ok(())
}

async fn upgrade_node(args: ChangeNodeArgs, client: &DiomClient) -> anyhow::Result<()> {
    let status = client.cluster_admin().status().await?;
    let Some(node) = status.nodes.iter().find(|n| n.node_id == args.node_id) else {
        anyhow::bail!("unable to find node {}", args.node_id);
    };
    if !args.yes_i_know_what_im_doing
        && !crate::utils::prompt(format!(
            "Are you sure you want to upgrade node {} ({})",
            args.node_id, node.address
        ))?
    {
        anyhow::bail!("aborting");
    }
    tracing::info!(
        node_id = args.node_id,
        address = node.address,
        "upgrading node"
    );
    let response = client
        .cluster_admin()
        .force_node_upgrade(ClusterForceNodeUpgradeIn {
            node_id: args.node_id,
        })
        .await?;
    tracing::debug!(?response, "done upgrading node");
    Ok(())
}

async fn downgrade_node(args: ChangeNodeArgs, client: &DiomClient) -> anyhow::Result<()> {
    let status = client.cluster_admin().status().await?;
    let Some(node) = status.nodes.iter().find(|n| n.node_id == args.node_id) else {
        anyhow::bail!("unable to find node {}", args.node_id);
    };
    if !args.yes_i_know_what_im_doing
        && !crate::utils::prompt(format!(
            "Are you sure you want to downgrade node {} ({})",
            args.node_id, node.address
        ))?
    {
        anyhow::bail!("aborting");
    }
    tracing::info!(
        node_id = args.node_id,
        address = node.address,
        "downgrading node"
    );
    let response = client
        .cluster_admin()
        .force_node_downgrade(ClusterForceNodeDowngradeIn {
            node_id: args.node_id,
        })
        .await?;
    tracing::debug!(?response, "done downgrading node");
    Ok(())
}

async fn initialize(client: &DiomClient) -> anyhow::Result<()> {
    match client
        .cluster_admin()
        .initialize(ClusterInitializeIn::default())
        .await
    {
        Ok(ClusterInitializeOut { cluster_id }) => {
            println!("cluster successfully initialized with ID {cluster_id}");
            Ok(())
        }
        Err(err) => {
            anyhow::bail!("error initializing cluster: {err:?}");
        }
    }
}
