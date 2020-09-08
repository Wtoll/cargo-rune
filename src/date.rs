include!(concat!(env!("OUT_DIR"), "/date.rs"));

pub(crate) fn get() -> &'static str {
    DATE
}
