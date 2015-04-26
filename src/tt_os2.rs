use ffi;

#[derive(Copy, Clone)]
pub struct TrueTypeOS2Table {
    raw: ffi::TT_OS2_Internal
}

impl TrueTypeOS2Table  {
    #[inline(always)]
    pub fn version(&self) -> ffi::FT_UShort {
        unsafe {
            (*self.raw).version
        }
    }

    #[inline(always)]
    pub fn avg_char_width(&self) -> ffi::FT_Short {
        unsafe {
            (*self.raw).xAvgCharWidth
        }
    }

    #[inline(always)]
    pub fn us_weight_class(&self) -> ffi::FT_UShort {
        unsafe {
            (*self.raw).usWeightClass
        }
    }

    #[inline(always)]
    pub fn us_width_class(&self) -> ffi::FT_UShort {
        unsafe {
            (*self.raw).usWidthClass
        }
    }

    #[inline(always)]
    pub fn fs_type(&self) -> ffi::FT_Short {
        unsafe {
            (*self.raw).fsType
        }
    }


    #[inline(always)]
    pub fn y_subscript_x_size(&self) -> ffi::FT_Short {
        unsafe {
            (*self.raw).ySubscriptXSize
        }
    }

    #[inline(always)]
    pub fn y_subscript_y_size(&self) -> ffi::FT_Short {
        unsafe {
            (*self.raw).ySubscriptYSize
        }
    }

    #[inline(always)]
    pub fn y_subscript_x_offset(&self) -> ffi::FT_Short {
        unsafe {
            (*self.raw).ySubscriptXOffset
        }
    }


    #[inline(always)]
    pub fn y_subscript_y_offset(&self) -> ffi::FT_Short {
        unsafe {
            (*self.raw).ySubscriptYOffset
        }
    }

    #[inline(always)]
    pub fn y_superscript_x_size(&self) -> ffi::FT_Short {
        unsafe {
            (*self.raw).ySuperscriptXSize
        }
    }

    #[inline(always)]
    pub fn y_superscript_y_size(&self) -> ffi::FT_Short {
        unsafe {
            (*self.raw).ySuperscriptYSize
        }
    }

    #[inline(always)]
    pub fn y_superscript_x_offset(&self) -> ffi::FT_Short {
        unsafe {
            (*self.raw).ySuperscriptXOffset
        }
    }

    #[inline(always)]
    pub fn y_superscript_y_offset(&self) -> ffi::FT_Short {
        unsafe {
            (*self.raw).ySuperscriptYOffset
        }
    }

    #[inline(always)]
    pub fn y_strikeout_size(&self) -> ffi::FT_Short {
        unsafe {
            (*self.raw).yStrikeoutSize
        }
    }

    #[inline(always)]
    pub fn y_strikeout_position(&self) -> ffi::FT_Short {
        unsafe {
            (*self.raw).yStrikeoutPosition
        }
    }

    #[inline(always)]
    pub fn s_family_class(&self) -> ffi::FT_Short {
        unsafe {
            (*self.raw).sFamilyClass
        }
    }
}
