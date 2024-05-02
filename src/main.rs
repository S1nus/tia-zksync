use tokio::main;

use celestia_rpc::{BlobClient, HeaderClient, Client};
use celestia_types::{Blob, nmt::Namespace};
//use celestia_types::blob::SubmitOptions;

use thiserror::Error;
use std::time::Instant;

#[tokio::main]
async fn main() {
    /* The auth token for your light node, obtained by running something like:
        export AUTH_TOKEN=$(celestia light auth admin --p2p.network mocha-4)
    */
    let token = std::env::var("CELESTIA_NODE_AUTH_TOKEN").expect("Token not provided");

    // Read the namespace from an environment variable
    let namespace = Namespace::new(0, 
        &std::env::var("ROLLUP_NAMESPACE").expect("Rollup namespace not provided").into_bytes()
    ).expect("Invalid namespace");

    // Create a websocket connection to your light node
    let client = Client::new("ws://localhost:26658", Some(&token))
        .await
        .expect("Failed creating rpc client");

    // Read a file and load it as a blob
    let blob_file_name = std::env::var("BLOB_FILE").expect("Blob file not provided");
    let blob_file = std::fs::read(blob_file_name).expect("Could not read blob file");
    let blob = Blob::new(namespace, blob_file).expect("Failed to create a blob");

    // Compute the blob commmitment
    let blob_commitment = blob.commitment.clone();
    println!("commitment: {:?}", blob_commitment);

    // Submit the blob to the light node, and receive the height of the block where it's included
    let height = client.blob_submit(&[blob], None.into()).await.expect("Failed to submit blob");
    
    // Retrieve the merkle blob incluson proof
    let proof = client.blob_get_proof(height, namespace, blob_commitment).await.expect("Failed to get proof");
    println!("proof: {:?}", proof);
}
