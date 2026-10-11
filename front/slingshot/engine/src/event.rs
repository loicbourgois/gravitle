#[derive(Clone)]
pub struct Event {
    pub func: String,
    pub cell_id: usize,
    pub value: f32,
}
