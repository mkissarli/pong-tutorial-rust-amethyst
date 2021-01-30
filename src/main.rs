use amethyst::{
    prelude::*,
    renderer::{
        plugins::{ RenderFlat2D, RenderToWindow },
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

pub struct Pong;

impl SimpleState for Pong {
    
}

fn main() -> amethyst::Result<()>{
     // Logger
    amethyst::start_logger(Default::default());

    // Set up configs.
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    // Basic Application Setup
    let game_data = GameDataBuilder::default();
    

   Ok(())
}
