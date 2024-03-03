use std::f64::consts::TAU;

use valence::prelude::*;
use valence::weather::{Rain, Thunder, WeatherBundle};
use valence_server::nbt::{compound, List};

pub fn main() {
    App::new()
        .insert_resource(NetworkSettings {
            connection_mode: ConnectionMode::Offline,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (init_clients, despawn_disconnected_clients, change_weather),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    server: Res<Server>,
    dimensions: Res<DimensionTypeRegistry>,
    biomes: Res<BiomeRegistry>,
) {
    let mut layer = LayerBundle::new(ident!("overworld"), &dimensions, &biomes, &server);

    for z in -5..5 {
        for x in -5..5 {
            layer.chunk.insert_chunk([x, z], UnloadedChunk::new());
        }
    }

    for z in -16..16 {
        for x in 0..16 {
            layer.chunk.set_block([x, -64, z], BlockState::GRASS_BLOCK);
            layer
                .chunk
                .set_block([-x, x - 64, z], BlockState::GRASS_BLOCK);
        }
    }

    layer.chunk.set_block(
        [8, -63, 2],
        Block {
            state: BlockState::OAK_SIGN.set(PropName::Rotation, PropValue::_8),
            nbt: Some(compound! {
                "front_text" => compound! {
                    "messages" => List::String(vec![
                        "Some blocks do".into_text().into(),
                        "not block motion".into_text().into(),
                        "such as rain.".into_text().into(),
                        "(e.g. a sapling)".into_text().into(),
                    ]),
                }
            }),
        },
    );
    layer.chunk.set_block([7, -63, 2], BlockState::OAK_SAPLING);

    layer.chunk.set_block(
        [6, -63, 2],
        Block {
            state: BlockState::OAK_SIGN.set(PropName::Rotation, PropValue::_8),
            nbt: Some(compound! {
                "front_text" => compound! {
                    "messages" => List::String(vec![
                        "However, liquids".into_text().into(),
                        "and waterlogged".into_text().into(),
                        "blocks do.".into_text().into(),
                        "".into_text().into(),
                    ]),
                }
            }),
        },
    );
    layer.chunk.set_block(
        [5, -63, 2],
        BlockState::POWERED_RAIL.set(PropName::Waterlogged, PropValue::True),
    );

    commands.spawn((layer, WeatherBundle::default()));
}

fn init_clients(
    mut clients: Query<
        (
            &mut EntityLayerId,
            &mut VisibleChunkLayer,
            &mut VisibleEntityLayers,
            &mut Position,
            &mut GameMode,
        ),
        Added<Client>,
    >,
    layers: Query<Entity, (With<ChunkLayer>, With<EntityLayer>)>,
) {
    for (
        mut layer_id,
        mut visible_chunk_layer,
        mut visible_entity_layers,
        mut pos,
        mut game_mode,
    ) in &mut clients
    {
        let layer = layers.single();

        layer_id.0 = layer;
        visible_chunk_layer.0 = layer;
        visible_entity_layers.0.insert(layer);
        pos.set([8.0, -63.0, 0.0]);
        *game_mode = GameMode::Creative;
    }
}

fn change_weather(
    mut layers: Query<(&mut Rain, &mut Thunder), With<ChunkLayer>>,
    server: Res<Server>,
) {
    let period = 10.0;

    let level = ((server.current_tick() as f64 / 20.0 * TAU / period).sin() + 1.0) / 2.0;

    for (mut rain, mut thunder) in &mut layers {
        rain.0 = level as f32;
        thunder.0 = level as f32;
    }
}
