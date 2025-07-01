use async_trait::async_trait;
use pumpkin_data::{Block, BlockDirection};
use pumpkin_data::block_properties::{BlockProperties, Integer1To8, SnowLikeProperties};
use pumpkin_data::tag::Tagable;
use pumpkin_macros::pumpkin_block;
use pumpkin_protocol::java::server::play::SUseItemOn;
use pumpkin_util::math::position::BlockPos;
use pumpkin_world::BlockStateId;
use pumpkin_world::world::BlockAccessor;
use crate::block::BlockIsReplacing;
use crate::block::pumpkin_block::PumpkinBlock;
use crate::entity::player::Player;
use crate::server::Server;
use crate::world::World;

#[pumpkin_block("minecraft:snow")]
pub struct SnowBlock;

#[async_trait]
impl PumpkinBlock for SnowBlock {
    async fn on_place(&self, _server: &Server, _world: &World, _player: &Player, block: &Block, _block_pos: &BlockPos, _face: BlockDirection, replacing: BlockIsReplacing, _use_item_on: &SUseItemOn) -> BlockStateId {
        if let BlockIsReplacing::Itself(state_id) = replacing {
            let mut snow_prop = SnowLikeProperties::from_state_id(state_id, block);
            if snow_prop.layers != Integer1To8::L8 {
                snow_prop.layers = match snow_prop.layers {
                    Integer1To8::L1 => Integer1To8::L2,
                    Integer1To8::L2 => Integer1To8::L3,
                    Integer1To8::L3 => Integer1To8::L4,
                    Integer1To8::L4 => Integer1To8::L5,
                    Integer1To8::L5 => Integer1To8::L6,
                    Integer1To8::L6 => Integer1To8::L7,
                    Integer1To8::L7 => Integer1To8::L8,
                    _ => Integer1To8::L8,
                };
            }
            return snow_prop.to_state_id(block);
        }
        block.default_state.id
    }

    async fn can_place_at(&self, _server: Option<&Server>, _world: Option<&World>, block_accessor: &dyn BlockAccessor, _player: Option<&Player>, block: &Block, block_pos: &BlockPos, _face: BlockDirection, _use_item_on: Option<&SUseItemOn>) -> bool {
        let (block_down, block_state_down) = block_accessor.get_block_and_block_state(&block_pos.down()).await;
        if block_down.is_tagged_with("minecraft:snow_layer_cannot_survive_on").unwrap(){
            return false;
        }
        if block_down.is_tagged_with("minecraft:snow_layer_can_survive_on").unwrap(){
            return true;
        }
        block_state_down.is_side_solid(BlockDirection::Up)
        || block_down.eq(block) && SnowLikeProperties::from_state_id(block_state_down.id, block).layers == Integer1To8::L8
    }

    async fn can_update_at(&self, _world: &World, block: &Block, state_id: BlockStateId, _block_pos: &BlockPos, _face: BlockDirection, _use_item_on: &SUseItemOn, _player: &Player) -> bool {
        let layer = SnowLikeProperties::from_state_id(state_id, block).layers;
        layer!=Integer1To8::L8
    }
}