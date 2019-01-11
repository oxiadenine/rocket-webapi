use rocket_contrib::json::Json;
use crate::models::Fruit;
use crate::helpers::JsonResponse;
use crate::helpers::Response;
use crate::entities::FruitEntity;

#[get("/api/fruits")]
pub fn all() -> Json<Response<Vec<Fruit>, & 'static str>> {
    let fruits = FruitEntity::all()
        .into_iter()
        .map(|fe| Fruit {
            id: Some(fe.id),
            no: Some(fe.no),
            description: fe.description
        })
        .collect();

    let response = JsonResponse::Success(fruits);

    Json(response.into_inner())
}

#[get("/api/fruits/<id>")]
pub fn one(id: i64) -> Json<Response<Fruit, & 'static str>> {
    let response = match FruitEntity::one(id) {
        Some(fe) => {
            let fruit = Fruit {
                id: Some(fe.id),
                no: Some(fe.no),
                description: fe.description
            };

            JsonResponse::Success(fruit)
        },
        None => JsonResponse::Failure("Fruit does not exists")
    };

    Json(response.into_inner())
}

#[post("/api/fruits", data = "<fruit>")]
pub fn new(fruit: Json<Fruit>) -> Json<Response<Fruit, & 'static str>> {
    let response = if FruitEntity::find(fruit.no.clone().unwrap()).is_none() {
        let mut fruit_entity = FruitEntity {
            id: 0,
            no: fruit.no.clone().unwrap(),
            description: fruit.description.clone()
        };

        fruit_entity = FruitEntity::insert(fruit_entity);

        let fruit = Fruit {
            id: Some(fruit_entity.id),
            no: Some(fruit_entity.no),
            description: fruit_entity.description
        };

        JsonResponse::Success(fruit)
    } else {
        JsonResponse::Failure("Fruit already exists")
    };

    Json(response.into_inner())
}

#[put("/api/fruits/<id>", data = "<fruit>")]
pub fn edit(id: i64, fruit: Json<Fruit>) -> Json<Response<Fruit, & 'static str>> {
    let fruit = fruit.into_inner();

    let response = match FruitEntity::one(id).and_then(|fe| {
        let fruit_entity = FruitEntity {
            id: fe.id,
            no: fe.no,
            description: fruit.description
        };

        Some(FruitEntity::update(fruit_entity))
    }) {
        Some(fe) => {
            let fruit = Fruit {
                id: Some(fe.id),
                no: Some(fe.no),
                description: fe.description
            };

            JsonResponse::Success(fruit)
        },
        None => JsonResponse::Failure("Fruit does not exists")
    };

    Json(response.into_inner())
}

#[delete("/api/fruits/<id>")]
pub fn delete(id: i64) -> Json<Response<Fruit, & 'static str>> {
    let response = if FruitEntity::one(id).is_none() {
        JsonResponse::Failure("Fruit does not exists")
    } else {
        let fruit_entity = FruitEntity::delete(id);

        let fruit = Fruit {
            id: Some(fruit_entity.id),
            no: Some(fruit_entity.no),
            description: fruit_entity.description
        };

        JsonResponse::Success(fruit)
    };

    Json(response.into_inner())
}
