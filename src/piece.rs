use bevy::{
    prelude::{shape, Assets, Color, Commands, Mesh, Query, Transform},
    sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle, SpriteBundle},
};
use bevy_ecs_tilemap::{
    helpers::square_grid::neighbors::Neighbors,
    prelude::{TilemapGridSize, TilemapSize, TilemapType},
    tiles::{TilePos, TileStorage},
};

pub enum Piece {
    Pawn,
    Rock,
    Knight,
    Bishop,
    Queen,
    King,
}

pub fn get_posible_movements(
    commands: &mut Commands,
    tile_storage: &TileStorage,
    grid_size: &TilemapGridSize,
    map_size: &TilemapSize,
    map_type: &TilemapType, //pice: &Piece,
    pos: TilePos,
    mesh: &mut Assets<Mesh>,
    material: &mut Assets<ColorMaterial>,
) {
    let neighbors_positions = Neighbors::get_square_neighboring_positions(&pos, map_size, true);

    for pos in neighbors_positions.iter() {
        let transform = pos.center_in_world(grid_size, map_type);

        commands.spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(mesh.add(Mesh::from(shape::Circle::new(16.0))).into()),
            transform: Transform::from_xyz(transform.x, transform.y, 2.0),
            material: material.add(ColorMaterial::from(
                Color::hex("B0A8B9").expect("Error here"),
            )),
            ..Default::default()
        });
    }
}
