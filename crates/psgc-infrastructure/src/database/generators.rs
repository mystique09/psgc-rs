use std::str::FromStr;

use chrono::Utc;
use psgc_domain::models::PaginateResult;
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

pub trait RBatisUuidExt {
    fn inner(&self) -> uuid::Uuid;
}

impl RBatisUuidExt for rbatis::rbdc::Uuid {
    fn inner(&self) -> uuid::Uuid {
        let uuid_str = &self.0;
        let uuid_val = uuid::Uuid::from_str(uuid_str).unwrap();
        uuid_val
    }
}

pub trait UuidExt {
    fn into_db(&self) -> rbatis::rbdc::Uuid;
}

impl UuidExt for uuid::Uuid {
    fn into_db(&self) -> rbatis::rbdc::Uuid {
        let uuid_str = self.to_string();
        let uuid = rbatis::rbdc::Uuid::from_str(&uuid_str).unwrap();
        uuid
    }
}

pub trait DateTimeUtcExt {
    fn inner(&self) -> chrono::DateTime<Utc>;
}

impl DateTimeUtcExt for rbatis::rbdc::DateTime {
    fn inner(&self) -> chrono::DateTime<Utc> {
        let datetime = &self.0.to_string();
        let chrono_datetime = chrono::DateTime::<Utc>::from_str(datetime).unwrap();

        chrono_datetime
    }
}

pub trait PageExt<T: Send + Sync> {
    fn into_domain<E: Send + Sync + From<T>>(self) -> PaginateResult<E>;
}

impl<T: Send + Sync> PageExt<T> for rbatis::plugin::Page<T> {
    fn into_domain<E: Send + Sync + From<T>>(self) -> PaginateResult<E> {
        PaginateResult {
            records: self.records.into_iter().map(T::into).collect::<Vec<E>>(),
            total: self.total,
            page_no: self.page_no,
            page_size: self.page_size,
        }
    }
}
