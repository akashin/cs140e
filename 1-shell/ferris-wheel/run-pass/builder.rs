// FIXME: Make me pass! Diff budget: 30 lines.

use std::fmt;

#[derive(Default)]
struct Builder {
    string: Option<String>,
    number: Option<usize>,
}

impl Builder {
    fn string<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.string = Some(value.into());
        self
    }

    fn number(&mut self, value: usize) -> &mut Self {
        self.number = Some(value);
        self
    }

    fn to_string(&self) -> String {
        if let Some(ref string) = self.string {
            return string.clone();
        }
        return String::from("");
    }
}

impl fmt::Display for Builder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut vec = Vec::<String>::new();
        if let Some(ref string) = self.string {
            vec.push(string.to_string());
        }
        if let Some(number) = self.number {
            vec.push(number.to_string());
        }
        write!(f, "{}", vec.join(" "))
    }
}

// Do not modify this function.
fn main() {
    let empty = Builder::default().to_string();
    assert_eq!(empty, "");

    let just_str = Builder::default().string("hi").to_string();
    assert_eq!(just_str, "hi");

    let just_num = Builder::default().number(254).to_string();
    assert_eq!(just_num, "254");

    let a = Builder::default()
        .string("hello, world!")
        .number(200)
        .to_string();

    assert_eq!(a, "hello, world! 200");

    let b = Builder::default()
        .string("hello, world!")
        .number(200)
        .string("bye now!")
        .to_string();

    assert_eq!(b, "bye now! 200");

    let c = Builder::default()
        .string("heap!".to_owned())
        .to_string();

    assert_eq!(c, "heap!");
}
