use rbatis::rbdc::{DateTime, Uuid};

pub fn uuid_now() -> Uuid {
    let uuid_v7 = uuid::Uuid::now_v7();
    let id = Uuid(uuid_v7.to_string());

    id
}

pub fn datetime_utc_now() -> DateTime {
    let utc = DateTime::utc();

    utc
}
