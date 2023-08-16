#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Attributes {
    brawn: u8,
    agility: u8,
    perception: u8,
    fortune: u8,
    charm: u8,
}
