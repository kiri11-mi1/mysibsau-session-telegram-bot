use odoo_api::{client, jvec, OdooClient};
use std::env::var;

pub struct PalladaService {
    client: OdooClient<client::Authed, client::ReqwestAsync>,
}

impl PalladaService {
    pub async fn new() -> Self {
        let database = var("PALADA_DATABASE").expect("PALADA_DATABASE env doesnt exist");
        let login = var("PALADA_ADMIN").expect("PALADA_ADMIN env doesnt exist");
        let password = var("PALADA_PASSWORD").expect("PALADA_PASSWORD env doesnt exist");

        let url = format!("https://{}.pallada.sibsau.ru", database);

        let client = OdooClient::new_reqwest_async(&url)
            .expect("Error via creating odoo client")
            .authenticate(&database, &login, &password)
            .await
            .expect("Auth error");

        return PalladaService { client: client };
    }

    pub async fn find_group_by_name(&mut self, name: String) -> Option<i64> {
        let fields = vec![String::from("name"), String::from("id")];
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

        let groups = data.data;
        if groups.len() == 0 {
            log::info!("Empty result");
            return None;
        }

        let target_group = &groups[0];
        let target_id = &target_group["id"];
        return target_id.as_i64();
    }
}
