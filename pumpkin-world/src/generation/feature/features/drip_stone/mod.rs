use pumpkin_data::tag;
use pumpkin_data::{Block, tag::Taggable};
use pumpkin_util::math::position::BlockPos;

use crate::ProtoChunk;

pub mod cluster;
pub mod large;
pub mod small;

pub(super) fn can_replace(block: &Block) -> bool {
    block == &Block::DRIPSTONE_BLOCK
        || block.is_tagged_with_by_tag(&tag::Block::MINECRAFT_DRIPSTONE_REPLACEABLE_BLOCKS)
}

pub(super) fn gen_dripstone(chunk: &mut ProtoChunk, pos: BlockPos) -> bool {
    let block = chunk.get_block_state(&pos.0).to_block();
    if block.is_tagged_with_by_tag(&tag::Block::MINECRAFT_DRIPSTONE_REPLACEABLE_BLOCKS) {
        chunk.set_block_state(&pos.0, Block::DRIPSTONE_BLOCK.default_state);
        return true;
    }
    false
}
