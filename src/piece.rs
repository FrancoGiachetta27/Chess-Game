use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_ecs_tilemap::{
    prelude::{TilemapGridSize, TilemapSize, TilemapType},
    tiles::{TilePos, TileStorage},
};
use bevy_mod_picking::{PickableBundle, PickingEvent};
use iyes_loopless::prelude::*;

use crate::{
    bishop::Bishop,
    board::{Tile, TileState},
    king::King,
    knight::Knight,
    movement::{get_piece_movements, move_piece, MoveEvent},
    pawn::Pawn,
    queen::Queen,
    rock::Rock,
};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Team {
    White,
    Black,
}
#[derive(Component)]
pub struct HighLight;

#[derive(Component, Clone, Copy)]
pub enum PieceType {
    Pawn(Pawn),
    Rock(Rock),
    Bishop(Bishop),
    Knight(Knight),
    Queen(Queen),
    King(King),
}

impl PieceType {
    pub fn get_team(self) -> Team {
        match self {
            Self::Pawn(p) => p.team,
            Self::Rock(r) => r.team,
            Self::Knight(kn) => kn.team,
            Self::Bishop(b) => b.team,
            Self::Queen(q) => q.team,
            Self::King(k) => k.team,
        }
    }
}

pub struct PieceDeathEvent(pub Entity);

pub struct PiecePlugin;

impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(get_piece_movements.run_on_event::<PickingEvent>())
            .add_system(move_piece.run_on_event::<PickingEvent>())
            .add_system(reset_neighbors.run_on_event::<MoveEvent>())
            .add_event::<MoveEvent>()
            .add_event::<PieceDeathEvent>()
            .add_system(handle_piece_death.run_on_event::<PieceDeathEvent>())
            .run();
    }
}

fn reset_neighbors(
    mut commands: Commands,
    mut tile_state: Query<&mut TileState>,
    move_event: EventReader<MoveEvent>,
    transform_q: Query<&mut Transform>,
    tile_query: Query<(&TileStorage, &TilemapGridSize, &TilemapSize, &TilemapType)>,
    highlight_pos: Query<Entity, With<HighLight>>,
) {
    let (tile_storage, grid_size, map_size, map_type) = tile_query.single();
    for ent in highlight_pos.iter() {
        let transform = transform_q.get(ent).unwrap();
        let tile_pos = TilePos::from_world_pos(
            &Vec2::new(transform.translation.x, transform.translation.y),
            map_size,
            grid_size,
            map_type,
        )
        .unwrap();
        let neighbor = tile_storage.get(&tile_pos).unwrap();
        let mut neigh_state = tile_state.get_mut(neighbor).unwrap();

        // avoid setting to Empty the state of the neighbor we have moved the piece to
        if let Tile::HighLighted = neigh_state.tile_type {
            match neigh_state.piece_ent {
                Some(_e) => neigh_state.tile_type = Tile::NotEmpty,
                None => neigh_state.tile_type = Tile::Empty,
            }
        }

        commands.entity(ent).despawn_recursive();
    }
}

pub fn highlight_tile(
    commands: &mut Commands,
    grid_size: &TilemapGridSize,
    map_type: &TilemapType,
    tile_pos: &TilePos,
    mesh: &mut Assets<Mesh>,
    material: &mut Assets<ColorMaterial>,
) {
    // 2D vector with the x and y of the tile transform
    let vec_t = tile_pos.center_in_world(grid_size, map_type);

    commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(mesh.add(Mesh::from(shape::Quad::new(Vec2::splat(56.0))))),
                transform: Transform::from_xyz(vec_t.x, vec_t.y, 0.1),
                material: material.add(ColorMaterial::from(
                    Color::hex("3181C6").expect("Error here"),
                )),
                ..Default::default()
            },
            PickableBundle::default(),
        ))
        .insert(HighLight);
}

fn handle_piece_death(mut commands: Commands, mut death_event: EventReader<PieceDeathEvent>) {
    for event in death_event.iter() {
        commands.entity(event.0).despawn_recursive();
    }
}
