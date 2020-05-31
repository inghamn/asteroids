use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

mod asteroids;
use crate::asteroids::Asteroids;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root       = application_root_dir()?;
    let resources      = app_root.join("resources");
    let display_config = resources.join("display.ron");
    let game_data      = GameDataBuilder::default()
                           .with_bundle(TransformBundle::new())?
                           .with_bundle(
                               RenderingBundle::<DefaultBackend>::new()
                                   // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                                   .with_plugin(
                                       RenderToWindow::from_config_path(display_config)?
                                           .with_clear([0.0, 0.0, 0.0, 1.0]),
                                   )
                                   // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
                                   .with_plugin(RenderFlat2D::default()),
                           )?;

    let mut game = Application::new(resources, Asteroids, game_data)?;

    game.run();

    Ok(())
}
