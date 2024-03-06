pub trait Scene {
    type Initialize;
    type Update;
    type UpdateBatch;
    type Draw;
    type DrawBatch;

    fn initialize(&mut self, init: &Self::Initialize);

    fn update(&mut self, update: &Self::Update, delta: f64) -> Self::UpdateBatch;

    fn draw(&self, draw: &Self::Draw, interp: f64) -> Self::DrawBatch;

    fn covering(&self) -> bool;

    fn blocking(&self) -> bool;
}
