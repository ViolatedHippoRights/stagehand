use log::{error, warn};
use sdl2::{
    event::Event,
    pixels::Color,
    rect::{Point, Rect},
    render::{Canvas, Texture},
    video::Window,
    Sdl, TimerSubsystem,
};
use std::rc::Rc;

use crate::{
    app::App,
    draw::{Draw, DrawBatch, DrawData, DrawDestination},
    input::{ActionState, ActionType, InputError, InputMap},
    loading::{ResourceError, Ticket},
    scene::Scene,
    utility2d::{Initialize, Update, UpdateAction, UpdateInfo},
    Stage, StageError,
};

use {input::SDLCommand, loading::SDLStorage};

mod draw;

pub mod input;
pub mod loading;

pub struct SDLApp<'a> {
    stage: Stage<
        'a,
        Initialize<SDLCommand, SDLStorage<'a>>,
        Update<SDLCommand>,
        Vec<UpdateAction>,
        (),
        DrawBatch<Draw, ()>,
    >,

    sdl: Sdl,
    canvas: Canvas<Window>,

    update: Update<SDLCommand>,

    storage: SDLStorage<'a>,

    timer: TimerSubsystem,
}

impl<'a> SDLApp<'a> {
    pub fn new(
        sdl: Sdl,
        canvas: Canvas<Window>,
        input: InputMap<SDLCommand>,
        storage: SDLStorage<'a>,
    ) -> Result<Self, String> {
        let timer = sdl.timer()?;

        Ok(SDLApp {
            stage: Stage::new(),

            sdl,
            canvas,

            update: Update {
                input,
                info: Vec::new(),
            },

            storage,

            timer,
        })
    }

    pub fn add_scene(
        &mut self,
        scene: Box<
            dyn Scene<
                    Initialize = Initialize<SDLCommand, SDLStorage<'a>>,
                    Update = Update<SDLCommand>,
                    Draw = (),
                    UpdateBatch = Vec<UpdateAction>,
                    DrawBatch = DrawBatch<Draw, ()>,
                > + 'a,
        >,
    ) {
        self.stage.push_scene(scene);
    }

    fn play_music(&mut self, ticket: Ticket, loops: i32, volume: f32) {
        match self.storage.music.get_by_ticket(ticket) {
            Ok(m) => {
                sdl2::mixer::Music::set_volume((volume * sdl2::mixer::MAX_VOLUME as f32) as i32);
                match m.play(loops) {
                    Ok(()) => {}
                    Err(e) => error!("Error playing music: {}", e),
                }
            }
            Err(e) => ResourceError::log_failure(e),
        }
    }

    fn render_texture(&mut self, texture: Rc<Texture<'_>>, data: &DrawData) {
        let query = texture.query();

        let source = match &data.source {
            Some(r) => Some(r.to_rect()),
            None => None,
        };

        let (angle, origin) = match &data.rotation {
            Some(r) => (
                r.angle as f64,
                Point::new(
                    (r.origin.0 * query.width as f32) as i32,
                    (r.origin.1 * query.height as f32) as i32,
                ),
            ),
            None => (0.0, Point::new(0, 0)),
        };

        let dest = match &data.destination {
            Some(d) => match d {
                DrawDestination::Location { x, y } => Some(Rect::new(
                    (*x as i32) - origin.x,
                    (*y as i32) - origin.y,
                    query.width,
                    query.height,
                )),
                DrawDestination::Rect(rect) => Some(rect.to_rect()),
            },
            None => None,
        };

        let (horizontal, vertical) = match &data.flip {
            Some(f) => (f.horizontal, f.vertical),
            None => (false, false),
        };

        if let Err(e) = self
            .canvas
            .copy_ex(&texture, source, dest, angle, origin, horizontal, vertical)
        {
            warn!("SDL2 Texture Rendering failed: {}", e);
        }
    }
}

impl<'a> App for SDLApp<'a> {
    type EventError = String;

    fn ticks(&self) -> u64 {
        self.timer.ticks64()
    }

    fn processed_events(&mut self) -> Result<bool, String> {
        let mut events = self.sdl.event_pump()?;

        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    return Ok(false);
                }
                _ => {}
            }
        }

        let keys = events.keyboard_state();

        for command_options in self.update.input.commands.iter() {
            let mut active = ActionType::Digital(ActionState::Up);
            'commands: for command in command_options.commands.iter() {
                match command {
                    SDLCommand::Key(c) => {
                        for key in c.iter() {
                            if keys.is_scancode_pressed(*key) {
                                active = ActionType::Digital(ActionState::Down);
                                break 'commands;
                            }
                        }
                    }
                    _ => {}
                };
            }

            match self.update.input.users[command_options.user_index]
                .update_action(command_options.action_index, active)
            {
                Err(e) => match e {
                    InputError::ActionIndexOutOfBounds => {
                        error!("Action index not found: {}", command_options.action_index)
                    }
                    _ => {}
                },
                _ => {}
            };
        }

        Ok(true)
    }

    fn update(&mut self, delta: f64) {
        self.update.info.clear();

        if !sdl2::mixer::Music::is_playing() {
            self.update.info.push(UpdateInfo::MusicStopped);
        }

        match self.stage.update(&self.update, delta) {
            Ok(v) => {
                for batch in v.iter() {
                    for command in batch.iter() {
                        match command {
                            UpdateAction::PlayMusic(ticket, loops, volume) => {
                                self.play_music(*ticket, *loops, *volume)
                            }
                            _ => {}
                        }
                    }
                }
            }
            Err(e) => match e {
                StageError::NoScenesToUpdateError => warn!("Stage has no scenes to update."),
                _ => {}
            },
        }
    }

    fn draw(&mut self, interp: f64, _total_time: u64) {
        self.canvas.set_draw_color(Color::RGB(55, 55, 55));
        self.canvas.clear();

        let batches = match self.stage.draw(&(), interp) {
            Ok(b) => b,
            Err(e) => {
                match e {
                    StageError::NoScenesToDrawError => warn!("Stage has no scenes to draw."),
                    _ => {}
                }

                return;
            }
        };

        for batch in batches.iter() {
            for draw in batch.instructions.iter() {
                let texture = match self.storage.textures.get_by_ticket(draw.ticket) {
                    Ok(t) => t,
                    Err(e) => {
                        ResourceError::log_failure(e);
                        return;
                    }
                };

                self.render_texture(texture, &draw.data);
            }
        }

        self.canvas.present();
    }
}
