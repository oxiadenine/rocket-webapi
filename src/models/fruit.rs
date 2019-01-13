#[derive(Serialize, Deserialize)]
pub struct Fruit {
    pub id: Option<i32>,
    pub no: Option<String>,
    pub description: Option<String>
}
