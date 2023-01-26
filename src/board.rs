use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_ecs_tilemap::{
    prelude::{
        TilemapGridSize, TilemapId, TilemapSize, TilemapTexture, TilemapTileSize, TilemapType,
    },
    tiles::{TileBundle, TileColor, TilePos, TileStorage},
    TilemapBundle,
};
use bevy_mod_picking::{PickableBundle, PickingEvent, Selection, SelectionEvent};

use crate::{piece, GameAssets};

pub const TILE_SIZE: f32 = 64.0;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::tilemap_builder)
            .add_startup_system_to_stage(StartupStage::PostStartup, Self::setup_pieces)
            .add_system(Self::select_piece);
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
                let white_tile = (x % 2 == 0 && y % 2 != 0) || (x % 2 != 0 && y % 2 == 0);
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
        let map_type = TilemapType::default();

        commands.entity(tilemap_entity).insert(TilemapBundle {
            grid_size,
            map_type,
            size: map_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_handle),
            tile_size,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        });
    }

    // Spawn the pieces in their correct positions
    fn setup_pieces(
        mut commands: Commands,
        game_assets: Res<GameAssets>,
        tile_storage_q: Query<(&TileStorage, &TilemapGridSize, &TilemapType)>,
        tile_query: Query<&TilePos>,
        mut meshes: ResMut<Assets<Mesh>>,
    ) {
        for (tile_storage, grid_size, map_type) in tile_storage_q.iter() {
            //Blacks

            // spawn black rocks
            Self::spawn_piece(
                &mut commands,
                TilePos { x: 0, y: 7 },
                tile_storage,
                &tile_query,
                grid_size,
                map_type,
                game_assets.black_rock.clone(),
                &mut meshes,
            );
            Self::spawn_piece(
                &mut commands,
                TilePos { x: 7, y: 7 },
                tile_storage,
                &tile_query,
                grid_size,
                map_type,
                game_assets.black_rock.clone(),
                &mut meshes,
            );

            // spawn black knights
            Self::spawn_piece(
                &mut commands,
                TilePos { x: 1, y: 7 },
                tile_storage,
                &tile_query,
                grid_size,
                map_type,
                game_assets.black_knight.clone(),
                &mut meshes,
            );
            Self::spawn_piece(
                &mut commands,
                TilePos { x: 6, y: 7 },
                tile_storage,
                &tile_query,
                grid_size,
                map_type,
                game_assets.black_knight.clone(),
                &mut meshes,
            );

            // spawn black bishops
            Self::spawn_piece(
                &mut commands,
                TilePos { x: 2, y: 7 },
                tile_storage,
                &tile_query,
                grid_size,
                map_type,
                game_assets.black_bishop.clone(),
                &mut meshes,
            );
            Self::spawn_piece(
                &mut commands,
                TilePos { x: 5, y: 7 },
                tile_storage,
                &tile_query,
                grid_size,
                map_type,
                game_assets.black_bishop.clone(),
                &mut meshes,
            );

            // spawn black queen
            Self::spawn_piece(
                &mut commands,
                TilePos { x: 3, y: 7 },
                tile_storage,
                &tile_query,
                grid_size,
                map_type,
                game_assets.black_queen.clone(),
                &mut meshes,
            );

            // spawn black king
            Self::spawn_piece(
                &mut commands,
                TilePos { x: 4, y: 7 },
                tile_storage,
                &tile_query,
                grid_size,
                map_type,
                game_assets.black_king.clone(),
                &mut meshes,
            );

            // spawn black pawns
            for x in 0..8 {
                Self::spawn_piece(
                    &mut commands,
                    TilePos { x, y: 6 },
                    tile_storage,
                    &tile_query,
                    grid_size,
                    map_type,
                    game_assets.black_pawn.clone(),
                    &mut meshes,
                );
            }

            // WHITES

            // spawn white rocks
            Self::spawn_piece(
                &mut commands,
                TilePos { x: 0, y: 0 },
                tile_storage,
                &tile_query,
                grid_size,
                map_type,
                game_assets.white_rock.clone(),
                &mut meshes,
            );
            Self::spawn_piece(
                &mut commands,
                TilePos { x: 7, y: 0 },
                tile_storage,
                &tile_query,
                grid_size,
                map_type,
                game_assets.white_rock.clone(),
                &mut meshes,
            );

            // spawn white knights
            Self::spawn_piece(
                &mut commands,
                TilePos { x: 1, y: 0 },
                tile_storage,
                &tile_query,
                grid_size,
                map_type,
                game_assets.white_knight.clone(),
                &mut meshes,
            );
            Self::spawn_piece(
                &mut commands,
                TilePos { x: 6, y: 0 },
                tile_storage,
                &tile_query,
                grid_size,
                map_type,
                game_assets.white_knight.clone(),
                &mut meshes,
            );

            // spawn white bishops
            Self::spawn_piece(
                &mut commands,
                TilePos { x: 2, y: 0 },
                tile_storage,
                &tile_query,
                grid_size,
                map_type,
                game_assets.white_bishop.clone(),
                &mut meshes,
            );
            Self::spawn_piece(
                &mut commands,
                TilePos { x: 5, y: 0 },
                tile_storage,
                &tile_query,
                grid_size,
                map_type,
                game_assets.white_bishop.clone(),
                &mut meshes,
            );

            // spawn white queen
            Self::spawn_piece(
                &mut commands,
                TilePos { x: 3, y: 0 },
                tile_storage,
                &tile_query,
                grid_size,
                map_type,
                game_assets.white_queen.clone(),
                &mut meshes,
            );

            // spawn white king
            Self::spawn_piece(
                &mut commands,
                TilePos { x: 4, y: 0 },
                tile_storage,
                &tile_query,
                grid_size,
                map_type,
                game_assets.white_king.clone(),
                &mut meshes,
            );

            // spawn white pawns
            for x in 0..8 {
                Self::spawn_piece(
                    &mut commands,
                    TilePos { x, y: 1 },
                    tile_storage,
                    &tile_query,
                    grid_size,
                    map_type,
                    game_assets.white_pawn.clone(),
                    &mut meshes,
                );
            }
        }
    }

    // helper function to spawn the pieces
    fn spawn_piece(
        commands: &mut Commands,
        pos: TilePos,
        tile_storage: &TileStorage,
        tile_query: &Query<&TilePos>,
        grid_size: &TilemapGridSize,
        map_type: &TilemapType,
        asset: Handle<Image>,
        meshes: &mut Assets<Mesh>,
    ) {
        // gets the entity of the tile in the given tile position
        let tile_entity = tile_storage.get(&pos).unwrap();
        // // gets the transform relative to the tile position selected
        let tile_pos = tile_query
            .get(tile_entity)
            .unwrap()
            .center_in_world(&grid_size, &map_type);

        commands
            .spawn((
                SpriteBundle {
                    texture: asset.clone(),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(64.0, 64.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(tile_pos.x, tile_pos.y, 1.0),
                    ..default()
                },
                meshes.add(Mesh::from(shape::Quad::new(Vec2::splat(64.0)))),
                PickableBundle::default(),
            ))
            .insert(Name::new("Piece"));
    }

    // detects wether a piece has been selected, then gets the tile where
    // the player wants it to be moved
    fn select_piece(
        mut commands: Commands,
        mut events: EventReader<PickingEvent>,
        mut transform_q: Query<&mut Transform>,
        windows: Res<Windows>,
        tile_storage_q: Query<(&TileStorage, &TilemapGridSize, &TilemapSize, &TilemapType)>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        for event in events.iter() {
            let (tile_storage, grid_size, map_size, map_type) = tile_storage_q.single();
            match event {
                PickingEvent::Selection(e) => match e {
                    SelectionEvent::JustSelected(_s) => {
                        let window = windows.get_primary().unwrap();

                        //get the cursor position, if it is on the window
                        if let Some(pos) = window.cursor_position() {
                            // gets the position of tile selected by the player
                            let tile_pos =
                                TilePos::from_world_pos(&pos, map_size, grid_size, map_type)
                                    .unwrap();

                            piece::get_posible_movements(
                                &mut commands,
                                tile_storage,
                                grid_size,
                                map_size,
                                map_type,
                                tile_pos,
                                &mut meshes,
                                &mut materials,
                            );
                        }
                    }
                    SelectionEvent::JustDeselected(s) => {
                        let window = windows.get_primary().unwrap();

                        //get the cursor position, if it is on the window
                        if let Some(pos) = window.cursor_position() {
                            // gets the position of tile selected by the player
                            let tile_pos =
                                TilePos::from_world_pos(&pos, map_size, grid_size, map_type)
                                    .unwrap();

                            // converts the tile position into the transform which is at the
                            // center of the selected tile
                            let new_pos = tile_pos.center_in_world(grid_size, map_type);
                            // gets the reference to the selection's transform to be changed
                            let mut selection_t = transform_q.get_mut(*s).unwrap();

                            selection_t.translation = Vec3::new(new_pos.x, new_pos.y, 1.0);
                        } else {
                            info!("No Tile in this position");
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
