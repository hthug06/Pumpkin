use async_trait::async_trait;
use rand::Rng;
use crate::block::BlockBehaviour;
use pumpkin_data::block_properties::{BlockProperties, Integer0To3, NetherWartLikeProperties};
use pumpkin_data::item::Item;
use pumpkin_data::sound::{Sound, SoundCategory};
use pumpkin_macros::pumpkin_block;
use pumpkin_world::item::ItemStack;
use pumpkin_world::world::BlockFlags;
use crate::block::blocks::plant::PlantBlockBase;
use crate::block::{CanPlaceAtArgs, NormalUseArgs, RandomTickArgs, UseWithItemArgs};
use crate::block::registry::BlockActionResult;

#[pumpkin_block("minecraft:sweet_berry_bush")]
pub struct SweetBerryBushBlock;

#[async_trait]
impl BlockBehaviour for SweetBerryBushBlock {
    async fn normal_use(&self, args: NormalUseArgs<'_>) -> BlockActionResult {
        let mut prop = NetherWartLikeProperties::from_state_id(args.world.get_block_state_id(args.position).await, args.block);

        if prop.age == Integer0To3::L2 || prop.age == Integer0To3::L3 {
            let mut nbr_of_berry = {
                let mut rng = rand::rng();
                rng.random_range(0..3)
            };
            if prop.age == Integer0To3::L3 {
                nbr_of_berry += 1;
            }
            args.world.drop_stack(args.position, ItemStack::new(nbr_of_berry, &Item::SWEET_BERRIES)).await;
            args.world.play_sound(Sound::BlockSweetBerryBushPickBerries, SoundCategory::Blocks, &args.position.to_f64()).await;

            //set the bush to age 1
            prop.age = Integer0To3::L1;
            args.world.set_block_state(args.position, prop.to_state_id(args.block), BlockFlags::NOTIFY_LISTENERS).await;
            return BlockActionResult::Success
        }
        BlockActionResult::Pass
    }
    async fn use_with_item(&self, args: UseWithItemArgs<'_>) -> BlockActionResult {
        let mut prop = NetherWartLikeProperties::from_state_id(args.world.get_block_state_id(args.position).await, args.block);

        if prop.age != Integer0To3::L3 && args.item_stack.lock().await.item.eq(&Item::BONE_MEAL) {
            prop.age = match prop.age{
                Integer0To3::L0 => Integer0To3::L1,
                Integer0To3::L1 => Integer0To3::L2,
                _ => Integer0To3::L3,
            };
            args.world.set_block_state(args.position, prop.to_state_id(args.block), BlockFlags::NOTIFY_LISTENERS).await;

            return BlockActionResult::Pass
        }

        BlockActionResult::PassToDefaultBlockAction
    }

    async fn random_tick(&self, args: RandomTickArgs<'_>) {
        let mut prop = NetherWartLikeProperties::from_state_id(args.world.get_block_state_id(args.position).await, args.block);

        //TODO check if light level is >= 9 | world.getBaseLightLevel(pos.up(), 0) >= 9 (in java)
        if prop.age != Integer0To3::L3 && rand::rng().random_range(0..5) == 0 {
            prop.age = match prop.age{
                Integer0To3::L0 => Integer0To3::L1,
                Integer0To3::L1 => Integer0To3::L2,
                _ => Integer0To3::L3,
            };
            args.world.set_block_state(args.position, prop.to_state_id(args.block), BlockFlags::NOTIFY_LISTENERS).await;
        }
    }

    async fn can_place_at(&self, args: CanPlaceAtArgs<'_>) -> bool {
        <Self as PlantBlockBase>::can_place_at(self, args.block_accessor, args.position).await
    }
}

impl PlantBlockBase for  SweetBerryBushBlock {}