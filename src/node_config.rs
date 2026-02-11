use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct NodeConfig {

    #[arg(long)]
    pub node_id: String,

    #[arg(long)]
    pub bind: String,

    #[arg(long)]
    pub peers: String,

    #[arg(long, default_value_t = 4)]
    pub federation_size: usize,
}
