#[get("/api")]
pub fn api() -> &'static str {
    "Welcome to Rocket Web API!"
}
