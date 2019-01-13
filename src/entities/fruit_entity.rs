use diesel::prelude::*;
use crate::schema::fruit;
use crate::schema::fruit::dsl::{fruit as fruits};

#[derive(Queryable, Insertable)]
#[table_name = "fruit"]
pub struct FruitEntity {
    pub id: i32,
    pub no: String,
    pub description: String
}

impl FruitEntity {
    pub fn all(conn: &SqliteConnection) -> Vec<FruitEntity> {
        fruits.load::<FruitEntity>(conn).unwrap()
    }

    pub fn one(conn: &SqliteConnection, id: i32) -> Option<FruitEntity> {
        match fruits.find(id).first(conn) {
            Ok(fe) => Some(fe),
            _ => None
        }
    }

    pub fn insert(conn: &SqliteConnection, fields: (String, String)) -> Option<FruitEntity> {
        let values = (fruit::no.eq(fields.0), fruit::description.eq(fields.1));

        let result = conn.transaction::<FruitEntity, _, _>(|| {
            diesel::insert_into(fruits)
                .values(&values)
                .execute(conn)
                .and_then(|_| { fruits.order(fruit::id.desc()).first(conn) })
        });

        match result {
            Ok(fe) => Some(fe),
            _ => None
        }
    }

    pub fn update(conn: &SqliteConnection, fields: (i32, String)) -> Option<FruitEntity> {
        let id = fields.0;

        let result = diesel::update(fruits.find(id))
            .set(fruit::description.eq(fields.1))
            .execute(conn)
            .and_then(|_| { fruits.find(id).first(conn) });

        match result {
            Ok(fe) => Some(fe),
            _ => None
        }
    }

    pub fn delete(conn: &SqliteConnection, id: i32) -> Option<FruitEntity> {
        let result = fruits.find(id)
            .first(conn)
            .and_then(|fe| {
                diesel::delete(fruits.filter(fruit::id.eq(id)))
                    .execute(conn);

                Ok(fe)
            });

        match result {
            Ok(fe) => Some(fe),
            _ => None
        }
    }
}
