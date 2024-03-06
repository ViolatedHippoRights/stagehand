use std::marker::PhantomData;

use stagehand::{
    draw::{Draw, DrawBatch, DrawData},
    input::{InputActions, InputMap},
    loading::{utility2d::StorageType, Ticket, TicketManager},
    scene::Scene,
};

const SPEED: f32 = 300.0;

struct Logo {
    pub position: (f32, f32),
    pub texture: Ticket,
}

struct PlayerControls {
    forward: usize,
    backward: usize,
    look: usize,
    pause: usize,
}

pub struct ExampleScene<C, I> {
    controls: PlayerControls,
    direction: (f32, f32),

    logo: Option<Logo>,

    phantom: PhantomData<(C, I)>,
}

impl<C, I> ExampleScene<C, I> {
    pub fn new() -> Self {
        ExampleScene {
            controls: PlayerControls {
                forward: usize::MAX,
                backward: usize::MAX,
                look: usize::MAX,
                pause: usize::MAX,
            },
            direction: (0.0, 0.0),

            logo: None,

            phantom: PhantomData,
        }
    }
}

impl<C, I> Scene for ExampleScene<C, I>
where
    C: TicketManager<StorageType, StorageType, String, str>,
{
    type Initialize = (InputMap<I>, C);
    type Update = Vec<InputActions>;
    type Draw = ();
    type UpdateBatch = ();
    type DrawBatch = DrawBatch<Draw, ()>;

    fn initialize(&mut self, init: &Self::Initialize) {
        self.controls.forward = init.0.users[0].get_index_by_key("Forward").unwrap();
        self.controls.backward = init.0.users[0].get_index_by_key("Backward").unwrap();
        self.controls.look = init.0.users[0].get_index_by_key("Look").unwrap();
        self.controls.pause = init.0.users[0].get_index_by_key("Pause").unwrap();

        self.logo = Some(Logo {
            texture: init
                .1
                .get_ticket_with_key(&StorageType::Texture, "Logo.png")
                .unwrap(),
            position: (400.0, 300.0),
        });
    }

    fn update(&mut self, update: &Self::Update, delta: f64) {
        let direction = (0.0, SPEED * delta as f32);

        if let Some(logo) = &mut self.logo {
            let position = logo.position;

            let x = update[0]
                .get_action_by_index(self.controls.forward)
                .unwrap();
            if x.is_down() {
                logo.position = (position.0 + direction.0, position.1 + direction.1);
            }

            if update[0]
                .get_action_by_index(self.controls.backward)
                .unwrap()
                .is_down()
            {
                logo.position = (position.0 - direction.0, position.1 - direction.1);
            }
        }
    }

    fn draw(&self, draw: &Self::Draw, interp: f64) -> Self::DrawBatch {
        let mut batch = DrawBatch::new(());

        if let Some(logo) = &self.logo {
            batch.instructions.push(Draw {
                ticket: logo.texture,
                data: DrawData::draw_rotated_at(logo.position.0, logo.position.1, 0.0, (0.5, 0.5)),
            });
        }

        batch
    }

    fn covering(&self) -> bool {
        return true;
    }

    fn blocking(&self) -> bool {
        return true;
    }
}
