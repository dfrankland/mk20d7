#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::VLLSCTRL {
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
#[doc = "Possible values of the field `VLLSM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VLLSMR {
    #[doc = "VLLS1"]
    _001,
    #[doc = "VLLS2"]
    _010,
    #[doc = "VLLS3"]
    _011,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl VLLSMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            VLLSMR::_001 => 1,
            VLLSMR::_010 => 2,
            VLLSMR::_011 => 3,
            VLLSMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> VLLSMR {
        match value {
            1 => VLLSMR::_001,
            2 => VLLSMR::_010,
            3 => VLLSMR::_011,
            i => VLLSMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == VLLSMR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == VLLSMR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == VLLSMR::_011
    }
}
#[doc = "Values that can be written to the field `VLLSM`"]
pub enum VLLSMW {
    #[doc = "VLLS1"]
    _001,
    #[doc = "VLLS2"]
    _010,
    #[doc = "VLLS3"]
    _011,
}
impl VLLSMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            VLLSMW::_001 => 1,
            VLLSMW::_010 => 2,
            VLLSMW::_011 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VLLSMW<'a> {
    w: &'a mut W,
}
impl<'a> _VLLSMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VLLSMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "VLLS1"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(VLLSMW::_001)
    }
    #[doc = "VLLS2"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(VLLSMW::_010)
    }
    #[doc = "VLLS3"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(VLLSMW::_011)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:2 - VLLS Mode Control"]
    #[inline]
    pub fn vllsm(&self) -> VLLSMR {
        VLLSMR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 3 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - VLLS Mode Control"]
    #[inline]
    pub fn vllsm(&mut self) -> _VLLSMW {
        _VLLSMW { w: self }
    }
}
