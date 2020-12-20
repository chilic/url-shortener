use diesel::prelude::*;
use crate::models::{Url, NewUrl};
use crate::schema::urls::dsl::*;

/// Run query using Diesel to insert a new database row and return the result.
pub fn insert_new_url(
    url_param: &str,
    conn: &PgConnection,
) -> Result<Url, diesel::result::Error> {
    let new_url = NewUrl {
        url: url_param.to_owned(),
    };

    let inserted_id = diesel::insert_into(urls).values(&new_url)
        .returning(id)
        .get_result::<i32>(conn)?;

    let url_item = Url {
        id: inserted_id,
        url: new_url.url
    };

    Ok(url_item)
}

/// Run query using Diesel to find user by uid and return it.
pub fn find_url_by_id(
    id_param: i32,
    conn: &PgConnection,
) -> Result<Option<Url>, diesel::result::Error> {

    let url_item = urls
        .filter(id.eq(id_param))
        .first::<Url>(conn)
        .optional()?;

    Ok(url_item)
}
