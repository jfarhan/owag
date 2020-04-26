pub enum DataTypes{
    Bool(bool),
    Number(f32),
}

#[derive(Copy,Clone)]
pub struct InteractionParams{
    pub x:f32,
    pub y:f32,
    pub width:f32,
    pub height:f32
}
