pub struct Stat {
    time: Duration,
    scramble: &str,
    datetime: Instant,
}

impl Stat {
    pub fn new(time: Duration, scramble: &str, datetime: Instant) -> Stat {
        Stat {
            time,
            scramble,
            datetime,
        }
    }
}
