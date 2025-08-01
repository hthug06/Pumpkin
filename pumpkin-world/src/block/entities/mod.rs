use std::{any::Any, sync::Arc};

use async_trait::async_trait;
use barrel::BarrelBlockEntity;
use bed::BedBlockEntity;
use chest::ChestBlockEntity;
use comparator::ComparatorBlockEntity;
use end_portal::EndPortalBlockEntity;
use furnace::FurnaceBlockEntity;
use piston::PistonBlockEntity;
use pumpkin_data::{Block, block_properties::BLOCK_ENTITY_TYPES};
use pumpkin_nbt::compound::NbtCompound;
use pumpkin_util::math::position::BlockPos;
use sign::SignBlockEntity;

use crate::block::entities::hopper::HopperBlockEntity;
use crate::block::entities::shulker_box::ShulkerBoxBlockEntity;
use crate::{
    BlockStateId, block::entities::chiseled_bookshelf::ChiseledBookshelfBlockEntity,
    block::entities::dropper::DropperBlockEntity, inventory::Inventory, world::SimpleWorld,
};

pub mod barrel;
pub mod bed;
pub mod chest;
pub mod chiseled_bookshelf;
pub mod command_block;
pub mod comparator;
pub mod dropper;
pub mod end_portal;
pub mod furnace;
pub mod hopper;
pub mod piston;
pub mod shulker_box;
pub mod sign;

//TODO: We need a mark_dirty for chests
#[async_trait]
pub trait BlockEntity: Send + Sync {
    async fn write_nbt(&self, nbt: &mut NbtCompound);
    fn from_nbt(nbt: &NbtCompound, position: BlockPos) -> Self
    where
        Self: Sized;
    async fn tick(&self, _world: Arc<dyn SimpleWorld>) {}
    fn resource_location(&self) -> &'static str;
    fn get_position(&self) -> BlockPos;
    async fn write_internal(&self, nbt: &mut NbtCompound) {
        nbt.put_string("id", self.resource_location().to_string());
        let position = self.get_position();
        nbt.put_int("x", position.0.x);
        nbt.put_int("y", position.0.y);
        nbt.put_int("z", position.0.z);
        self.write_nbt(nbt).await;
    }
    fn get_id(&self) -> u32 {
        pumpkin_data::block_properties::BLOCK_ENTITY_TYPES
            .iter()
            .position(|block_entity_name| {
                *block_entity_name == self.resource_location().split(":").last().unwrap()
            })
            .unwrap() as u32
    }
    fn chunk_data_nbt(&self) -> Option<NbtCompound> {
        None
    }
    fn get_inventory(self: Arc<Self>) -> Option<Arc<dyn Inventory>> {
        None
    }
    fn set_block_state(&mut self, _block_state: BlockStateId) {}
    async fn on_block_replaced(self: Arc<Self>, world: Arc<dyn SimpleWorld>, position: BlockPos) {
        if let Some(inventory) = self.get_inventory() {
            world.scatter_inventory(&position, &inventory).await;
        }
    }
    fn is_dirty(&self) -> bool {
        false
    }
    fn as_any(&self) -> &dyn Any;
    fn to_property_delegate(self: Arc<Self>) -> Option<Arc<dyn PropertyDelegate>> {
        None
    }
}

pub fn block_entity_from_generic<T: BlockEntity>(nbt: &NbtCompound) -> T {
    let x = nbt.get_int("x").unwrap();
    let y = nbt.get_int("y").unwrap();
    let z = nbt.get_int("z").unwrap();
    T::from_nbt(nbt, BlockPos::new(x, y, z))
}

pub fn block_entity_from_nbt(nbt: &NbtCompound) -> Option<Arc<dyn BlockEntity>> {
    Some(match nbt.get_string("id").unwrap() {
        ChestBlockEntity::ID => Arc::new(block_entity_from_generic::<ChestBlockEntity>(nbt)),
        SignBlockEntity::ID => Arc::new(block_entity_from_generic::<SignBlockEntity>(nbt)),
        BedBlockEntity::ID => Arc::new(block_entity_from_generic::<BedBlockEntity>(nbt)),
        ComparatorBlockEntity::ID => {
            Arc::new(block_entity_from_generic::<ComparatorBlockEntity>(nbt))
        }
        BarrelBlockEntity::ID => Arc::new(block_entity_from_generic::<BarrelBlockEntity>(nbt)),
        HopperBlockEntity::ID => Arc::new(block_entity_from_generic::<HopperBlockEntity>(nbt)),
        DropperBlockEntity::ID => Arc::new(block_entity_from_generic::<DropperBlockEntity>(nbt)),
        ShulkerBoxBlockEntity::ID => {
            Arc::new(block_entity_from_generic::<ShulkerBoxBlockEntity>(nbt))
        }
        PistonBlockEntity::ID => Arc::new(block_entity_from_generic::<PistonBlockEntity>(nbt)),
        EndPortalBlockEntity::ID => {
            Arc::new(block_entity_from_generic::<EndPortalBlockEntity>(nbt))
        }
        ChiseledBookshelfBlockEntity::ID => Arc::new(block_entity_from_generic::<
            ChiseledBookshelfBlockEntity,
        >(nbt)),
        FurnaceBlockEntity::ID => Arc::new(block_entity_from_generic::<FurnaceBlockEntity>(nbt)),
        _ => return None,
    })
}

pub fn has_block_block_entity(block: &Block) -> bool {
    BLOCK_ENTITY_TYPES.contains(&block.name)
}

pub trait PropertyDelegate: Sync + Send {
    fn get_property(&self, _index: i32) -> i32;
    fn set_property(&self, _index: i32, _value: i32);
    fn get_properties_size(&self) -> i32;
}
