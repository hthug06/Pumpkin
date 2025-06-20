use crate::block::BlockIsReplacing;
use crate::block::pumpkin_block::PumpkinBlock;
use crate::entity::player::Player;
use crate::server::Server;
use crate::world::World;
use async_trait::async_trait;
use pumpkin_data::block_properties::{
    BlockProperties, PointedDripstoneLikeProperties, VerticalDirection,
};
use pumpkin_data::{Block, BlockDirection};
use pumpkin_macros::pumpkin_block;
use pumpkin_protocol::server::play::SUseItemOn;
use pumpkin_util::math::position::BlockPos;
use pumpkin_world::BlockStateId;
use pumpkin_world::chunk::TickPriority;
use pumpkin_world::world::{BlockAccessor, BlockFlags};
use std::sync::Arc;

#[pumpkin_block("minecraft:pointed_dripstone")]
pub struct PointedDripStoneBlock;

#[async_trait]
impl PumpkinBlock for PointedDripStoneBlock {
    async fn on_place(
        &self,
        _server: &Server,
        world: &World,
        _player: &Player,
        block: &Block,
        block_pos: &BlockPos,
        face: BlockDirection,
        replacing: BlockIsReplacing,
        use_item_on: &SUseItemOn,
    ) -> BlockStateId {
        let mut dripstone_prop = PointedDripstoneLikeProperties::default(block);
        dripstone_prop.waterlogged = replacing.water_source();
        dripstone_prop.vertical_direction = match face {
            BlockDirection::Down => VerticalDirection::Up,
            BlockDirection::Up => VerticalDirection::Down,
            _ => match use_item_on.cursor_pos.y {
                0.0..0.5 => {
                    if world.get_block_state(&block_pos.down()).await.is_air() {
                        VerticalDirection::Down
                    } else {
                        VerticalDirection::Up
                    }
                }
                _ => {
                    if world.get_block_state(&block_pos.up()).await.is_air() {
                        VerticalDirection::Up
                    } else {
                        VerticalDirection::Down
                    }
                }
            },
        };

        dripstone_prop.to_state_id(block)
    }
    async fn can_place_at(
        &self,
        _server: Option<&Server>,
        _world: Option<&World>,
        block_accessor: &dyn BlockAccessor,
        _player: Option<&Player>,
        _block: &Block,
        block_pos: &BlockPos,
        face: BlockDirection,
        _use_item_on: Option<&SUseItemOn>,
    ) -> bool {
        block_accessor
            .get_block_state(&block_pos.down())
            .await
            .is_side_solid(face)
            || block_accessor
                .get_block_state(&block_pos.up())
                .await
                .is_side_solid(face)
            || block_accessor.get_block(&block_pos.down()).await == Block::POINTED_DRIPSTONE
    }
}
