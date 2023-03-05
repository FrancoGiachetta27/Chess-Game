use bevy::{
    prelude::{
        shape, Assets, Color, Commands, Component, Handle, Image, Mesh, Name, Query, Transform,
        Vec2,
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
pub struct Bishop {
    pub team: Team,
}

impl Bishop {
    pub fn movement(
        self,
        commands: &mut Commands,
        tile_storage: &TileStorage,
        grid_size: &TilemapGridSize,
        map_size: &TilemapSize,
        map_type: &TilemapType,
        tile_state_q: &mut Query<&mut TileState>,
        piece_type: &Query<&PieceType>,
        tile_pos: TilePos,
        mesh: &mut Assets<Mesh>,
        material: &mut Assets<ColorMaterial>,
    ) {
        let dir: Vec<SquareDirection> = vec![
            SquareDirection::NorthWest,
            SquareDirection::NorthEast,
            SquareDirection::SouthWest,
            SquareDirection::SouthEast,
        ];
        let tile_neighbors = Neighbors::get_square_neighboring_positions(&tile_pos, map_size, true);

        // spawn in every specified direction
        dir.iter().for_each(|dir| {
            if let Some(pos) = tile_neighbors.get(*dir) {
                let mut current_pos = *pos;
                let mut tile_ent = tile_storage.get(&pos).unwrap();
                //tile state
                let mut tile_s = tile_state_q.get_mut(tile_ent).unwrap();

                // checks if the first the closest tile is empty or else
                // if it has a piece with opposite color of the selection
                if matches!(tile_s.tile_type, Tile::Empty) {
                    tile_s.tile_type = Tile::HighLighted;
                    highlight_tile(commands, grid_size, map_type, &pos, mesh, material);

                    // gets the neighbor which is in the direction specified, and spawns the circle, it
                    // keeps doing it until there's a piece or it reaches the end
                    while let Some(n_pos) =
                        Neighbors::get_square_neighboring_positions(&current_pos, map_size, true)
                            .get(*dir)
                    {
                        tile_ent = tile_storage.get(&n_pos).unwrap();
                        tile_s = tile_state_q.get_mut(tile_ent).unwrap();

                        // changes the position to be the neighbor's last accessed
                        current_pos = TilePos {
                            x: n_pos.x,
                            y: n_pos.y,
                        };

                        if let Tile::Empty = tile_s.tile_type {
                            tile_s.tile_type = Tile::HighLighted;
                            highlight_tile(commands, grid_size, map_type, &n_pos, mesh, material);
                        } else if let Some(e) = tile_s.piece_ent {
                            let piece = piece_type.get(e).unwrap();

                            // checks if it's color is the opposite of the selection's
                            if piece.get_team() != self.team {
                                tile_s.tile_type = Tile::HighLighted;
                                highlight_tile(
                                    commands, grid_size, map_type, &n_pos, mesh, material,
                                );
                            }

                            break;
                        }
                    }
                } else {
                    let piece = piece_type.get(tile_s.piece_ent.unwrap()).unwrap();

                    if piece.get_team() != self.team {
                        tile_s.tile_type = Tile::HighLighted;
                        highlight_tile(commands, grid_size, map_type, &pos, mesh, material);
                    }
                }
            }
        });
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
            .insert(PieceType::Bishop(Bishop { team: piece_team }))
            .insert(Name::new("Piece"))
            .id();

        tile_state.tile_type = Tile::NotEmpty;
        tile_state.piece_ent = Some(piece_ent);
    }
}
