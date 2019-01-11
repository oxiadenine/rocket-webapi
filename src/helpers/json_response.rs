pub enum JsonResponse<T, E> {
    Success(T),
    Failure(E)
}

impl<T, E> JsonResponse<T, E> {
    pub fn into_inner(self) -> Response<T, E> {
        match self {
            JsonResponse::Success(data) => Response {
                ok: true,
                data: Some(data),
                error: None
            },
            JsonResponse::Failure(error) => Response {
                ok: false,
                data: None,
                error: Some(error)
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Response<T, E> {
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<E>
}
