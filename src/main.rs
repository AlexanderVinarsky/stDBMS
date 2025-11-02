use std::path::Path;
use stDBMS::{Directory, DirectoryColumn, Page};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let columns = vec![
        DirectoryColumn::new_int("id"),
        DirectoryColumn::new_string("name"),
        DirectoryColumn::new_float("price"),
    ];
    
    let mut dir = Directory::new("products", Some(columns));
    
    let page1 = Page::new("page1", b"1|Widget|19.99")?;
    let page2 = Page::new("page2", b"2|Gadget|29.99")?;
    
    dir.add_page(&page1)?;
    dir.add_page(&page2)?;
    
    dir.save(Path::new("products.dr"))?;
    page1.save(Path::new("page1.pg"))?;
    page2.save(Path::new("page2.pg"))?;
    
    println!("Directory '{}' created with {} pages", 
        dir.get_name(), dir.header.page_count);
    
    let loaded_dir = Directory::load(Path::new("products.dr"))?;
    let loaded_page = Page::load(Path::new("page1.pg"))?;
    
    println!("Directory loaded: {}", loaded_dir.get_name());
    println!("Page content: {}", String::from_utf8_lossy(loaded_page.get_content()));
    
    Ok(())
}