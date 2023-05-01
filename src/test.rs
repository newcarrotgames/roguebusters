use specs::World;

pub struct UI2<'a> {
    messages: Vec<String>,
    world: &'a World,
}

impl<'a> UI2<'a> {
    pub fn new(world: &'a World) -> Self {
        UI2 {
            messages: Vec::new(),
            world,
        }
    }
}