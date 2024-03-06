use sdl2::keyboard::Scancode;

use stagehand::{
    app::gameloop,
    input::{ActionState, ActionType, InputMap},
    scene::Scene,
    sdl2::{input::SDLCommand, loading::SDLStorage, SDLApp},
};

use scene::ExampleScene;

pub mod scene;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    sdl2::image::init(sdl2::image::InitFlag::PNG)?;

    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Stagehand SDL2 Example", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let mut input = InputMap::<SDLCommand>::new();
    let player = input.add_user();
    input
        .add_action(
            player,
            "Forward".to_string(),
            vec![
                SDLCommand::Key(vec![Scancode::W]),
                SDLCommand::Key(vec![Scancode::Up]),
            ],
            ActionType::Digital(ActionState::Up),
        )
        .unwrap();
    input
        .add_action(
            player,
            "Backward".to_string(),
            vec![
                SDLCommand::Key(vec![Scancode::S]),
                SDLCommand::Key(vec![Scancode::Down]),
            ],
            ActionType::Digital(ActionState::Up),
        )
        .unwrap();
    input
        .add_action(
            player,
            "Look".to_string(),
            vec![],
            ActionType::Analog { x: 0.0, y: 0.0 },
        )
        .unwrap();
    input
        .add_action(
            player,
            "Pause".to_string(),
            vec![],
            ActionType::Digital(ActionState::Up),
        )
        .unwrap();

    let mut storage = SDLStorage::new(&texture_creator);
    storage
        .textures
        .load("Logo.png".to_string(), "example-assets/Logo.png")
        .unwrap();
    storage.textures.lock();

    let initialize = (input, storage);

    let mut scene = ExampleScene::new();
    scene.initialize(&initialize);

    let mut app = SDLApp::new(sdl_context, canvas, initialize.0, initialize.1)?;

    app.add_scene(Box::new(scene));

    gameloop(&mut app, 60)?;

    Ok(())
}
