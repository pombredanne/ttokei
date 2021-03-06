
use diesel;
use diesel::pg::PgConnection;
use diesel::RunQueryDsl;
use diesel::result::QueryResult;

use super::model::*;

const MAX_PER_BULK_INSERT: usize = 5000;
const BULK_INSERT_CHUNK: usize = 1000;

pub fn create_parse<'a>(conn: &PgConnection, new_parse: NewParse<'a>) -> i32 {
    use super::schema::parses;

    diesel::insert_into(parses::table)
        .values(&new_parse)
        .returning(parses::parse_id)
        .get_result(conn)
        .expect("create_parse execute")
}

pub fn create_completed_parse(conn: &PgConnection, parse_id: i32) -> i32 {
    use super::schema::completed_parses;

    diesel::insert_into(completed_parses::table)
        .values(&NewCompletedParse { parse_id: parse_id })
        .returning(completed_parses::completed_parse_id)
        .get_result(conn)
        .expect("create_completed_parse execute")
}


pub fn create_language<'a>(conn: &PgConnection, new_language: NewLanguage<'a>) -> i64 {
    use super::schema::languages;

    diesel::insert_into(languages::table)
        .values(&new_language)
        .returning(languages::language_id)
        .get_result(conn)
        .expect("create_language execute")
}

pub fn create_language_stats<'a>(conn: &PgConnection,
                                 new_language_stats_values: Vec<NewLanguageStats<'a>>)
                                 -> i64 {
    use super::schema::language_stats;

    if new_language_stats_values.len() > MAX_PER_BULK_INSERT {
        let mut last_stats_id = 0;
        for stats_values in new_language_stats_values.chunks(BULK_INSERT_CHUNK) {
            last_stats_id = diesel::insert_into(language_stats::table)
                .values(stats_values)
                .returning(language_stats::language_stat_id)
                .get_result(conn)
                .expect("create_language_stat execute");
        }
        last_stats_id
    } else {
        diesel::insert_into(language_stats::table)
            .values(&new_language_stats_values)
            .returning(language_stats::language_stat_id)
            .get_result(conn)
            .expect("create_language_stat execute")
    }
}

pub fn create_git_repo<'a>(conn: &PgConnection, new_git_repo: NewGitRepo<'a>) -> QueryResult<i32> {
    use super::schema::git_repos;

    diesel::insert_into(git_repos::table)
        .values(&new_git_repo)
        .returning(git_repos::git_repo_id)
        .get_result(conn)
}

pub fn create_git_tags<'a>(conn: &PgConnection,
                           new_git_tags: Vec<NewGitTag<'a>>)
                           -> QueryResult<i64> {
    use super::schema::git_tags;

    diesel::insert_into(git_tags::table)
        .values(&new_git_tags)
        .returning(git_tags::git_tag_id)
        .get_result(conn)
}

pub fn create_git_commit<'a>(conn: &PgConnection, new_commits: Vec<NewGitCommit<'a>>) -> i64 {
    use super::schema::git_commits;

    if new_commits.len() > MAX_PER_BULK_INSERT {
        let mut last_id = 0;
        for values in new_commits.chunks(BULK_INSERT_CHUNK) {
            last_id = diesel::insert_into(git_commits::table)
                .values(values)
                .returning(git_commits::git_commit_id)
                .get_result(conn)
                .expect("create git commit");
        }
        last_id
    } else {
        diesel::insert_into(git_commits::table)
            .values(&new_commits)
            .returning(git_commits::git_commit_id)
            .get_result(conn)
            .expect("create git commit")
    }
}
