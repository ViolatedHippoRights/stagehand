use std::collections::HashMap;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ActionState {
    Down,
    Pressed,
    Released,
    Up,
}

impl ActionState {
    pub fn is_down(&self) -> bool {
        *self == ActionState::Down || *self == ActionState::Pressed
    }

    pub fn is_delta(&self) -> bool {
        *self == ActionState::Pressed || *self == ActionState::Released
    }
}

#[derive(Clone, Copy)]
pub enum ActionType {
    Digital(ActionState),
    Axis(f32),
    Analog { x: f32, y: f32 },
}

impl ActionType {
    pub fn is_down(&self) -> bool {
        match self {
            ActionType::Digital(state) => ActionState::is_down(&state),
            _ => false,
        }
    }

    pub fn update(old: ActionType, new: ActionType, updated: bool) -> ActionType {
        match new {
            ActionType::Digital(new_action) => match old {
                ActionType::Digital(old_action) => {
                    if !updated && old_action.is_delta() {
                        return old;
                    }

                    match (old_action.is_down(), new_action.is_down()) {
                        (true, true) => ActionType::Digital(ActionState::Down),
                        (true, false) => ActionType::Digital(ActionState::Released),
                        (false, true) => ActionType::Digital(ActionState::Pressed),
                        (false, false) => ActionType::Digital(ActionState::Up),
                    }
                }
                _ => match new_action {
                    ActionState::Up => old,
                    _ => new,
                },
            },
            _ => new,
        }
    }
}

pub struct InputActions {
    actions: Vec<ActionType>,
    action_map: HashMap<String, usize>,
    updated: bool,
}

impl InputActions {
    pub fn new() -> Self {
        InputActions {
            actions: Vec::new(),
            action_map: HashMap::new(),
            updated: false,
        }
    }

    pub fn add_action(&mut self, key: String, action: ActionType) -> usize {
        self.actions.push(action);

        let index = self.actions.len() - 1;
        self.action_map.insert(key, index);

        index
    }

    pub fn update_action(&mut self, index: usize, new: ActionType) -> Result<(), InputError> {
        if index >= self.actions.len() {
            return Err(InputError::ActionIndexOutOfBounds);
        }

        self.actions[index] = ActionType::update(self.actions[index], new, self.updated);

        Ok(())
    }

    pub fn updated(&mut self) {
        self.updated = true;
    }

    pub fn set(&mut self) {
        self.updated = false;
    }

    pub fn get_index_by_key(&self, key: &str) -> Result<usize, InputError> {
        match self.action_map.get(key) {
            Some(i) => Ok(*i),
            None => Err(InputError::UnrecognizedAction),
        }
    }

    pub fn get_action_by_index(&self, index: usize) -> Result<ActionType, InputError> {
        if index >= self.actions.len() {
            return Err(InputError::ActionIndexOutOfBounds);
        }

        Ok(self.actions[index])
    }

    pub fn get_action_by_key(&self, key: &str) -> Result<ActionType, InputError> {
        Ok(self.actions[self.get_index_by_key(key)?])
    }
}

pub struct InputCommand<C> {
    pub user_index: usize,
    pub action_index: usize,
    pub commands: Vec<C>,
}

pub struct InputMap<C> {
    pub users: Vec<InputActions>,
    pub commands: Vec<InputCommand<C>>,
}

impl<C> InputMap<C> {
    pub fn new() -> Self {
        InputMap {
            users: Vec::new(),
            commands: Vec::new(),
        }
    }

    pub fn add_user(&mut self) -> usize {
        self.users.push(InputActions::new());

        self.users.len() - 1
    }

    pub fn updated(&mut self) {
        for user in self.users.iter_mut() {
            user.updated();
        }
    }

    pub fn set(&mut self) {
        for user in self.users.iter_mut() {
            user.set();
        }
    }

    pub fn add_action(
        &mut self,
        user: usize,
        key: String,
        commands: Vec<C>,
        action: ActionType,
    ) -> Result<(), InputError> {
        if user < self.users.len() {
            let index = self.users[user].add_action(key, action);

            let command = InputCommand {
                user_index: user,
                action_index: index,
                commands,
            };
            self.commands.push(command);

            return Ok(());
        }

        Err(InputError::UserIndexOutOfBounds)
    }
}

#[derive(Debug)]
pub enum InputError {
    ActionIndexOutOfBounds,
    UnrecognizedAction,
    UserIndexOutOfBounds,
}
