use bevy::{
    prelude::{
        info, shape, Assets, Color, Commands, Component, Entity, Handle, Image, Mesh, Name, Query,
        Transform, Vec2,
    },
    sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle, Sprite, SpriteBundle},
    utils::default,
};
use bevy_ecs_tilemap::{
    helpers::square_grid::neighbors::{Neighbors, SquareDirection},
    prelude::{TilemapGridSize, TilemapSize, TilemapType},
    tiles::{TilePos, TileStorage},
};
use bevy_mod_picking::PickableBundle;

use crate::{
    board::{Tile, TileState},
    piece::{highlight_tile, PieceType, Team},
};

#[derive(Component, Clone, Copy)]
pub struct Pawn {
    initial_pos: TilePos,
    pub team: Team,
}

impl Pawn {
    pub fn movement(
        self,
        commands: &mut Commands,
        tile_pos: TilePos,
        tile_storage: &TileStorage,
        tile_state_q: &mut Query<&mut TileState>,
        piece_type: &Query<&PieceType>,
        grid_size: &TilemapGridSize,
        map_size: &TilemapSize,
        map_type: &TilemapType,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
    ) {
        let square_neighbors =
            Neighbors::get_square_neighboring_positions(&tile_pos, map_size, true);
        let neighbor_directions: [SquareDirection; 3];

        if let Team::White = self.team {
            neighbor_directions = [
                SquareDirection::NorthWest,
                SquareDirection::North,
                SquareDirection::NorthEast,
            ];
        } else {
            neighbor_directions = [
                SquareDirection::SouthWest,
                SquareDirection::South,
                SquareDirection::SouthEast,
            ];
        }

        neighbor_directions.into_iter().for_each(|dir| {
            if let Some(front_neighbor) = square_neighbors.get(dir) {
                let mut tile_ent = tile_storage.get(&front_neighbor).unwrap();
                let mut tile_s = tile_state_q.get_mut(tile_ent).unwrap();
                match dir {
                    SquareDirection::North | SquareDirection::South => {
                        if let Tile::Empty = tile_s.tile_type {
                            tile_s.tile_type = Tile::HighLighted;
                            highlight_tile(
                                commands,
                                grid_size,
                                map_type,
                                front_neighbor,
                                meshes,
                                materials,
                            );

                            // checks if the pawn still is at its initial position
                            if self.initial_pos == tile_pos {
                                let next_neighbors = Neighbors::get_square_neighboring_positions(
                                    front_neighbor,
                                    map_size,
                                    false,
                                );
                                let next_front_neighbor = next_neighbors.get(dir).unwrap();
                                tile_ent = tile_storage.get(&next_front_neighbor).unwrap();
                                tile_s = tile_state_q.get_mut(tile_ent).unwrap();

                                tile_s.tile_type = Tile::HighLighted;
                                highlight_tile(
                                    commands,
                                    grid_size,
                                    map_type,
                                    next_front_neighbor,
                                    meshes,
                                    materials,
                                );
                            }
                        }
                    }
                    _ => {
                        if let Some(e) = tile_s.piece_ent {
                            let piece = piece_type.get(e).unwrap();

                            // checks if it's color is the opposite of the selection's
                            if piece.get_team() != self.team {
                                tile_s.tile_type = Tile::HighLighted;
                                highlight_tile(
                                    commands,
                                    grid_size,
                                    map_type,
                                    front_neighbor,
                                    meshes,
                                    materials,
                                );
                            }
                        }
                    }
                }
            }
        })
    }
}

// helper function to spawn the pieces
pub fn spawn_piece(
    commands: &mut Commands,
    piece_team: Team,
    pos: TilePos,
    tile_storage: &TileStorage,
    tile_query: &mut Query<(&TilePos, &mut TileState)>,
    grid_size: &TilemapGridSize,
    map_type: &TilemapType,
    asset: Handle<Image>,
    meshes: &mut Assets<Mesh>,
    material: &mut Assets<ColorMaterial>,
) {
    // gets the entity of the tile in the given tile position
    if let Some(tile_entity) = tile_storage.get(&pos) {
        // gets the transform relative to the tile position selected
        // and the state of the it
        let (tile_pos, mut tile_state) = tile_query.get_mut(tile_entity).unwrap();
        let vector_pos = tile_pos.center_in_world(grid_size, map_type);

        let piece_ent = commands
            .spawn((SpriteBundle {
                texture: asset.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(64.0, 64.0)),
                    ..default()
                },
                transform: Transform::from_xyz(vector_pos.x, vector_pos.y, 1.0),
                ..default()
            },))
            .insert((
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Mesh::from(shape::Quad::new(Vec2::splat(64.0))))),
                    transform: Transform::from_xyz(vector_pos.x, vector_pos.y, 0.1),
                    material: material.add(ColorMaterial::from(Color::NONE)),
                    ..Default::default()
                },
                PickableBundle::default(),
            ))
            .insert(PieceType::Pawn(Pawn {
                initial_pos: pos,
                team: piece_team,
            }))
            .insert(Name::new("Piece"))
            .id();

        tile_state.tile_type = Tile::NotEmpty;
        tile_state.piece_ent = Some(piece_ent);
    }
}
