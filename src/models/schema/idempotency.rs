#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "header_pair")]
pub struct HeaderPairRecord {
    pub name: String,
    pub value: Vec<u8>,
}


#[derive(
    //Default, Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize,
    Default, Debug, sqlx::FromRow
)]
pub struct Model {
    pub order_id: String,
    pub idempotency_key: String,
    pub response_status_code: i64,
    pub response_headers: Vec<HeaderPairRecord>,
    pub response_body: Vec<u8>,
    pub created_at: chrono::NaiveDateTime,
}

/*
   user_id uuid NOT NULL REFERENCES users(user_id),
   idempotency_key TEXT NOT NULL,
   response_status_code SMALLINT NOT NULL,
   response_headers header_pair[] NOT NULL,
   response_body BYTEA NOT NULL,
   created_at timestamptz NOT NULL,
   PRIMARY KEY(user_id, idempotency_key)
*/

/*
#[allow(unused)]
#[derive(Copy, Clone, Debug, Iden, derive_more::FromStr)]
pub enum Column {
    Id,
    Name,
    RecordType,
    RecordId,
    BlobId,
    CreatedAt,
}
impl Column {
    pub fn asterisk() -> &'static [Column] {
        &[
            Column::Id,
            Column::Name,
            Column::RecordType,
            Column::RecordId,
            Column::BlobId,
            Column::CreatedAt,
        ]
    }
}

impl super::CMSAlias for Column {
    fn alias(&self) -> &'static str {
        match self {
            Column::Id => "active_storage_attachments.id",
            Column::Name => "active_storage_attachments.name",
            Column::RecordType => "active_storage_attachments.record_type",
            Column::RecordId => "active_storage_attachments.record_id",
            Column::BlobId => "active_storage_attachments.blob_id",
            Column::CreatedAt => "active_storage_attachments.created_at",
        }
    }
}

impl super::CMSTableName for Column {
    type Output = TableName;
    fn table_name(&self) -> Self::Output {
        TableName
    }
}
*/
