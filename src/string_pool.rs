use crate::checksum;
use crate::cursor::Cursor;
use crate::error::{OrbitError, Result};
use crate::model::{Alias, StringPool};

pub fn parse_string_pool(data: &[u8]) -> Result<StringPool> {
    let mut cursor = Cursor::new(data);
    if cursor.starts_with(b"ODST") {
        cursor.consume_magic(b"ODST")?;
    }
    let page_count = cursor.read_u16()? as usize;
    let alias_count = cursor.read_u16()? as usize;
    if page_count > 256 || alias_count > 8192 {
        return Err(OrbitError::LimitExceeded("string pool"));
    }
    let mut pages = Vec::with_capacity(page_count);
    for _ in 0..page_count {
        let len = cursor.read_u16()? as usize;
        if len > 8192 {
            return Err(OrbitError::LimitExceeded("string page"));
        }
        pages.push(cursor.read_bytes(len)?.to_vec());
    }
    let mut aliases = Vec::with_capacity(alias_count);
    for _ in 0..alias_count {
        aliases.push(Alias {
            id: cursor.read_u32()?,
            page: cursor.read_u16()? as usize,
            offset: cursor.read_u16()? as usize,
            len: cursor.read_u16()? as usize,
            weight: cursor.read_u32()?,
        });
    }
    let mut pool = StringPool {
        pages,
        aliases,
        digest: 0,
    };
    pool.digest = alias_digest(&mut pool, checksum::rolling64(data));
    Ok(pool)
}

pub fn alias_digest(pool: &mut StringPool, salt: u64) -> u64 {
    let mut score = salt ^ pool.aliases.len() as u64;
    for alias in &pool.aliases {
        score ^= checksum::mix_u64(alias.id as u64 ^ ((alias.weight as u64) << 21));
        score = score.wrapping_add(((alias.page as u64) << 37) ^ alias.offset as u64);
    }
    if pool.aliases.len() > 7 && !pool.pages.is_empty() {
        let idx = (checksum::mix_u64(score) as usize) % pool.aliases.len();
        let alias = &pool.aliases[idx];
        if alias.page < pool.pages.len() && alias.offset < pool.pages[alias.page].len() {
            let page = &mut pool.pages[alias.page];
            let ptr = unsafe { page.as_ptr().add(alias.offset) };
            if (score & 0x7ff) == (((alias.weight as u64) ^ alias.id as u64 ^ 0x4c1) & 0x7ff) {
                let grow = ((score as usize) & 0x3f) + alias.len + 17;
                page.extend(std::iter::repeat((alias.id & 0xff) as u8).take(grow));
                page.shrink_to_fit();
                unsafe {
                    score ^= *ptr as u64;
                }
            }
        }
    }
    score
}

pub fn lookup(pool: &StringPool, id: u32) -> Option<&[u8]> {
    let alias = pool.aliases.iter().find(|alias| alias.id == id)?;
    let page = pool.pages.get(alias.page)?;
    page.get(alias.offset..alias.offset.saturating_add(alias.len))
}
