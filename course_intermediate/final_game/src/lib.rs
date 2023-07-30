use rand::prelude::*;
use rusty_engine::prelude::*;

pub struct GameState {
    high_score: u32,
    score: u32,
    red_ball_index: i32,
    spawn_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            high_score: 0,
            score: 0,
            red_ball_index: 0,
            spawn_timer: Timer::from_seconds(2.0, true),
        }
    }
}

pub fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // Close the game if Q is pressed
    if engine.keyboard_state.just_pressed(KeyCode::Q) {
        engine.should_exit = true;
    }

    // Create tiny animation for text
    // Range between -5 and 5 using cosine wave
    // 3.0 to make it faster
    let offset = ((engine.time_since_startup_f64 * 3.0).cos() * 5.0) as f32;
    // Keep text in the right position if window is rescaled
    let mut score = engine.texts.get_mut("score").unwrap();
    score.translation.x = engine.window_dimensions.x / 2.0 - 80.0;
    score.translation.y = engine.window_dimensions.y / 2.0 - 30.0 + offset;
    let mut high_score = engine.texts.get_mut("high_score").unwrap();
    high_score.translation.x = -engine.window_dimensions.x / 2.0 + 110.0;
    high_score.translation.y = engine.window_dimensions.y / 2.0 - 30.0;

    // Handle collisions
    for event in engine.collision_events.drain(..) {
        // An event is a struct with a state (Begin/End) and a pair which contains the labels
        // of the colliding objects
        // println!("{:#?}", event);
        if event.state == CollisionState::Begin && event.pair.one_starts_with("player") {
            // Remove the sprite the player collided with
            for label in [event.pair.0, event.pair.1] {
                if label != "player" {
                    engine.sprites.remove(&label);
                }
            }

            // Play a sound effect on hits
            engine.audio_manager.play_sfx(SfxPreset::Impact1, 0.3);

            game_state.score += 1;
            let score = engine.texts.get_mut("score").unwrap();
            score.value = format!("Score: {}", game_state.score);

            if game_state.score > game_state.high_score {
                game_state.high_score = game_state.score;
                let high_score = engine.texts.get_mut("high_score").unwrap();
                high_score.value = format!("High Score: {}", game_state.high_score);
            }
        }
    }

    // Handle player movement
    let player = engine.sprites.get_mut("player").unwrap();
    // player.translation.x += 100.0 * engine.delta_f32;
    const MOVEMENT_SPEED: f32 = 100.0;

    // Keyboard state is the key pressed at the start of the current frame
    // To handle stuff like typing is better to use key events
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Up, KeyCode::W])
    {
        player.translation.y += MOVEMENT_SPEED * engine.delta_f32;
    }

    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Down, KeyCode::S])
    {
        player.translation.y -= MOVEMENT_SPEED * engine.delta_f32;
    }

    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Right, KeyCode::D])
    {
        player.translation.x += MOVEMENT_SPEED * engine.delta_f32;
    }

    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Left, KeyCode::A])
    {
        player.translation.x -= MOVEMENT_SPEED * engine.delta_f32;
    }

    // Handle mouse input
    // We'll generate red balls on mouse clicks
    if engine.mouse_state.just_pressed(MouseButton::Left) {
        // If the click is inside the game window
        if let Some(mouse_location) = engine.mouse_state.location() {
            let label = format!("ferris_{}", game_state.red_ball_index);
            game_state.red_ball_index += 1;
            let red_ball = engine.add_sprite(label, SpritePreset::RollingBallRed);
            red_ball.translation = Vec2::new(mouse_location.x, mouse_location.y);
            red_ball.collision = true;
        }
    }

    // Reset score
    if engine.keyboard_state.just_pressed(KeyCode::R) {
        game_state.score = 0;
        let score = engine.texts.get_mut("score").unwrap();
        score.value = "Score: 0".to_string();
    }

    // Spawn red dots based on timer
    // Tick will advance the timer
    if game_state.spawn_timer.tick(engine.delta).just_finished() {
        let label = format!("ferris_{}", game_state.red_ball_index);
        game_state.red_ball_index += 1;
        let red_ball = engine.add_sprite(label, SpritePreset::RollingBallRed);
        red_ball.translation.x = thread_rng().gen_range(-550.0..550.0);
        red_ball.translation.y = thread_rng().gen_range(-320.0..320.0);
        red_ball.collision = true;
    }
}
