#[cfg(test)]
mod general_ticktime_tests {
    use crate::*;

    #[test]
    fn ticktime_with_0_tick_should_return_0_for_all_fields() {
        let tick_time = TickTime::init(
            0,
            TickTimeType::EarthLike {
                seconds_per_tick: 1,
                month_type: EarthLikeCalendarMode::Lunar,
            },
        )
        .unwrap();
        assert_eq!((0, 0, 0, 0, 0, 0, 0), tick_time.values());
    }

    #[test]
    fn ticktime_tick_should_increment_tick_number_by_one() {
        let mut tick_time = TickTime::init(
            0,
            TickTimeType::EarthLike {
                seconds_per_tick: 1,
                month_type: EarthLikeCalendarMode::Lunar,
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
            TickTimeType::EarthLike {
                seconds_per_tick: 0,
                month_type: EarthLikeCalendarMode::Lunar,
            },
        );
        assert_eq!(true, tick_time_result.is_err());
    }

    #[test]
    fn init_earthlike_with_correct_val_should_return_ok() {
        let tick_time_result = TickTime::init(
            0,
            TickTimeType::EarthLike {
                seconds_per_tick: 1,
                month_type: EarthLikeCalendarMode::Lunar,
            },
        );
        assert_eq!(true, tick_time_result.is_ok());
    }

    #[test]
    fn earthlike_ticktime_1_tick_should_compute_second_based_on_ticktime_type() {
        let mut tick_time = TickTime::init(
            0,
            TickTimeType::EarthLike {
                seconds_per_tick: 1,
                month_type: EarthLikeCalendarMode::Lunar,
            },
        )
        .unwrap();
        for _ in 0..10 {
            tick_time.tick();
        }
        assert_eq!((0, 0, 0, 0, 0, 0, 10), tick_time.values());

        let mut tick_time = TickTime::init(
            0,
            TickTimeType::EarthLike {
                seconds_per_tick: 10,
                month_type: EarthLikeCalendarMode::Lunar,
            },
        )
        .unwrap();
        for _ in 0..4 {
            tick_time.tick();
        }
        assert_eq!((0, 0, 0, 0, 0, 0, 40), tick_time.values());
    }

    #[test]
    fn earthlike_ticktime_tick_should_compute_minute_based_on_ticktime_type() {
        let mut tick_time = TickTime::init(
            0,
            TickTimeType::EarthLike {
                seconds_per_tick: 10,
                month_type: EarthLikeCalendarMode::Lunar,
            },
        )
        .unwrap();
        for _ in 0..10 {
            tick_time.tick();
        }
        assert_eq!((0, 0, 0, 0, 0, 1, 40), tick_time.values());

        let mut tick_time = TickTime::init(
            0,
            TickTimeType::EarthLike {
                seconds_per_tick: 10,
                month_type: EarthLikeCalendarMode::Lunar,
            },
        )
        .unwrap();
        for _ in 0..20 {
            tick_time.tick();
        }
        assert_eq!((0, 0, 0, 0, 0, 3, 20), tick_time.values());
    }

    #[test]
    fn earthlike_ticktime_should_compute_hour_based_on_ticktime_type() {
        let mut tick_time = TickTime::init(
            0,
            TickTimeType::EarthLike {
                seconds_per_tick: 60,
                month_type: EarthLikeCalendarMode::Lunar,
            },
        )
        .unwrap();
        for _ in 0..61 {
            tick_time.tick();
        }
        assert_eq!((0, 0, 0, 0, 1, 1, 0), tick_time.values());
    }

    // Lunar Month type tests
    #[test]
    fn earthlike_ticktime_should_compute_lunar_day_based_on_ticktime_type() {
        let mut tick_time = TickTime::init(
            0,
            TickTimeType::EarthLike {
                seconds_per_tick: 3600,
                month_type: EarthLikeCalendarMode::Lunar,
            },
        )
        .unwrap();
        for _ in 0..25 {
            tick_time.tick();
        }
        assert_eq!((0, 0, 0, 1, 1, 0, 0), tick_time.values());
        let mut tick_time = TickTime::init(
            0,
            TickTimeType::EarthLike {
                seconds_per_tick: 3600,
                month_type: EarthLikeCalendarMode::Lunar,
            },
        )
        .unwrap();
        for _ in 0..50 {
            tick_time.tick();
        }
        assert_eq!((0, 0, 0, 2, 2, 0, 0), tick_time.values());
    }

    #[test]
    fn earthlike_ticktime_should_compute_lunar_month_based_on_ticktime_type() {
        let mut tick_time = TickTime::init(
            0,
            TickTimeType::EarthLike {
                seconds_per_tick: 3600 * 24,
                month_type: EarthLikeCalendarMode::Lunar,
            },
        )
        .unwrap();
        for _ in 0..31 {
            tick_time.tick();
        }
        assert_eq!((0, 0, 1, 1, 0, 0, 0), tick_time.values());
        let mut tick_time = TickTime::init(
            0,
            TickTimeType::EarthLike {
                seconds_per_tick: 3600 * 24,
                month_type: EarthLikeCalendarMode::Lunar,
            },
        )
        .unwrap();
        for _ in 0..61 {
            tick_time.tick();
        }
        assert_eq!((0, 0, 2, 1, 0, 0, 0), tick_time.values());
    }

    #[test]
    fn earthlike_ticktime_should_compute_lunar_season_based_on_ticktime_type() {
        let mut tick_time = TickTime::init(
            0,
            TickTimeType::EarthLike {
                seconds_per_tick: 3600 * 24,
                month_type: EarthLikeCalendarMode::Lunar,
            },
        )
        .unwrap();
        for _ in 0..91 {
            tick_time.tick();
        }
        assert_eq!((0, 1, 3, 1, 0, 0, 0), tick_time.values());
        let mut tick_time = TickTime::init(
            0,
            TickTimeType::EarthLike {
                seconds_per_tick: 3600 * 24,
                month_type: EarthLikeCalendarMode::Lunar,
            },
        )
        .unwrap();
        for _ in 0..181 {
            tick_time.tick();
        }
        assert_eq!((0, 2, 6, 1, 0, 0, 0), tick_time.values());
    }

    #[test]
    fn earthlike_ticktime_should_compute_lunar_year_based_on_ticktime_type() {
        let mut tick_time = TickTime::init(
            0,
            TickTimeType::EarthLike {
                seconds_per_tick: 3600 * 24,
                month_type: EarthLikeCalendarMode::Lunar,
            },
        )
        .unwrap();
        for _ in 0..360 {
            tick_time.tick();
        }
        assert_eq!((1, 0, 0, 0, 0, 0, 0), tick_time.values());
        let mut tick_time = TickTime::init(
            0,
            TickTimeType::EarthLike {
                seconds_per_tick: 3600 * 24,
                month_type: EarthLikeCalendarMode::Lunar,
            },
        )
        .unwrap();
        for _ in 0..720 {
            tick_time.tick();
        }
        assert_eq!((2, 0, 0, 0, 0, 0, 0), tick_time.values());
    }

    // Real month type tests
    #[test]
    fn earthlike_ticktime_should_compute_real_date_values_based_on_ticktime_type() {
        let mut tick_time = TickTime::init(
            0,
            TickTimeType::EarthLike {
                seconds_per_tick: 3600 * 24,
                month_type: EarthLikeCalendarMode::Real,
            },
        )
        .unwrap();
        for _ in 0..250 {
            tick_time.tick();
        }
        assert_eq!((0, 2, 8, 6, 0, 0, 0), tick_time.values());

        let mut tick_time = TickTime::init(
            0,
            TickTimeType::EarthLike {
                seconds_per_tick: 3600 * 24,
                month_type: EarthLikeCalendarMode::Real,
            },
        )
        .unwrap();
        for _ in 0..(366 + 250) {
            tick_time.tick();
        }
        assert_eq!((1, 2, 8, 7, 0, 0, 0), tick_time.values());
    }
}
