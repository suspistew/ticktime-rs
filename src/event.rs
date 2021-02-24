/// Contains all the updated values during this tick. Usefull to create an event oriented
/// system.
#[derive(Default, Debug)]
pub struct TickTimeEvent {
    /// Some(val) if computed second has been updated, None otherwise.
    /// Note : old value and new value can be the same if the tick is 60 seconds
    pub second_update: Option<TicketTimeEventValue>,
    /// Some(val) if computed minute has been updated, None otherwise.
    /// Note : old value and new value can be the same if the tick is n hour(s)
    pub minute_update: Option<TicketTimeEventValue>,
    /// Some(val) if computed hour has been updated, None otherwise.
    /// Note : old value and new value can be the same if the tick is n day(s)
    pub hour_update: Option<TicketTimeEventValue>,
    /// Some(val) if computed day has been updated, None otherwise.
    /// Note : old value and new value can be the same if the tick is n month or n year
    pub day_update: Option<TicketTimeEventValue>,
    /// Some(val) if computed week has been updated, None otherwise.
    /// Note : old value and new value can be the same if the tick is n year
    pub week_update: Option<TicketTimeEventValue>,
    /// Some(val) if computed month has been updated, None otherwise.
    /// Note : old value and new value can be the same if the tick is n year
    pub month_update: Option<TicketTimeEventValue>,
    /// Some(val) if computed season has been updated, None otherwise.
    /// Note : old value and new value can be the same if the tick is n year
    pub season_update: Option<TicketTimeEventValue>,
    /// Some(val) if  computed year has been updated, None otherwise.
    pub year_update: Option<TicketTimeEventValue>,
}

/// Represents the computed values before and after the current triggered event
#[derive(Debug)]
pub struct TicketTimeEventValue {
    /// The value before the tick is updated
    pub old_value: usize,
    /// The value after the tick is updated
    pub new_value: usize
}