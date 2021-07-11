pub trait ToJson {
    type Json;

    fn get_json(&self) -> Self::Json;
}
