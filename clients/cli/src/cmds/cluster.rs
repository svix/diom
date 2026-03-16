use clap::{Args, Subcommand};
use colored_json::Paint;
use comfy_table::{Attribute, Cell, Table};
use diom_client::DiomClient;
use itertools::Itertools;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct ClusterAdminArgs {
    #[command(subcommand)]
    pub command: ClusterCommands,
}

#[derive(Subcommand)]
pub enum ClusterCommands {
    /// Print information about the cluster and its nodes
    Status {
        /// Output results as JSON instead of a table
        #[arg(long)]
        json: bool,
    },
}

impl ClusterCommands {
    pub async fn exec(
        self,
        client: &DiomClient,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Status { json } => print_status(json, client, color_mode).await,
        }
    }
}

async fn print_status(
    json: bool,
    client: &DiomClient,
    color_mode: colored_json::ColorMode,
) -> anyhow::Result<()> {
    let raw_status = client.admin().cluster_status().await?;
    if json {
        return crate::json::print_json_output(&raw_status, color_mode);
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
    ]);
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
                format!("{}-{}", term, index)
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
