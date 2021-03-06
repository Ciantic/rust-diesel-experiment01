pub mod models;
pub mod repositories;
pub mod schema;


use diesel::{
    backend::Backend, deserialize, serialize, serialize::Output, types::FromSql,
    types::ToSql, AsExpression,
};
use std::io::Write;

macro_rules! generate_uuid_field {
    ( $name:ident ) => {
        #[derive(
            Debug,
            Clone,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            serde::Serialize,
            serde::Deserialize,
            AsExpression,
            FromSqlRow,
        )]
        #[sql_type = "diesel::sql_types::Text"]
        pub struct $name(uuid::Uuid);

        impl $name {
            pub fn generate() -> $name {
                $name(uuid::Uuid::new_v4())
            }

            pub fn new(uuid: uuid::Uuid) -> $name {
                $name(uuid)
            }
        }

        impl<DB> ToSql<diesel::sql_types::Text, DB> for $name
        where
            DB: Backend,
            String: ToSql<diesel::sql_types::Text, DB>,
        {
            fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
                <String as ToSql<diesel::sql_types::Text, DB>>::to_sql(
                    &self.0.to_hyphenated().to_string(),
                    out,
                )
            }
        }

        impl<DB> FromSql<diesel::sql_types::Text, DB> for $name
        where
            DB: Backend,
            String: FromSql<diesel::sql_types::Text, DB>,
        {
            fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
                let db_value_str =
                    <String as FromSql<diesel::sql_types::Text, DB>>::from_sql(bytes)?;
                let uuid_value = uuid::Uuid::parse_str(&db_value_str)?;
                Ok($name::new(uuid_value))
            }
        }

        // TODO: UUID FromSql, ToSql for other types, like binary or very long integer?
        // 1. Add diesel feature = "uuid",
        // 2. Create FromSql and ToSql for  diesel::pg::types::sql_types::Uuid
        //
        // More about: https://docs.diesel.rs/diesel/pg/types/sql_types/struct.Uuid.html
    };
}


generate_uuid_field! {
    ArticleId
} 

generate_uuid_field! { 
    ResourceId
}

generate_uuid_field! { 
    ImageId
}

generate_uuid_field! { 
    UrlId
}