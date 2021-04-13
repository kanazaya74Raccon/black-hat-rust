use anyhow::Result;
use reqwest::{redirect, Client};
use std::{env, time::Duration};

mod error;
pub use error::Error;
mod model;
mod ports;
mod subdomains;
use model::Subdomain;
mod common_ports;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(Error::CliUsage.into());
    }

    let target = args[1].as_str();

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Building tokio's runtime");

    let http_timeout = Duration::from_secs(5);
    let http_client = Client::builder()
        .redirect(redirect::Policy::limited(4))
        .timeout(http_timeout)
        .build()?;

    let scan_result = runtime.block_on(async move {
        subdomains::enumerate(&http_client, target)
            .await?
            .into_iter()
            .map(ports::scan_ports)
            .map(|subdomain| ports::scan_http(&http_client, subdomain))
            .collect()
    });

    for subdomain in scan_result {
        println!("{}:", &subdomain.domain);
        for port in &subdomain.open_ports {
            let protocol = if port.is_http { "http" } else { "tcp" };
            println!("    {}: {}", port.port, protocol);
        }

        println!("");
    }

    Ok(())
}
