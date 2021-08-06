#![allow(unused_imports)]
use icu::datetime::{mock::datetime::MockDateTime, options::length, DateTimeFormat};
use icu::locid::macros::langid;
use icu::locid::Locale;
use icu_provider_fs::FsDataProvider;

#[test]
fn test_icu() {
    let loc: Locale = langid!("pl").into();

    let date: MockDateTime = "2020-10-14T13:21:00"
        .parse()
        .expect("Failed to parse a datetime.");

    let provider = FsDataProvider::try_new("./data").expect("Failed to initialize Data Provider.");

    let options = length::Bag {
        time: Some(length::Time::Medium),
        date: Some(length::Date::Long),
        ..Default::default()
    }
    .into();

    let dtf = DateTimeFormat::try_new(loc, &provider, &options)
        .expect("Failed to initialize DateTimeFormat");

    let formatted_date: String = format!("{}", dtf.format(&date));
    assert_eq!(formatted_date, "14 października 2020 13:21:00");
}
