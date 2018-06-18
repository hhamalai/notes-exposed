#![feature(plugin)]
#![plugin(rocket_codegen)]
use std::str;
use std::fs::File;
use std::io::{BufRead, BufReader};
use chrono::prelude::*;
use nom::*;
use std::collections::LinkedList;
use rocket_contrib::{Json};



#[macro_use] extern crate nom;
extern crate rocket;
extern crate rocket_contrib;
extern crate chrono;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;



#[get("/")]
fn index() -> Json<LinkedList<Link>>  {
    let file = File::open("file.txt");
    let result = BufReader::new(file.unwrap()).lines().map(|line| {
        let c = line.unwrap();
        match linkline(&c.as_bytes()[..]).unwrap() {
            (rest, date) if rest.starts_with(b"http") => Link { date: date, url: Some(str::from_utf8(rest).unwrap().clone().to_string()), comment: None },
            (rest, date) => Link { date: date, comment: Some(str::from_utf8(rest).unwrap().clone().to_string()), url: None }
        }
    }).collect::<LinkedList<_>>();
    return Json(result)
}

mod date_format {
    use chrono::{Date, Utc};
    use serde::Serializer;

    const FORMAT: &'static str = "%d.%m.%Y";

    pub fn serialize<S>(
        date: &Date<Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }
}

#[derive(Serialize, Debug)]
struct Link {
    #[serde(with = "date_format")]
    date: chrono::Date<Utc>,
    url: Option<String>,
    comment: Option<String>
}

named!(int32<&[u8], i32>, do_parse!(
    value: map_res!(digit, str::from_utf8) >>
    (value.parse::<i32>().unwrap())
));

named!(uint32<&[u8], u32>, do_parse!(
    value: map_res!(digit, str::from_utf8) >>
    (value.parse::<u32>().unwrap())
));

named!(date<&[u8], Date<Utc>>, do_parse!(
    day: uint32 >>
    tag!(".") >>
    month: uint32 >>
    tag!(".") >>
    year: int32 >>
    tag!(" ") >>
    (if year < 100 {Utc.ymd(year+2000, month, day)}
    else {Utc.ymd(year, month, day)})
));

named!(linkline <&[u8], Date<Utc>>, do_parse!(
    tag!("--") >>
    date: date >>
    (date)
));

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}

