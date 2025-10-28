use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

const PAGE_MAGIC: u8 = 0xD1;
const DIRECTORY_MAGIC: u8 = 0xD1;




#[derive(Debug)]
pub enum FileManagerError {
    InvalidMagic { expected: u8, found: u8 },
    InvalidPageCount(u8),
    Io(std::io::Error),
}

impl From<std::io::Error> for FileManagerError {
    fn from(error: std::io::Error) -> Self {
        FileManagerError::Io(error)
    }
}

impl std::fmt::Display for FileManagerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}



type Result<T> = std::result::Result<T, FileManagerError>;





pub struct PageHeader {
    pub magic_check: u8,
    pub name: [u8; 8],
    pub content_size: u8,
}

pub struct Page {
    pub header: PageHeader,
    pub content: Vec <u8>,

}

pub struct DirectoryHeader {
    pub magic_check: u8,              
    pub name: [u8; 8],          
    pub page_count: u8,        
}

pub struct Directory {
    pub header: DirectoryHeader,    
    pub names: Vec<[u8; 8]>,      
}





impl Page {
    pub fn new(name: [u8; 8], content: Vec<u8>)->Result<Self> {
        if content.len() > u8::MAX as usize {
            return Err(FileManagerError::InvalidPageCount(content.len() as u8));
        }

        Ok(Page {
            header: PageHeader {
                magic_check: PAGE_MAGIC,
                name,
                content_size: content.len() as u8},
            content
        })
    }
    
    pub fn save(&self, path: &Path)->Result<()> {
        let mut file = File::create(path)?;
        
        file.write_all(&[self.header.magic_check])?;
        file.write_all(&self.header.name)?;
        file.write_all(&[self.header.content_size])?;
        file.write_all(&self.content)?;
        
        Ok(())
    }

    pub fn load(path: &Path)->Result<Self> {
        let mut file = File::open(path)?;
        
        let mut magic_buf = [0u8; 1];
        file.read_exact(&mut magic_buf)?;
        let magic_check = magic_buf[0];
        
        let mut name = [0u8; 8];
        file.read_exact(&mut name)?;
        
        let mut size_buf = [0u8; 1];
        file.read_exact(&mut size_buf)?;
        let content_size = size_buf[0];
        
        if magic_check != PAGE_MAGIC {
            return Err(FileManagerError::InvalidMagic {
                expected: PAGE_MAGIC,
                found: magic_check
            });
        }
        
        let mut content = vec![0u8; content_size as usize];
        file.read_exact(&mut content)?;
        
        Ok(Page {
            header: PageHeader {
                magic_check,
                name,
                content_size,
            },
            content,
        })
    }
}





impl Directory {
    pub fn new(name: [u8; 8])->Self {
        Directory {
            header: DirectoryHeader {
                magic_check: DIRECTORY_MAGIC,
                name,
                page_count: 0,
            },
            names: Vec::new(),
        }
    }

    pub fn add_page(&mut self, page_name: [u8; 8]) {
        self.names.push(page_name);
        self.header.page_count = self.names.len() as u8;
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let mut file = File::create(path)?;
        
        file.write_all(&[self.header.magic_check])?;
        file.write_all(&self.header.name)?;
        file.write_all(&[self.header.page_count])?;
        
        for name in &self.names {
            file.write_all(name)?;
        }
        Ok(())
    }

    pub fn load(path: &Path) -> Result<Self> {
        let mut file = File::open(path)?;
        
        let mut magic_buf = [0u8; 1];
        file.read_exact(&mut magic_buf)?;
        let magic_check = magic_buf[0];
        
        let mut name = [0u8; 8];
        file.read_exact(&mut name)?;
        
        let mut count_buf = [0u8; 1];
        file.read_exact(&mut count_buf)?;
        let page_count = count_buf[0];
        
        if magic_check != DIRECTORY_MAGIC {
            return Err(FileManagerError::InvalidMagic {
                expected: DIRECTORY_MAGIC,
                found: magic_check,
            });
        }
        
        let mut names = Vec::with_capacity(page_count as usize);
        for _i in 0..page_count {
            let mut page_name = [0u8; 8];
            file.read_exact(&mut page_name)?;
            names.push(page_name);
        }
        
        Ok(Directory {
            header: DirectoryHeader {
                magic_check,
                name,
                page_count,
            },
            names,
        })
    }
}

pub fn string_to_fixed_name(s: &str) -> [u8; 8] {
    let mut name = [0u8; 8];
    let bytes = s.as_bytes();
    let len = bytes.len().min(8);
    name[..len].copy_from_slice(&bytes[..len]);
    name
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_page_operations() -> Result<()> {
        let temp_dir = std::env::temp_dir();
        let page_path = temp_dir.join("test_page.spg");
        
        let page_name = string_to_fixed_name("testpage");
        let content = b"stDBMS first test!".to_vec();
        let page = Page::new(page_name, content)?;
        page.save(&page_path)?;
        
        let loaded_page = Page::load(&page_path)?;
    
        assert_eq!(loaded_page.header.name, page_name);
        assert_eq!(loaded_page.content, b"stDBMS first test!");
        fs::remove_file(page_path)?;
        Ok(())
    }

    #[test]
    pub fn test_directory_operations() -> Result<()> {
        let temp_dir = std::env::temp_dir();
        let dir_path = temp_dir.join("test_dir.sdir");
        
        let mut directory = Directory::new(string_to_fixed_name("testdir"));
        directory.add_page(string_to_fixed_name("page1"));
        directory.add_page(string_to_fixed_name("page2"));
        directory.save(&dir_path)?;
        
        let loaded_dir = Directory::load(&dir_path)?;
        
        assert_eq!(loaded_dir.header.page_count, 2);
        assert_eq!(loaded_dir.names.len(), 2);
        
        fs::remove_file(dir_path)?;
        Ok(())
    }
}


fn main() {
    println!("...");
}
