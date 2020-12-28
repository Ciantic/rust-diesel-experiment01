#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

mod db;

use crate::db::models::Article;
use diesel::{SqliteConnection};
use diesel_migrations::embed_migrations;

embed_migrations!("migrations");

fn main() {
   
    let conman = diesel::r2d2::ConnectionManager::<SqliteConnection>::new(".cache.db");
    let pool = r2d2::Pool::builder().max_size(15).build(conman).unwrap();
    // let conn = init_db();

    let thread = std::thread::spawn(move || {
        println!("OPEN THREAD");
        let conn = pool.get().unwrap();
        let _ = embedded_migrations::run_with_output(&conn, &mut std::io::stdout());
        Article::new().save(&conn);
        Article::new().save(&conn);
        let results2 = Article::get_all(&conn);
        println!("{:?}", results2);
    });

    thread.join().unwrap();

}
