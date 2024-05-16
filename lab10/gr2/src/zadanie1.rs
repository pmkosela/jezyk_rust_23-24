#[derive(Clone, Copy)]
enum Month {
    Jan,
    Feb,
    Mar,
    Apr,
    May,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec
}

impl Month {
    fn from_u8(num: u8) -> Month {
        match num {
			1 => Month::Jan,
			2 => Month::Feb,
			3 => Month::Mar,
			4 => Month::Apr,
			5 => Month::May,
			6 => Month::Jun,
			7 => Month::Jul,
			8 => Month::Aug,
			9 => Month::Sep,
			10 => Month::Oct,
			11 => Month::Nov,
			12 => Month::Dec,
            _ => Month::Jan
        }
    }
}

struct Date {
    day: u8,
    month: Month,
    year: u16
}

use std::str::FromStr;
impl Date {
    fn from_3(day: u8, month: Month, year: u16) -> Date {
       Date{day, month, year}
    }

    fn from_string(string: &str, delim: char) -> Date {
        let string = string.to_string();
        let mut tokens = string.split(delim);
        let day = u8::from_str(tokens.next().unwrap()).expect("");
        let month = Month::from_u8(u8::from_str(tokens.next().unwrap()).expect(""));
        let year = u16::from_str(tokens.next().unwrap()).expect("");
        Date{day, month, year}
    }

    fn to_string(&self) -> String {
        let mut out = self.day.to_string();
        out.push('.');
        out.push_str((self.month as u8 + 1).to_string().as_str());
        out.push('.');
        out.push_str(self.year.to_string().as_str());
        out
    }
}

fn main() {
    let d1 = Date::from_3(5, Month::Oct, 2011);
    let d2 = Date::from_string("11.11.2011", '.');
    println!{"{}", d1.to_string()};
    println!{"{}", d2.to_string()};
}
