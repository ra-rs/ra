#[doc = "Register `SFMSST` reader"]
pub struct R(crate::R<SFMSST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFMSST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFMSST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFMSST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PFCNT` reader - Number of bytes of prefetched data"]
pub type PFCNT_R = crate::FieldReader<u8, PFCNT_A>;
#[doc = "Number of bytes of prefetched data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PFCNT_A {
    #[doc = "0: 0 byte"]
    _0X00 = 0,
    #[doc = "1: 1 byte"]
    _0X01 = 1,
    #[doc = "2: 2 bytes"]
    _0X02 = 2,
    #[doc = "3: 3 bytes"]
    _0X03 = 3,
    #[doc = "4: 4 bytes"]
    _0X04 = 4,
    #[doc = "5: 5 bytes"]
    _0X05 = 5,
    #[doc = "6: 6 bytes"]
    _0X06 = 6,
    #[doc = "7: 7 bytes"]
    _0X07 = 7,
    #[doc = "8: 8 bytes"]
    _0X08 = 8,
    #[doc = "9: 9 bytes"]
    _0X09 = 9,
    #[doc = "10: 10 bytes"]
    _0X0A = 10,
    #[doc = "11: 11 bytes"]
    _0X0B = 11,
    #[doc = "12: 12 bytes"]
    _0X0C = 12,
    #[doc = "13: 13 bytes"]
    _0X0D = 13,
    #[doc = "14: 14 bytes"]
    _0X0E = 14,
    #[doc = "15: 15 bytes"]
    _0X0F = 15,
    #[doc = "16: 16 bytes"]
    _0X10 = 16,
    #[doc = "17: 17 bytes"]
    _0X11 = 17,
    #[doc = "18: 18 bytes"]
    _0X12 = 18,
}
impl From<PFCNT_A> for u8 {
    #[inline(always)]
    fn from(variant: PFCNT_A) -> Self {
        variant as _
    }
}
impl PFCNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PFCNT_A> {
        match self.bits {
            0 => Some(PFCNT_A::_0X00),
            1 => Some(PFCNT_A::_0X01),
            2 => Some(PFCNT_A::_0X02),
            3 => Some(PFCNT_A::_0X03),
            4 => Some(PFCNT_A::_0X04),
            5 => Some(PFCNT_A::_0X05),
            6 => Some(PFCNT_A::_0X06),
            7 => Some(PFCNT_A::_0X07),
            8 => Some(PFCNT_A::_0X08),
            9 => Some(PFCNT_A::_0X09),
            10 => Some(PFCNT_A::_0X0A),
            11 => Some(PFCNT_A::_0X0B),
            12 => Some(PFCNT_A::_0X0C),
            13 => Some(PFCNT_A::_0X0D),
            14 => Some(PFCNT_A::_0X0E),
            15 => Some(PFCNT_A::_0X0F),
            16 => Some(PFCNT_A::_0X10),
            17 => Some(PFCNT_A::_0X11),
            18 => Some(PFCNT_A::_0X12),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == PFCNT_A::_0X00
    }
    #[doc = "Checks if the value of the field is `_0X01`"]
    #[inline(always)]
    pub fn is_0x01(&self) -> bool {
        *self == PFCNT_A::_0X01
    }
    #[doc = "Checks if the value of the field is `_0X02`"]
    #[inline(always)]
    pub fn is_0x02(&self) -> bool {
        *self == PFCNT_A::_0X02
    }
    #[doc = "Checks if the value of the field is `_0X03`"]
    #[inline(always)]
    pub fn is_0x03(&self) -> bool {
        *self == PFCNT_A::_0X03
    }
    #[doc = "Checks if the value of the field is `_0X04`"]
    #[inline(always)]
    pub fn is_0x04(&self) -> bool {
        *self == PFCNT_A::_0X04
    }
    #[doc = "Checks if the value of the field is `_0X05`"]
    #[inline(always)]
    pub fn is_0x05(&self) -> bool {
        *self == PFCNT_A::_0X05
    }
    #[doc = "Checks if the value of the field is `_0X06`"]
    #[inline(always)]
    pub fn is_0x06(&self) -> bool {
        *self == PFCNT_A::_0X06
    }
    #[doc = "Checks if the value of the field is `_0X07`"]
    #[inline(always)]
    pub fn is_0x07(&self) -> bool {
        *self == PFCNT_A::_0X07
    }
    #[doc = "Checks if the value of the field is `_0X08`"]
    #[inline(always)]
    pub fn is_0x08(&self) -> bool {
        *self == PFCNT_A::_0X08
    }
    #[doc = "Checks if the value of the field is `_0X09`"]
    #[inline(always)]
    pub fn is_0x09(&self) -> bool {
        *self == PFCNT_A::_0X09
    }
    #[doc = "Checks if the value of the field is `_0X0A`"]
    #[inline(always)]
    pub fn is_0x0a(&self) -> bool {
        *self == PFCNT_A::_0X0A
    }
    #[doc = "Checks if the value of the field is `_0X0B`"]
    #[inline(always)]
    pub fn is_0x0b(&self) -> bool {
        *self == PFCNT_A::_0X0B
    }
    #[doc = "Checks if the value of the field is `_0X0C`"]
    #[inline(always)]
    pub fn is_0x0c(&self) -> bool {
        *self == PFCNT_A::_0X0C
    }
    #[doc = "Checks if the value of the field is `_0X0D`"]
    #[inline(always)]
    pub fn is_0x0d(&self) -> bool {
        *self == PFCNT_A::_0X0D
    }
    #[doc = "Checks if the value of the field is `_0X0E`"]
    #[inline(always)]
    pub fn is_0x0e(&self) -> bool {
        *self == PFCNT_A::_0X0E
    }
    #[doc = "Checks if the value of the field is `_0X0F`"]
    #[inline(always)]
    pub fn is_0x0f(&self) -> bool {
        *self == PFCNT_A::_0X0F
    }
    #[doc = "Checks if the value of the field is `_0X10`"]
    #[inline(always)]
    pub fn is_0x10(&self) -> bool {
        *self == PFCNT_A::_0X10
    }
    #[doc = "Checks if the value of the field is `_0X11`"]
    #[inline(always)]
    pub fn is_0x11(&self) -> bool {
        *self == PFCNT_A::_0X11
    }
    #[doc = "Checks if the value of the field is `_0X12`"]
    #[inline(always)]
    pub fn is_0x12(&self) -> bool {
        *self == PFCNT_A::_0X12
    }
}
#[doc = "Field `PFFUL` reader - Prefetch buffer state"]
pub type PFFUL_R = crate::BitReader<PFFUL_A>;
#[doc = "Prefetch buffer state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PFFUL_A {
    #[doc = "0: Prefetch buffer has free space"]
    _0 = 0,
    #[doc = "1: Prefetch buffer is full"]
    _1 = 1,
}
impl From<PFFUL_A> for bool {
    #[inline(always)]
    fn from(variant: PFFUL_A) -> Self {
        variant as u8 != 0
    }
}
impl PFFUL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFFUL_A {
        match self.bits {
            false => PFFUL_A::_0,
            true => PFFUL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PFFUL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PFFUL_A::_1
    }
}
#[doc = "Field `PFOFF` reader - Prefetch function operating state"]
pub type PFOFF_R = crate::BitReader<PFOFF_A>;
#[doc = "Prefetch function operating state\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PFOFF_A {
    #[doc = "0: Prefetch function operating"]
    _0 = 0,
    #[doc = "1: Prefetch function not enabled or not operating"]
    _1 = 1,
}
impl From<PFOFF_A> for bool {
    #[inline(always)]
    fn from(variant: PFOFF_A) -> Self {
        variant as u8 != 0
    }
}
impl PFOFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFOFF_A {
        match self.bits {
            false => PFOFF_A::_0,
            true => PFOFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PFOFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PFOFF_A::_1
    }
}
impl R {
    #[doc = "Bits 0:4 - Number of bytes of prefetched data"]
    #[inline(always)]
    pub fn pfcnt(&self) -> PFCNT_R {
        PFCNT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Prefetch buffer state"]
    #[inline(always)]
    pub fn pfful(&self) -> PFFUL_R {
        PFFUL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Prefetch function operating state"]
    #[inline(always)]
    pub fn pfoff(&self) -> PFOFF_R {
        PFOFF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfmsst](index.html) module"]
pub struct SFMSST_SPEC;
impl crate::RegisterSpec for SFMSST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sfmsst::R](R) reader structure"]
impl crate::Readable for SFMSST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SFMSST to value 0x80"]
impl crate::Resettable for SFMSST_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
