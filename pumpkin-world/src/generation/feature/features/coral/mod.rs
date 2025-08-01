use pumpkin_data::{
    Block, BlockDirection, BlockState,
    block_properties::{BlockProperties, EnumVariants, Integer1To4, SeaPickleLikeProperties},
    tag,
    tag::{RegistryKey, Taggable, get_tag_values},
};
use pumpkin_util::{
    math::position::BlockPos,
    random::{RandomGenerator, RandomImpl},
};

use crate::ProtoChunk;

pub mod coral_claw;
pub mod coral_mushroom;
pub mod coral_tree;

pub struct CoralFeature;

impl CoralFeature {
    pub fn generate_coral_piece(
        chunk: &mut ProtoChunk,
        random: &mut RandomGenerator,
        state: &BlockState,
        pos: BlockPos,
    ) -> bool {
        let block = chunk.get_block_state(&pos.0).to_block();
        let above_block = chunk.get_block_state(&pos.up().0).to_block();

        if block != &Block::WATER && !block.is_tagged_with_by_tag(&tag::Block::MINECRAFT_CORALS)
            || above_block != &Block::WATER
        {
            return false;
        }
        chunk.set_block_state(&pos.0, state);
        if random.next_f32() < 0.25 {
            chunk.set_block_state(
                &pos.0,
                Self::get_random_tag_entry("minecraft:corals", random),
            );
        } else if random.next_f32() < 0.05 {
            let mut props = SeaPickleLikeProperties::default(&Block::SEA_PICKLE);
            props.pickles = Integer1To4::from_index(random.next_bounded_i32(4) as u16); // TODO: vanilla adds + 1, but this can crash
            chunk.set_block_state(
                &pos.0,
                BlockState::from_id(props.to_state_id(&Block::SEA_PICKLE)),
            );
        }
        for dir in BlockDirection::horizontal() {
            let dir_pos = pos.offset(dir.to_offset());
            if random.next_f32() >= 0.2
                || chunk.get_block_state(&dir_pos.0).to_block() != &Block::WATER
            {
                continue;
            }
            let wall_coral = Self::get_random_tag_entry_block("minecraft:wall_corals", random);
            let original_props = &wall_coral
                .properties(wall_coral.default_state.id)
                .unwrap()
                .to_props();
            let facing = dir.to_facing();
            // Set the right Axis
            let props: Vec<(&str, &str)> = original_props
                .iter()
                .map(|(key, value)| {
                    if key == "facing" {
                        (key.as_str(), facing.to_value())
                    } else {
                        (key.as_str(), value.as_str())
                    }
                })
                .collect();
            chunk.set_block_state(
                &dir_pos.0,
                BlockState::from_id(wall_coral.from_properties(&props).to_state_id(wall_coral)),
            );
        }

        true
    }

    pub fn get_random_tag_entry(tag: &str, random: &mut RandomGenerator) -> &'static BlockState {
        let block = Self::get_random_tag_entry_block(tag, random);
        block.default_state
    }

    pub fn get_random_tag_entry_block(tag: &str, random: &mut RandomGenerator) -> &'static Block {
        let values = get_tag_values(RegistryKey::Block, tag).unwrap();
        let value = values[random.next_bounded_i32(values.len() as i32) as usize];
        Block::from_name(value).unwrap()
    }
}
