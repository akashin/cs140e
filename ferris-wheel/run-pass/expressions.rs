// FIXME: Make me pass! Diff budget: 10 lines.
// Do not `use` any items.

// Do not change the following two lines.
#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
struct IntWrapper(isize);

impl Eq for IntWrapper { }

impl Ord for IntWrapper {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(&other).expect("comparison of integers must be total")
    }
}

pub fn main() {
    assert_eq!(std::cmp::max(1usize, 3), 3);
    assert_eq!(std::cmp::max(1u8, 3), 3);
    assert_eq!(std::cmp::max(1u8, 3), 3);
    assert_eq!(std::cmp::max(IntWrapper(120), IntWrapper(248)), IntWrapper(248));
}
