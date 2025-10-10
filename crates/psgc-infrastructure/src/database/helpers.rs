use std::collections::HashMap;

use rbatis::{executor::RBatisConnExecutor, rbdc::Uuid};

use crate::database::models::{
    city::City, municipality::Municipality, province::Province, region::Region,
};

pub async fn get_region_map(executor: &mut RBatisConnExecutor) -> HashMap<String, Uuid> {
    Region::select_all(executor)
        .await
        .unwrap()
        .iter()
        .fold(HashMap::new(), |mut map, r| {
            let id = r.id.clone();
            let code = r.correspondence_code.clone();
            let code = &code[0..2];

            map.insert(code.to_string(), id);
            map
        })
}

pub async fn get_province_map(executor: &mut RBatisConnExecutor) -> HashMap<String, Uuid> {
    Province::select_all(executor)
        .await
        .unwrap()
        .iter()
        .fold(HashMap::new(), |mut map, p| {
            let id = p.id.clone();
            let code = p.correspondence_code.clone();
            let code = &code[0..4];

            map.insert(code.to_string(), id);
            map
        })
}

pub async fn get_province_map_2(executor: &mut RBatisConnExecutor) -> HashMap<String, Uuid> {
    Province::select_all(executor)
        .await
        .unwrap()
        .iter()
        .fold(HashMap::new(), |mut map, p| {
            let id = p.id.clone();
            let code = p.code.clone();
            let code = &code[0..4];

            map.insert(code.to_string(), id);
            map
        })
}

pub async fn get_city_map(executor: &mut RBatisConnExecutor) -> HashMap<String, Uuid> {
    City::select_all(executor)
        .await
        .unwrap()
        .iter()
        .fold(HashMap::new(), |mut map, c| {
            let id = c.id.clone();
            let code = c.correspondence_code.clone();
            let code = &code[0..5];

            map.insert(code.to_string(), id);
            map
        })
}

pub async fn get_municipality_map(executor: &mut RBatisConnExecutor) -> HashMap<String, Uuid> {
    Municipality::select_all(executor)
        .await
        .unwrap()
        .iter()
        .fold(HashMap::new(), |mut map, c| {
            let id = c.id.clone();
            let code = c.correspondence_code.clone();
            let code = &code[0..8];

            map.insert(code.to_string(), id);
            map
        })
}
