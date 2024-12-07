#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "header_pair")]
pub struct HeaderPairRecord {
    pub name: String,
    pub value: Vec<u8>,
}

//impl sqlx::postgres::PgHasArrayType for HeaderPairRecord {
//    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
//        sqlx::postgres::PgTypeInfo::with_name("_header_pair")
//    }
//}

#[derive(Default, Debug, sqlx::FromRow)]
pub struct Model {
    pub idempotency_key: String,
    pub response_status_code: i16,
    pub response_headers: Vec<HeaderPairRecord>,
    pub response_body: Vec<u8>,
    pub created_at: chrono::NaiveDateTime,
}
