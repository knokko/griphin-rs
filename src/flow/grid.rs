use crate::*;

#[derive(Clone)]
pub struct GridState {

    grid: Grid,
    id: u32
}

impl GridState {

    pub(super) fn new(id: u32, grid: &Grid) -> Self {
        Self { id, grid: grid.clone()}
    }
}

impl PartialEq for GridState {

    fn eq(&self, other: &GridState) -> bool {
        self.id == other.id
    }
}

impl Eq for GridState {}