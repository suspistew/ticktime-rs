use ticktime::*;

fn main() {
    // Initialize a custom ticktime where one tick is 3600 seconds
    // where each day is 12 hours long
    // and year are composed of 4 months with 1 day
    let mut ticktime = TickTime::init(
        0, TickTimeOptions {
            tick_time_type:
            TickTimeType::Custom {
                seconds_per_tick: 3600,
                hours_in_a_day: 12,
                months_durations: vec![1, 1, 1, 1],
                season_duration: vec![4],
                week_duration: 7,
            },
            compute_events: false,
        },
    ).unwrap();

    // Calling tick to simulate 40 days
    for _ in 0..(24 * 40) {
        ticktime.tick();
    }

    println!("{}", ticktime.to_string()); // Year 20, Month 0, Day 0
}
