use anyhow::Result;
use bevy::prelude::*;
use bevy_chess::*;

mod common;

#[test]
fn spawns_entity_with_name() -> Result<()> {
    let mut app = common::bevy_test_app();
    app.add_plugin(BevyChessPlugin);

    app.update();

    let e = app
        .world
        .query_filtered::<Entity, With<BevyChessComponent>>()
        .iter(&app.world)
        .next()
        .unwrap();

    assert_eq!(app.world.query::<&Name>().get(&app.world, e)?.as_str(), "BevyChessPlugin Root");

    Ok(())
}
