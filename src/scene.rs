use crate::Response;

pub trait Scene {
    type Key;
    type Initialize;
    type Update;
    type Message;
    type Instruction;
    type Draw;
    type DrawBatch;

    fn initialize(&mut self, init: &mut Self::Initialize);

    fn update(
        &mut self,
        update: &Self::Update,
        delta: f64,
    ) -> Vec<Response<Self::Key, Self::Message, Self::Instruction>>;

    fn draw(&self, draw: &Self::Draw, interp: f64) -> Self::DrawBatch;

    fn receive_message(&mut self, message: &Self::Message);

    fn covering(&self) -> bool;

    fn blocking(&self) -> bool;
}
