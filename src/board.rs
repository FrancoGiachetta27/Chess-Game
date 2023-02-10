use bevy::prelude::*;
use bevy_ecs_tilemap::{
    prelude::{
        get_tilemap_center_transform, TilemapGridSize, TilemapId, TilemapSize, TilemapTexture,
        TilemapTileSize, TilemapType,
    },
    tiles::{TileBundle, TileColor, TilePos, TileStorage},
    TilemapBundle,
};

use crate::{
    piece::{self, Piece, Team},
    GameAssets,
};

pub const TILE_SIZE: f32 = 64.0;

#[derive(Debug)]
pub enum Tile {
    Empty,
    NotEmpty,
    WithCircle,
}

#[derive(Component, Debug)]
pub struct TileState {
    pub tile_type: Tile,
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::Startup, Self::tilemap_builder)
            .add_startup_system_to_stage(StartupStage::PostStartup, Self::setup_pieces);
    }
}

impl BoardPlugin {
    // Creates a tilemap where the pieces will be set
    fn tilemap_builder(mut commands: Commands, asset_server: Res<AssetServer>) {
        let texture_handle: Handle<Image> = asset_server.load("tile.png");
        let map_size = TilemapSize { x: 8, y: 8 };
        let tilemap_entity = commands.spawn_empty().id(); // the entity associated to the tilemap
        let mut tile_storage = TileStorage::empty(map_size); // the storage for tiles

        for x in 0..map_size.x {
            for y in 0..map_size.y {
                let white_tile = ((x % 2 == 0) && (y % 2 != 0)) || ((x % 2 != 0) && (y % 2 == 0));
                let color: TileColor = match white_tile {
                    true => Color::rgba(255.0, 255.0, 255.0, 1.0).into(),
                    false => Color::rgba(0.0, 0.0, 0.0, 1.0).into(),
                };
                let tile_pos = TilePos { x, y };
                let tile_entity = commands
                    .spawn(TileBundle {
                        color,
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_entity),
                        ..default()
                    })
                    .insert(TileState {
                        tile_type: Tile::Empty,
                    })
                    .insert(Name::new(format!("Tile ({}, {})", x, y)))
                    .id();

                tile_storage.set(&tile_pos, tile_entity);
            }
        }

        let tile_size = TilemapTileSize {
            x: TILE_SIZE,
            y: TILE_SIZE,
        };
        let grid_size: TilemapGridSize = tile_size.into();
        let map_type = TilemapType::Square;

        commands.entity(tilemap_entity).insert(TilemapBundle {
            grid_size,
            size: map_size,
            storage: tile_storage,
            map_type,
            texture: TilemapTexture::Single(texture_handle),
            tile_size,
            transform: Transform::from_translation(
                get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0).translation
                    + Transform::from_xyz(TILE_SIZE * 4.0, TILE_SIZE * 4.0, 0.0).translation,
            ),
            ..default()
        });
    }

    // Spawn the pieces in their correct positions
    fn setup_pieces(
        mut commands: Commands,
        game_assets: Res<GameAssets>,
        tile_storage_q: Query<(&TileStorage, &TilemapGridSize, &TilemapType)>,
        mut tile_query: Query<(&TilePos, &mut TileState)>,
        mut meshes: ResMut<Assets<Mesh>>,
    ) {
        for (tile_storage, grid_size, map_type) in tile_storage_q.iter() {
            //Blacks

            // spawn black rocks
            piece::spawn_piece(
                &mut commands,
                Piece::Rock,
                Team::Black,
                TilePos { x: 0, y: 7 },
                tile_storage,
                &mut tile_query,
                grid_size,
                map_type,
                game_assets.black_rock.clone(),
                &mut meshes,
            );
            piece::spawn_piece(
                &mut commands,
                Piece::Rock,
                Team::Black,
                TilePos { x: 7, y: 7 },
                tile_storage,
                &mut tile_query,
                grid_size,
                map_type,
                game_assets.black_rock.clone(),
                &mut meshes,
            );

            // spawn black knights
            piece::spawn_piece(
                &mut commands,
                Piece::Knight,
                Team::Black,
                TilePos { x: 1, y: 7 },
                tile_storage,
                &mut tile_query,
                grid_size,
                map_type,
                game_assets.black_knight.clone(),
                &mut meshes,
            );
            piece::spawn_piece(
                &mut commands,
                Piece::Knight,
                Team::Black,
                TilePos { x: 6, y: 7 },
                tile_storage,
                &mut tile_query,
                grid_size,
                map_type,
                game_assets.black_knight.clone(),
                &mut meshes,
            );

            // spawn black bishops
            piece::spawn_piece(
                &mut commands,
                Piece::Bishop,
                Team::Black,
                TilePos { x: 2, y: 7 },
                tile_storage,
                &mut tile_query,
                grid_size,
                map_type,
                game_assets.black_bishop.clone(),
                &mut meshes,
            );
            piece::spawn_piece(
                &mut commands,
                Piece::Bishop,
                Team::Black,
                TilePos { x: 5, y: 7 },
                tile_storage,
                &mut tile_query,
                grid_size,
                map_type,
                game_assets.black_bishop.clone(),
                &mut meshes,
            );

            // spawn black queen
            piece::spawn_piece(
                &mut commands,
                Piece::Queen,
                Team::Black,
                TilePos { x: 3, y: 7 },
                tile_storage,
                &mut tile_query,
                grid_size,
                map_type,
                game_assets.black_queen.clone(),
                &mut meshes,
            );

            // spawn black king
            piece::spawn_piece(
                &mut commands,
                Piece::King,
                Team::Black,
                TilePos { x: 4, y: 7 },
                tile_storage,
                &mut tile_query,
                grid_size,
                map_type,
                game_assets.black_king.clone(),
                &mut meshes,
            );

            // spawn black pawns
            for x in 0..8 {
                piece::spawn_piece(
                    &mut commands,
                    Piece::Pawn,
                    Team::Black,
                    TilePos { x, y: 6 },
                    tile_storage,
                    &mut tile_query,
                    grid_size,
                    map_type,
                    game_assets.black_pawn.clone(),
                    &mut meshes,
                );
            }

            // WHITES

            // spawn white rocks
            piece::spawn_piece(
                &mut commands,
                Piece::Rock,
                Team::White,
                TilePos { x: 0, y: 0 },
                tile_storage,
                &mut tile_query,
                grid_size,
                map_type,
                game_assets.white_rock.clone(),
                &mut meshes,
            );
            piece::spawn_piece(
                &mut commands,
                Piece::Rock,
                Team::White,
                TilePos { x: 7, y: 0 },
                tile_storage,
                &mut tile_query,
                grid_size,
                map_type,
                game_assets.white_rock.clone(),
                &mut meshes,
            );

            // spawn white knights
            piece::spawn_piece(
                &mut commands,
                Piece::Knight,
                Team::White,
                TilePos { x: 1, y: 0 },
                tile_storage,
                &mut tile_query,
                grid_size,
                map_type,
                game_assets.white_knight.clone(),
                &mut meshes,
            );
            piece::spawn_piece(
                &mut commands,
                Piece::Knight,
                Team::White,
                TilePos { x: 6, y: 0 },
                tile_storage,
                &mut tile_query,
                grid_size,
                map_type,
                game_assets.white_knight.clone(),
                &mut meshes,
            );

            // spawn white bishops
            piece::spawn_piece(
                &mut commands,
                Piece::Bishop,
                Team::White,
                TilePos { x: 2, y: 0 },
                tile_storage,
                &mut tile_query,
                grid_size,
                map_type,
                game_assets.white_bishop.clone(),
                &mut meshes,
            );
            piece::spawn_piece(
                &mut commands,
                Piece::Bishop,
                Team::White,
                TilePos { x: 5, y: 0 },
                tile_storage,
                &mut tile_query,
                grid_size,
                map_type,
                game_assets.white_bishop.clone(),
                &mut meshes,
            );

            // spawn white queen
            piece::spawn_piece(
                &mut commands,
                Piece::Queen,
                Team::White,
                TilePos { x: 3, y: 0 },
                tile_storage,
                &mut tile_query,
                grid_size,
                map_type,
                game_assets.white_queen.clone(),
                &mut meshes,
            );

            // spawn white king
            piece::spawn_piece(
                &mut commands,
                Piece::King,
                Team::White,
                TilePos { x: 4, y: 0 },
                tile_storage,
                &mut tile_query,
                grid_size,
                map_type,
                game_assets.white_king.clone(),
                &mut meshes,
            );

            // spawn white pawns
            for x in 0..8 {
                piece::spawn_piece(
                    &mut commands,
                    Piece::Pawn,
                    Team::White,
                    TilePos { x, y: 1 },
                    tile_storage,
                    &mut tile_query,
                    grid_size,
                    map_type,
                    game_assets.white_pawn.clone(),
                    &mut meshes,
                );
            }
        }
    }
}
