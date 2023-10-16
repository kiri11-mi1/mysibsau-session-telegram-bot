use crate::schemas::GroupList;
use chrono::Utc;
use odoo_api::{client, jvec, OdooClient};
use std::env::var;

pub struct PalladaService {
    client: OdooClient<client::Authed, client::ReqwestAsync>,
}

impl PalladaService {
    pub async fn new() -> Option<Self> {
        let database = var("PALADA_DATABASE").expect("PALADA_DATABASE env doesnt exist");
        let login = var("PALADA_ADMIN").expect("PALADA_ADMIN env doesnt exist");
        let password = var("PALADA_PASSWORD").expect("PALADA_PASSWORD env doesnt exist");

        let url = format!("https://{}.pallada.sibsau.ru", database);

        let client = OdooClient::new_reqwest_async(&url)
            .expect("Error via creating odoo client")
            .authenticate(&database, &login, &password)
            .await;

        if client.is_err() {
            log::error!("{:?}", client.err());
            return None;
        }
        let client_ok = client.ok()?;
        return Option::from(PalladaService { client: client_ok });
    }

    pub async fn find_group_by_name(&mut self, name: String) -> Option<i64> {
        let fields = vec!["name".to_string(), "id".to_string()];
        let domain = jvec![["name", "=", name.to_uppercase()]];

        let response = self
            .client
            .search_read("info.groups", domain, fields, None, None, None)
            .send()
            .await;

        if response.is_err() {
            log::error!("{:?}", response.err());
            return None;
        }
        let data = response.ok()?;

        let groups = GroupList::from(data.data);
        if groups.data.len() == 0 {
            log::info!("Empty result");
            return None;
        }
        return Option::from(groups.data[0].id);
    }

    pub async fn get_exams_timetable(&mut self, group_id: i64) -> Option<Vec<String>> {
        let fields = vec![
            "year".to_string(),
            "group".to_string(),
            "employee_name_init".to_string(),
            "lesson".to_string(),
            "place".to_string(),
            "day_week".to_string(),
            "time".to_string(),
            "date".to_string(),
        ];
        let now = Utc::now();
        let domain = jvec![["group", "=", group_id], ["date", "=>", now.to_string()]]; // TODO: подумать здесь
        let order = "date asc".to_string();

        let response = self
            .client
            .search_read(
                "info.timetable",
                domain,
                fields,
                None,
                None,
                Option::from(order),
            )
            .send()
            .await;

        if response.is_err() {
            log::error!("{:?}", response.err());
            return None;
        }
        let data = response.ok()?; // TODO: обернуть в структуру
        let mut result = vec![];
        for exam in data.data {
            result.push(format!(
                "Название: {} - Дата: {}\n",
                exam["lesson"].to_string(),
                exam["date"].to_string()
            ))
        }
        return Option::from(result);
    }
}
