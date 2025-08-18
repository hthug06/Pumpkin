use crate::block::{BlockBehaviour, OnPlaceArgs, RandomTickArgs};
use async_trait::async_trait;
use pumpkin_data::block_properties::BlockProperties;
use pumpkin_data::block_properties::EndRodLikeProperties;
use pumpkin_data::particle::Particle;
use pumpkin_data::{Block, FacingExt};
use pumpkin_macros::pumpkin_block;
use pumpkin_util::math::vector3::Vector3;
use pumpkin_world::BlockStateId;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

#[pumpkin_block("minecraft:end_rod")]
pub struct EndRodBlock;

#[async_trait]
impl BlockBehaviour for EndRodBlock {
    async fn on_place(&self, args: OnPlaceArgs<'_>) -> BlockStateId {
        let mut props = EndRodLikeProperties::default(args.block);

        let blockstate = args
            .world
            .get_block_state_id(&args.position.offset(args.direction.to_offset()))
            .await;

        if Block::from_state_id(blockstate).eq(args.block)
            && EndRodLikeProperties::from_state_id(blockstate, args.block).facing
                == args.direction.to_facing().opposite()
        {
            props.facing = args.direction.to_facing();
        } else {
            props.facing = args.direction.to_facing().opposite();
        }

        props.to_state_id(args.block)
    }
}
