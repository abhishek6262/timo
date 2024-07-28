use rusqlite::Connection;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./src/sql_migrations");
}

pub fn run_migrations(connection: &mut Connection) {
    embedded::migrations::runner().run(connection).unwrap();
}
