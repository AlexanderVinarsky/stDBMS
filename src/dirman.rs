use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use crate::common::{string_to_fixed, fixed_to_string, MIN_DIR_NAME_SIZE, MIN_COL_NAME_SIZE};
use crate::error::{DbError, Result};
use crate::pageman::{Page, PAGE_NAME_SIZE};

pub const DIRECTORY_MAGIC: u8 = 0xCC;
pub const PAGES_PER_DIRECTORY: usize = 32;
pub const DIRECTORY_NAME_SIZE: usize = MIN_DIR_NAME_SIZE;

pub const COLUMN_MAGIC: u8 = 0xEA;
pub const COLUMN_NAME_SIZE: usize = MIN_COL_NAME_SIZE;
pub const COLUMN_INT: u8 = 0x00;
pub const COLUMN_FLOAT: u8 = 0x01;
pub const COLUMN_STRING: u8 = 0x02;

#[derive(Debug, Clone)]
pub struct DirectoryColumn {
    pub type_: u8,
    pub name: [u8; COLUMN_NAME_SIZE],
}

#[derive(Debug)]
pub struct DirectoryHeader {
    pub magic: u8,
    pub name: [u8; DIRECTORY_NAME_SIZE],
    pub page_count: u8,
    pub column_count: u8,
}

#[derive(Debug)]
pub struct Directory {
    pub header: DirectoryHeader,
    pub columns: Vec<DirectoryColumn>,
    pub names: Vec<[u8; PAGE_NAME_SIZE]>,
}

impl Directory {
    pub fn new(name: &str, columns: Option<Vec<DirectoryColumn>>) -> Self {
        let columns_vec = columns.unwrap_or_default();
        
        Directory {
            header: DirectoryHeader {
                magic: DIRECTORY_MAGIC,
                name: string_to_fixed(name),
                page_count: 0,
                column_count: columns_vec.len() as u8,
            },
            columns: columns_vec,
            names: Vec::with_capacity(PAGES_PER_DIRECTORY),
        }
    }

    pub fn add_page(&mut self, page: &Page) -> Result<()> {
        if self.names.len() >= PAGES_PER_DIRECTORY {
            return Err(DbError::InvalidPageCount(self.names.len() as u8));
        }
        
        self.names.push(page.header.name);
        self.header.page_count = self.names.len() as u8;
        Ok(())
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let mut file = File::create(path)?;
        
        file.write_all(&[self.header.magic])?;
        file.write_all(&self.header.name)?;
        file.write_all(&[self.header.page_count])?;
        file.write_all(&[self.header.column_count])?;
        
        for column in &self.columns {
            file.write_all(&[column.type_])?;
            file.write_all(&column.name)?;
        }
        
        for name in &self.names {
            file.write_all(name)?;
        }
        
        Ok(())
    }

    pub fn load(path: &Path) -> Result<Self> {
        let mut file = File::open(path)?;
        
        let mut magic_buf = [0u8; 1];
        file.read_exact(&mut magic_buf)?;
        let magic = magic_buf[0];
        
        let mut name = [0u8; DIRECTORY_NAME_SIZE];
        file.read_exact(&mut name)?;
        
        let mut page_count_buf = [0u8; 1];
        file.read_exact(&mut page_count_buf)?;
        let page_count = page_count_buf[0];
        
        let mut column_count_buf = [0u8; 1];
        file.read_exact(&mut column_count_buf)?;
        let column_count = column_count_buf[0];
        
        if magic != DIRECTORY_MAGIC {
            return Err(DbError::InvalidMagic {
                expected: DIRECTORY_MAGIC,
                found: magic,
            });
        }
        
        let mut columns = Vec::with_capacity(column_count as usize);
        for _i in 0..column_count {
            let mut type_buf = [0u8; 1];
            file.read_exact(&mut type_buf)?;
            let type_ = type_buf[0];
            
            let mut col_name = [0u8; COLUMN_NAME_SIZE];
            file.read_exact(&mut col_name)?;
            
            columns.push(DirectoryColumn {
                type_,
                name: col_name,
            });
        }
        
        let mut names = Vec::with_capacity(page_count as usize);
        for _i in 0..page_count {
            let mut page_name = [0u8; PAGE_NAME_SIZE];
            file.read_exact(&mut page_name)?;
            names.push(page_name);
        }
        
        Ok(Directory {
            header: DirectoryHeader {
                magic,
                name,
                page_count,
                column_count,
            },
            columns,
            names,
        })
    }

    pub fn get_name(&self) -> String {
        fixed_to_string(&self.header.name)
    }

    pub fn get_page_names(&self) -> Vec<String> {
        let mut result = Vec::new();
        for name in &self.names {
            let string_name = String::from_utf8_lossy(name);
            result.push(string_name.trim_end_matches('\0').to_string());
        }
        result
    }
}

impl DirectoryColumn {
    pub fn new_int(name: &str) -> Self {
        Self {
            type_: COLUMN_INT,
            name: string_to_fixed(name),
        }
    }
    
    pub fn new_float(name: &str) -> Self {
        Self {
            type_: COLUMN_FLOAT,
            name: string_to_fixed(name),
        }
    }
    
    pub fn new_string(name: &str) -> Self {
        Self {
            type_: COLUMN_STRING,
            name: string_to_fixed(name),
        }
    }
    
    pub fn get_name(&self) -> String {
        fixed_to_string(&self.name)
    }
}