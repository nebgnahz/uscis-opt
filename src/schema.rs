table! {
    records (id) {
        id -> Unsigned<Bigint>,
        title -> Varchar,
        description -> Text,
        update_date -> Date,
        crawl_time -> Datetime,
    }
}
