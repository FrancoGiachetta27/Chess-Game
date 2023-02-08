use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    transform::TransformSystem,
};
use bevy_ecs_tilemap::{
    helpers::square_grid::neighbors::{Neighbors, SquareDirection},
    prelude::{TilemapGridSize, TilemapSize, TilemapType},
    tiles::{TilePos, TileStorage},
};
use bevy_mod_picking::{selection::Selection, PickableBundle, PickingEvent, SelectionEvent};
use iyes_loopless::prelude::IntoConditionalSystem;

use crate::board::{Tile, TileState};

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
        app.add_system(get_piece_movements.run_on_event::<PickingEvent>())
            .add_system(
                move_piece
                    .run_on_event::<PickingEvent>()
                    .after(TransformSystem::TransformPropagate),
            )
            .run();
    }
}

// detects wether a piece has been selected and shows, with a circle, where the player can move
// the piece to, depending on it's type

fn get_piece_movements(
    mut commands: Commands,
    mut events: EventReader<PickingEvent>,
    mut tile_state_q: Query<&mut TileState>,
    piece_type: Query<&Piece>,
    windows: Res<Windows>,
    tile_storage_q: Query<(&TileStorage, &TilemapGridSize, &TilemapSize, &TilemapType)>,
    transform_q: Query<&mut Transform>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for event in events.iter() {
        let (tile_storage, grid_size, map_size, map_type) = tile_storage_q.single();

        if let PickingEvent::Selection(e) = event {
            if let SelectionEvent::JustSelected(s) = e {
                if let Ok(piece_t) = piece_type.get(*s) {
                    //get the cursor position, if it is on the window
                    if let Ok(t) = transform_q.get(*s) {
                        let pos = Vec2::new(t.translation.x, t.translation.y);
                        // gets the position of tile selected by the player
                        let tile_pos =
                            TilePos::from_world_pos(&pos, map_size, grid_size, map_type).unwrap();

                        match piece_t {
                            Piece::Rock => {
                                let neighbor_directions: Vec<SquareDirection> = vec![
                                    SquareDirection::North,
                                    SquareDirection::South,
                                    SquareDirection::West,
                                    SquareDirection::East,
                                ];

                                get_complex_movements(
                                    &mut commands,
                                    tile_storage,
                                    grid_size,
                                    map_size,
                                    map_type,
                                    &mut tile_state_q,
                                    tile_pos,
                                    &mut meshes,
                                    &mut materials,
                                    neighbor_directions,
                                );
                            }
                            Piece::Knight => {}
                            Piece::Bishop => {
                                let neighbor_directions: Vec<SquareDirection> = vec![
                                    SquareDirection::NorthWest,
                                    SquareDirection::NorthEast,
                                    SquareDirection::SouthWest,
                                    SquareDirection::SouthEast,
                                ];

                                get_complex_movements(
                                    &mut commands,
                                    tile_storage,
                                    grid_size,
                                    map_size,
                                    map_type,
                                    &mut tile_state_q,
                                    tile_pos,
                                    &mut meshes,
                                    &mut materials,
                                    neighbor_directions,
                                );
                            }
                            Piece::Queen => {
                                let neighbor_directions: Vec<SquareDirection> = vec![
                                    SquareDirection::North,
                                    SquareDirection::South,
                                    SquareDirection::West,
                                    SquareDirection::East,
                                    SquareDirection::NorthWest,
                                    SquareDirection::NorthEast,
                                    SquareDirection::SouthWest,
                                    SquareDirection::SouthEast,
                                ];

                                get_complex_movements(
                                    &mut commands,
                                    tile_storage,
                                    grid_size,
                                    map_size,
                                    map_type,
                                    &mut tile_state_q,
                                    tile_pos,
                                    &mut meshes,
                                    &mut materials,
                                    neighbor_directions,
                                );
                            }
                            Piece::King => {
                                let neighbors_positions =
                                    Neighbors::get_square_neighboring_positions(
                                        &tile_pos, map_size, true,
                                    );

                                for pos in neighbors_positions.iter() {
                                    let neigh_ent = tile_storage.get(&pos).unwrap();
                                    //tile state
                                    let mut tile_s = tile_state_q.get_mut(neigh_ent).unwrap();

                                    //check wether there is a piece on the tile
                                    if let Tile::Empty = tile_s.tile_type {
                                        tile_s.tile_type = Tile::WithCircle;
                                        spawn_circle(
                                            &mut commands,
                                            grid_size,
                                            map_type,
                                            pos,
                                            &mut meshes,
                                            &mut materials,
                                        );
                                    }
                                }
                            }
                            Piece::Pawn => {
                                let neighbor_position = Neighbors::get_square_neighboring_positions(
                                    &tile_pos, map_size, true,
                                );

                                let north_neighbor =
                                    neighbor_position.get(SquareDirection::North).unwrap();
                                let tile_ent = tile_storage.get(&north_neighbor).unwrap();
                                let mut tile_s = tile_state_q.get_mut(tile_ent).unwrap();

                                if let Tile::Empty = tile_s.tile_type {
                                    tile_s.tile_type = Tile::WithCircle;
                                    spawn_circle(
                                        &mut commands,
                                        grid_size,
                                        map_type,
                                        north_neighbor,
                                        &mut meshes,
                                        &mut materials,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn move_piece(
    mut commands: Commands,
    mut events: EventReader<PickingEvent>,
    mut tile_state_q: Query<&mut TileState>,
    mut transform_q: Query<&mut Transform>,
    tile_storage_q: Query<(&TileStorage, &TilemapGridSize, &TilemapSize, &TilemapType)>,
    highlight_pos: Query<Entity, With<Position>>,
    selected_pos: Query<Entity, (Changed<Selection>, With<Position>)>,
) {
    for event in events.iter() {
        if let PickingEvent::Selection(e) = event {
            if let SelectionEvent::JustDeselected(s) = e {
                let (tile_storage, grid_size, map_size, map_type) = tile_storage_q.single();

                //get the entity of the selected circle
                for selection in selected_pos.iter() {
                    //get the transform of the selected circle
                    if let Ok(transform_s) = transform_q.get(selection) {
                        // convert the transform into a 2d vec
                        let pos = Vec2::new(transform_s.translation.x, transform_s.translation.y);
                        // get the position of tile selected by the player
                        let tile_pos =
                            TilePos::from_world_pos(&pos, map_size, grid_size, map_type).unwrap();

                        // get the state of the entity of the tile just clicked
                        let mut tile_s = tile_state_q
                            .get_mut(tile_storage.get(&tile_pos).unwrap())
                            .unwrap();

                        // checks wether the movement is correct
                        if let Tile::WithCircle = tile_s.tile_type {
                            // converts the tile position into the transform which is at the
                            // center of the selected tile
                            let new_pos = tile_pos.center_in_world(grid_size, map_type);

                            // gets the reference to the selection's transform to be changed
                            let mut selection_t = transform_q.get_mut(*s).unwrap();

                            tile_s.tile_type = Tile::NotEmpty;

                            // get the old tile position
                            let old_tile = TilePos::from_world_pos(
                                &Vec2::new(selection_t.translation.x, selection_t.translation.y),
                                map_size,
                                grid_size,
                                map_type,
                            )
                            .unwrap();

                            //get the old tile state and change its type to Empty
                            tile_s = tile_state_q
                                .get_mut(tile_storage.get(&old_tile).unwrap())
                                .unwrap();

                            tile_s.tile_type = Tile::Empty;

                            selection_t.translation = Vec3::new(new_pos.x, new_pos.y, 1.0);
                        }
                    }
                }

                // despawns the meshes the shows posible movements
                for ent in highlight_pos.iter() {
                    reset_neighbors(
                        &mut commands,
                        &mut tile_state_q,
                        &transform_q,
                        tile_storage,
                        grid_size,
                        map_size,
                        map_type,
                        ent,
                    )
                }
            }
        }
    }
}

fn reset_neighbors(
    commands: &mut Commands,
    tile_state_q: &mut Query<&mut TileState>,
    transform_q: &Query<&mut Transform>,
    tile_storage: &TileStorage,
    grid_size: &TilemapGridSize,
    map_size: &TilemapSize,
    map_type: &TilemapType,
    ent: Entity,
) {
    let transform = transform_q.get(ent).unwrap();
    let tile_pos = TilePos::from_world_pos(
        &Vec2::new(transform.translation.x, transform.translation.y),
        map_size,
        grid_size,
        map_type,
    )
    .unwrap();
    let neighbor = tile_storage.get(&tile_pos).unwrap();
    let mut neigh_state = tile_state_q.get_mut(neighbor).unwrap();

    // avoid setting to Empty the state of the neighbor we have moved the piece to
    if let Tile::WithCircle = neigh_state.tile_type {
        neigh_state.tile_type = Tile::Empty;
    }

    commands.entity(ent).despawn_recursive();
}

// gets
fn get_complex_movements(
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

// helper function to spawn the pieces
pub fn spawn_piece(
    commands: &mut Commands,
    piece_type: Piece,
    pos: TilePos,
    tile_storage: &TileStorage,
    tile_query: &mut Query<(&TilePos, &mut TileState)>,
    grid_size: &TilemapGridSize,
    map_type: &TilemapType,
    asset: Handle<Image>,
    meshes: &mut Assets<Mesh>,
) {
    // gets the entity of the tile in the given tile position
    let tile_entity = tile_storage.get(&pos).unwrap();

    // gets the transform relative to the tile position selected
    // and the state of the it
    let tile_pos = {
        let (pos, mut state_t) = tile_query.get_mut(tile_entity).unwrap();

        state_t.tile_type = Tile::NotEmpty;

        pos.center_in_world(grid_size, map_type)
    };

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

fn spawn_circle(
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
        .spawn((MaterialMesh2dBundle {
            mesh: Mesh2dHandle(mesh.add(Mesh::from(shape::Circle::new(16.0))).into()),
            transform: Transform::from_xyz(vec_t.x, vec_t.y, 2.0),
            material: material.add(ColorMaterial::from(
                Color::hex("6E667B").expect("Error here"),
            )),
            ..Default::default()
        },))
        .insert(mesh.add(Mesh::from(shape::Quad::new(Vec2::splat(64.0)))))
        .insert(PickableBundle::default())
        .insert(Position);
}

fn handle_piece_death() {}
