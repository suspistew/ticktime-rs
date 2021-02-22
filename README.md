# TickTime-rs
A basic utilities to convert tick number to a date time to use in game. 
It was first made to fulfill the need of city building games.

It is divided into three main calendar types, an Earthlike lunar calendar, Earthlike real calendar and Custom calendar where you decide the value of each units. 

## Examples 

### Earthlike lunar calendar

```rust
    // Initialize a lunar ticktime where one tick is 3600 seconds
    let mut ticktime = TickTime::init(
        0,TickTimeType::EarthLike { seconds_per_tick: 3600, month_type: EarthLikeMonthType::Lunar }).unwrap();

    // Calling tick to simulate 40 days
    for _ in 0..(24*40) {
        ticktime.tick();
    }

    println!("Month : {} / Day: {}", ticktime.month(), ticktime.day()); // Month 1, Day 10
``` 

### Earthlike real calendar

```rust
    // Initialize a real ticktime where one tick is 3600 seconds
    let mut ticktime = TickTime::init(
        0,TickTimeType::EarthLike { seconds_per_tick: 3600, month_type: EarthLikeMonthType::Real }).unwrap();

    // Calling tick to simulate 40 days
    for _ in 0..(24*40) {
        ticktime.tick();
    }

    println!("Month : {} / Day: {}", ticktime.month(), ticktime.day()); // Month 1, Day 9
``` 

### Custom calendar

```rust
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
``` 
