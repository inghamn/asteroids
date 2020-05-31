use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub struct Asteroids;
pub const ARENA_WIDTH:  f32 = 1024.0;
pub const ARENA_HEIGHT: f32 = 1024.0;
pub const SHIP_WIDTH:   f32 = 28.0;
pub const SHIP_HEIGHT:  f32 = 16.0;

impl SimpleState for Asteroids
{
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>)
    {
        let world   = data.world;
        let sprites = load_sprites(world);
        world.register::<Ship>();
        init_ship  (world, sprites);
        init_camera(world);
    }
}


fn init_camera(world: &mut World)
{
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world.create_entity()
         .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
         .with(transform)
         .build();
}

fn init_ship(world: &mut World, sprites: Handle<SpriteSheet>)
{
    let mut transform = Transform::default();
    let render = SpriteRender {
        sprite_sheet: sprites,
        sprite_number: 0
    };

    transform.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);
    world.create_entity()
         .with(Ship::new())
         .with(transform)
         .with(render.clone())
         .build();
}

fn load_sprites(world: &mut World) -> Handle<SpriteSheet>
{
    let textures = {
        let loader = world.read_resource::<Loader>();
        let image  = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "textures/sprites.png",
            ImageFormat::default(),
            (),
            &image,
        )
    };

    let loader  = world.read_resource::<Loader>();
    let sprites = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "textures/sprites.ron", // Here we load the associated ron file
        SpriteSheetFormat(textures),
        (),
        &sprites,
    )
}

pub struct Ship {
    pub width:  f32,
    pub height: f32
}

impl Ship {
    fn new() -> Ship {
        Ship{
            width:  SHIP_WIDTH,
            height: SHIP_HEIGHT
        }
    }
}

impl Component for Ship {
    type Storage = DenseVecStorage<Self>;
}
