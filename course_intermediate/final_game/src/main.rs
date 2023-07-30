use final_game::{game_logic, GameState};
use rusty_engine::prelude::*; // Prelude is one of the three usages of import * in Rust

fn main() {
    let mut game = Game::new();

    // Sprites have a name, the file can be either a preset or a path to something on disk
    let player = game.add_sprite("player", SpritePreset::RacingCarBlue);
    // The sprite is put in the center of the screen at position 0,0
    // player.translation = Vec2::new(200.0, 100.0); // Shift to right and up

    // Rotation is in radiant, sprites are expected to be facing right (0°), PI is 180°
    // player.rotation = FRAC_PI_2; // The car will point upwards
    // Utility constant defined by rusty engine, it's the same as the line above
    // player.rotation = UP;

    player.scale = 1.0; // Above 1 will upscale, below will downscale

    // Determines the layer index aka which object will be rendered first
    // (the larger the more priority)
    player.layer = 0.0;
    player.collision = true; // Enable collision detection for the object

    let car1 = game.add_sprite("car1", SpritePreset::RacingCarYellow);
    car1.translation = Vec2::new(300.0, 0.0);
    car1.collision = true;

    // We pass to the run function a game state which is a struct that contains all the info for
    // our game, everything we want to persist betweeen each frame
    game.add_logic(game_logic);

    // If we want we can show the colliders around the sprites, this colliders represent the
    // hitbox of our sprites and are stored in the same location of the images
    // Creating the collider for a new sprite is a slow job, there's an helper example
    // cargo install rusty_engine --example collider
    // Then call collider with the path of the image (read the stdout for docs)
    game.show_colliders = true;

    // Display scores
    let score = game.add_text("score", "Score: 0");
    score.translation = Vec2::new(520.0, 320.0);

    let highscore = game.add_text("high_score", "High Score: 0");
    highscore.translation = Vec2::new(-520.0, 320.0);

    // Setup game audio
    game.audio_manager.play_music(MusicPreset::Classy8Bit, 0.01);

    // Setup game window
    game.window_settings(WindowDescriptor {
        // width: 1400.0,
        // height: 500.0,
        title: "Exercise game".to_string(),
        ..Default::default()
    });
    game.run(GameState::default());
}
