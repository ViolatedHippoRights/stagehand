pub mod app;
pub mod scene;

#[cfg(feature = "2d")]
pub mod utility2d;

#[cfg(feature = "draw2d")]
pub mod draw;

#[cfg(feature = "input")]
pub mod input;

#[cfg(feature = "loading")]
pub mod loading;

#[cfg(feature = "sdl2")]
pub mod sdl2;

use scene::Scene;

pub struct Stage<'a, I, U, UB, D, DB> {
    scenes: Vec<
        Box<dyn Scene<Initialize = I, Update = U, Draw = D, UpdateBatch = UB, DrawBatch = DB> + 'a>,
    >,
}

impl<'a, I, U, UB, D, DB> Stage<'a, I, U, UB, D, DB> {
    pub fn new() -> Self {
        Stage { scenes: Vec::new() }
    }

    pub fn push_scene(
        &mut self,
        scene: Box<
            dyn Scene<Initialize = I, Update = U, Draw = D, UpdateBatch = UB, DrawBatch = DB> + 'a,
        >,
    ) {
        self.scenes.push(scene);
    }

    pub fn update(&mut self, update: &U, delta: f64) -> Result<Vec<UB>, StageError> {
        if self.scenes.len() > 0 {
            let mut batches = Vec::new();

            let mut i = self.scenes.len() - 1;
            batches.push(self.scenes[i].update(update, delta));

            while i > 0 && !self.scenes[i].blocking() {
                i -= 1;
                batches.push(self.scenes[i].update(update, delta));
            }

            return Ok(batches);
        }

        Err(StageError::NoScenesToUpdateError)
    }

    pub fn draw(&self, draw: &D, interp: f64) -> Result<Vec<DB>, StageError> {
        if self.scenes.len() > 0 {
            let mut batches: Vec<DB> = Vec::new();

            let mut start = self.scenes.len() - 1;
            while start > 0 && !self.scenes[start].covering() {
                start -= 1;
            }

            for i in start..self.scenes.len() {
                batches.push(self.scenes[i].draw(draw, interp));
            }

            return Ok(batches);
        }

        Err(StageError::NoScenesToDrawError)
    }
}

#[derive(Debug)]
pub enum StageError {
    NoScenesToUpdateError,
    NoScenesToDrawError,
}
