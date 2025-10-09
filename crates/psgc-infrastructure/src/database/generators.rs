use rbatis::rbdc::Uuid;

pub fn uuid_now() -> Uuid {
    let uuid_v7 = uuid::Uuid::now_v7();
    let id = rbatis::rbdc::Uuid(uuid_v7.to_string());

    id
}
