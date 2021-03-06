#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::C4 {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct SCFTRIMR {
    bits: bool,
}
impl SCFTRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct FCTRIMR {
    bits: u8,
}
impl FCTRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `DRST_DRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRST_DRSR {
    #[doc = "Encoding 0 - Low range (reset default)."]
    _00,
    #[doc = "Encoding 1 - Mid range."]
    _01,
    #[doc = "Encoding 2 - Mid-high range."]
    _10,
    #[doc = "Encoding 3 - High range."]
    _11,
}
impl DRST_DRSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DRST_DRSR::_00 => 0,
            DRST_DRSR::_01 => 1,
            DRST_DRSR::_10 => 2,
            DRST_DRSR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DRST_DRSR {
        match value {
            0 => DRST_DRSR::_00,
            1 => DRST_DRSR::_01,
            2 => DRST_DRSR::_10,
            3 => DRST_DRSR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == DRST_DRSR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == DRST_DRSR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == DRST_DRSR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == DRST_DRSR::_11
    }
}
#[doc = "Possible values of the field `DMX32`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMX32R {
    #[doc = "DCO has a default range of 25%."]
    _0,
    #[doc = "DCO is fine-tuned for maximum frequency with 32.768 kHz reference."]
    _1,
}
impl DMX32R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DMX32R::_0 => false,
            DMX32R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMX32R {
        match value {
            false => DMX32R::_0,
            true => DMX32R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DMX32R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DMX32R::_1
    }
}
#[doc = r" Proxy"]
pub struct _SCFTRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _SCFTRIMW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FCTRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _FCTRIMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DRST_DRS`"]
pub enum DRST_DRSW {
    #[doc = "Encoding 0 - Low range (reset default)."]
    _00,
    #[doc = "Encoding 1 - Mid range."]
    _01,
    #[doc = "Encoding 2 - Mid-high range."]
    _10,
    #[doc = "Encoding 3 - High range."]
    _11,
}
impl DRST_DRSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DRST_DRSW::_00 => 0,
            DRST_DRSW::_01 => 1,
            DRST_DRSW::_10 => 2,
            DRST_DRSW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DRST_DRSW<'a> {
    w: &'a mut W,
}
impl<'a> _DRST_DRSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DRST_DRSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Encoding 0 - Low range (reset default)."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(DRST_DRSW::_00)
    }
    #[doc = "Encoding 1 - Mid range."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(DRST_DRSW::_01)
    }
    #[doc = "Encoding 2 - Mid-high range."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(DRST_DRSW::_10)
    }
    #[doc = "Encoding 3 - High range."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(DRST_DRSW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMX32`"]
pub enum DMX32W {
    #[doc = "DCO has a default range of 25%."]
    _0,
    #[doc = "DCO is fine-tuned for maximum frequency with 32.768 kHz reference."]
    _1,
}
impl DMX32W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMX32W::_0 => false,
            DMX32W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMX32W<'a> {
    w: &'a mut W,
}
impl<'a> _DMX32W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMX32W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DCO has a default range of 25%."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMX32W::_0)
    }
    #[doc = "DCO is fine-tuned for maximum frequency with 32.768 kHz reference."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMX32W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Slow Internal Reference Clock Fine Trim"]
    #[inline]
    pub fn scftrim(&self) -> SCFTRIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        SCFTRIMR { bits }
    }
    #[doc = "Bits 1:4 - Fast Internal Reference Clock Trim Setting"]
    #[inline]
    pub fn fctrim(&self) -> FCTRIMR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        FCTRIMR { bits }
    }
    #[doc = "Bits 5:6 - DCO Range Select"]
    #[inline]
    pub fn drst_drs(&self) -> DRST_DRSR {
        DRST_DRSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 7 - DCO Maximum Frequency with 32.768 kHz Reference"]
    #[inline]
    pub fn dmx32(&self) -> DMX32R {
        DMX32R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Slow Internal Reference Clock Fine Trim"]
    #[inline]
    pub fn scftrim(&mut self) -> _SCFTRIMW {
        _SCFTRIMW { w: self }
    }
    #[doc = "Bits 1:4 - Fast Internal Reference Clock Trim Setting"]
    #[inline]
    pub fn fctrim(&mut self) -> _FCTRIMW {
        _FCTRIMW { w: self }
    }
    #[doc = "Bits 5:6 - DCO Range Select"]
    #[inline]
    pub fn drst_drs(&mut self) -> _DRST_DRSW {
        _DRST_DRSW { w: self }
    }
    #[doc = "Bit 7 - DCO Maximum Frequency with 32.768 kHz Reference"]
    #[inline]
    pub fn dmx32(&mut self) -> _DMX32W {
        _DMX32W { w: self }
    }
}
