use loco_rs::cli;
use migration::Migrator;
use recursive_furnace_reproduce::app::App;

#[tokio::main]
async fn main() -> loco_rs::Result<()> {
    cli::main::<App, Migrator>().await
}
