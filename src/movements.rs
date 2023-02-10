use bevy::{
    prelude::{Assets, Commands, Entity, Mesh, Query},
    sprite::ColorMaterial,
};
use bevy_ecs_tilemap::{
    helpers::square_grid::neighbors::{Neighbors, SquareDirection},
    prelude::{TilemapGridSize, TilemapSize, TilemapType},
    tiles::{TilePos, TileStorage},
};

use crate::{
    board::{Tile, TileState},
    piece::{spawn_circle, Team},
};

pub fn pawn_movement(
    commands: &mut Commands,
    selection: Entity,
    tile_pos: TilePos,
    tile_storage: &TileStorage,
    tile_state_q: &mut Query<&mut TileState>,
    grid_size: &TilemapGridSize,
    map_size: &TilemapSize,
    map_type: &TilemapType,
    piece_team: &Query<&Team>,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    let neighbor_position = Neighbors::get_square_neighboring_positions(&tile_pos, map_size, true);

    let north_neighbor: &TilePos;

    if let Team::White = piece_team.get(selection).unwrap() {
        north_neighbor = neighbor_position.get(SquareDirection::North).unwrap();
    } else {
        north_neighbor = neighbor_position.get(SquareDirection::South).unwrap();
    }

    let tile_ent = tile_storage.get(&north_neighbor).unwrap();
    let mut tile_s = tile_state_q.get_mut(tile_ent).unwrap();

    if let Tile::Empty = tile_s.tile_type {
        tile_s.tile_type = Tile::WithCircle;
        spawn_circle(
            commands,
            grid_size,
            map_type,
            north_neighbor,
            meshes,
            materials,
        );
    }
}

pub fn king_movement(
    commands: &mut Commands,
    tile_storage: &TileStorage,
    tile_pos: TilePos,
    tile_state_q: &mut Query<&mut TileState>,
    grid_size: &TilemapGridSize,
    map_size: &TilemapSize,
    map_type: &TilemapType,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    let neighbors_positions =
        Neighbors::get_square_neighboring_positions(&tile_pos, map_size, true);
    let mut neigh_ent: Entity;

    for pos in neighbors_positions.iter() {
        neigh_ent = tile_storage.get(&pos).unwrap();
        //tile state
        let mut tile_s = tile_state_q.get_mut(neigh_ent).unwrap();

        //check wether there is a piece on the tile
        if let Tile::Empty = tile_s.tile_type {
            tile_s.tile_type = Tile::WithCircle;
            spawn_circle(commands, grid_size, map_type, pos, meshes, materials);
        }
    }
}

pub fn knight_movement(
    commands: &mut Commands,
    tile_storage: &TileStorage,
    tile_pos: TilePos,
    tile_state_q: &mut Query<&mut TileState>,
    grid_size: &TilemapGridSize,
    map_type: &TilemapType,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    let directions: Vec<(i32, i32)> = vec![
        (1, 2),
        (-1, 2),
        (1, -2),
        (-1, -2),
        (2, 1),
        (2, -1),
        (-2, 1),
        (-2, -1),
    ];

    for direction in directions.iter() {
        // get the posible move's position
        let (x, y) = (
            tile_pos.x as i32 + direction.0,
            tile_pos.y as i32 + direction.1,
        );

        if (x >= 0 && x <= 7) && (y >= 0 && y <= 7) {
            let new_pos = TilePos {
                x: x as u32,
                y: y as u32,
            };

            if let Some(neigh_ent) = tile_storage.get(&new_pos) {
                //tile state
                let mut tile_s = tile_state_q.get_mut(neigh_ent).unwrap();

                //check wether there is a piece on the tile
                if let Tile::Empty = tile_s.tile_type {
                    tile_s.tile_type = Tile::WithCircle;
                    spawn_circle(commands, grid_size, map_type, &new_pos, meshes, materials);
                }
            }
        }
    }
}

pub fn sequencial_pieces(
    commands: &mut Commands,
    tile_storage: &TileStorage,
    grid_size: &TilemapGridSize,
    map_size: &TilemapSize,
    map_type: &TilemapType,
    tile_state_q: &mut Query<&mut TileState>,
    pos: TilePos,
    mesh: &mut Assets<Mesh>,
    material: &mut Assets<ColorMaterial>,
    neighbor_directions: Vec<SquareDirection>,
) {
    let tile_neighbors = Neighbors::get_square_neighboring_positions(&pos, map_size, true);

    //spawn in every specified direction
    neighbor_directions.iter().for_each(|dir| {
        if let Some(pos) = tile_neighbors.get(*dir) {
            let mut new_pos = *pos;
            let mut tile_ent = tile_storage.get(&pos).unwrap();
            //tile state
            let mut tile_s = tile_state_q.get_mut(tile_ent).unwrap();

            //check wether there is a piece on the tile
            if let Tile::Empty = tile_s.tile_type {
                tile_s.tile_type = Tile::WithCircle;
                spawn_circle(commands, grid_size, map_type, &pos, mesh, material);

                //gets the neighbor which is in the direction specified, and spawns the circle, it
                //keeps doing it until there's a piece or it reaches the end
                while let Some(n) =
                    Neighbors::get_square_neighboring_positions(&new_pos, map_size, true).get(*dir)
                {
                    tile_ent = tile_storage.get(&n).unwrap();
                    tile_s = tile_state_q.get_mut(tile_ent).unwrap();

                    // changes the position to be the last accessed neighbor's
                    new_pos = TilePos { x: n.x, y: n.y };
                    if let Tile::Empty = tile_s.tile_type {
                        tile_s.tile_type = Tile::WithCircle;
                        spawn_circle(commands, grid_size, map_type, &n, mesh, material);
                    } else {
                        break;
                    }
                }
            }
        }
    });
}
