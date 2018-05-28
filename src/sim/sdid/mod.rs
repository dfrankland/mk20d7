#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SDID {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `PINID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINIDR {
    #[doc = "64-pin"]
    _0101,
    #[doc = "80-pin"]
    _0110,
    #[doc = "81-pin"]
    _0111,
    #[doc = "100-pin"]
    _1000,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PINIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PINIDR::_0101 => 5,
            PINIDR::_0110 => 6,
            PINIDR::_0111 => 7,
            PINIDR::_1000 => 8,
            PINIDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PINIDR {
        match value {
            5 => PINIDR::_0101,
            6 => PINIDR::_0110,
            7 => PINIDR::_0111,
            8 => PINIDR::_1000,
            i => PINIDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == PINIDR::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == PINIDR::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == PINIDR::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == PINIDR::_1000
    }
}
#[doc = "Possible values of the field `FAMID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAMIDR {
    #[doc = "K10"]
    _000,
    #[doc = "K20"]
    _001,
    #[doc = "K30"]
    _010,
    #[doc = "K40"]
    _011,
    #[doc = "K50"]
    _110,
    #[doc = "K51"]
    _111,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FAMIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FAMIDR::_000 => 0,
            FAMIDR::_001 => 1,
            FAMIDR::_010 => 2,
            FAMIDR::_011 => 3,
            FAMIDR::_110 => 6,
            FAMIDR::_111 => 7,
            FAMIDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FAMIDR {
        match value {
            0 => FAMIDR::_000,
            1 => FAMIDR::_001,
            2 => FAMIDR::_010,
            3 => FAMIDR::_011,
            6 => FAMIDR::_110,
            7 => FAMIDR::_111,
            i => FAMIDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == FAMIDR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == FAMIDR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == FAMIDR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == FAMIDR::_011
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == FAMIDR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == FAMIDR::_111
    }
}
#[doc = r" Value of the field"]
pub struct REVIDR {
    bits: u8,
}
impl REVIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Pincount identification"]
    #[inline]
    pub fn pinid(&self) -> PINIDR {
        PINIDR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:6 - Kinetis family identification"]
    #[inline]
    pub fn famid(&self) -> FAMIDR {
        FAMIDR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - Device revision number"]
    #[inline]
    pub fn revid(&self) -> REVIDR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        REVIDR { bits }
    }
}
