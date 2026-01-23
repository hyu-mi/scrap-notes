use uuid::Uuid;

pub struct ExtendReport {
    pub inserted_count: usize,
    pub conflict_ids: Vec<Uuid>,
}

impl ExtendReport {
    pub fn new(inserted_count: usize, conflict_ids: Vec<Uuid>) -> Self {
        return Self {
            inserted_count,
            conflict_ids,
        };
    }
}
