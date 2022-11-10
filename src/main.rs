use std::time::Duration;
use bevy::prelude::Timer;
use rusty_engine::prelude::*;
use car_shoot::constant::*;

struct GameState {
    marble_labels: Vec<String>,
    cars_left: i8,
    spawn_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self { 
            marble_labels: vec!["marble1".into(), "marble2".into(), "marble3".into()], 
            cars_left: 25, 
            spawn_timer: Timer::new(Duration::from_millis(0), false), 
        }
    }
}
fn main() {
    let mut game = Game::new();

    let game_state = GameState{
        ..Default::default()
    };

    // game setup goes here

    // set the windows dimention
    game.window_settings(WindowDescriptor{
        title: "Car Shoot".into(),
        width: 800.0,
        height: 300.0,
        ..Default::default()
    });

    // set music playing
    game.audio_manager.play_music(MusicPreset::Classy8Bit, 0.2);

    // add player sprite
    let player = game.add_sprite(ID_PLAYER, SpritePreset::RacingBarrelBlue);
    player.translation.y = -325.0;
    player.rotation = UP;
    player.scale = 0.5;
    player.layer = 10.0;

    // add text cars left
    let cars_left = game.add_text("cars_left", format!("Cars Left: {}", game_state.cars_left));
    cars_left.translation = Vec2::new(540.0, -320.0);

    game.add_logic(game_logic);
    game.run(game_state);
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // game logic goes here
    let player = engine.sprites.get_mut(ID_PLAYER).unwrap();
    let player_x = player.translation.x;
    if let Some(location) = engine.mouse_state.location() {
        player.translation.x = location.x;
    }
}
