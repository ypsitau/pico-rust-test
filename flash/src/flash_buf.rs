use embedded_storage as es;

pub struct FlashBuf<'a, Flash> {
    flash: Flash,
    offset_xip_cached: u32,
    bytes_cached: u32,
    buf_cache: &'a mut [u8],
}

impl<'a, Flash> FlashBuf<'a, Flash>
where
    Flash: es::nor_flash::ReadNorFlash + es::nor_flash::NorFlash,
{
    const ERASE_SIZE: usize = Flash::ERASE_SIZE;

    pub fn new(flash: Flash, buf_cache: &'a mut [u8]) -> Self {
        assert!(Self::ERASE_SIZE > 0 && Self::ERASE_SIZE <= u32::MAX as usize);
        assert_eq!(buf_cache.len(), Self::ERASE_SIZE);
        Self { flash, offset_xip_cached: u32::MAX, bytes_cached: 0, buf_cache }
    }

    pub fn read(&mut self, mut offset_xip: u32, buf: &mut [u8]) -> Result<(), Flash::Error> {
        if self.offset_xip_cached == u32::MAX {
            self.flash.read(offset_xip, buf)?;
        } else if (offset_xip < self.offset_xip_cached) {
    		// ----------[buf_cache]----------
    		// ---rrrrr-----------------------
    		// ---rrrrrrrrrrrr----------------
    		// ---rrrrrrrrrrrrrrrrrrrrrrr-----
            let mut offset_buf: u32 = 0;
            let mut bytes_rest = buf.len() as u32;
            let bytes_to_copy = core::cmp::min(bytes_rest, self.offset_xip_cached - offset_xip);
            self.flash.read(offset_xip, &mut buf[offset_buf as usize..(offset_buf + bytes_to_copy) as usize])?;
            bytes_rest -= bytes_to_copy;
            if bytes_rest == 0 { return Ok(()); }
            offset_buf += bytes_to_copy;
            offset_xip += bytes_to_copy;
            let bytes_to_copy = core::cmp::min(bytes_rest, self.bytes_cached);
            buf[offset_buf as usize..(offset_buf + bytes_to_copy) as usize]
                .copy_from_slice(&self.buf_cache[0..bytes_to_copy as usize]);
            bytes_rest -= bytes_to_copy;
            if bytes_rest == 0 { return Ok(()); }
            offset_buf += bytes_to_copy;
            offset_xip += bytes_to_copy;
            self.flash.read(offset_xip, &mut buf[offset_buf as usize..(offset_buf + bytes_rest) as usize])?;
        } else if offset_xip < self.offset_xip_cached + self.bytes_cached as u32 {
    		// ----------[buf_cache]----------
    		// -----------rrrrr---------------
    		// -----------rrrrrrrrrrrr--------
            let offset_buf = 0;
            let offset_in_cache = offset_xip - self.offset_xip_cached;
            let mut bytes_rest = buf.len() as u32;
            let bytes_to_copy = core::cmp::min(bytes_rest, self.bytes_cached - offset_in_cache);
            buf[offset_buf as usize..(offset_buf + bytes_to_copy) as usize]
                .copy_from_slice(&self.buf_cache[offset_in_cache as usize..(offset_in_cache + bytes_to_copy) as usize]);
            bytes_rest -= bytes_to_copy;
            if bytes_rest == 0 { return Ok(()); }
            offset_buf += bytes_to_copy;
            offset_xip += bytes_to_copy;
            self.flash.read(offset_xip, &mut buf[offset_buf as usize..(offset_buf + bytes_rest) as usize])?;
        } else {
            // ----------[buf_cache]----------
            // -----------------------rrrrr---
            self.flash.read(offset_xip, buf)?;
        }
        Ok(())
    }

    pub fn write(&mut self, offset_xip: u32, buf: &[u8]) -> Result<(), Flash::Error> {
        Ok(())
    }

    pub fn sync(&mut self) -> Result<(), Flash::Error> {
        self.flush_cache()
    }

    fn flush_cache(&mut self) -> Result<(), Flash::Error> {
        if self.offset_xip_cached == u32::MAX || self.bytes_cached == 0 {
            return Ok(());
        }

        let erase_end = self
            .offset_xip_cached
            .checked_add(Self::ERASE_SIZE as u32)
            .expect("cache range exceeds the flash address space");
        self.flash.erase(self.offset_xip_cached, erase_end)?;
        self.flash.write(self.offset_xip_cached, &self.buf_cache)?;
        Ok(())
    }
}
