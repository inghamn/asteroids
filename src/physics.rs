use specs::prelude::*;

struct Physics {}

impl<'a> System<'a> for Physics
{
    type SystemData = (ReadStorage<'a, Velocity>, WriteStorage<'a, Position>);

    fn run(&mut self, (velocity, mut position): Self::SystemData)
    {
        for (v, p) in (&velocity, &mut position).join() {
            p.x += v.x;
            p.y += v.y;
        }
    }
}
