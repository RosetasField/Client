use bevy::prelude::*;

pub trait Structure {
    fn build(&self,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>) -> PbrBundle;
}

pub struct HeadQuarters {
    pub x:f32,
    pub y:f32,
    pub z:f32,
}

pub struct ManaGenerator {
    pub x:f32,
    pub y:f32,
    pub z:f32,
}

