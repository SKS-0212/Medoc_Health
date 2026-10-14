use crate::feature::player::Player;
use std::thread;
use std::time::{Duration, Instant};

pub fn initialize() {
    println!("Initializing ECHO game engine...");
}



pub fn run(){
    let mut player = Player::new();
    let fixed_dt = Duration::from_secs_f64(1.0 / 60.0);
    let target_frame_time = Duration::from_secs_f64(1.0 / 30.0);
    let mut accumulator = Duration::ZERO;
    let mut previous_frame = Instant::now();

    for frame in 1..=60{

        let frame_start = Instant::now();
        let elapsed = frame_start.duration_since(previous_frame);
        previous_frame = frame_start;
        accumulator +=elapsed;

        let mut updates_this_frame = 0;
        while accumulator >= fixed_dt{
            player.update(fixed_dt.as_secs_f32());
            updates_this_frame += 1;
            accumulator -=fixed_dt;
        }
        println!(
            "Frame {}: Updates = {}, Position = {:.2}",
            frame,
            updates_this_frame,
            player.x
        );
        let processing_time = frame_start.elapsed();
        if processing_time < target_frame_time{
            thread::sleep(target_frame_time - processing_time);
        }
    }

}



// pub fn run() {
//     let target_frame_time = Duration::from_secs_f64(1.0 / 30.0);
//     let game_start = Instant::now();
//     let mut slow_frame = 0;
//
//     let mut player = Player::new();
//     let mut previous_frame = Instant::now();
//
//     for frame in 1..=60 {
//         let frame_start = Instant::now();
//
//         let dt = frame_start.duration_since(previous_frame).as_secs_f32();
//
//         previous_frame = frame_start;
//
//         player.update(dt);
//         println!("Player: {player:?}");
//         let elapsed = frame_start.elapsed();
//
//         if elapsed < target_frame_time {
//             thread::sleep(target_frame_time - elapsed);
//         } else {
//             slow_frame += 1;
//         }
//     }
//     println!("Game loop finished");
//     println!(
//         "Total execution time: {:.2} seconds",
//         game_start.elapsed().as_secs_f64()
//     );
//     println!("Total number of slow frames: {}", slow_frame);
// }
