# TickTime-rs
A utilities to convert tick number to a date time to use in game. 
It was first made to fulfill the need of city building games but can also be used in any game that needs date time handled from ticks.

## Features

- **Configurable calendar:**
  - *Earth-like real calendar:* Computed time will be done following the real earth calendar rules
  - *Earth-like lunar calendar:* Computed time will be done with a lunar calendar of 12 months of 30 days  
  - *Custom calendar:* Computed time will be be done by using custom unit time given at init. 
- **Update event:** Each tick will compute the time and, if enabled, will return an event with all the fields updated. 

## Examples 

### Earth-like lunar calendar

```rust
    // Initialize a lunar ticktime where one tick is 3600 seconds
    let mut ticktime = TickTime::init(
        0, TickTimeOptions {
            tick_time_type: TickTimeType::EarthLike { seconds_per_tick: 3600, month_type: EarthLikeMonthType::Lunar },
            compute_events: false,
        }
    ).unwrap();

    // Calling tick to simulate 40 days
    for _ in 0..(24*40) {
        ticktime.tick();
    }

    println!("{}", ticktime.to_string()); // Month 1, Day 10
``` 

### Earth-like real calendar

```rust
    // Initialize a real ticktime where one tick is 3600 seconds
    let mut ticktime = TickTime::init(
        0, TickTimeOptions {
            tick_time_type: TickTimeType::EarthLike { seconds_per_tick: 3600, month_type: EarthLikeMonthType::Real },
            compute_events: false,
            }
        ).unwrap();

    // Calling tick to simulate 40 days
    for _ in 0..(24*40) {
        ticktime.tick();
    }

    println!("{}", ticktime.to_string()); // Month 1, Day 9
``` 

### Custom calendar

```rust
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
            seasons_durations: vec![4],
            week_duration: 7,
        },
        compute_events: false,
        },
    ).unwrap();

    // Calling tick to simulate 40 days
    for _ in 0..(24*40) {
        ticktime.tick();
    }

    println!("{}", ticktime.to_string()); // Year 20, Month 0, Day 0
``` 

See the examples for more use cases.

# Contributing

As in any OS project, contributions are welcome. Issues or PR. Please check [the issues](https://github.com/grzi/ticktime-rs/issues)  to know if you can get involved in existing demands.