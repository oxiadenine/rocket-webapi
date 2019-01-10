use rocket_contrib::json::Json;
use crate::models::Fruit;
use crate::helpers::JsonResponse;
use crate::entities::FruitEntity;

#[get("/api/fruits")]
pub fn all() -> Json<JsonResponse<Vec<Fruit>, & 'static str>> {
    let fruits = FruitEntity::all()
        .into_iter()
        .map(|fe| Fruit {
            id: Some(fe.id),
            no: Some(fe.no),
            description: fe.description
        })
        .collect();

    let response = JsonResponse::success(fruits);

    Json(response)
}

#[get("/api/fruits/<id>")]
pub fn one(id: i64) -> Json<JsonResponse<Fruit, & 'static str>> {
    let response = match FruitEntity::one(id) {
        Some(fe) => {
            let fruit = Fruit {
                id: Some(fe.id),
                no: Some(fe.no),
                description: fe.description
            };

            JsonResponse::success(fruit)
        },
        None => JsonResponse::failure("Fruit does not exists")
    };

    Json(response)
}

#[post("/api/fruits", data = "<fruit>")]
pub fn new(fruit: Json<Fruit>) -> Json<JsonResponse<Fruit, & 'static str>> {
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

        JsonResponse::success(fruit)
    } else {
        JsonResponse::failure("Fruit already exists")
    };

    Json(response)
}

#[put("/api/fruits/<id>", data = "<fruit>")]
pub fn edit(id: i64, fruit: Json<Fruit>) -> Json<JsonResponse<Fruit, & 'static str>> {
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

            JsonResponse::success(fruit)
        },
        None => JsonResponse::failure("Fruit does not exists")
    };

    Json(response)
}

#[delete("/api/fruits/<id>")]
pub fn delete(id: i64) -> Json<JsonResponse<Fruit, & 'static str>> {
    let response = if FruitEntity::one(id).is_none() {
        JsonResponse::failure("Fruit does not exists")
    } else {
        let fruit_entity = FruitEntity::delete(id);

        let fruit = Fruit {
            id: Some(fruit_entity.id),
            no: Some(fruit_entity.no),
            description: fruit_entity.description
        };

        JsonResponse::success(fruit)
    };

    Json(response)
}
