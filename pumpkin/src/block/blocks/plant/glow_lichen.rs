use async_trait::async_trait;
use pumpkin_data::block_properties::{BlockProperties, GlowLichenLikeProperties};
use pumpkin_data::BlockDirection;
use pumpkin_macros::pumpkin_block;
use pumpkin_util::math::position::BlockPos;
use pumpkin_world::BlockStateId;
use pumpkin_world::generation::Direction;
use crate::block::{BlockBehaviour, BlockIsReplacing, CanPlaceAtArgs, CanUpdateAtArgs, OnPlaceArgs};
use crate::world::World;

#[pumpkin_block("minecraft:glow_lichen")]
pub struct GlowLichenBlock;

fn glow_lichen_placement(mut property: GlowLichenLikeProperties, block_direction: BlockDirection) -> GlowLichenLikeProperties{
    match block_direction {
        BlockDirection::East => property.east = true,
        BlockDirection::West => property.west = true,
        BlockDirection::North => property.north = true,
        BlockDirection::South => property.south = true,
        BlockDirection::Down => property.down = true,
        BlockDirection::Up => property.up = true,
    };
    property
}

async fn can_grow_on(world: &World, mut pos: &BlockPos, dir: BlockDirection) -> bool{
    let new_pos = &pos.offset(dir.to_offset());
    let blockstate = world.get_block_state(new_pos).await;
    blockstate.is_side_solid(dir.opposite())

}

#[async_trait]
impl BlockBehaviour for GlowLichenBlock{

    async fn on_place(&self, args: OnPlaceArgs<'_>) -> BlockStateId {

        if let BlockIsReplacing::Itself(state_id) = args.replacing {
            let mut glow_lichen_props = GlowLichenLikeProperties::from_state_id(state_id, args.block);

            glow_lichen_props = glow_lichen_placement(glow_lichen_props, args.direction);
            return glow_lichen_props.to_state_id(args.block);
        }

        let mut props = GlowLichenLikeProperties::default(args.block);

        props = glow_lichen_placement(props, args.direction);

        props.to_state_id(args.block)
    }

    async fn can_place_at(&self, args: CanPlaceAtArgs<'_>) -> bool {
        let mut bl = false;
        for dir in BlockDirection::all(){
            if !can_grow_on(args.world.unwrap(), args.position, dir).await {
                return false;
            }
            bl = true;
        }
        bl
    }

    async fn can_update_at(&self, _args: CanUpdateAtArgs<'_>) -> bool {
        true
    }
}