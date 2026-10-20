//! The menu bar's clock, as Mac OS 9 read the time: "3:07 PM".

/// A time of day to the minute.
///
/// Invariant: `hour < 24 && minute < 60`; [`ClockTime::new`] is the only constructor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClockTime {
    hour: u8,
    minute: u8,
}

impl ClockTime {
    /// Returns the time `hour`:`minute`, or nothing if either is out of range.
    pub fn new(hour: u32, minute: u32) -> Option<Self> {
        let hour = u8::try_from(hour).ok().filter(|&h| h < 24)?;
        let minute = u8::try_from(minute).ok().filter(|&m| m < 60)?;
        Some(Self { hour, minute })
    }

    /// Returns the time on a twelve-hour clock, as the menu bar shows it.
    pub fn label(self) -> String {
        let meridiem = if self.hour < 12 { "AM" } else { "PM" };
        let hour = match self.hour % 12 {
            0 => 12,
            hour => hour,
        };
        format!("{hour}:{minute:02} {meridiem}", minute = self.minute)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn label(hour: u32, minute: u32) -> Option<String> {
        ClockTime::new(hour, minute).map(ClockTime::label)
    }

    #[test]
    fn midnight_and_noon_read_twelve() {
        assert_eq!(label(0, 0).as_deref(), Some("12:00 AM"));
        assert_eq!(label(12, 0).as_deref(), Some("12:00 PM"));
    }

    #[test]
    fn minutes_are_two_digits_and_hours_are_not_padded() {
        assert_eq!(label(15, 7).as_deref(), Some("3:07 PM"));
        assert_eq!(label(9, 41).as_deref(), Some("9:41 AM"));
        assert_eq!(label(23, 59).as_deref(), Some("11:59 PM"));
    }

    #[test]
    fn out_of_range_times_do_not_exist() {
        assert_eq!(ClockTime::new(24, 0), None);
        assert_eq!(ClockTime::new(0, 60), None);
        assert_eq!(ClockTime::new(u32::MAX, 0), None);
    }
}
