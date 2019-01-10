#[derive(Serialize, Deserialize)]
pub struct Fruit {
    pub id: Option<i64>,
    pub no: Option<String>,
    pub description: Option<String>
}
