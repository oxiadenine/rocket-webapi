use rocket_contrib::json::Json;
use crate::helpers::JsonResponse;
use crate::helpers::Response;
use crate::models::Fruit;
use crate::entities::FruitEntity;
use crate::DbConnection;

#[get("/api/fruits")]
pub fn all(conn: DbConnection) -> Json<Response<Vec<Fruit>, & 'static str>> {
    let fruits = FruitEntity::all(&*conn)
        .into_iter()
        .map(|fe| Fruit {
            id: Some(fe.id),
            no: Some(fe.no),
            description: Some(fe.description)
        })
        .collect();

    let response = JsonResponse::Success(fruits);

    Json(response.into_inner())
}

#[get("/api/fruits/<id>")]
pub fn one(conn: DbConnection, id: i32) -> Json<Response<Fruit, & 'static str>> {
    let response = match FruitEntity::one(&*conn, id) {
        Some(fe) => {
            let fruit = Fruit {
                id: Some(fe.id),
                no: Some(fe.no),
                description: Some(fe.description)
            };

            JsonResponse::Success(fruit)
        },
        None => JsonResponse::Failure("Fruit does not exists")
    };

    Json(response.into_inner())
}

#[post("/api/fruits", data = "<body>")]
pub fn new(conn: DbConnection, body: Json<Fruit>) -> Json<Response<Fruit, & 'static str>> {
    let fruit = body.into_inner();

    let fields = (fruit.no.unwrap(), fruit.description.unwrap());

    let response = match FruitEntity::insert(&*conn, fields) {
        Some(fe) => {
            let fruit = Fruit {
                id: Some(fe.id),
                no: Some(fe.no),
                description: Some(fe.description)
            };

            JsonResponse::Success(fruit)
        },
        None => JsonResponse::Failure("Fruit already exists")
    };

    Json(response.into_inner())
}


#[put("/api/fruits/<id>", data = "<body>")]
pub fn edit(conn: DbConnection, id: i32, body: Json<Fruit>) -> Json<Response<Fruit, & 'static str>> {
    let fruit = body.into_inner();

    let fields = (id, fruit.description.unwrap());

    let response = match FruitEntity::update(&*conn, fields) {
        Some(fe) => {
            let fruit = Fruit {
                id: Some(fe.id),
                no: Some(fe.no),
                description: Some(fe.description)
            };

            JsonResponse::Success(fruit)
        },
        None => JsonResponse::Failure("Fruit does not exists")
    };

    Json(response.into_inner())
}

#[delete("/api/fruits/<id>")]
pub fn delete(conn: DbConnection, id: i32) -> Json<Response<Fruit, & 'static str>> {
    let response = match FruitEntity::delete(&*conn, id) {
        Some(fe) => {
            let fruit = Fruit {
                id: Some(fe.id),
                no: Some(fe.no),
                description: Some(fe.description)
            };

            JsonResponse::Success(fruit)
        },
        None => JsonResponse::Failure("Fruit does not exists")
    };

    Json(response.into_inner())
}
