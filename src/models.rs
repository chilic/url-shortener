use serde::{Deserialize, Serialize};
use diesel::{Insertable, Queryable};

use crate::schema::urls;

#[derive(Debug, Clone, Serialize, Queryable)]
pub struct Url {
    pub id: i32,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "urls"]
pub struct NewUrl {
    pub url: String,
}
