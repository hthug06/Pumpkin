use std::io::Write;

use crate::{ClientPacket, WritingError, ser::NetworkWriteExt};

use pumpkin_data::{
    Block,
    fluid::Fluid,
    packet::clientbound::CONFIG_UPDATE_TAGS,
    tag::{RegistryKey, get_registry_key_tags},
};
use pumpkin_macros::packet;
use pumpkin_util::resource_location::ResourceLocation;

#[packet(CONFIG_UPDATE_TAGS)]
pub struct CUpdateTags<'a> {
    tags: &'a [pumpkin_data::tag::RegistryKey],
}

impl<'a> CUpdateTags<'a> {
    pub fn new(tags: &'a [pumpkin_data::tag::RegistryKey]) -> Self {
        Self { tags }
    }
}

impl ClientPacket for CUpdateTags<'_> {
    fn write_packet_data(&self, write: impl Write) -> Result<(), WritingError> {
        let mut write = write;
        write.write_list(self.tags, |p, registry_key| {
            p.write_resource_location(&ResourceLocation::vanilla(
                registry_key.identifier_string(),
            ))?;

            let values = get_registry_key_tags(registry_key);
            p.write_var_int(&values.len().try_into().map_err(|_| {
                WritingError::Message(format!("{} isn't representable as a VarInt", values.len()))
            })?)?;

            for (key, values) in values.entries() {
                // This is technically a `ResourceLocation` but same thing
                p.write_string_bounded(key, u16::MAX as usize)?;
                p.write_list(values.0, |p, string_id| {
                    let id = match registry_key {
                        RegistryKey::Block => Block::from_name(string_id).unwrap().id as i32,
                        RegistryKey::Fluid => Fluid::ident_to_fluid_id(string_id).unwrap() as i32,
                        _ => unimplemented!(),
                    };

                    p.write_var_int(&id.into())
                })?;
            }

            Ok(())
        })
    }
}
