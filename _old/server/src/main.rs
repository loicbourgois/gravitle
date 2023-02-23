mod data;
mod maths;
mod tests;
use core::part;
use core::point;
mod entity;
mod gravitle;
mod link;
mod plan;
mod websocket;
mod websocket_async;
type Float = f32;
type Depth = u8;
type CellId = usize;
type PartId = usize;
type Pid = PartId;
type DnaId = u128;
use std::io::Error;
#[tokio::main]
async fn main() -> Result<(), Error> {
    let _ = env_logger::try_init();
    gravitle::start().await;
    Ok(())
}
