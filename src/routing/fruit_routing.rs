use rocket_contrib::json::Json;
use crate::helpers::JsonResponse;
use crate::helpers::JsonResult;
use crate::models::Fruit;
use crate::entities::FruitEntity;
use crate::DbConnection;

#[get("/api/fruits")]
pub fn all(conn: DbConnection) -> Json<JsonResponse<Vec<Fruit>, & 'static str>> {
    let fruits = FruitEntity::all(&*conn)
        .into_iter()
        .map(|fe| Fruit {
            id: Some(fe.id),
            no: Some(fe.no),
            description: Some(fe.description)
        })
        .collect();

    let result = JsonResult::Success { data: fruits };

    Json(result.into_response())
}

#[get("/api/fruits/<id>")]
pub fn one(conn: DbConnection, id: i32) -> Json<JsonResponse<Fruit, & 'static str>> {
    let result = match FruitEntity::one(&*conn, id) {
        Some(fe) => {
            let fruit = Fruit {
                id: Some(fe.id),
                no: Some(fe.no),
                description: Some(fe.description)
            };

            JsonResult::Success { data: fruit }
        },
        None => JsonResult::Failure { error: "Fruit does not exists" }
    };

    Json(result.into_response())
}

#[post("/api/fruits", data = "<body>")]
pub fn new(conn: DbConnection, body: Json<Fruit>) -> Json<JsonResponse<Fruit, & 'static str>> {
    let fruit = body.into_inner();

    let fields = (fruit.no.unwrap(), fruit.description.unwrap());

    let result = match FruitEntity::insert(&*conn, fields) {
        Some(fe) => {
            let fruit = Fruit {
                id: Some(fe.id),
                no: Some(fe.no),
                description: Some(fe.description)
            };

            JsonResult::Success { data: fruit }
        },
        None => JsonResult::Failure { error: "Fruit already exists" }
    };

    Json(result.into_response())
}


#[put("/api/fruits/<id>", data = "<body>")]
pub fn edit(conn: DbConnection, id: i32, body: Json<Fruit>) -> Json<JsonResponse<Fruit, & 'static str>> {
    let fruit = body.into_inner();

    let fields = (id, fruit.description.unwrap());

    let result = match FruitEntity::update(&*conn, fields) {
        Some(fe) => {
            let fruit = Fruit {
                id: Some(fe.id),
                no: Some(fe.no),
                description: Some(fe.description)
            };

            JsonResult::Success { data: fruit }
        },
        None => JsonResult::Failure { error: "Fruit does not exists" }
    };

    Json(result.into_response())
}

#[delete("/api/fruits/<id>")]
pub fn delete(conn: DbConnection, id: i32) -> Json<JsonResponse<Fruit, & 'static str>> {
    let result = match FruitEntity::delete(&*conn, id) {
        Some(fe) => {
            let fruit = Fruit {
                id: Some(fe.id),
                no: Some(fe.no),
                description: Some(fe.description)
            };

            JsonResult::Success { data: fruit }
        },
        None => JsonResult::Failure { error: "Fruit does not exists" }
    };

    Json(result.into_response())
}
