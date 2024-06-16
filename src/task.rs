pub struct Task {
    pub id: usize,
    pub content: String,
}

impl Task {
    pub fn from(id: usize, content: String) -> Self {
        Self { id, content }
    }
}
