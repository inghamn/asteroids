use sdl2::rect::Point;
use sdl2::render::{WindowCanvas};
/**
 * Shapes are stored as matrixes rendered at 320x240.
 * To draw at different resolutions, you must scale the matrix.
 * These are the whole number scaling factors for different resolutions.
 *    320 x  240  1
 *    640 x  480  2
 *   1280 x  960  4
 *   1440 x 1080  6
 */
const SHAPE_SCALE: f32 = 4.0;
const COORD_SCALE: f32 = 1.0; // Ration of arena size to window size

pub struct Renderable {
    pub direction: f32, // Radians
    pub shape:     Vec<(i8, i8)>,
    pub radius:    u8
}

/**
 * Draw a renderable onto a canvas
 * @param x  Entity position in the Arena
 * @param y  Entity position in the Arena
 */
pub fn draw_renderable(canvas: &mut WindowCanvas, entity: &Renderable, x: f32, y: f32)
{
    let polygon = vertices_screen(entity, (x, y));
    let ghost   = wrapped_ghost(entity, x, y);
    let _       = canvas.draw_lines(&polygon[..]);

    if ghost.is_some() {
        let polygon = vertices_screen(entity, ghost.unwrap());
        let _       = canvas.draw_lines(&polygon[..]);
    }
}


/**
 * Returns a possible ghost polygon to draw when an entity is near the border
 *
 * Entity X,Y are Arena coordinates
 */
fn wrapped_ghost(entity: &Renderable, entity_x: f32, entity_y: f32) -> Option<(f32, f32)>
{
    let mut ghost_x: Option<f32> = None;
    let mut ghost_y: Option<f32> = None;

    let border = (entity.radius as f32) * SHAPE_SCALE;

    if entity_x <= border { ghost_x = Some(entity_x + crate::ARENA_WIDTH ); }
    if entity_y <= border { ghost_y = Some(entity_y + crate::ARENA_HEIGHT); }
    if entity_x >= crate::ARENA_WIDTH  - border { ghost_x = Some(entity_x - crate::ARENA_WIDTH ); }
    if entity_y >= crate::ARENA_HEIGHT - border { ghost_y = Some(entity_y - crate::ARENA_HEIGHT); }

    if ghost_x.is_some() && ghost_y.is_none() { ghost_y = Some(entity_y); }
    if ghost_y.is_some() && ghost_x.is_none() { ghost_x = Some(entity_x); }

    if ghost_x.is_some() {
        return Some((ghost_x.unwrap(), ghost_y.unwrap()));
    }
    None
}

/**
 * Returns a vector of polygon vertices using screen coordinates
 * @param zoom   Zoom factor for current window size to arena size
 */
fn vertices_screen(entity: &Renderable, location: (f32, f32)) -> Vec<Point>
{
    let sin_t  = entity.direction.sin() * SHAPE_SCALE;
    let cos_t  = entity.direction.cos() * SHAPE_SCALE;
    let mut shape = vec![Point::new(0, 0); entity.shape.len()];

    for i in 0..entity.shape.len() {
        let x = (entity.shape[i].0 as f32 * cos_t) - (entity.shape[i].1 as f32 * sin_t);
        let y = (entity.shape[i].1 as f32 * cos_t) + (entity.shape[i].0 as f32 * sin_t);

        shape[i] = Point::new(
            x.round() as i32 + (location.0 * COORD_SCALE).round() as i32,
            y.round() as i32 + (location.1 * COORD_SCALE).round() as i32
        );
    }
    shape
}
