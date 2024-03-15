use std::{collections::HashMap, hash::Hash};

pub mod app;
pub mod scene;

#[cfg(feature = "utility")]
pub mod utility;

#[cfg(feature = "draw2d")]
pub mod draw;

#[cfg(feature = "example")]
pub mod example;

#[cfg(feature = "input")]
pub mod input;

#[cfg(feature = "loading")]
pub mod loading;

use scene::Scene;

pub struct Stage<'a, Key, Initialize, Update, Message, Instruction, Draw, DrawBatch>
where
    Key: Hash + Eq + ToString,
{
    scenes: HashMap<
        Key,
        Box<
            dyn Scene<
                    Key = Key,
                    Initialize = Initialize,
                    Update = Update,
                    Draw = Draw,
                    Message = Message,
                    Instruction = Instruction,
                    DrawBatch = DrawBatch,
                > + 'a,
        >,
    >,
    active: Vec<Key>,
}

pub enum Response<Key, Message, Instruction> {
    Message(Key, Message),
    Instruction(Instruction),
}

impl<'a, Key, Initialize, Update, Message, Instruction, Draw, DrawBatch>
    Stage<'a, Key, Initialize, Update, Message, Instruction, Draw, DrawBatch>
where
    Key: Clone + Hash + Eq + ToString,
{
    pub fn new() -> Self {
        Stage {
            scenes: HashMap::new(),
            active: Vec::new(),
        }
    }

    pub fn add_scene(
        &mut self,
        key: Key,
        scene: Box<
            dyn Scene<
                    Key = Key,
                    Initialize = Initialize,
                    Update = Update,
                    Draw = Draw,
                    Message = Message,
                    Instruction = Instruction,
                    DrawBatch = DrawBatch,
                > + 'a,
        >,
        active: bool,
    ) {
        if active {
            self.active.push(key.clone());
        }

        self.scenes.insert(key, scene);
    }

    pub fn update(&mut self, update: &Update, delta: f64) -> Result<Vec<Instruction>, StageError> {
        if self.scenes.len() > 0 {
            let mut instructions = Vec::new();

            let mut start = self.scenes.len() - 1;
            while start > 0 && !self.scenes[&self.active[start]].blocking() {
                start -= 1;
            }

            for i in start..self.scenes.len() {
                match self.scenes.get_mut(&self.active[i]) {
                    Some(scene) => {
                        let responses = scene.update(update, delta);
                        for response in responses.into_iter() {
                            match response {
                                Response::Message(k, m) => match self.scenes.get_mut(&k) {
                                    Some(s) => s.receive_message(&m),
                                    None => {
                                        return Err(StageError::MessageSceneNotFoundError(
                                            self.active[i].to_string(),
                                        ));
                                    }
                                },
                                Response::Instruction(i) => instructions.push(i),
                            }
                        }
                    }
                    None => {
                        return Err(StageError::UpdateSceneNotFoundError(
                            self.active[i].to_string(),
                        ));
                    }
                }
            }

            return Ok(instructions);
        }

        Err(StageError::NoScenesToUpdateError)
    }

    pub fn draw(&self, draw: &Draw, interp: f64) -> Result<Vec<DrawBatch>, StageError> {
        if self.scenes.len() > 0 {
            let mut batches: Vec<DrawBatch> = Vec::new();

            let mut start = self.scenes.len() - 1;
            while start > 0 && !self.scenes[&self.active[start]].covering() {
                start -= 1;
            }

            for i in start..self.scenes.len() {
                batches.push(self.scenes[&self.active[i]].draw(draw, interp));
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
    UpdateSceneNotFoundError(String),
    MessageSceneNotFoundError(String),
}
