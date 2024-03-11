use std::marker::PhantomData;

use crate::{
    draw::{Draw, DrawBatch, DrawData},
    loading::{Ticket, TicketManager},
    scene::Scene,
    utility2d::{Initialize, StorageType, Update, UpdateAction, UpdateInfo},
};

pub const WINDOW_WIDTH: u32 = 800;
pub const WINDOW_HEIGHT: u32 = 600;

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

    music: Option<Ticket>,
    oob: Option<Ticket>,

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

            music: None,
            oob: None,

            phantom: PhantomData,
        }
    }
}

impl<C, I> Scene for ExampleScene<C, I>
where
    C: TicketManager<StorageType, StorageType, String, str>,
{
    type Initialize = Initialize<I, C>;
    type Update = Update<I>;
    type Draw = ();
    type UpdateBatch = Vec<UpdateAction>;
    type DrawBatch = DrawBatch<Draw, ()>;

    fn initialize(&mut self, init: &mut Self::Initialize) {
        self.controls.forward = init.input.users[0].get_index_by_key("Forward").unwrap();
        self.controls.backward = init.input.users[0].get_index_by_key("Backward").unwrap();
        self.controls.look = init.input.users[0].get_index_by_key("Look").unwrap();
        self.controls.pause = init.input.users[0].get_index_by_key("Pause").unwrap();

        self.logo = Some(Logo {
            texture: init
                .content
                .get_ticket_with_key(&StorageType::Texture, "Logo.png")
                .unwrap(),
            position: (WINDOW_WIDTH as f32 * 0.5, WINDOW_HEIGHT as f32 * 0.5),
        });

        self.music = Some(
            init.content
                .get_ticket_with_key(&StorageType::Music, "Music.wav")
                .unwrap(),
        );

        self.oob = Some(
            init.content
                .get_ticket_with_key(&StorageType::Sound, "OoB.wav")
                .unwrap(),
        );
    }

    fn update(&mut self, update: &Self::Update, delta: f64) -> Vec<UpdateAction> {
        let mut actions = Vec::new();

        for info in update.info.iter() {
            match info {
                UpdateInfo::MusicStopped => {
                    actions.push(UpdateAction::PlayMusic(self.music.unwrap(), -1, 0.25))
                }
                _ => {}
            }
        }

        self.direction = (0.0, 1.0);

        let multiplier = -SPEED * delta as f32;
        let direction = (self.direction.0 * multiplier, self.direction.1 * multiplier);

        if let Some(logo) = &mut self.logo {
            let position = logo.position;

            if update.input.users[0]
                .get_action_by_index(self.controls.forward)
                .unwrap()
                .is_down()
            {
                logo.position = (position.0 + direction.0, position.1 + direction.1);
            }

            if update.input.users[0]
                .get_action_by_index(self.controls.backward)
                .unwrap()
                .is_down()
            {
                logo.position = (position.0 - direction.0, position.1 - direction.1);
            }

            if logo.position.0 < 0.0
                || logo.position.0 > WINDOW_WIDTH as f32
                || logo.position.1 < 0.0
                || logo.position.1 > WINDOW_HEIGHT as f32
            {
                actions.push(UpdateAction::PlaySound(self.oob.unwrap(), 0.25));
            }

            logo.position = (
                logo.position.0.clamp(0.0, WINDOW_WIDTH as f32),
                logo.position.1.clamp(0.0, WINDOW_HEIGHT as f32),
            );
        }

        actions
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
