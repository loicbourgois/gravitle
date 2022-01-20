mod data;
mod maths;
use core::part;
use core::point;
mod gravitle;
mod websocket;
mod link;
mod plan;
mod entity;
mod websocket_async;
type Float = f32;
type Depth = u8;
type CellId = usize;
type ThreadId = u8;
type PartId = usize;
type PID = PartId;
use std::io::Error;
#[tokio::main]
async fn main() -> Result<(), Error> {
    let _ = env_logger::try_init();
    gravitle::start().await;
    Ok(())
}
