use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use crate::common::{string_to_fixed, fixed_to_string, MIN_PAGE_NAME_SIZE};
use crate::error::{DbError, Result};

pub const PAGE_MAGIC: u8 = 0xCA;
pub const PAGE_CONTENT_SIZE: usize = 256;
pub const PAGE_END: u8 = 0xED;
pub const COLUMN_DELIMITER: u8 = 0xEE;
pub const PAGE_NAME_SIZE: usize = MIN_PAGE_NAME_SIZE;

#[derive(Debug)]
pub struct PageHeader {
    pub magic: u8,
    pub name: [u8; PAGE_NAME_SIZE],
}

#[derive(Debug)]
pub struct Page {
    pub header: PageHeader,
    pub content: [u8; PAGE_CONTENT_SIZE],
}

impl Page {
    pub fn new(name: &str, buffer: &[u8]) -> Result<Self> {
        if name.len() > PAGE_NAME_SIZE {
            return Err(DbError::InvalidInput {
                expected: name.to_string(),
                found: PAGE_NAME_SIZE,
            });
        }

        let mut content = [0u8; PAGE_CONTENT_SIZE];
        let copy_len = buffer.len().min(PAGE_CONTENT_SIZE - 1);
        
        content[..copy_len].copy_from_slice(&buffer[..copy_len]);
        content[copy_len] = PAGE_END;

        Ok(Page {
            header: PageHeader {
                magic: PAGE_MAGIC,
                name: string_to_fixed(name),
            },
            content,
        })
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let mut file = File::create(path)?;
        
        file.write_all(&[self.header.magic])?;
        file.write_all(&self.header.name)?;
        file.write_all(&self.content)?;
        
        Ok(())
    }

    pub fn load(path: &Path) -> Result<Self> {
        let mut file = File::open(path)?;
        
        let mut magic_buf = [0u8; 1];
        file.read_exact(&mut magic_buf)?;
        let magic = magic_buf[0];
        
        let mut name = [0u8; PAGE_NAME_SIZE];
        file.read_exact(&mut name)?;
        
        if magic != PAGE_MAGIC {
            return Err(DbError::InvalidMagic {
                expected: PAGE_MAGIC,
                found: magic,
            });
        }
        
        let mut content = [0u8; PAGE_CONTENT_SIZE];
        file.read_exact(&mut content)?;
        
        Ok(Page {
            header: PageHeader { magic, name },
            content,
        })
    }

    pub fn get_name(&self) -> String {
        fixed_to_string(&self.header.name)
    }

    pub fn get_content(&self) -> &[u8] {
        let end_pos = self.content.iter()
            .position(|&b| b == PAGE_END)
            .unwrap_or(PAGE_CONTENT_SIZE);
        &self.content[..end_pos]
    }
}