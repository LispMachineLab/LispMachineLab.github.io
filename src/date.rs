//! Days on the site's calendar: when a post or a clip was first published.

/// A day, as the site dates what it publishes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    /// The year.
    pub year: u16,
    /// The month, 1 to 12.
    pub month: u8,
    /// The day of the month.
    pub day: u8,
}

impl Date {
    /// Returns the date as ISO 8601 writes it, `2026-10-22`.
    pub fn iso(self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }

    /// Returns the date as a reader writes it, `22 October 2026`.
    pub fn long(self) -> String {
        format!("{} {} {}", self.day, month_name(self.month), self.year)
    }
}

/// Returns the name of the month numbered `month`, from 1, or nothing past 12.
const fn month_name(month: u8) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dates_read_both_ways() {
        let date = Date {
            year: 2026,
            month: 10,
            day: 2,
        };
        assert_eq!(date.iso(), "2026-10-02");
        assert_eq!(date.long(), "2 October 2026");
        assert_eq!(month_name(13), "");
    }
}
