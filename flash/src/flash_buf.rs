use embedded_storage as es;

pub struct FlashBuf<'a, Flash> {
    flash: Flash,
    offset_xip_cached: u32,
    buf_cache: &'a mut [u8],
}

#[allow(dead_code)]
impl<'a, Flash> FlashBuf<'a, Flash>
where
    Flash: es::nor_flash::ReadNorFlash + es::nor_flash::NorFlash,
{
    const ERASE_SIZE: u32 = Flash::ERASE_SIZE as u32;

    pub fn new(flash: Flash, buf_cache: &'a mut [u8]) -> Self {
        assert_eq!(buf_cache.len(), Self::ERASE_SIZE as usize);
        Self { flash, offset_xip_cached: u32::MAX, buf_cache }
    }

    pub fn is_cache_invalid(&self) -> bool {
        self.offset_xip_cached == u32::MAX
    }

    pub fn read(&mut self, mut offset_xip: u32, dst: &mut [u8]) -> Result<(), Flash::Error> {
        if self.is_cache_invalid() {
            self.flash.read(offset_xip, dst)?;
        } else if offset_xip < self.offset_xip_cached {
            //    offset_xip
            //    |      self.offset_xip_cached
            //    |      |
    		// ----------[buf_cache]----------
    		// ---RRRRR-----------------------
    		// ---RRRRRRRCCCCC----------------
    		// ---RRRRRRRCCCCCCCCCCCRRRRR-----
            let mut offset_dst: u32 = 0;
            let mut bytes_rest = dst.len() as u32;
            let bytes_to_read = core::cmp::min(bytes_rest, self.offset_xip_cached - offset_xip);
            self.flash.read(offset_xip, &mut dst[offset_dst as usize..(offset_dst + bytes_to_read) as usize])?;
            bytes_rest -= bytes_to_read;
            if bytes_rest > 0 {
                offset_dst += bytes_to_read;
                offset_xip += bytes_to_read;
                let bytes_to_copy = core::cmp::min(bytes_rest, Self::ERASE_SIZE);
                dst[offset_dst as usize..(offset_dst + bytes_to_copy) as usize]
                    .copy_from_slice(&self.buf_cache[0..bytes_to_copy as usize]);
                bytes_rest -= bytes_to_copy;
                if bytes_rest > 0 {
                    offset_dst += bytes_to_copy;
                    offset_xip += bytes_to_copy;
                    let bytes_to_read = bytes_rest;
                    self.flash.read(offset_xip, &mut dst[offset_dst as usize..(offset_dst + bytes_to_read) as usize])?;
                }
            }
        } else if offset_xip < self.offset_xip_cached + Self::ERASE_SIZE { 
            //           self.offset_xip_cached
            //           | offset_xip
            //           | |        self.offset_xip_cached + Self::ERASE_SIZE
            //           | |        |
    		// ----------[buf_cache]----------
    		// ------------CCCCC---------------
    		// ------------CCCCCCCCCRRRR-------
            let mut offset_dst = 0;
            let offset_in_cache = offset_xip - self.offset_xip_cached;
            let mut bytes_rest = dst.len() as u32;
            let bytes_to_copy = core::cmp::min(bytes_rest, Self::ERASE_SIZE - offset_in_cache);
            dst[offset_dst as usize..(offset_dst + bytes_to_copy) as usize]
                .copy_from_slice(&self.buf_cache[offset_in_cache as usize..(offset_in_cache + bytes_to_copy) as usize]);
            bytes_rest -= bytes_to_copy;
            if bytes_rest > 0 {
                offset_dst += bytes_to_copy;
                offset_xip += bytes_to_copy;
                let bytes_to_read = bytes_rest;
                self.flash.read(offset_xip, &mut dst[offset_dst as usize..(offset_dst + bytes_to_read) as usize])?;
            }
        } else {
            // ----------[buf_cache]----------
            // -----------------------RRRRR---
            self.flash.read(offset_xip, dst)?;
        }
        Ok(())
    }

    pub fn write(&mut self, offset_xip: u32, src: &[u8]) -> Result<(), Flash::Error> {
        if self.is_cache_invalid() {
            self.offset_xip_cached = offset_xip & !(Self::ERASE_SIZE - 1);
            self.flash.read(self.offset_xip_cached, &mut self.buf_cache)?;
        }
        let bytes_src = src.len() as u32;
        if offset_xip + bytes_src < self.offset_xip_cached {
            //         offset_xip + bytes_src
            //         | self.offset_xip_cached
            //         | |
            // ----------[buf_cache]----------
            // ---WWWWWW----------------------
            self.flush_cache()?;
        } else if self.offset_xip_cached + Self::ERASE_SIZE <= offset_xip {
            //                     self.offset_xip_cached + Self::ERASE_SIZE
            //                     | offset_xip
            //                     | |
            // ----------[buf_cache]----------
            // ----------------------WWWWWW---
            self.flush_cache()?;
        } else if offset_xip < self.offset_xip_cached {
            //    offset_xip
            //    |      self.offset_xip_cached
            //    |      |
            // ----------[buf_cache]----------
            // ---WWWWWWWCCCCC----------------
            // ---WWWWWWWCCCCCCCCCCCWWWWWWW---
            let bytes_to_copy = core::cmp::min(offset_xip + bytes_src - self.offset_xip_cached, Self::ERASE_SIZE);
            let offset_src = self.offset_xip_cached - offset_xip;
            self.buf_cache.copy_from_slice(&src[offset_src as usize..(offset_src + bytes_to_copy) as usize]);
            self.flush_cache()?;
        } else if offset_xip + bytes_src <= self.offset_xip_cached + Self::ERASE_SIZE {
            //           self.offset_xip_cached
            //           | offset_xip
            //           | |   offset_xip + bytes_src
            //           | |   |   self.offset_xip_cached + Self::ERASE_SIZE
            //           | |   |   |
            // ----------[buf_cache]----------
            // ----------- CCCCC--------------
            let offset_in_cache = offset_xip - self.offset_xip_cached;
            self.buf_cache[offset_in_cache as usize..(offset_in_cache + bytes_src) as usize]
                .copy_from_slice(&src[0..bytes_src as usize]);
            return Ok(());	// no need to flash-program
        } else {
            // ----------[buf_cache]----------
            // -------------CCCCCCCCWWW-------
            let offset_in_cache = offset_xip - self.offset_xip_cached;
            let bytes_to_copy = Self::ERASE_SIZE - offset_in_cache;
            self.buf_cache[offset_in_cache as usize..(offset_in_cache + bytes_to_copy) as usize]
                .copy_from_slice(&src[0..bytes_to_copy as usize]);
            self.flush_cache()?;
        }
        let offset_xip_to_skip = self.offset_xip_cached;
        self.offset_xip_cached = offset_xip & !(Self::ERASE_SIZE - 1);
        let mut offset_src = 0;
        let mut offset_in_cache = offset_xip - self.offset_xip_cached;
        let mut bytes_rest = bytes_src;
        loop {
            if offset_in_cache > 0 || bytes_rest < Self::ERASE_SIZE {
                self.flash.read(self.offset_xip_cached, &mut self.buf_cache)?;
            }
            let bytes_to_copy = core::cmp::min(bytes_rest, Self::ERASE_SIZE - offset_in_cache);
            self.buf_cache[offset_in_cache as usize..(offset_in_cache + bytes_to_copy) as usize]
                .copy_from_slice(&src[offset_src as usize..(offset_src + bytes_to_copy) as usize]);
            bytes_rest -= bytes_to_copy;
            if bytes_rest == 0 { break; }
            if self.offset_xip_cached != offset_xip_to_skip {
                self.flush_cache()?;
            }
            offset_in_cache = 0;
            self.offset_xip_cached += Self::ERASE_SIZE;
            offset_src += bytes_to_copy;
        }
        Ok(())
    }

    fn flush_cache(&mut self) -> Result<(), Flash::Error> {
        self.flash.erase(self.offset_xip_cached, self.offset_xip_cached + Self::ERASE_SIZE)?;
        self.flash.write(self.offset_xip_cached, &self.buf_cache)?;
        Ok(())
    }
}

/*
impl<'a, Flash> es::nor_flash::ErrorType for FlashBuf<'a, Flash>
where
    Flash: es::nor_flash::ReadNorFlash + es::nor_flash::NorFlash,
{
    type Error = Flash::Error;
}

impl<'a, Flash> es::nor_flash::ReadNorFlash for FlashBuf<'a, Flash>
where
    Flash: es::nor_flash::ReadNorFlash + es::nor_flash::NorFlash,
{
	const READ_SIZE: usize = 1;
	fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
        self.read(offset, bytes)
    }
	fn capacity(&self) -> usize {
		self.flash.capacity()
	}
}

impl<'a, Flash> es::nor_flash::NorFlash for FlashBuf<'a, Flash>
where
    Flash: es::nor_flash::ReadNorFlash + es::nor_flash::NorFlash,
{
    const ERASE_SIZE: usize = 1;
    const WRITE_SIZE: usize = 1;

    fn erase(&mut self, from: u32, to: u32) -> Result<(), Self::Error> {
        self.flush_cache()?;
        self.flash.erase(from, to)
    }

    fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error> {
        self.write(offset, bytes)
    }
}
*/
