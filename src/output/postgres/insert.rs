
use diesel;
use diesel::pg::PgConnection;
use diesel::RunQueryDsl;

use super::model::{NewParse, NewLanguage, NewLanguageStats};

pub fn create_parse<'a>(conn: &PgConnection, new_parse: NewParse<'a>) -> usize {
    use super::schema::parses;

    diesel::insert_into(parses::table)
        .values(&new_parse)
        .returning(parses::parse_id)
        .execute(conn)
        .expect("create_parse execute")
}

pub fn create_language<'a>(conn: &PgConnection, new_language: NewLanguage<'a>) -> usize {
    use super::schema::languages;

    diesel::insert_into(languages::table)
        .values(&new_language)
        .returning(languages::language_id)
        .execute(conn)
        .expect("create_language execute")
}

pub fn create_language_stats<'a>(conn: &PgConnection,
                                 new_language_stat: NewLanguageStats<'a>)
                                 -> usize {
    use super::schema::language_stats;

    diesel::insert_into(language_stats::table)
        .values(&new_language_stat)
        .returning(language_stats::language_stat_id)
        .execute(conn)
        .expect("create_language_stat execute")
}