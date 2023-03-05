use bevy::{
    prelude::{
        info, Assets, Changed, Commands, Entity, EventReader, EventWriter, Mesh, Query, ResMut,
        Transform, Vec2, Vec3, With,
    },
    sprite::ColorMaterial,
};
use bevy_ecs_tilemap::{
    prelude::{TilemapGridSize, TilemapSize, TilemapType},
    tiles::{TilePos, TileStorage},
};
use bevy_mod_picking::{PickingEvent, Selection, SelectionEvent};

use crate::{
    board::{Tile, TileState},
    piece::{HighLight, PieceDeathEvent, PieceType},
};

pub struct MoveEvent;

// detects wether a piece has been selected and shows, with a circle, where the player can move
// the piece to, depending on it's type
pub fn get_piece_movements(
    mut commands: Commands,
    mut events: EventReader<PickingEvent>,
    mut tile_state_q: Query<&mut TileState>,
    piece_type: Query<&PieceType>,
    tile_storage_q: Query<(&TileStorage, &TilemapGridSize, &TilemapSize, &TilemapType)>,
    transform_q: Query<&mut Transform>,
    highlight_pos: Query<Entity, With<HighLight>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for event in events.iter() {
        let (tile_storage, grid_size, map_size, map_type) = tile_storage_q.single();
        if highlight_pos.is_empty() {
            if let PickingEvent::Selection(e) = event {
                if let SelectionEvent::JustSelected(s) = e {
                    if let Ok(piece_t) = piece_type.get(*s) {
                        //get the cursor position, if it is on the window
                        if let Ok(t) = transform_q.get(*s) {
                            let pos = Vec2::new(t.translation.x, t.translation.y);
                            // gets the position of tile selected by the player
                            let tile_pos =
                                TilePos::from_world_pos(&pos, map_size, grid_size, map_type)
                                    .unwrap();
                            match piece_t {
                                PieceType::Rock(r) => r.movement(
                                    &mut commands,
                                    tile_storage,
                                    grid_size,
                                    map_size,
                                    map_type,
                                    &mut tile_state_q,
                                    &piece_type,
                                    tile_pos,
                                    &mut meshes,
                                    &mut materials,
                                ),
                                PieceType::Knight(kn) => kn.knight_movement(
                                    &mut commands,
                                    tile_storage,
                                    tile_pos,
                                    &mut tile_state_q,
                                    &piece_type,
                                    grid_size,
                                    map_type,
                                    &mut meshes,
                                    &mut materials,
                                ),
                                PieceType::Bishop(b) => b.movement(
                                    &mut commands,
                                    tile_storage,
                                    grid_size,
                                    map_size,
                                    map_type,
                                    &mut tile_state_q,
                                    &piece_type,
                                    tile_pos,
                                    &mut meshes,
                                    &mut materials,
                                ),
                                PieceType::Queen(q) => q.movement(
                                    &mut commands,
                                    tile_storage,
                                    grid_size,
                                    map_size,
                                    map_type,
                                    &mut tile_state_q,
                                    &piece_type,
                                    tile_pos,
                                    &mut meshes,
                                    &mut materials,
                                ),
                                PieceType::King(k) => k.movement(
                                    &mut commands,
                                    tile_storage,
                                    tile_pos,
                                    &mut tile_state_q,
                                    &piece_type,
                                    grid_size,
                                    map_size,
                                    map_type,
                                    &mut meshes,
                                    &mut materials,
                                ),
                                PieceType::Pawn(p) => p.movement(
                                    &mut commands,
                                    tile_pos,
                                    tile_storage,
                                    &mut tile_state_q,
                                    &piece_type,
                                    grid_size,
                                    map_size,
                                    map_type,
                                    &mut meshes,
                                    &mut materials,
                                ),
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn move_piece(
    mut _commands: Commands,
    mut events: EventReader<PickingEvent>,
    mut tile_state_q: Query<&mut TileState>,
    mut transform_q: Query<&mut Transform>,
    tile_storage_q: Query<(&TileStorage, &TilemapGridSize, &TilemapSize, &TilemapType)>,
    selected_pos: Query<Entity, Changed<Selection>>,
    mut move_event: EventWriter<MoveEvent>,
    mut death_event: EventWriter<PieceDeathEvent>,
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
                        info!("{:?}", tile_pos);

                        // checks wether the movement is correct
                        if let Tile::HighLighted = tile_state_q
                            .get_mut(tile_storage.get(&tile_pos).unwrap())
                            .unwrap()
                            .tile_type
                        {
                            // gets the reference to the selection's transform to be changed
                            let mut selection_t = transform_q.get_mut(*s).unwrap();
                            // converts the tile position into the transform which is at the
                            // center of the selected tile
                            let new_pos = tile_pos.center_in_world(grid_size, map_type);
                            // get the old tile position
                            let old_tile = TilePos::from_world_pos(
                                &Vec2::new(selection_t.translation.x, selection_t.translation.y),
                                map_size,
                                grid_size,
                                map_type,
                            )
                            .unwrap();

                            //get the old tile state and change its type to empty
                            let mut tile_s = tile_state_q
                                .get_mut(tile_storage.get(&old_tile).unwrap())
                                .unwrap();
                            let piece = tile_s.piece_ent.unwrap();

                            tile_s.tile_type = Tile::Empty;
                            tile_s.piece_ent = None;

                            //get the selected tile state and change its type to empty
                            tile_s = tile_state_q
                                .get_mut(tile_storage.get(&tile_pos).unwrap())
                                .unwrap();

                            // if theres some piece on the tile just selected, send a death event
                            if let Some(e) = tile_s.piece_ent {
                                death_event.send(PieceDeathEvent(e));
                            }

                            tile_s.tile_type = Tile::NotEmpty;
                            tile_s.piece_ent = Some(piece);

                            selection_t.translation = Vec3::new(new_pos.x, new_pos.y, 1.0);
                        }
                    }
                }

                move_event.send(MoveEvent)
            }
        }
    }
}
