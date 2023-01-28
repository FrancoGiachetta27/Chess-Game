use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_ecs_tilemap::{
    helpers::square_grid::neighbors::Neighbors,
    prelude::{TilemapGridSize, TilemapSize, TilemapType},
    tiles::{TilePos, TileStorage},
};
use bevy_mod_picking::{PickableBundle, PickingEvent, SelectionEvent};
use iyes_loopless::prelude::IntoConditionalSystem;

use crate::board::{Tile, TileComponent};

pub struct PiecePlugin;

#[derive(Component)]
pub enum Piece {
    Pawn,
    Rock,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Component)]
pub struct Position;

impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_piece.run_on_event::<PickingEvent>())
            .run();
    }
}

// detects wether a piece has been selected, then gets the tile where
// the player wants it to be moved
fn move_piece(
    mut commands: Commands,
    mut events: EventReader<PickingEvent>,
    mut tile_query: Query<&mut TileComponent>,
    mut transform_q: Query<&mut Transform>,
    piece_type: Query<&Piece>,
    windows: Res<Windows>,
    tile_storage_q: Query<(&TileStorage, &TilemapGridSize, &TilemapSize, &TilemapType)>,
    highlight_pos: Query<Entity, With<Position>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for event in events.iter() {
        let (tile_storage, grid_size, map_size, map_type) = tile_storage_q.single();

        match event {
            PickingEvent::Selection(e) => match e {
                SelectionEvent::JustSelected(s) => {
                    if let Ok(t) = piece_type.get(*s) {
                        let window = windows.get_primary().unwrap();
                        //get the cursor position, if it is on the window
                        if let Some(pos) = window.cursor_position() {
                            // gets the position of tile selected by the player
                            let tile_pos =
                                TilePos::from_world_pos(&pos, map_size, grid_size, map_type)
                                    .unwrap();

                            get_posible_movements(
                                t,
                                &mut commands,
                                tile_storage,
                                grid_size,
                                map_size,
                                map_type,
                                &tile_query,
                                tile_pos,
                                &mut meshes,
                                &mut materials,
                            );
                        }
                    }
                }
                SelectionEvent::JustDeselected(s) => {
                    let window = windows.get_primary().unwrap();

                    //get the cursor position, if it is on the window
                    if let Some(pos) = window.cursor_position() {
                        // gets the position of tile selected by the player
                        let tile_pos =
                            TilePos::from_world_pos(&pos, map_size, grid_size, map_type).unwrap();
                        // gets the entity of the tile just clicked
                        let tile_ent = tile_storage.get(&tile_pos).unwrap();

                        // checks wether the movement is correct
                        if let Ok(_e) = highlight_pos.get(tile_ent) {
                            // despawns the meshes the shows posible movements
                            for ent in highlight_pos.iter() {
                                commands.entity(ent).despawn_recursive();
                            }

                            //get the tile component and change its type to NotEmpty
                            let mut tile_component = tile_query
                                .get_mut(tile_storage.get(&tile_pos).unwrap())
                                .unwrap();

                            tile_component.tile_type = Tile::NotEmpty;

                            // converts the tile position into the transform which is at the
                            // center of the selected tile
                            let new_pos = tile_pos.center_in_world(grid_size, map_type);

                            // gets the reference to the selection's transform to be changed
                            let mut selection_t = transform_q.get_mut(*s).unwrap();

                            let old_tile = TilePos::from_world_pos(
                                &Vec2::new(selection_t.translation.x, selection_t.translation.y),
                                map_size,
                                grid_size,
                                map_type,
                            )
                            .unwrap();

                            //get the tile component and change its type to Empty
                            let mut tile_component = tile_query
                                .get_mut(tile_storage.get(&old_tile).unwrap())
                                .unwrap();

                            tile_component.tile_type = Tile::Empty;

                            selection_t.translation = Vec3::new(new_pos.x, new_pos.y, 1.0);
                        }
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
// shows, with a circle, where the player can move the piece to depending on it's type
fn get_posible_movements(
    piece_type: &Piece,
    commands: &mut Commands,
    tile_storage: &TileStorage,
    grid_size: &TilemapGridSize,
    map_size: &TilemapSize,
    map_type: &TilemapType,
    tile_component: &Query<&mut TileComponent>,
    pos: TilePos,
    mesh: &mut Assets<Mesh>,
    material: &mut Assets<ColorMaterial>,
) {
    match piece_type {
        Piece::Rock => {}
        Piece::Knight => {}
        Piece::Bishop => {}
        Piece::Queen => {}
        Piece::King => {
            let neighbors_positions =
                Neighbors::get_square_neighboring_positions(&pos, map_size, true);

            for pos in neighbors_positions.iter() {
                let tile_ent = tile_storage.get(&pos).unwrap();
                let transform = pos.center_in_world(grid_size, map_type);
                let tile_c = tile_component.get(tile_ent).unwrap();

                //check wether there is a piece on the tile
                if let Tile::Empty = tile_c.tile_type {
                    commands
                        .spawn(MaterialMesh2dBundle {
                            mesh: Mesh2dHandle(
                                mesh.add(Mesh::from(shape::Circle::new(16.0))).into(),
                            ),
                            transform: Transform::from_xyz(transform.x, transform.y, 1.0),
                            material: material.add(ColorMaterial::from(
                                Color::hex("B0A8B9").expect("Error here"),
                            )),
                            ..Default::default()
                        })
                        .insert(Position);
                }
            }
        }
        Piece::Pawn => {}
    }
}

// helper function to spawn the pieces
pub fn spawn_piece(
    commands: &mut Commands,
    piece_type: Piece,
    pos: TilePos,
    tile_storage: &TileStorage,
    tile_query: &mut Query<(&TilePos, &mut TileComponent)>,
    grid_size: &TilemapGridSize,
    map_type: &TilemapType,
    asset: Handle<Image>,
    meshes: &mut Assets<Mesh>,
) {
    // gets the entity of the tile in the given tile position
    let tile_entity = tile_storage.get(&pos).unwrap();

    // // gets the transform relative to the tile position selected
    let (tile_pos, mut component) = {
        let (pos, cmp) = tile_query.get_mut(tile_entity).unwrap();

        (pos.center_in_world(grid_size, map_type), cmp)
    };

    component.tile_type = Tile::NotEmpty;

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
        .insert(piece_type)
        .insert(Name::new("Piece"));
}
