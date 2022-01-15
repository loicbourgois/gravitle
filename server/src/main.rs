mod data;
mod maths;
mod part;
mod point;
mod test2;
mod websocket;
mod websocket_async;
type Float = f32;
type Depth = u8;
type CellId = usize;
type PartId = usize;
use std::io::Error;
#[tokio::main]
async fn main() -> Result<(), Error> {
    let _ = env_logger::try_init();
    test2::main().await;
    Ok(())
}
