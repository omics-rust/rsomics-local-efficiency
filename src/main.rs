use clap::Parser;
use rsomics_local_efficiency::{local_efficiency, parse_edge_list_ordered};
use std::io::{self, Read};

#[derive(Parser, Debug)]
#[command(
    name = "rsomics-local-efficiency",
    about = "Compute local efficiency of an undirected graph (networkx.local_efficiency)"
)]
struct Cli {
    /// Output as JSON object {"local_efficiency": <value>}
    #[arg(long)]
    json: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let (n, adj) = parse_edge_list_ordered(&input);
    let result = local_efficiency(n, &adj);

    if cli.json {
        println!("{}", serde_json::json!({"local_efficiency": result}));
    } else {
        println!("{:.17e}", result);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn cli_debug_assert() {
        Cli::command().debug_assert();
    }
}
