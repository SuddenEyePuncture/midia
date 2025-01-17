use serde::{Deserialize, Serialize};
use tetra::graphics::Color;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Named {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Colored {
    pub color: Color,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Readable {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LookLike {
    pub look_like: String,
}

// TODO: slots and stuff
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Wearable {}
