use diesel::prelude::*;
use diesel_derive_enum::DbEnum;

#[derive(DbEnum, Debug, PartialEq, SqlType)]
pub enum NewsSourceType {
    Rss,
    Atom,
    Email,
}


#[derive(DbEnum, Debug, PartialEq, SqlType)]
pub enum EmailAuthMode {
    #[db_rename = "password plain"]
    PasswordPlain,
    #[db_rename = "password crypted"]
    PasswordEncrypted,
}

#[derive(DbEnum, Debug, PartialEq, SqlType)]
pub enum EmailConnectionMode {
    Insecure,
    #[db_rename = "SSL/TLS"]
    SslTls,
    #[db_rename = "STARTTLS"]
    StartTls,
}

table! {
    use super::NewsSourceTypeMapping;
    use diesel::sql_types::Varchar;
    use diesel::sql_types::Text;
    use diesel::sql_types::Integer;
    use diesel::sql_types::Timestamp;
    use diesel::sql_types::Nullable;
    news (id) {
        id -> Integer,
        guid -> Nullable<Varchar>,
        source_type -> Nullable<NewsSourceTypeMapping>,
        source_id -> Nullable<Integer>,
        insert_date -> Nullable<Timestamp>,
        title -> Nullable<Varchar>,
        content -> Nullable<Text>,
        link -> Nullable<Varchar>,
        color -> Nullable<Varchar>,
    }
}

table! {
    use diesel::sql_types::Varchar;
    use diesel::sql_types::Text;
    use diesel::sql_types::Integer;
    use diesel::sql_types::Nullable;
    source_atom (id) {
        id -> Integer,
        url -> Nullable<Text>,
        display_name -> Nullable<Varchar>,
        color -> Nullable<Varchar>,
    }
}

table! {
    use super::EmailConnectionModeMapping;
    use super::EmailAuthModeMapping;
    use diesel::sql_types::Varchar;
    use diesel::sql_types::Integer;
    use diesel::sql_types::Nullable;
    source_email (id) {
        id -> Integer,
        domain -> Nullable<Varchar>,
        username -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
        port -> Nullable<Integer>,
        connection_mode -> Nullable<EmailConnectionModeMapping>,
        auth_mode -> Nullable<EmailAuthModeMapping>,
        display_name -> Nullable<Varchar>,
        color -> Nullable<Varchar>,
    }
}

table! {
    use diesel::sql_types::Varchar;
    use diesel::sql_types::Text;
    use diesel::sql_types::Integer;
    use diesel::sql_types::Nullable;
    source_rss (id) {
        id -> Integer,
        url -> Nullable<Text>,
        display_name -> Nullable<Varchar>,
        color -> Nullable<Varchar>,
    }
}

allow_tables_to_appear_in_same_query!(news, source_atom, source_email, source_rss,);
