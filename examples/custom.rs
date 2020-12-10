use ticktime::*;

fn main() {
    // Initialize a custom ticktime where one tick is 3600 seconds
    // where each day is 12 hours long
    // and year are composed of 4 months with 1 day
    let mut ticktime = TickTime::init(
        0,TickTimeType::Custom {
            seconds_per_tick: 3600,
            hours_in_a_day: 12,
            months_durations: vec![1, 1, 1, 1],
            season_duration: vec![4]
        }
            ).unwrap();

    // Calling tick to simulate 40 days
    for _ in 0..(24*40) {
        ticktime.tick();
    }

    println!("Year: {} / Month : {} / Day: {}",
             ticktime.year(), ticktime.month(), ticktime.day()); // Year 20, Month 0, Day 0

}