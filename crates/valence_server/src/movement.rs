use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use valence_entity::{HeadYaw, Look, OnGround, Position};
use valence_math::DVec3;
use valence_protocol::packets::play::{
    FullC2s, LookAndOnGroundC2s, OnGroundOnlyC2s, PositionAndOnGroundC2s, VehicleMoveC2s,
};

use valence_protocol::game_mode::GameMode;
use crate::math::Vec2;
use crate::client::Client;

use crate::event_loop::{EventLoopPreUpdate, PacketEvent};
use crate::teleport::TeleportState;

pub struct MovementPlugin;
use crate::client::VisibleChunkLayer;
use crate::BlockPos;
use crate::ChunkLayer;

use std::collections::VecDeque;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MovementSettings>()
            .add_event::<MovementEvent>()
            .add_systems(EventLoopPreUpdate, (
                    init_clients,
                    handle_client_movement,
                              ));
    }
}

/// Configuration resource for client movement checks.
#[derive(Resource, Default)]
pub struct MovementSettings {
    // TODO
}

/// Event sent when a client successfully moves.
#[derive(Event, Clone, Debug)]
pub struct MovementEvent {
    pub client: Entity,
    pub position: DVec3,
    pub old_position: DVec3,
    pub look: Look,
    pub old_look: Look,
    pub on_ground: bool,
    pub old_on_ground: bool,
}

#[derive(Component, Clone, Debug)]
struct MovementChecker {
    last_delta_y: f32,
    last_delta_xz: VecDeque<Vec2>,
    avg_delta_xz: f32,
    ticks: u32,
}

fn init_clients(
    mut clients:  Query<(
                Entity,
            ),
            Added<Client>
        >,
    mut commands: Commands,
) {
    for (entity,) in &mut clients {
        commands.entity(entity).insert(MovementChecker{last_delta_y: 0.0, last_delta_xz: VecDeque::new(), avg_delta_xz: 0.0, ticks: 0});
    }
}
fn handle_client_movement(
    mut packets: EventReader<PacketEvent>,
    mut clients: Query<(
        &mut Position,
        &mut Look,
        &mut HeadYaw,
        &mut OnGround,
        &mut TeleportState,
        &GameMode,
        &mut MovementChecker,
        &VisibleChunkLayer,
    )>,
    layers: Query<&ChunkLayer>,
    mut movement_events: EventWriter<MovementEvent>,
) {
    for packet in packets.read() {
        if let Some(pkt) = packet.decode::<PositionAndOnGroundC2s>() {
            if let Ok((pos, look, head_yaw, on_ground, teleport_state,game_mode, mut movement_checker, vis_chunk_layer)) =
                clients.get_mut(packet.client)
            {
                let chunk_layer = layers.get(**vis_chunk_layer).unwrap();
                let mov = MovementEvent {
                    client: packet.client,
                    position: pkt.position,
                    old_position: pos.0,
                    look: *look,
                    old_look: *look,
                    on_ground: pkt.on_ground,
                    old_on_ground: on_ground.0,
                };

                handle(
                    mov,
                    pos,
                    look,
                    head_yaw,
                    on_ground,
                    teleport_state,
                    &mut movement_events,
                    &game_mode,
                    &mut movement_checker,
                    chunk_layer,
                );
            }
        } else if let Some(pkt) = packet.decode::<FullC2s>() {
            if let Ok((pos, look, head_yaw, on_ground, teleport_state,game_mode, mut movement_checker, vis_chunk_layer)) =
                clients.get_mut(packet.client)
            {
                let chunk_layer = layers.get(**vis_chunk_layer).unwrap();
                let mov = MovementEvent {
                    client: packet.client,
                    position: pkt.position,
                    old_position: pos.0,
                    look: Look {
                        yaw: pkt.yaw,
                        pitch: pkt.pitch,
                    },
                    old_look: *look,
                    on_ground: pkt.on_ground,
                    old_on_ground: on_ground.0,
                };

                handle(
                    mov,
                    pos,
                    look,
                    head_yaw,
                    on_ground,
                    teleport_state,
                    &mut movement_events,
                    &game_mode,
                    &mut movement_checker,
                    chunk_layer,
                );
            }
        } else if let Some(pkt) = packet.decode::<LookAndOnGroundC2s>() {
            if let Ok((pos, look, head_yaw, on_ground, teleport_state,game_mode, mut movement_checker, vis_chunk_layer)) =
                clients.get_mut(packet.client)
            {
                let chunk_layer = layers.get(**vis_chunk_layer).unwrap();
                let mov = MovementEvent {
                    client: packet.client,
                    position: pos.0,
                    old_position: pos.0,
                    look: Look {
                        yaw: pkt.yaw,
                        pitch: pkt.pitch,
                    },
                    old_look: *look,
                    on_ground: pkt.on_ground,
                    old_on_ground: on_ground.0,
                };

                handle(
                    mov,
                    pos,
                    look,
                    head_yaw,
                    on_ground,
                    teleport_state,
                    &mut movement_events,
                    &game_mode,
                    &mut movement_checker,
                    chunk_layer,
                );
            }
        } else if let Some(pkt) = packet.decode::<OnGroundOnlyC2s>() {
            if let Ok((pos, look, head_yaw, on_ground, teleport_state,game_mode, mut movement_checker, vis_chunk_layer)) =
                clients.get_mut(packet.client)
            {
                let chunk_layer = layers.get(**vis_chunk_layer).unwrap();
                let mov = MovementEvent {
                    client: packet.client,
                    position: pos.0,
                    old_position: pos.0,
                    look: *look,
                    old_look: *look,
                    on_ground: pkt.on_ground,
                    old_on_ground: on_ground.0,
                };

                handle(
                    mov,
                    pos,
                    look,
                    head_yaw,
                    on_ground,
                    teleport_state,
                    &mut movement_events,
                    &game_mode,
                    &mut movement_checker,
                    chunk_layer,
                );
            }
        } else if let Some(pkt) = packet.decode::<VehicleMoveC2s>() {
            if let Ok((pos, look, head_yaw, on_ground, teleport_state,game_mode, mut movement_checker, vis_chunk_layer)) =
                clients.get_mut(packet.client)
            {
                let chunk_layer = layers.get(**vis_chunk_layer).unwrap();
                let mov = MovementEvent {
                    client: packet.client,
                    position: pkt.position,
                    old_position: pos.0,
                    look: Look {
                        yaw: pkt.yaw,
                        pitch: pkt.pitch,
                    },
                    old_look: *look,
                    on_ground: on_ground.0,
                    old_on_ground: on_ground.0,
                };

                handle(
                    mov,
                    pos,
                    look,
                    head_yaw,
                    on_ground,
                    teleport_state,
                    &mut movement_events,
                    &game_mode,
                    &mut movement_checker,
                    chunk_layer,
                );
            }
        }
    }
}

fn handle(
    mov: MovementEvent,
    mut pos: Mut<Position>,
    mut look: Mut<Look>,
    mut head_yaw: Mut<HeadYaw>,
    mut on_ground: Mut<OnGround>,
    mut teleport_state: Mut<TeleportState>,
    movement_events: &mut EventWriter<MovementEvent>,
    game_mode: &GameMode,
    movement_checker: &mut MovementChecker,
    chunk_layer: &ChunkLayer,
) {
    if teleport_state.pending_teleports() != 0 {
        return;
    }

    let delta_xz = Vec2::new( mov.position.x as f32, mov.position.z as f32) - Vec2::new(pos.x as f32, pos.z as f32);
    let delta_y: f32 = mov.position.y as f32 - pos.y as f32;
    if *game_mode == GameMode::Survival || *game_mode ==  GameMode::Adventure && movement_checker.ticks > 10 {
        // println!("client moving with speed: {} in horizontal plane and {} in vertical axis", delta_xz.length(), delta_y);
        // println!("Client on ground? {} {}", on_ground.0, mov.on_ground);
        //
        // Checks if client is grounded (has blocks benith their feet)
        let bp = BlockPos::new((pos.x - 0.5).floor() as i32, 
                               (pos.y-1.0).floor() as i32, 
                               (pos.z - 0.5).floor() as i32);
        let is_grounded_real = 
           chunk_layer.block(BlockPos::new(bp.x, bp.y, bp.z)).unwrap_or_default().state.blocks_motion()
        || chunk_layer.block(BlockPos::new(bp.x, bp.y, bp.z+1)).unwrap_or_default().state.blocks_motion()
        || chunk_layer.block(BlockPos::new(bp.x+1, bp.y, bp.z)).unwrap_or_default().state.blocks_motion()
        || chunk_layer.block(BlockPos::new(bp.x+1, bp.y, bp.z+1)).unwrap_or_default().state.blocks_motion();
        if !is_grounded_real && mov.on_ground {
            println!("Player is faking grounded state block at {:?} pos: {:?} ({:?} {:?})", bp, pos, on_ground, mov.on_ground);
        }
        // Checks if client is not flying
        if !on_ground.0 && !mov.on_ground && (delta_y - movement_checker.last_delta_y) > -0.07 && delta_y >= -0.51 {
            println!("Client is attempting to fly (grav: {}) at {}", delta_y-movement_checker.last_delta_y, delta_y);
        }
        // Checks if client is not moving to fast over 20 ticks
        if movement_checker.avg_delta_xz > 7.5 {
            println!("Client is moving {} times to fast (sustained)", movement_checker.avg_delta_xz/7.16);

        }
        // Checks if client is not moving to fast at instant
        if delta_xz.length() > 1.78*7.127/20.0 {
            println!("Client is moving {} times to fast", delta_xz.length()/(7.127/20.0));
        }
    }
    // Antyflight
    // if !(on_ground.0) && mov.position.y - pos.y > 0.0 {
    //     return;
    // }
    // println!("{:?} {:?}", on_ground.0, mov.position - **pos);
    // TODO: check that the client isn't moving too fast / flying.
    // TODO: check that the client isn't clipping through blocks.

    pos.set_if_neq(Position(mov.position));
    teleport_state.synced_pos = mov.position;
    look.set_if_neq(mov.look);
    teleport_state.synced_look = mov.look;
    head_yaw.set_if_neq(HeadYaw(mov.look.yaw));
    on_ground.set_if_neq(OnGround(mov.on_ground));

    movement_checker.last_delta_y = delta_y;
    movement_checker.last_delta_xz.push_back(delta_xz);
    movement_checker.avg_delta_xz += delta_xz.length();
    movement_checker.ticks +=1;
    if movement_checker.last_delta_xz.len() > 20 {
        let poped = movement_checker.last_delta_xz.pop_front().unwrap();
        movement_checker.avg_delta_xz -= poped.length();
    }
    movement_events.send(mov);
}
