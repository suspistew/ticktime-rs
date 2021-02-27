#[cfg(test)]
mod general_ticktime_tests {
    use crate::*;

    #[test]
    fn ticktime_with_0_tick_should_return_0_for_all_fields() {
        let tick_time = TickTime::init(
            0,
            TickTimeOptions {
                tick_time_type: TickTimeType::EarthLike {
                    seconds_per_tick: 1,
                    month_type: EarthLikeMonthType::Lunar,
                },
                compute_events: false,
            },
        )
            .unwrap();
        assert_eq!((0, 0, 0, 0, 0, 0, 0, 0), tick_time.values());
    }

    #[test]
    fn ticktime_tick_should_increment_tick_number_by_one() {
        let mut tick_time = TickTime::init(
            0,
            TickTimeOptions {
                tick_time_type:
                TickTimeType::EarthLike {
                    seconds_per_tick: 1,
                    month_type: EarthLikeMonthType::Lunar,
                },
                compute_events: false,
            },
        )
            .unwrap();
        tick_time.tick();
        assert_eq!(1, tick_time.current_tick());

        for _ in 0..123 {
            tick_time.tick();
        }
        assert_eq!(124, tick_time.current_tick());
    }
}

#[cfg(test)]
mod earthlike_ticktime_tests {
    use crate::*;

    #[test]
    fn init_earthlike_with_wrong_val_should_return_err() {
        let tick_time_result = TickTime::init(
            0,
            TickTimeOptions {
                tick_time_type:
                TickTimeType::EarthLike {
                    seconds_per_tick: 0,
                    month_type: EarthLikeMonthType::Lunar,
                },
                compute_events: false,
            }
            ,
        );
        assert_eq!(true, tick_time_result.is_err());
    }

    #[test]
    fn init_earthlike_with_correct_val_should_return_ok() {
        let tick_time_result = TickTime::init(
            0,
            TickTimeOptions {
                tick_time_type:
                TickTimeType::EarthLike {
                    seconds_per_tick: 1,
                    month_type: EarthLikeMonthType::Lunar,
                },
                compute_events: false,
            },
        );
        assert_eq!(true, tick_time_result.is_ok());
    }

    #[test]
    fn earthlike_ticktime_1_tick_should_compute_second_based_on_ticktime_type() {
        let mut tick_time = TickTime::init(
            0,
            TickTimeOptions {
                tick_time_type:
                TickTimeType::EarthLike {
                    seconds_per_tick: 1,
                    month_type: EarthLikeMonthType::Lunar,
                }
                ,
                compute_events: false,
            },
        )
            .unwrap();
        for _ in 0..10 {
            tick_time.tick();
        }
        assert_eq!((0, 0, 0, 0, 0, 0, 0, 10), tick_time.values());

        let mut tick_time = TickTime::init(
            0,
            TickTimeOptions {
                tick_time_type:
                TickTimeType::EarthLike {
                    seconds_per_tick: 10,
                    month_type: EarthLikeMonthType::Lunar,
                },
                compute_events: false,
            },
        )
            .unwrap();
        for _ in 0..4 {
            tick_time.tick();
        }
        assert_eq!((0, 0, 0,0, 0, 0, 0, 40), tick_time.values());
    }

    #[test]
    fn earthlike_ticktime_tick_should_compute_minute_based_on_ticktime_type() {
        let mut tick_time = TickTime::init(
            0,
            TickTimeOptions {
                tick_time_type:
                TickTimeType::EarthLike {
                    seconds_per_tick: 10,
                    month_type: EarthLikeMonthType::Lunar,
                },
                compute_events: false,
            },
        )
            .unwrap();
        for _ in 0..10 {
            tick_time.tick();
        }
        assert_eq!((0, 0, 0, 0, 0, 0, 1, 40), tick_time.values());

        let mut tick_time = TickTime::init(
            0,
            TickTimeOptions {
                tick_time_type:
                TickTimeType::EarthLike {
                    seconds_per_tick: 10,
                    month_type: EarthLikeMonthType::Lunar,
                },
                compute_events: false,
            },
        )
            .unwrap();
        for _ in 0..20 {
            tick_time.tick();
        }
        assert_eq!((0, 0, 0, 0, 0, 0, 3, 20), tick_time.values());
    }

    #[test]
    fn earthlike_ticktime_should_compute_hour_based_on_ticktime_type() {
        let mut tick_time = TickTime::init(
            0,
            TickTimeOptions {
                tick_time_type:
                TickTimeType::EarthLike {
                    seconds_per_tick: 60,
                    month_type: EarthLikeMonthType::Lunar,
                },
                compute_events: false,
            },
        )
            .unwrap();
        for _ in 0..61 {
            tick_time.tick();
        }
        assert_eq!((0, 0, 0, 0, 0, 1, 1, 0), tick_time.values());
    }

    // Lunar Month type tests
    #[test]
    fn earthlike_ticktime_should_compute_lunar_day_based_on_ticktime_type() {
        let mut tick_time = TickTime::init(
            0,
            TickTimeOptions {
                tick_time_type:
                TickTimeType::EarthLike {
                    seconds_per_tick: 3600,
                    month_type: EarthLikeMonthType::Lunar,
                },
                compute_events: false,
            },
        )
            .unwrap();
        for _ in 0..25 {
            tick_time.tick();
        }
        assert_eq!((0, 0, 0, 0, 1, 1, 0, 0), tick_time.values());
        let mut tick_time = TickTime::init(
            0,
            TickTimeOptions {
                tick_time_type:
                TickTimeType::EarthLike {
                    seconds_per_tick: 3600,
                    month_type: EarthLikeMonthType::Lunar,
                },
                compute_events: false,
            },
        )
            .unwrap();
        for _ in 0..50 {
            tick_time.tick();
        }
        assert_eq!((0, 0, 0, 0, 2, 2, 0, 0), tick_time.values());
    }

    #[test]
    fn earthlike_ticktime_should_compute_lunar_month_based_on_ticktime_type() {
        let mut tick_time = TickTime::init(
            0,
            TickTimeOptions {
                tick_time_type:
                TickTimeType::EarthLike {
                    seconds_per_tick: 3600 * 24,
                    month_type: EarthLikeMonthType::Lunar,
                },
                compute_events: false,
            },
        )
            .unwrap();
        for _ in 0..31 {
            tick_time.tick();
        }
        assert_eq!((0, 0, 4, 1, 1, 0, 0, 0), tick_time.values());
        let mut tick_time = TickTime::init(
            0,
            TickTimeOptions {
                tick_time_type:
                TickTimeType::EarthLike {
                    seconds_per_tick: 3600 * 24,
                    month_type: EarthLikeMonthType::Lunar,
                },
                compute_events: false,
            },
        )
            .unwrap();
        for _ in 0..61 {
            tick_time.tick();
        }
        assert_eq!((0, 0, 8, 2, 1, 0, 0, 0), tick_time.values());
    }

    #[test]
    fn earthlike_ticktime_should_compute_lunar_season_based_on_ticktime_type() {
        let mut tick_time = TickTime::init(
            0,
            TickTimeOptions {
                tick_time_type:
                TickTimeType::EarthLike {
                    seconds_per_tick: 3600 * 24,
                    month_type: EarthLikeMonthType::Lunar,
                },
                compute_events: false,
            },
        )
            .unwrap();
        for _ in 0..91 {
            tick_time.tick();
        }
        assert_eq!((0, 1, 13, 3, 1, 0, 0, 0), tick_time.values());
        let mut tick_time = TickTime::init(
            0,
            TickTimeOptions {
                tick_time_type:
                TickTimeType::EarthLike {
                    seconds_per_tick: 3600 * 24,
                    month_type: EarthLikeMonthType::Lunar,
                },
                compute_events: false,
            },
        )
            .unwrap();
        for _ in 0..181 {
            tick_time.tick();
        }
        assert_eq!((0, 2, 25, 6, 1, 0, 0, 0), tick_time.values());
    }

    #[test]
    fn earthlike_ticktime_should_compute_lunar_year_based_on_ticktime_type() {
        let mut tick_time = TickTime::init(
            0,
            TickTimeOptions {
                tick_time_type:
                TickTimeType::EarthLike {
                    seconds_per_tick: 3600 * 24,
                    month_type: EarthLikeMonthType::Lunar,
                },
                compute_events: false,
            },
        )
            .unwrap();
        for _ in 0..360 {
            tick_time.tick();
        }
        assert_eq!((1, 0, 0,0,  0, 0, 0, 0), tick_time.values());
        let mut tick_time = TickTime::init(
            0,
            TickTimeOptions {
                tick_time_type:
                TickTimeType::EarthLike {
                    seconds_per_tick: 3600 * 24,
                    month_type: EarthLikeMonthType::Lunar,
                },
                compute_events: false,
            },
        )
            .unwrap();
        for _ in 0..720 {
            tick_time.tick();
        }
        assert_eq!((2, 0, 0, 0, 0, 0, 0, 0), tick_time.values());
    }

    // Real month type tests
    #[test]
    fn earthlike_ticktime_should_compute_real_date_values_based_on_ticktime_type() {
        let mut tick_time = TickTime::init(
            0,
            TickTimeOptions {
                tick_time_type:
                TickTimeType::EarthLike {
                    seconds_per_tick: 3600 * 24,
                    month_type: EarthLikeMonthType::Real,
                },
                compute_events: false,
            },
        )
            .unwrap();
        for _ in 0..250 {
            tick_time.tick();
        }
        assert_eq!((0, 2, 35, 8, 6, 0, 0, 0), tick_time.values());

        let mut tick_time = TickTime::init(
            0,
            TickTimeOptions {
                tick_time_type:
                TickTimeType::EarthLike {
                    seconds_per_tick: 3600 * 24,
                    month_type: EarthLikeMonthType::Real,
                },
                compute_events: false,
            },
        )
            .unwrap();
        for _ in 0..(366 + 250) {
            tick_time.tick();
        }
        assert_eq!((1, 2, 35, 8, 7, 0, 0, 0), tick_time.values());
    }
}

#[cfg(test)]
mod custom_ticktime_tests {
    use crate::*;

    #[test]
    fn custom_ticktime_should_compute_year() {
        let mut tick_time = TickTime::init(
            0,
            TickTimeOptions {
                tick_time_type:
                TickTimeType::Custom {
                    seconds_per_tick: 3600 * 24,
                    hours_in_a_day: 24,
                    months_durations: vec![1, 1],
                    seasons_durations: vec![2],
                    week_duration: 7,
                },
                compute_events: false,
            },
        ).unwrap();

        for _ in 0..12 {
            tick_time.tick();
        }

        assert_eq!((6, 0, 0, 0, 0, 0, 0, 0), tick_time.values());
    }

    #[test]
    fn custom_ticktime_should_compute_month() {
        let mut tick_time = TickTime::init(
            0,
            TickTimeOptions {
                tick_time_type:
                TickTimeType::Custom {
                    seconds_per_tick: 3600 * 24,
                    hours_in_a_day: 24,
                    months_durations: vec![1, 1, 3, 1, 1],
                    seasons_durations: vec![7],
                    week_duration: 7,
                },
                compute_events: false,
            },
        ).unwrap();

        for _ in 0..12 {
            tick_time.tick();
        }

        assert_eq!((1, 0, 0, 3, 0, 0, 0, 0), tick_time.values());
    }
}

#[cfg(test)]
mod event_tests {

    use crate::*;

    #[test]
    fn should_not_return_event_when_event_computing_disabled() {
        let mut ticktime = TickTime::init(
            0, TickTimeOptions {
                tick_time_type: TickTimeType::EarthLike { seconds_per_tick: 123, month_type: EarthLikeMonthType::Real },
                compute_events: false,
            }).unwrap();

        if let Some(_) = ticktime.tick() {
            panic!("Ticktime must not return any event when event computing is disabled");
        }
    }

    #[test]
    fn should_return_event_when_event_computing_enable() {
        let mut ticktime = TickTime::init(
            0, TickTimeOptions {
                tick_time_type: TickTimeType::EarthLike { seconds_per_tick: 123, month_type: EarthLikeMonthType::Real },
                compute_events: true,
            }).unwrap();

        if let None = ticktime.tick() {
            panic!("Ticktime must return an event when event computing is enabled");
        }
    }

    #[test]
    fn should_compute_second_event() {
        let mut ticktime = TickTime::init(
            0, TickTimeOptions {
                tick_time_type: TickTimeType::EarthLike { seconds_per_tick: 1, month_type: EarthLikeMonthType::Real },
                compute_events: true,
            }).unwrap();

        if let Some(TickTimeEvent{second_update: Some(second_event), ..}) = ticktime.tick() {
            assert_eq!(0, second_event.old_value);
            assert_eq!(1, second_event.new_value);
        }else{
            panic!("No event returned but it should");
        }
    }

    #[test]
    fn should_compute_minute_event() {
        let mut ticktime = TickTime::init(
            0, TickTimeOptions {
                tick_time_type: TickTimeType::EarthLike { seconds_per_tick: 60, month_type: EarthLikeMonthType::Real },
                compute_events: true,
            }).unwrap();

        if let Some(TickTimeEvent{second_update: Some(second_event), minute_update: Some(minute_event), ..}) = ticktime.tick() {
            assert_eq!(0, second_event.old_value);
            assert_eq!(0, second_event.new_value);
            assert_eq!(0, minute_event.old_value);
            assert_eq!(1, minute_event.new_value);
        }else{
            panic!("No event returned but it should");
        }
    }

    #[test]
    fn should_compute_hour_event() {
        let mut ticktime = TickTime::init(
            0, TickTimeOptions {
                tick_time_type: TickTimeType::EarthLike { seconds_per_tick: 3600, month_type: EarthLikeMonthType::Real },
                compute_events: true,
            }).unwrap();

        if let Some(TickTimeEvent{  second_update: Some(second_event),
                                    minute_update: Some(minute_event),
                                    hour_update: Some(hour_event),
                        ..}) = ticktime.tick() {
            assert_eq!(0, second_event.old_value);
            assert_eq!(0, second_event.new_value);
            assert_eq!(0, minute_event.old_value);
            assert_eq!(0, minute_event.new_value);
            assert_eq!(0, hour_event.old_value);
            assert_eq!(1, hour_event.new_value);
        }else{
            panic!("No event returned but it should");
        }
    }

    #[test]
    fn should_compute_day_event() {
        let mut ticktime = TickTime::init(
            0, TickTimeOptions {
                tick_time_type: TickTimeType::EarthLike { seconds_per_tick: 3600 * 24, month_type: EarthLikeMonthType::Real },
                compute_events: true,
            }).unwrap();

        if let Some(TickTimeEvent{  second_update: Some(second_event),
                        minute_update: Some(minute_event),
                        hour_update: Some(hour_event),
                        day_update: Some(day_event),
                        ..}) = ticktime.tick() {
            assert_eq!(0, second_event.old_value);
            assert_eq!(0, second_event.new_value);
            assert_eq!(0, minute_event.old_value);
            assert_eq!(0, minute_event.new_value);
            assert_eq!(0, hour_event.old_value);
            assert_eq!(0, hour_event.new_value);
            assert_eq!(0, day_event.old_value);
            assert_eq!(1, day_event.new_value);
        }else{
            panic!("No event returned but it should");
        }
    }

    #[test]
    fn should_compute_week_event() {
        let mut ticktime = TickTime::init(
            0, TickTimeOptions {
                tick_time_type: TickTimeType::EarthLike { seconds_per_tick: 3600 * 24 * 7, month_type: EarthLikeMonthType::Real },
                compute_events: true,
            }).unwrap();

        if let Some(TickTimeEvent{
                        week_update: Some(week_event),
                        ..}) = ticktime.tick() {
            assert_eq!(0, week_event.old_value);
            assert_eq!(1, week_event.new_value);
        }else{
            panic!("No event returned but it should");
        }
    }

    #[test]
    fn should_compute_year_event() {
        let mut ticktime = TickTime::init(
            0, TickTimeOptions {
                tick_time_type: TickTimeType::EarthLike { seconds_per_tick: 3600 * 24 * 366, month_type: EarthLikeMonthType::Real },
                compute_events: true,
            }).unwrap();

        if let Some(TickTimeEvent{  second_update: Some(second_event),
                        minute_update: Some(minute_event),
                        hour_update: Some(hour_event),
                        day_update: Some(day_event),
                        month_update: Some(month_event),
                        season_update: Some(_season_event),
                        year_update: Some(year_event),
                        ..}) = ticktime.tick() {
            assert_eq!(0, second_event.old_value);
            assert_eq!(0, second_event.new_value);
            assert_eq!(0, minute_event.old_value);
            assert_eq!(0, minute_event.new_value);
            assert_eq!(0, hour_event.old_value);
            assert_eq!(0, hour_event.new_value);
            assert_eq!(0, day_event.old_value);
            assert_eq!(0, day_event.new_value);
            assert_eq!(0, month_event.old_value);
            assert_eq!(0, month_event.new_value);
            assert_eq!(0, year_event.old_value);
            assert_eq!(1, year_event.new_value);
        }else{
            panic!("No event returned but it should");
        }
    }

    #[test]
    fn should_compute_month_event() {
        let mut ticktime = TickTime::init(
            0, TickTimeOptions {
                tick_time_type: TickTimeType::EarthLike { seconds_per_tick: 3600 * 24 * 31, month_type: EarthLikeMonthType::Real },
                compute_events: true,
            }).unwrap();

        if let Some(TickTimeEvent{  second_update: Some(second_event),
                        minute_update: Some(minute_event),
                        hour_update: Some(hour_event),
                        day_update: Some(day_event),
                        month_update: Some(month_event),
                        season_update: None,
                        year_update: None,
                        ..}) = ticktime.tick() {
            assert_eq!(0, second_event.old_value);
            assert_eq!(0, second_event.new_value);
            assert_eq!(0, minute_event.old_value);
            assert_eq!(0, minute_event.new_value);
            assert_eq!(0, hour_event.old_value);
            assert_eq!(0, hour_event.new_value);
            assert_eq!(0, day_event.old_value);
            assert_eq!(0, day_event.new_value);
            assert_eq!(0, month_event.old_value);
            assert_eq!(1, month_event.new_value);
        }else{
            panic!("No event returned but it should");
        }
    }
}