use std::{env, process};

use tsubakuro_rust_client::prelude::*;

mod job;
mod sub;

#[tokio::main]
async fn main() -> Result<(), TgError> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("Usage: {} <endpoint-url>", args[0]);
        process::exit(1);
    }

    let endpoint = match args.get(1) {
        Some(arg) => arg,
        None => panic!(),
    };

    sub::execute(endpoint).await?;
    job::execute(endpoint).await?;

    Ok(())
}

fn create_connection_option(endpoint: &str) -> Result<ConnectionOption, TgError> {
    let mut option = ConnectionOption::new();
    option.set_application_name("tsubakuro-rust-dbtest");
    option.set_endpoint_url(endpoint)?;

    Ok(option)
}
