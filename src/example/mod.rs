use std::marker::PhantomData;

use crate::{
    draw::{Draw, DrawBatch, DrawData, DrawType},
    input::{ActionState, ActionType},
    loading::{Ticket, TicketManager},
    scene::Scene,
    utility::{Initialize, StorageType, Update, UpdateInfo, UpdateInstruction},
    Response,
};

pub mod ui;

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

pub struct ExampleScene<I, S> {
    controls: PlayerControls,
    direction: (f32, f32),

    logo: Option<Logo>,

    music: Option<Ticket>,
    oob: Option<Ticket>,

    ui: bool,

    phantom: PhantomData<(I, S)>,
}

impl<I, S> ExampleScene<I, S> {
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

            ui: true,

            phantom: PhantomData,
        }
    }
}

impl<I, S> Scene for ExampleScene<I, S>
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
        self.controls.forward = init.input.users[0].get_index_by_key("Forward").unwrap();
        self.controls.backward = init.input.users[0].get_index_by_key("Backward").unwrap();
        self.controls.look = init.input.users[0].get_index_by_key("Look").unwrap();
        self.controls.pause = init.input.users[0].get_index_by_key("Pause").unwrap();

        self.logo = Some(Logo {
            texture: init
                .storage
                .get_ticket_with_key(&StorageType::Texture, "Logo.png")
                .unwrap(),
            position: (WINDOW_WIDTH as f32 * 0.5, WINDOW_HEIGHT as f32 * 0.5),
        });

        self.music = Some(
            init.storage
                .get_ticket_with_key(&StorageType::Music, "Music.wav")
                .unwrap(),
        );

        self.oob = Some(
            init.storage
                .get_ticket_with_key(&StorageType::Sound, "OoB.wav")
                .unwrap(),
        );
    }

    fn update(
        &mut self,
        update: &Self::Update,
        delta: f64,
    ) -> Vec<Response<Self::Key, Self::Message, Self::Instruction>> {
        let mut actions = Vec::new();

        for info in update.info.iter() {
            match info {
                UpdateInfo::MusicStopped => actions.push(Response::Instruction(
                    UpdateInstruction::PlayMusic(self.music.unwrap(), -1, 0.25),
                )),
            }
        }

        if let Some(logo) = &mut self.logo {
            let position = logo.position;

            self.direction = (0.0, 1.0);

            if let ActionType::Analog { x, y } = update.input.users[0]
                .get_action_by_index(self.controls.look)
                .unwrap()
            {
                let (relative_x, relative_y) = (x - position.0, y - position.1);
                let length = f32::sqrt(relative_x * relative_x + relative_y * relative_y);

                self.direction = (relative_x / length, relative_y / length);
            }

            let multiplier = SPEED * delta as f32;
            let direction = (self.direction.0 * multiplier, self.direction.1 * multiplier);

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
                actions.push(Response::Message("UI".to_string(), "Collision".to_string()));
                actions.push(Response::Instruction(UpdateInstruction::PlaySound(
                    self.oob.unwrap(),
                    1.0,
                )));
            }

            logo.position = (
                logo.position.0.clamp(0.0, WINDOW_WIDTH as f32),
                logo.position.1.clamp(0.0, WINDOW_HEIGHT as f32),
            );

            match update.input.users[0]
                .get_action_by_index(self.controls.pause)
                .unwrap()
            {
                ActionType::Digital(s) => {
                    if s == ActionState::Pressed {
                        self.ui = !self.ui;
                        match self.ui {
                            true => actions.push(Response::AddScene("UI".to_string())),
                            false => actions.push(Response::RemoveScene("UI".to_string())),
                        }
                    }
                }
                _ => {}
            };
        }

        actions
    }

    fn draw(&self, _draw: &Self::Draw, _interp: f64) -> Self::DrawBatch {
        let mut batch = DrawBatch::new(());

        if let Some(logo) = &self.logo {
            let angle =
                (self.direction.1.atan2(self.direction.0) * 180.0 / std::f32::consts::PI) + 90.0;

            batch.instructions.push(Draw {
                ticket: logo.texture,
                draw_type: DrawType::Texture,
                data: DrawData::draw_rotated_at(
                    logo.position.0,
                    logo.position.1,
                    angle,
                    (0.5, 0.5),
                ),
            });
        }

        batch
    }

    fn receive_message(&mut self, _message: &Self::Message) {}

    fn covering(&self) -> bool {
        return true;
    }

    fn blocking(&self) -> bool {
        return true;
    }
}
