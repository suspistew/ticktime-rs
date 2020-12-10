use ticktime::*;

fn main() {
    // Initialize a real ticktime where one tick is 3600 seconds
    let mut ticktime = TickTime::init(
        0,TickTimeType::EarthLike { seconds_per_tick: 3600, month_type: EarthLikeMonthType::Real }).unwrap();

    // Calling tick to simulate 40 days
    for _ in 0..(24*40) {
        ticktime.tick();
    }

    println!("Month : {} / Day: {}", ticktime.month(), ticktime.day()); // Month 1, Day 9

}
