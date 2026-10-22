use embedded_storage as es;

pub struct FlashBuf<'a, Flash> {
    flash: Flash,
    offset_xip_cached: u32,
    buff_cache: &'a mut [u8],
    cache_dirty: bool,
}

impl<'a, Flash> FlashBuf<'a, Flash>
where
    Flash: es::nor_flash::ReadNorFlash + es::nor_flash::NorFlash,
{
    const ERASE_SIZE: usize = Flash::ERASE_SIZE;

    /// `buff_cache` must be exactly `Flash::ERASE_SIZE` bytes long.
    pub fn new(flash: Flash, buff_cache: &'a mut [u8]) -> Self {
        assert!(Self::ERASE_SIZE > 0 && Self::ERASE_SIZE <= u32::MAX as usize);
        assert_eq!(buff_cache.len(), Self::ERASE_SIZE);

        Self {
            flash,
            offset_xip_cached: u32::MAX,
            buff_cache,
            cache_dirty: false,
        }
    }

    pub fn read(&mut self, offset: u32, buf: &mut [u8]) -> Result<(), Flash::Error> {
        if buf.is_empty() {
            return Ok(());
        }

        let bytes = u32::try_from(buf.len()).expect("read buffer exceeds the flash address space");
        let end = offset
            .checked_add(bytes)
            .expect("read range exceeds the flash address space");
        if self.offset_xip_cached == u32::MAX {
            return self.flash.read(offset, buf);
        }

        let cache_end = self
            .offset_xip_cached
            .checked_add(Self::ERASE_SIZE as u32)
            .expect("cache range exceeds the flash address space");
        if offset >= cache_end || end <= self.offset_xip_cached {
            return self.flash.read(offset, buf);
        }

        let overlap_start = offset.max(self.offset_xip_cached);
        let overlap_end = end.min(cache_end);

        if offset < overlap_start {
            let prefix_len = (overlap_start - offset) as usize;
            self.flash.read(offset, &mut buf[..prefix_len])?;
        }

        let cache_start = (overlap_start - self.offset_xip_cached) as usize;
        let output_start = (overlap_start - offset) as usize;
        let overlap_len = (overlap_end - overlap_start) as usize;
        buf[output_start..output_start + overlap_len]
            .copy_from_slice(&self.buff_cache[cache_start..cache_start + overlap_len]);

        if overlap_end < end {
            let suffix_start = (overlap_end - offset) as usize;
            self.flash.read(overlap_end, &mut buf[suffix_start..])?;
        }

        Ok(())
    }

    pub fn write(&mut self, offset: u32, buf: &[u8]) -> Result<(), Flash::Error> {
        if buf.is_empty() {
            return Ok(());
        }

        let bytes = u32::try_from(buf.len()).expect("write buffer exceeds the flash address space");
        offset
            .checked_add(bytes)
            .expect("write range exceeds the flash address space");

        let erase_size = Self::ERASE_SIZE as u32;
        let mut written = 0;
        while written < buf.len() {
            let position = offset + written as u32;
            let block_offset = position - position % erase_size;

            if self.offset_xip_cached != block_offset {
                self.flush_cache()?;
                self.flash.read(block_offset, &mut self.buff_cache)?;
                self.offset_xip_cached = block_offset;
            }

            let offset_in_cache = (position - block_offset) as usize;
            let bytes_to_copy = (buf.len() - written).min(Self::ERASE_SIZE - offset_in_cache);
            self.buff_cache[offset_in_cache..offset_in_cache + bytes_to_copy]
                .copy_from_slice(&buf[written..written + bytes_to_copy]);
            self.cache_dirty = true;
            written += bytes_to_copy;
        }

        Ok(())
    }

    pub fn sync(&mut self) -> Result<(), Flash::Error> {
        self.flush_cache()
    }

    fn flush_cache(&mut self) -> Result<(), Flash::Error> {
        if self.offset_xip_cached == u32::MAX || !self.cache_dirty {
            return Ok(());
        }

        let erase_end = self
            .offset_xip_cached
            .checked_add(Self::ERASE_SIZE as u32)
            .expect("cache range exceeds the flash address space");
        self.flash.erase(self.offset_xip_cached, erase_end)?;
        self.flash.write(self.offset_xip_cached, &self.buff_cache)?;
        self.cache_dirty = false;
        Ok(())
    }
}
