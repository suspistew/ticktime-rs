use ticktime::*;

fn main() {
    // Initialize a lunar ticktime where one tick is 3600 seconds
    let mut ticktime = TickTime::init(
        0, TickTimeOptions {
            tick_time_type: TickTimeType::EarthLike { seconds_per_tick: 3600, month_type: EarthLikeMonthType::Lunar },
            compute_events: true,
        }).unwrap();

    // Calling tick to simulate 29 days and 23 hours
    for _ in 0..((24 * 29) + 23) {
        ticktime.tick();
    }

    // calling tick one more time to get to the 30th day
    let day_31_event = ticktime.tick().unwrap();

    println!("{:?}", day_31_event );
    /*
    TickTimeEvent {
        second_update: Some(TicketTimeEventValue { old_value: 0, new_value: 0 }),
        minute_update: Some(TicketTimeEventValue { old_value: 0, new_value: 0 }),
        hour_update: Some(TicketTimeEventValue { old_value: 23, new_value: 0 }),
        day_update: Some(TicketTimeEventValue { old_value: 29, new_value: 0 }),
        week_update: None,
        month_update: Some(TicketTimeEventValue { old_value: 0, new_value: 1 }),
        season_update: None, year_update: None }
     */
}
