use std::process;
use std::time::Duration;
use error_mapper::{TheResult};
use tokio::sync::broadcast::Receiver;

pub(super) async fn command_receiver(mut receiver: Receiver<()>) -> TheResult<()>{

    println!("Awaiting signal to actuate....");

    loop {
        if let Err(e) = receiver.recv().await {
            println!("{}", e);
            break;
        }
        println!("Received signal, displaying hello message...");
        tokio::time::sleep(Duration::from_secs(3)).await;
        println!("Hello there ^.^");
    }

    println!("Closed channel, shutting down thread...");

    Ok(())
}

pub(super) async fn stop_process() {

    tokio::time::sleep(Duration::from_secs(5)).await;
    process::exit(0);
}
