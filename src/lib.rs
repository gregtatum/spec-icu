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

#[test]
fn test_favorite_locale() {
    Locale::from_bytes("de-u-ca-gregory-fw-mon-hc-h23-co-phonebk-ka-noignore-kb-false-kc-false-kf-false-kh-false-kk-false-kn-false-kr-space-ks-level1-kv-space-cf-standard-cu-eur-ms-metric-nu-latn-lb-strict-lw-normal-ss-none-tz-atvie-em-default-rg-atzzzz-sd-atat1-va-posix".as_bytes()).expect("Failed to create locale.");
}

#[test]
fn test_locales() {
    let locale = Locale::from_bytes("es-Latn-ES-valencia-u-ca-gregory".as_bytes())
        .expect("Failed to create locale.");

    assert_eq!("es", locale.id.language.as_str());
    assert_eq!("Latn", locale.id.script.expect("Expected script").as_str());
    assert_eq!("ES", locale.id.region.expect("Expected region").as_str());
    assert_eq!(
        "valencia",
        locale
            .id
            .variants
            .into_raw()
            .expect("Couldn't get variant")
            .get(0)
            .expect("The first variant was not there.")
            .as_str()
    );

    use icu::locid::extensions::unicode::{Key, Keywords, Value};

    let key = "ca".parse::<Key>().expect("Expected to parse a key");
    assert!(locale.extensions.unicode.keywords.contains_key(key));
    assert_eq!(
        &"gregory"
            .parse::<Value>()
            .expect("Expected to parse a value."),
        locale
            .extensions
            .unicode
            .keywords
            .get(key)
            .expect("Failed to get keyword")
    );
}

#[test]
fn test_locale_canonicalization() {
    let locale = Locale::from_bytes("ES_LATN_ES".as_bytes()).expect("Failed to create locale.");
    assert_eq!("es-Latn-ES", &format!("{}", locale));
}
