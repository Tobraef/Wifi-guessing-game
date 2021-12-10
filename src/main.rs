pub mod communication;
pub mod game;
pub mod preparation;
pub mod test;
pub mod message;
pub mod error_type;
pub mod db;
pub mod randomizer;

pub use message::Message;
pub use error_type::ErrorType;

fn main() {
    for ip in communication::pinger::fetch_local_ips() {
        println!("FOUND {}", ip);
    }
    println!("thats it")
    // let initation = preparation::game_prepare::init(&mut communication::ManualInput::default()).expect("Couldn't init");
    // let ctx = db::opentdb::opentdb_ctx::OpentdbCtx::new();
    // println!("Beginning loop");
    // let mut game_loop = game::GameLoop::new(initation, ctx, 10_000);
    // game_loop.main_loop();
}
