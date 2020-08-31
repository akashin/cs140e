// FIXME: Make me pass! Diff budget: 25 lines.

#[derive(Debug)]
enum Duration {
    MilliSeconds(u64),
    Seconds(u32),
    Minutes(u16)
}

impl Duration {
    fn to_milliseconds(&self) -> u64 {
        match self {
            Duration::MilliSeconds(value) => *value,
            Duration::Seconds(value) => (*value as u64) * 1000,
            Duration::Minutes(value) => (*value as u64) * 1000 * 60,
        }
    }
}

impl PartialEq for Duration {
    fn eq(&self, other: &Self) -> bool {
        self.to_milliseconds() == other.to_milliseconds()
    }
}

impl Eq for Duration { }

fn main() {
    assert_eq!(Duration::Seconds(120), Duration::Minutes(2));
    assert_eq!(Duration::Seconds(420), Duration::Minutes(7));
    assert_eq!(Duration::MilliSeconds(420000), Duration::Minutes(7));
    assert_eq!(Duration::MilliSeconds(43000), Duration::Seconds(43));
}
