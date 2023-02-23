// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "some_enum"))]
    pub struct SomeEnum;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "some_enum_2"))]
    pub struct SomeEnum2;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::SomeEnum;
    use super::sql_types::SomeEnum2;
    use super::sql_types::SomeEnum;
    use super::sql_types::SomeEnum2;

    resource (resource_id) {
        resource_id -> Int4,
        some_field -> SomeEnum,
        some_field_2 -> SomeEnum2,
    }
}
