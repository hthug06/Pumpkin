use pumpkin_data::packet::clientbound::PLAY_ANIMATE;
use pumpkin_macros::packet;
use serde::Serialize;

use crate::VarInt;

#[derive(Serialize)]
#[packet(PLAY_ANIMATE)]
pub struct CEntityAnimation {
    entity_id: VarInt,
    /// See `Animation`
    animation: u8,
}

impl CEntityAnimation {
    pub fn new(entity_id: VarInt, animation: Animation) -> Self {
        Self {
            entity_id,
            animation: animation as u8,
        }
    }
}

#[derive(Debug)]
pub enum Animation {
    SwingMainArm,
    LeaveBed = 2,
    SwingOffhand,
    CriticalEffect,
    MagicCriticaleffect,
}
