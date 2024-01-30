use std::env;

pub fn set_env_vars() {
    let db_path = env::var("DUPS_DB");
    if db_path.is_err() {
        env::set_var("DUPS_DB", "/home/pipi/insertdb/dups.db");
    };
    let unable_to_open = env::var("UNABLE_TO_OPEN");
    if unable_to_open.is_err() {
        env::set_var("UNABLE_TO_OPEN", "/home/pipi/insertdb/unable_to_open/");
    };
}
