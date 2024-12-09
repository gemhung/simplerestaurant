use clap::{Parser, Subcommand};
use std::io::Write;

mod cancel;
mod order;
mod query;
mod utils;

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help=true)]
struct Cli {
    #[clap(short, long, default_value = "127.0.0.1:8080")]
    addr: String,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Order {
        #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
        food: Vec<String>,
    },
    Query {
        #[clap(short, long, value_parser)]
        table: i32,
        #[clap(short, long, value_parser)]
        name: Option<String>,
    },
    Cancel {
        #[clap(short, long)]
        table: i32,
        #[clap(short, long)]
        item_id: i64,
    },
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    let cli = Cli::parse();
    let addr = format!("http://{}", cli.addr);
    match cli.command.unwrap() {
        Commands::Order { food, .. } => {
            let mut orders = vec![];
            for inner in food.iter() {
                match inner.split_once(":") {
                    Some((name, table)) => {
                        orders.push((name, table.trim()));
                    }
                    None => {
                        panic!("Bad inputs, expectings something like noodle:1 where noodle is the food name and 1 is the table number ")
                    }
                }
            }
            let json = order::send_create_orders(&addr, orders).await;
            std::io::stdout()
                .lock()
                .write_all(json.to_string().as_bytes())
                .expect("Write failed");
        }
        Commands::Query {
            table,
            name: Some(name),
            ..
        } => {
            let json = query::query_table_with_name(&addr, table, name).await;
            std::io::stdout()
                .lock()
                .write_all(json.to_string().as_bytes())
                .expect("Write failed");
        }
        Commands::Query {
            table, name: None, ..
        } => {
            let json = query::query_table(&addr, table).await;
            std::io::stdout()
                .lock()
                .write_all(json.to_string().as_bytes())
                .expect("Write failed");
        }
        Commands::Cancel { table, item_id, .. } => {
            let json = cancel::cancel_order(&addr, table, item_id).await;
            std::io::stdout()
                .lock()
                .write_all(json.to_string().as_bytes())
                .expect("Write failed");
        }
    }

    Ok(())
}
