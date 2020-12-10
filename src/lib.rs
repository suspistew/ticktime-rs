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
        season_duration: Vec<usize>,
    },
}

#[derive(Clone, Debug)]
pub enum EarthLikeMonthType {
    /// A simple mode where each month is 30 days long
    Lunar,
    /// A mode where real month duration will be computed as long as leap years
    Real,
}

/// A `TickTime` helps to keep track of the current tick in the game.
/// Following a `TickTimeType`, it will translate the current tick to
/// a list of computed values, representing year, season, month...
#[derive(Clone, Debug)]
pub struct TickTime {
    /// Number of tick since the beginning of the game.
    current_tick: usize,
    /// Type of time to use when computing values to display
    tick_time_type: TickTimeType,
    /// Computed year, according to the tick_time_type
    year: usize,
    /// Computed season, according to the tick_time_type
    season: usize,
    /// Computed month, according to the tick_time_type
    month: usize,
    /// Computed day, according to the tick_time_type
    day: usize,
    /// Computed hour, according to the tick_time_type
    hour: usize,
    /// Computed minute, according to the tick_time_type
    minute: usize,
    /// Computed second, according to the tick_time_type
    second: usize,
}

impl TickTime {
    /// Initialise a TickTime with a given tick (usefull to reload the state of a save) and
    /// a `TickTimeType`.
    pub fn init(current_tick: usize, tick_time_type: TickTimeType) -> Result<Self, &'static str> {
        if let Err(e) = verify_tick_time_type_values(&tick_time_type) {
            return Err(e);
        }
        let mut tick_time = TickTime {
            current_tick,
            tick_time_type,
            year: 0,
            season: 0,
            month: 0,
            day: 0,
            hour: 0,
            minute: 0,
            second: 0,
        };
        tick_time.apply_current_tick();
        Ok(tick_time)
    }

    /// Add a tick to the current_tick. Will also compute values
    pub fn tick(&mut self) {
        self.current_tick += 1;
        self.apply_current_tick();
    }

    /// Return a tuple of computed usizes for (year, season, month, day, hour, minute, second)
    pub fn values(&self) -> (usize, usize, usize, usize, usize, usize, usize) {
        (
            self.year,
            self.season,
            self.month,
            self.day,
            self.hour,
            self.minute,
            self.second,
        )
    }

    /// Total tick count
    pub fn current_tick(&self) -> usize {
        self.current_tick
    }

    /// Return the read only computed year
    pub fn year(&self) -> usize {
        self.year
    }

    /// Return the read only computed month
    pub fn month(&self) -> usize {
        self.month
    }

    /// Return the read only computed day
    pub fn day(&self) -> usize {
        self.day
    }

    /// Return the read only computed hour
    pub fn hour(&self) -> usize {
        self.hour
    }

    /// Return the read only computed minute
    pub fn minute(&self) -> usize {
        self.minute
    }

    /// Return the read only computed second
    pub fn second(&self) -> usize {
        self.second
    }

    fn apply_current_tick(&mut self) {
        match self.tick_time_type{
            TickTimeType::EarthLike { .. } =>  { self.compute_earthlike_time(); },
            TickTimeType::Custom {.. } => { self.compute_custom_date_time_values() }
        }
    }

    fn compute_earthlike_time(&mut self) {
        if let TickTimeType::EarthLike {
            seconds_per_tick,
            month_type,
        } = &self.tick_time_type
        {
            let total_seconds = self.current_tick * seconds_per_tick;
            self.second = total_seconds % 60;
            self.minute = (total_seconds / 60) % 60;
            self.hour = (total_seconds / 3600) % 24;
            let total_days = total_seconds / 86400;
            let (day, month, season, year) = match month_type {
                EarthLikeMonthType::Lunar => compute_lunar_calendar_value(total_days),
                EarthLikeMonthType::Real => compute_real_calendar_value(total_days)
            };
            self.day = day;
            self.month = month;
            self.season = season;
            self.year = year;
        }
    }

    fn compute_custom_date_time_values(&mut self) {
        if let TickTimeType::Custom {
            seconds_per_tick, hours_in_a_day, months_durations, season_duration
        } = &self.tick_time_type
        {
            let total_seconds = self.current_tick * seconds_per_tick;
            self.second = total_seconds % 60;
            self.minute = (total_seconds / 60) % 60;
            self.hour = (total_seconds / 3600) % hours_in_a_day;
            let total_days = total_seconds / 3600 / hours_in_a_day;
            let year_duration: usize = months_durations.iter().sum();
            let (day, month, season, year) = {
                let (day, current_year) = (total_days % year_duration, total_days / year_duration);

                let (month, day_of_month) = find_correct_index_and_day_in_section(
                    day,
                    months_durations.len(),
                    months_durations,
                );

                let (season, _) = find_correct_index_and_day_in_section(
                    day,
                    season_duration.len(),
                    season_duration,
                );

                (day_of_month, month, season % 4, current_year)
            };
            self.day = day;
            self.month = month;
            self.season = season;
            self.year = year;
        }
    }
}

fn compute_real_calendar_value(total_days: usize) -> (usize, usize, usize, usize) {
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

    (day_of_month, month, season % 4, current_year)
}

fn compute_lunar_calendar_value(total_days: usize) -> (usize, usize, usize, usize) {
    (
        total_days % LUNAR_YEAR_DURATION % LUNAR_MONTH_DURATION,
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
        },
        TickTimeType::Custom {
            seconds_per_tick, hours_in_a_day: _, months_durations, season_duration
        } => {
            if *seconds_per_tick == 0 {
                return Err("The minimum value for Custom::seconds_per_tick is 1");
            }
            if months_durations.iter().sum::<usize>() != season_duration.iter().sum::<usize>() {
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
