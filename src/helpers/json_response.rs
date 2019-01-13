#[derive(Serialize)]
#[serde(untagged)]
pub enum JsonResult<T, E> {
    Success { data: T },
    Failure { error: E }
}

impl<T, E> JsonResult<T, E> {
    pub fn into_response(self) -> JsonResponse<T, E> {
        match self {
            JsonResult::Success { data: _} => JsonResponse { ok: true, result: self },
            JsonResult::Failure { error: _ } => JsonResponse { ok: false, result: self }
        }
    }
}

#[derive(Serialize)]
pub struct JsonResponse<T, E> {
    ok: bool,

    #[serde(flatten)]
    result: JsonResult<T, E>
}
