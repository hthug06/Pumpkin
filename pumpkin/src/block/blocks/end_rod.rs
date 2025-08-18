use crate::block::{BlockBehaviour, OnPlaceArgs, RandomTickArgs};
use async_trait::async_trait;
use rand::Rng;
use pumpkin_data::{BlockDirection, FacingExt};
use pumpkin_data::block_properties::{Axis, EndRodLikeProperties};
use pumpkin_data::block_properties::BlockProperties;
use pumpkin_data::particle::Particle;
use pumpkin_macros::pumpkin_block;
use pumpkin_protocol::java::client::play::ArgumentType::Vec3;
use pumpkin_util::math::vector3::Vector3;
use pumpkin_world::BlockStateId;
use pumpkin_world::generation::Direction;

#[pumpkin_block("minecraft:end_rod")]
pub struct EndRodBlock;

#[async_trait]
impl BlockBehaviour for EndRodBlock {
    async fn on_place(&self, args: OnPlaceArgs<'_>) -> BlockStateId {
        let mut props = EndRodLikeProperties::default(args.block);
        props.facing = args.direction.to_facing();
        if args.world.get_block(&args.use_item_on.position).await.eq(args.block) {
            props.facing = args.direction.to_facing().opposite();
        }
        
        props.to_state_id(args.block)
    }
    async fn random_tick(&self, args: RandomTickArgs<'_>) {
        let direction = EndRodLikeProperties::from_state_id(args.world.get_block_state_id(args.position).await, args.block)
            .facing
            .to_block_direction();

        let mut random = rand::rng();

        let x = args.position.0.x as f64 + 0.55 - (random.random::<f64>() * 0.1);
        let y = args.position.0.y as f64 + 0.55 - (random.random::<f64>() * 0.1);
        let z = args.position.0.z as f64 + 0.55 - (random.random::<f64>() * 0.1);

        let offset = 0.4 - (random.random::<f64>() + random.random::<f64>()) * 0.4;
        
        if random.random::<i16>() == 5 {
            args.world.spawn_particle(
                Vector3::new((x + direction.to_offset().x as f64) * offset,
                             (y + direction.to_offset().y as f64) * offset,
                             (z + direction.to_offset().z as f64) * offset),
                Vector3::new(random.random::<f32>() * 0.005, random.random::<f32>() * 0.005, random.random::<f32>() * 0.005),
                5.0,
                3,
                Particle::EndRod
            ).await;
        }

    }
}
