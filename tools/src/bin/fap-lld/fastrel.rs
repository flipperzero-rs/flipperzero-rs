use std::{
    collections::HashMap,
    fs,
    io::{self, Write},
    path::Path,
    process::Command,
};

use elf::{
    ElfBytes, abi::SHT_REL, endian::LittleEndian, relocation::RelIterator,
    string_table::StringTable, symbol::SymbolTable,
};
use md5::{Digest, Md5};
use tempfile::tempdir;

use super::Error;

/// Version of the `.fast.rel` encoding that this module supports.
const VERSION: u8 = 1;

pub(crate) fn postprocess_fap(output_fap: &Path, objcopy: &Path) -> Result<(), Error> {
    // Parse the FAP as an ELF binary.
    let fap_data = fs::read(output_fap)?;
    let fap = ElfBytes::<LittleEndian>::minimal_parse(&fap_data)?;

    // Get the section header table alongside its string table.
    let (shdrs_opt, strtab_opt) = fap.section_headers_with_strtab()?;
    let (shdrs, strtab) = shdrs_opt.zip(strtab_opt).ok_or(Error::NoSectionHeaders)?;

    // Collect the sections with relocations.
    let rel_sections = shdrs
        .iter()
        .filter(|shdr| shdr.sh_type == SHT_REL)
        .map(|shdr| -> Result<_, Error> {
            let name = strtab.get(shdr.sh_name as usize)?;
            let section = fap.section_data_as_rels(&shdr)?;
            Ok((name, section))
        })
        .collect::<Result<Vec<_>, _>>()?;

    // Convert the relocations into `.fast.rel` sections.
    let (symtab, strtab) = fap.symbol_table()?.ok_or(Error::NoSymbolTable)?;
    let fastrel_sections = rel_sections
        .into_iter()
        .map(|(section_name, section)| FastRelSection::new(section_name, section, &symtab, &strtab))
        .collect::<Result<Vec<_>, _>>()?;

    // Write the `.fast.rel` sections into the binary.
    let temp_dir = tempdir()?;
    for section in fastrel_sections {
        let fastrel_section_name = hex::encode(Md5::digest(&section.name));
        let data_path = temp_dir
            .path()
            .join(fastrel_section_name)
            .with_extension("bin");
        let mut data = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&data_path)?;
        section.write(&mut data)?;
        data.flush()?;

        let res = Command::new(objcopy)
            .arg("--add-section")
            .arg(format!("{}={}", section.name, data_path.display()))
            .arg(output_fap)
            .status()?;
        if !res.success() {
            return Err(Error::ObjcopyFailed);
        }
    }

    Ok(())
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct FastRel<'data> {
    section_index: u16,
    section_value: u64,
    r_type: u32,
    name: &'data str,
}

impl FastRel<'_> {
    fn gnu_sym_hash(&self) -> u32 {
        self.name
            .as_bytes()
            .iter()
            .fold(0x1505, |h, c| (h << 5) + h + u32::from(*c))
    }
}

/// A `.fast.rel` section.
#[derive(Debug)]
struct FastRelSection<'data> {
    name: String,
    fastrel_offsets: HashMap<FastRel<'data>, Vec<u64>>,
}

impl<'data> FastRelSection<'data> {
    fn new(
        section_name: &str,
        section: RelIterator<'_, LittleEndian>,
        symtab: &SymbolTable<'_, LittleEndian>,
        strtab: &StringTable<'data>,
    ) -> Result<Self, Error> {
        assert!(section_name.starts_with(".rel"));

        let mut fastrel_offsets = HashMap::<_, Vec<u64>>::new();
        for rel in section {
            let symbol = symtab.get(rel.r_sym as usize)?;
            let name = if symbol.st_name == 0 {
                ""
            } else {
                strtab.get(symbol.st_name as usize)?
            };

            fastrel_offsets
                .entry(FastRel {
                    section_index: symbol.st_shndx,
                    section_value: symbol.st_value,
                    r_type: rel.r_type,
                    name,
                })
                .or_default()
                .push(rel.r_offset);
        }

        Ok(FastRelSection {
            name: format!(".fast{}", section_name),
            fastrel_offsets,
        })
    }

    fn write(&self, mut w: impl Write) -> io::Result<()> {
        w.write_all(&[VERSION])?;
        w.write_all(&(self.fastrel_offsets.len() as u32).to_le_bytes())?;
        for (unique, offsets) in &self.fastrel_offsets {
            if unique.section_index > 0 {
                w.write_all(&[(1 << 7) | (unique.r_type & 0x7F) as u8])?;
                w.write_all(&u32::from(unique.section_index).to_le_bytes())?;
                w.write_all(&u32::try_from(unique.section_value).unwrap().to_le_bytes())?;
            } else {
                w.write_all(&[(unique.r_type & 0x7F) as u8])?;
                w.write_all(&unique.gnu_sym_hash().to_le_bytes())?;
            }

            w.write_all(&(offsets.len() as u32).to_le_bytes())?;
            for offset in offsets {
                w.write_all(&offset.to_le_bytes()[..3])?;
            }
        }

        Ok(())
    }
}
