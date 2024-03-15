use std::marker::PhantomData;

use crate::{
    draw::{Draw, DrawBatch, DrawColor, DrawData, DrawType},
    loading::{Ticket, TicketManager},
    scene::Scene,
    utility::{Initialize, StorageType, Update, UpdateInstruction},
    Response,
};

pub struct UIScene<I, S> {
    input: (String, String),
    font: Option<Ticket>,

    collision: u32,

    phantom: PhantomData<(I, S)>,
}

impl<I, S> UIScene<I, S> {
    pub fn new() -> Self {
        UIScene {
            input: (
                "W/Shift+Up/LMB/DPad Up to move forward".to_string(),
                "S/Shift+Down/RMB/DPad Down to move backward".to_string(),
            ),
            font: None,

            collision: 0,

            phantom: PhantomData,
        }
    }
}

impl<I, S> Scene for UIScene<I, S>
where
    S: TicketManager<StorageType, StorageType, String, str>,
{
    type Key = String;
    type Initialize = Initialize<I, S, ()>;
    type Update = Update<I, ()>;
    type Message = String;
    type Instruction = UpdateInstruction;
    type Draw = ();
    type DrawBatch = DrawBatch<Draw, ()>;

    fn initialize(&mut self, init: &mut Self::Initialize) {
        self.font = Some(
            init.storage
                .get_ticket_with_key(&StorageType::Font, "Napalm.ttf")
                .unwrap(),
        );
    }

    fn update(
        &mut self,
        _update: &Self::Update,
        _delta: f64,
    ) -> Vec<Response<Self::Key, Self::Message, Self::Instruction>> {
        Vec::new()
    }

    fn draw(&self, _draw: &Self::Draw, _interp: f64) -> Self::DrawBatch {
        let mut batch = DrawBatch::new(());
        let color = DrawColor {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 0.5,
        };

        match self.font {
            Some(f) => {
                batch.instructions.push(Draw {
                    ticket: f,
                    data: DrawData::draw_centered_at((super::WINDOW_WIDTH / 2) as f32, 100.0),
                    draw_type: DrawType::Text(self.input.0.clone(), color),
                });
                batch.instructions.push(Draw {
                    ticket: f,
                    data: DrawData::draw_centered_at((super::WINDOW_WIDTH / 2) as f32, 150.0),
                    draw_type: DrawType::Text(self.input.1.clone(), color),
                });
                batch.instructions.push(Draw {
                    ticket: f,
                    data: DrawData::draw_centered_at((super::WINDOW_WIDTH / 2) as f32, 550.0),
                    draw_type: DrawType::Text(
                        self.collision.to_string() + " frames bumping wall",
                        color,
                    ),
                });
            }
            None => {}
        };

        batch
    }

    fn receive_message(&mut self, message: &Self::Message) {
        match message.as_str() {
            "Collision" => self.collision += 1,
            _ => {}
        };
    }

    fn covering(&self) -> bool {
        return false;
    }

    fn blocking(&self) -> bool {
        return false;
    }
}
