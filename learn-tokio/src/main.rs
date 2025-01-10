use log::Level;
use tokio::time;


async fn run() {
    log::info!("Sleeping");
    time::sleep(time::Duration::from_secs(1)).await;
    log::info!("AWAKE");
}
///! # main     
///! This is the main function that will run the tokio runtime and block on the future returned by
///! the `run` function. The `run` function is an async function that will log a message, sleep for
///! 1 second, and then log another message. The `simple_logger` crate is used to initialize the logger 
///! with the `Info` log level. the `tokio` crate is used to create the runtime and the `time`
///! 
fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();

    let rt = tokio::runtime::Runtime::new().unwrap();

    let future = run();

    rt.block_on(future);
}
