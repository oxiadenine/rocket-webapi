use std::sync::Mutex;

#[derive(Clone)]
pub struct FruitEntity {
    pub id: i64,
    pub no: String,
    pub description: Option<String>
}

lazy_static! {
    static ref FRUITS: Mutex<Vec<FruitEntity>> = Mutex::new(vec![]);
    static ref INDEX: Mutex<i64> = Mutex::new(1);
}

impl FruitEntity {
    pub fn all() -> Vec<FruitEntity> {
        FRUITS.lock().unwrap().clone()
    }

    pub fn find(no: String) -> Option<FruitEntity> {
        let fruit_entities = FRUITS.lock().unwrap();

        match fruit_entities.iter().find(|fe| fe.no == no) {
            Some(fe) => Some(fe.clone()),
            None => None
        }
    }

    pub fn one(id: i64) -> Option<FruitEntity> {
        let fruit_entities = FRUITS.lock().unwrap();

        match fruit_entities.iter().find(|fe| fe.id == id) {
            Some(fe) => Some(fe.clone()),
            None => None
        }
    }

    pub fn insert(mut fruit_entity: FruitEntity) -> FruitEntity {
        let mut fruit_entities = FRUITS.lock().unwrap();

        fruit_entity.id = *INDEX.lock().unwrap();

        fruit_entities.push(fruit_entity);

        *INDEX.lock().unwrap() += 1;

        fruit_entities.last().unwrap().clone()
    }

    pub fn update(fruit_entity: FruitEntity) -> FruitEntity {
        let mut fruit_entities = FRUITS.lock().unwrap();

        let old_fruit_entity = fruit_entities.iter_mut()
            .find(|fe| fe.id == fruit_entity.id).unwrap();

        old_fruit_entity.description = fruit_entity.description.clone();

        old_fruit_entity.clone()
    }

    pub fn delete(id: i64) -> FruitEntity {
        let mut fruit_entities = FRUITS.lock().unwrap();

        let fruit_entity = fruit_entities.iter()
            .find(|fe| fe.id == id).unwrap().clone();

        fruit_entities.retain(|fe| fe.id != id);

        fruit_entity
    }
}
