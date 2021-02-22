use ticktime::*;

fn main() {
    // Initialize a lunar ticktime where one tick is 3600 seconds
    let mut ticktime = TickTime::init(
        0,TickTimeType::EarthLike { seconds_per_tick: 3600, month_type: EarthLikeMonthType::Lunar }).unwrap();

    // Calling tick to simulate 40 days
    for _ in 0..(24*40) {
        ticktime.tick();
    }

    println!("{}", ticktime.to_string()); // Month 1, Day 10

}
