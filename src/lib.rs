use std::fmt;
use std::fmt::Formatter;
use crate::event::{TickTimeEvent, TicketTimeEventValue};

pub mod event;
mod lib_tests;

const LUNAR_MONTH_DURATION: usize = 30;
const LUNAR_YEAR_DURATION: usize = LUNAR_MONTH_DURATION * 12;

/// The way the in game datetime will be handled
#[derive(Clone, Debug)]
pub enum TickTimeType {
    /// The date and time is like on the planet earth (12 months, 24 hours a day, 60 minutes an hour, 60 seconds a minute)
    EarthLike {
        /// How much seconds represent a tick. Should be minimum 1.
        seconds_per_tick: usize,
        /// Which kind of calendar to compute values
        month_type: EarthLikeMonthType,
    },
    /// A configurable date and time type. An hour will still be 60 minutes and a minute 60 seconds.
    /// Note that sum of `season_duration` and `months_durations` must match to be consistent.
    Custom {
        /// How much seconds represent a tick. Should be minimum 1.
        seconds_per_tick: usize,
        /// The duration of a day
        hours_in_a_day: usize,
        /// A list of month durations.
        months_durations: Vec<usize>,
        /// A list of seasons durations.
        seasons_durations: Vec<usize>,
        /// duration of a single week.
        week_duration: usize,
    },
}

/// List of available month type for an Earth-like calendar
#[derive(Clone, Debug)]
pub enum EarthLikeMonthType {
    /// A simple mode where each month is 30 days long
    Lunar,
    /// A mode where real month duration will be computed as long as leap years
    Real,
}

/// Options to give to `TickTime` to enable/configure features
#[derive(Clone, Debug)]
pub struct TickTimeOptions {
    /// Type of time to use when computing values to display
    pub tick_time_type: TickTimeType,
    /// Flag to decide whether or not the tick() function compute and returns update events
    pub compute_events: bool,
}

#[derive(Clone, Debug, Default)]
struct TickTimeValue {
    /// Computed year, according to the tick_time_type
    year: usize,
    /// Computed season, according to the tick_time_type
    season: usize,
    /// Computed month, according to the tick_time_type
    month: usize,
    /// Computed week, according to the tick_time_type
    week: usize,
    /// Computed day, according to the tick_time_type
    day: usize,
    /// Computed hour, according to the tick_time_type
    hour: usize,
    /// Computed minute, according to the tick_time_type
    minute: usize,
    /// Computed second, according to the tick_time_type
    second: usize,
}

/// A `TickTime` helps to keep track of the current tick in the game.
/// Following a `TickTimeType`, it will translate the current tick to
/// a list of computed values, representing year, season, month...
#[derive(Clone, Debug)]
pub struct TickTime {
    /// Options to configure / enable / disable features from the computing step
    options: TickTimeOptions,
    /// Number of tick since the beginning of the game.
    current_tick: usize,
    /// Computed values from the tick method
    values: TickTimeValue,
    /// Last tick Computed values from the tick method
    old_values: TickTimeValue,
}

impl TickTime {
    /// Initialise a TickTime with a given tick (usefull to reload the state of a save) and
    /// a `TickTimeType`.
    pub fn init(current_tick: usize, options: TickTimeOptions) -> Result<Self, &'static str> {
        if let Err(e) = verify_tick_time_type_values(&options.tick_time_type) {
            return Err(e);
        }
        let mut tick_time = TickTime {
            current_tick,
            options,
            values: Default::default(),
            old_values: Default::default()
        };
        tick_time.apply_current_tick();
        Ok(tick_time)
    }

    /// Add a tick to the current_tick. Will also compute values
    pub fn tick(&mut self) -> Option<TickTimeEvent> {
        self.current_tick += 1;
        self.apply_current_tick();
        if self.options.compute_events {
            Some(self.compute_event())
        }else{
            None
        }
    }

    /// Return a tuple of computed usizes for (year, season, month, day, hour, minute, second)
    pub fn values(&self) -> (usize, usize, usize, usize, usize, usize, usize, usize) {
        (
            self.values.year,
            self.values.season,
            self.values.week,
            self.values.month,
            self.values.day,
            self.values.hour,
            self.values.minute,
            self.values.second,
        )
    }

    fn compute_event(&self) -> TickTimeEvent {
        let mut event = TickTimeEvent::default();
        let mut update_level = 0;

        if self.old_values.year != self.values.year {
            update_level += 1;
            event.year_update = Some(TicketTimeEventValue{ old_value: self.old_values.year, new_value: self.values.year });
        }

        if update_level > 0 || self.old_values.season != self.values.season {
            event.season_update = Some(TicketTimeEventValue{ old_value: self.old_values.season, new_value: self.values.season });
        }

        if update_level > 0 || self.old_values.week != self.values.week {
            event.week_update = Some(TicketTimeEventValue{ old_value: self.old_values.week, new_value: self.values.week });
        }

        if update_level > 0 || self.old_values.month != self.values.month {
            update_level += 1;
            event.month_update = Some(TicketTimeEventValue{ old_value: self.old_values.month, new_value: self.values.month });
        }

        if update_level > 0 || self.old_values.day != self.values.day {
            update_level += 1;
            event.day_update = Some(TicketTimeEventValue{ old_value: self.old_values.day, new_value: self.values.day });
        }

        if update_level > 0 || self.old_values.hour != self.values.hour {
            update_level += 1;
            event.hour_update = Some(TicketTimeEventValue{ old_value: self.old_values.hour, new_value: self.values.hour });
        }

        if update_level > 0 || self.old_values.minute != self.values.minute {
            update_level += 1;
            event.minute_update = Some(TicketTimeEventValue{ old_value: self.old_values.minute, new_value: self.values.minute });
        }

        if update_level > 0 || self.old_values.second != self.values.second {
            event.second_update = Some(TicketTimeEventValue{ old_value: self.old_values.second, new_value: self.values.second });
        }

        event
    }

    /// Total tick count
    pub fn current_tick(&self) -> usize {
        self.current_tick
    }

    /// Return the read only computed year
    pub fn year(&self) -> usize {
        self.values.year
    }

    /// Return the read only computed month
    pub fn month(&self) -> usize {
        self.values.month
    }

    /// Return the read only computed season
    pub fn season(&self) -> usize {
        self.values.season
    }

    /// Return the read only computed week
    pub fn week(&self) -> usize {
        self.values.week
    }

    /// Return the read only computed day
    pub fn day(&self) -> usize {
        self.values.day
    }

    /// Return the read only computed hour
    pub fn hour(&self) -> usize {
        self.values.hour
    }

    /// Return the read only computed minute
    pub fn minute(&self) -> usize {
        self.values.minute
    }

    /// Return the read only computed second
    pub fn second(&self) -> usize {
        self.values.second
    }

    fn apply_current_tick(&mut self) {
        if self.options.compute_events {
            self.old_values = self.values.clone();
        }
        match self.options.tick_time_type {
            TickTimeType::EarthLike { .. } => { self.compute_earthlike_time(); }
            TickTimeType::Custom { .. } => { self.compute_custom_date_time_values() }
        }
    }

    fn compute_earthlike_time(&mut self) {
        if let TickTimeType::EarthLike {
            seconds_per_tick,
            month_type,
        } = &self.options.tick_time_type
        {
            let total_seconds = self.current_tick * seconds_per_tick;
            self.values.second = total_seconds % 60;
            self.values.minute = (total_seconds / 60) % 60;
            self.values.hour = (total_seconds / 3600) % 24;
            let total_days = total_seconds / 86400;
            let (day, week, month, season, year) = match month_type {
                EarthLikeMonthType::Lunar => compute_lunar_calendar_value(total_days),
                EarthLikeMonthType::Real => compute_real_calendar_value(total_days)
            };
            self.values.day = day;
            self.values.month = month;
            self.values.week = week;
            self.values.season = season;
            self.values.year = year;
        }
    }

    fn compute_custom_date_time_values(&mut self) {
        if let TickTimeType::Custom {
            seconds_per_tick, hours_in_a_day, months_durations, seasons_durations, week_duration
        } = &self.options.tick_time_type
        {
            let total_seconds = self.current_tick * seconds_per_tick;
            self.values.second = total_seconds % 60;
            self.values.minute = (total_seconds / 60) % 60;
            self.values.hour = (total_seconds / 3600) % hours_in_a_day;
            let total_days = total_seconds / 3600 / hours_in_a_day;
            let year_duration: usize = months_durations.iter().sum();
            let (day, week, month, season, year) = {
                let (day, current_year) = (total_days % year_duration, total_days / year_duration);

                let (month, day_of_month) = find_correct_index_and_day_in_section(
                    day,
                    months_durations.len(),
                    months_durations,
                );

                let (season, _) = find_correct_index_and_day_in_section(
                    day,
                    seasons_durations.len(),
                    seasons_durations,
                );

                (day_of_month, day / week_duration, month, season % 4, current_year)
            };
            self.values.day = day;
            self.values.week = week;
            self.values.month = month;
            self.values.season = season;
            self.values.year = year;
        }
    }
}

impl fmt::Display for TickTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Tick time: [ Current tick: {}, Year: {}, Season: {}, Week: {} Month: {}, Day: {}, Hour: {}, Minute: {}, Second: {}]",
               self.current_tick, self.year(), self.season(), self.week(), self.month(), self.day(), self.hour(), self.minute(), self.second())
    }
}

fn compute_real_calendar_value(total_days: usize) -> (usize, usize, usize, usize, usize) {
    let (day, current_year, is_leap_year) =
        normalize_total_day_to_year_information(total_days);

    let (month, day_of_month) = find_correct_index_and_day_in_section(
        day,
        12,
        &get_month_duration(is_leap_year),
    );

    let (season, _) = find_correct_index_and_day_in_section(
        day,
        4,
        &get_season_duration(is_leap_year),
    );

    (day_of_month, day / 7, month, season % 4, current_year)
}

fn compute_lunar_calendar_value(total_days: usize) -> (usize, usize, usize, usize, usize) {
    (
        total_days % LUNAR_YEAR_DURATION % LUNAR_MONTH_DURATION,
        total_days % LUNAR_YEAR_DURATION / 7,
        total_days % LUNAR_YEAR_DURATION / LUNAR_MONTH_DURATION,
        (total_days % LUNAR_YEAR_DURATION) / (LUNAR_YEAR_DURATION / 4),
        total_days / LUNAR_YEAR_DURATION,
    )
}

fn get_month_duration(is_leap_year: bool) -> Vec<usize> {
    vec![31, if is_leap_year { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
}

fn get_season_duration(is_leap_year: bool) -> Vec<usize> {
    vec![if is_leap_year { 81 } else { 80 }, 92, 92, 91]
}

fn verify_tick_time_type_values(tick_time_type: &TickTimeType) -> Result<(), &'static str> {
    match tick_time_type {
        TickTimeType::EarthLike {
            seconds_per_tick, ..
        } => {
            if *seconds_per_tick == 0 {
                return Err("The minimum value for EarthLike::seconds_per_tick is 1");
            }
        }
        TickTimeType::Custom {
            seconds_per_tick, hours_in_a_day: _, months_durations, seasons_durations, ..
        } => {
            if *seconds_per_tick == 0 {
                return Err("The minimum value for Custom::seconds_per_tick is 1");
            }
            if months_durations.iter().sum::<usize>() != seasons_durations.iter().sum::<usize>() {
                return Err("The sum of values of Custom::months_durations and Custom::season_duration should be the same to keep consistent");
            }
        }
    }
    Ok(())
}

fn normalize_total_day_to_year_information(total_days: usize) -> (usize, usize, bool) {
    let base_4_year_days = total_days % 1461;
    let base_4_year_start = (total_days / 1461) * 4;
    match base_4_year_days {
        0..=365 => (base_4_year_days, base_4_year_start, true),
        366..=730 => (base_4_year_days - 366, base_4_year_start + 1, false),
        731..=1095 => (base_4_year_days - 731, base_4_year_start + 2, false),
        _ => (base_4_year_days - 1095, base_4_year_start + 3, false),
    }
}

fn find_correct_index_and_day_in_section(
    day: usize,
    max: usize,
    array: &Vec<usize>,
) -> (usize, usize) {
    let (mut day_counter, mut stop, mut index) = (day, false, 0);
    while !stop && index < max {
        let next_month_duration = array[index];
        if day_counter < next_month_duration {
            stop = true;
        } else {
            day_counter -= next_month_duration;
            index += 1;
        }
    }
    (index, day_counter)
}
