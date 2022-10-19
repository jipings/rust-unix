use tokio::time::timeout;
use tokio::sync::oneshot;

use std::time::Duration;

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel::<String>();

    tokio::spawn(async move {
        // if let Err(_) = tx.send(3) {
        //     println!("the receiver dropped");
        // }
    });

    // Wrap the function with a `Timeout` set to expire in 10 milliseconds
    if let Err(_) = timeout(Duration::from_millis(10), rx).await {
        println!("did not receive value within 10 ms");
    }
}