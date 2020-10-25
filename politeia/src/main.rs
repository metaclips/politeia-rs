pub mod model;
mod types;

#[tokio::main]
async fn main() {
    let mut client = model::new().expect("Unable to create a new reqwest client");

    let tokens = client.fetch_tokens().await.expect("Unable to fetch tokens");

    let proposals = client.fetch_all_proposals(tokens).await;

    println!("{}", proposals.unwrap().pre.proposals.len())
}
