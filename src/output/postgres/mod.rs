
pub mod model;
pub mod schema;
pub mod insert;

use super::Outputter;

use self::model::{NewParse, NewLanguage, NewLanguageStats};

use tokei::{Languages, LanguageType, Language};
use diesel::pg::PgConnection;
use diesel::Connection;
use chrono::{DateTime, FixedOffset};

pub struct PgOutputter {
    conn: PgConnection,
}

impl PgOutputter {
    fn new(db_url: &str) -> PgOutputter {

        PgOutputter {
            conn: PgConnection::establish(&db_url)
                .expect(&format!("Error connecting to {}", db_url)),
        }
    }
}

impl Outputter for PgOutputter {
    fn output<'a>(&self,
                  languages: Languages,
                  time: &'a DateTime<FixedOffset>,
                  git_tag: Option<&'a str>) {
        let parse_id = insert::create_parse(&self.conn,
                                            NewParse {
                                                time: time,
                                                git_tag: git_tag,
                                            });

        let language_map = languages.remove_empty();

        for (name, language) in language_map {
            //print_language(language, name);
            let language_id = insert::create_language(&self.conn,
                                                      NewLanguage {
                                                          parse_id: parse_id as i32,
                                                          name: name.name(),
                                                          blanks: language.blanks as i64,
                                                          code: language.code as i64,
                                                          comments: language.comments as i64,
                                                          lines: language.lines as i64,
                                                          nested: language.nested,
                                                      });

            for stats in language.stats {
                let language_stats_id =
                    insert::create_language_stats(&self.conn,
                                                  NewLanguageStats {
                                                      language_id: language_id as i64,
                                                      name: stats.name
                                                          .to_str()
                                                          .expect("stats name is not utf8"),
                                                      blanks: stats.blanks as i64,
                                                      code: stats.code as i64,
                                                      comments: stats.comments as i64,
                                                      lines: stats.lines as i64,
                                                  });
                println!("inserted language stats {}", language_stats_id);
            }
            println!("inserted language: {}", language_id);
        }
        println!("inserted parse: {}", parse_id);
    }
}
