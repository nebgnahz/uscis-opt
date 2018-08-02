use chrono::naive::{NaiveDate, NaiveDateTime};
use schema::records;

#[derive(Debug, Queryable, Insertable)]
#[table_name = "records"]
pub struct Record {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub update_date: NaiveDate,
    pub crawl_time: NaiveDateTime,
}
