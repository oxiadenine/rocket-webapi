#[derive(Serialize)]
pub struct JsonResponse<T, E> {
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<E>
}

impl<T, E> JsonResponse<T, E> {
    pub fn success(data: T) -> JsonResponse<T, E> {
        JsonResponse { ok: true, data: Some(data), error: None }
    }

    pub fn failure(error: E) -> JsonResponse<T, E> {
        JsonResponse { ok: false, data: None, error: Some(error) }
    }
}
