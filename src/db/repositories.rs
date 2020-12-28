use diesel::SqliteConnection;

use super::models::Article;
use super::schema::articles::dsl::*;
use diesel::prelude::*;

impl Article {
    pub fn get_all(connection: &SqliteConnection) -> Vec<Article> {
        articles.limit(5).load::<Article>(connection).expect("Error loading posts")
    }

    pub fn save(&self, connection: &SqliteConnection) {
        // SQLite and MySQL
        diesel::replace_into(articles)
            .values(self)
            .execute(connection)
            .unwrap();

        // PG (and upcoming 1.4.6 Diesel release):
        // diesel::insert_into(articles)
        //     .values(self)
        //     .on_conflict(id)
        //     .do_update()
        //     .set(self)
        //     .execute(connection);
    }

    pub fn delete(&self, connection: &SqliteConnection) {
        diesel::delete(articles)
            .filter(id.eq(&self.id))
            .execute(connection)
            .unwrap();
    }
}
