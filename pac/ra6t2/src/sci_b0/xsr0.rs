#[doc = "Register `XSR0` reader"]
pub struct R(crate::R<XSR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XSR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XSR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XSR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SFSF` reader - Start Frame Status flag"]
pub type SFSF_R = crate::BitReader<SFSF_A>;
#[doc = "Start Frame Status flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFSF_A {
    #[doc = "0: Start Frame detection disabled or Start Frame detection complete"]
    _0 = 0,
    #[doc = "1: Before Start Frame detection or during detection"]
    _1 = 1,
}
impl From<SFSF_A> for bool {
    #[inline(always)]
    fn from(variant: SFSF_A) -> Self {
        variant as u8 != 0
    }
}
impl SFSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFSF_A {
        match self.bits {
            false => SFSF_A::_0,
            true => SFSF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SFSF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SFSF_A::_1
    }
}
#[doc = "Field `RXDSF` reader - RXDn input status flag"]
pub type RXDSF_R = crate::BitReader<RXDSF_A>;
#[doc = "RXDn input status flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDSF_A {
    #[doc = "0: RXDn input to SCI is enabled"]
    _0 = 0,
    #[doc = "1: RXDn input to SCI is disabled"]
    _1 = 1,
}
impl From<RXDSF_A> for bool {
    #[inline(always)]
    fn from(variant: RXDSF_A) -> Self {
        variant as u8 != 0
    }
}
impl RXDSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDSF_A {
        match self.bits {
            false => RXDSF_A::_0,
            true => RXDSF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXDSF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXDSF_A::_1
    }
}
#[doc = "Field `BFOF` reader - Break Field Output completion flag"]
pub type BFOF_R = crate::BitReader<BFOF_A>;
#[doc = "Break Field Output completion flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFOF_A {
    #[doc = "0: When Break Field is not output or during output"]
    _0 = 0,
    #[doc = "1: When Break Field output is completed"]
    _1 = 1,
}
impl From<BFOF_A> for bool {
    #[inline(always)]
    fn from(variant: BFOF_A) -> Self {
        variant as u8 != 0
    }
}
impl BFOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFOF_A {
        match self.bits {
            false => BFOF_A::_0,
            true => BFOF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BFOF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BFOF_A::_1
    }
}
#[doc = "Field `BCDF` reader - Bus Conflict detection flag"]
pub type BCDF_R = crate::BitReader<BCDF_A>;
#[doc = "Bus Conflict detection flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BCDF_A {
    #[doc = "0: When no Bus Conflict is detected"]
    _0 = 0,
    #[doc = "1: When Bus Conflict is detected"]
    _1 = 1,
}
impl From<BCDF_A> for bool {
    #[inline(always)]
    fn from(variant: BCDF_A) -> Self {
        variant as u8 != 0
    }
}
impl BCDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCDF_A {
        match self.bits {
            false => BCDF_A::_0,
            true => BCDF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BCDF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BCDF_A::_1
    }
}
#[doc = "Field `BFDF` reader - Break Field detection flag"]
pub type BFDF_R = crate::BitReader<BFDF_A>;
#[doc = "Break Field detection flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFDF_A {
    #[doc = "0: When Break Field is not detected"]
    _0 = 0,
    #[doc = "1: When Break Field is detected"]
    _1 = 1,
}
impl From<BFDF_A> for bool {
    #[inline(always)]
    fn from(variant: BFDF_A) -> Self {
        variant as u8 != 0
    }
}
impl BFDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFDF_A {
        match self.bits {
            false => BFDF_A::_0,
            true => BFDF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BFDF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BFDF_A::_1
    }
}
#[doc = "Field `CF0MF` reader - Control Field 0 compare match flag"]
pub type CF0MF_R = crate::BitReader<CF0MF_A>;
#[doc = "Control Field 0 compare match flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CF0MF_A {
    #[doc = "0: When Control-Field-0 data and the compare data does not match"]
    _0 = 0,
    #[doc = "1: When Control-Field-0 data and the compare data match"]
    _1 = 1,
}
impl From<CF0MF_A> for bool {
    #[inline(always)]
    fn from(variant: CF0MF_A) -> Self {
        variant as u8 != 0
    }
}
impl CF0MF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CF0MF_A {
        match self.bits {
            false => CF0MF_A::_0,
            true => CF0MF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CF0MF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CF0MF_A::_1
    }
}
#[doc = "Field `CF1MF` reader - Control Field 1 compare match flag"]
pub type CF1MF_R = crate::BitReader<CF1MF_A>;
#[doc = "Control Field 1 compare match flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CF1MF_A {
    #[doc = "0: When Control-Field-1 data and the compare data does not match"]
    _0 = 0,
    #[doc = "1: When Control-Field-1 data and the compare data match"]
    _1 = 1,
}
impl From<CF1MF_A> for bool {
    #[inline(always)]
    fn from(variant: CF1MF_A) -> Self {
        variant as u8 != 0
    }
}
impl CF1MF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CF1MF_A {
        match self.bits {
            false => CF1MF_A::_0,
            true => CF1MF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CF1MF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CF1MF_A::_1
    }
}
#[doc = "Field `PIBDF` reader - Priority interrupt bit detection flag"]
pub type PIBDF_R = crate::BitReader<PIBDF_A>;
#[doc = "Priority interrupt bit detection flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIBDF_A {
    #[doc = "0: When Priority interrupt bit is not detected"]
    _0 = 0,
    #[doc = "1: When Priority interrupt bit is detected"]
    _1 = 1,
}
impl From<PIBDF_A> for bool {
    #[inline(always)]
    fn from(variant: PIBDF_A) -> Self {
        variant as u8 != 0
    }
}
impl PIBDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIBDF_A {
        match self.bits {
            false => PIBDF_A::_0,
            true => PIBDF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIBDF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIBDF_A::_1
    }
}
#[doc = "Field `COF` reader - Counter Overflow flag"]
pub type COF_R = crate::BitReader<COF_A>;
#[doc = "Counter Overflow flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COF_A {
    #[doc = "0: When the counter for Break Field detection does not overflows"]
    _0 = 0,
    #[doc = "1: When the counter for Break Field detection overflows"]
    _1 = 1,
}
impl From<COF_A> for bool {
    #[inline(always)]
    fn from(variant: COF_A) -> Self {
        variant as u8 != 0
    }
}
impl COF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COF_A {
        match self.bits {
            false => COF_A::_0,
            true => COF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == COF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == COF_A::_1
    }
}
#[doc = "Field `AEDF` reader - Active Edge detection flag"]
pub type AEDF_R = crate::BitReader<AEDF_A>;
#[doc = "Active Edge detection flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AEDF_A {
    #[doc = "0: When Active edge is not detected"]
    _0 = 0,
    #[doc = "1: When Active edge is detected"]
    _1 = 1,
}
impl From<AEDF_A> for bool {
    #[inline(always)]
    fn from(variant: AEDF_A) -> Self {
        variant as u8 != 0
    }
}
impl AEDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AEDF_A {
        match self.bits {
            false => AEDF_A::_0,
            true => AEDF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AEDF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AEDF_A::_1
    }
}
#[doc = "Field `CF0RD` reader - Control Field 0 received data"]
pub type CF0RD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CF1RD` reader - Control Field 1 received data"]
pub type CF1RD_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Start Frame Status flag"]
    #[inline(always)]
    pub fn sfsf(&self) -> SFSF_R {
        SFSF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RXDn input status flag"]
    #[inline(always)]
    pub fn rxdsf(&self) -> RXDSF_R {
        RXDSF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Break Field Output completion flag"]
    #[inline(always)]
    pub fn bfof(&self) -> BFOF_R {
        BFOF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Bus Conflict detection flag"]
    #[inline(always)]
    pub fn bcdf(&self) -> BCDF_R {
        BCDF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Break Field detection flag"]
    #[inline(always)]
    pub fn bfdf(&self) -> BFDF_R {
        BFDF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Control Field 0 compare match flag"]
    #[inline(always)]
    pub fn cf0mf(&self) -> CF0MF_R {
        CF0MF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Control Field 1 compare match flag"]
    #[inline(always)]
    pub fn cf1mf(&self) -> CF1MF_R {
        CF1MF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Priority interrupt bit detection flag"]
    #[inline(always)]
    pub fn pibdf(&self) -> PIBDF_R {
        PIBDF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Counter Overflow flag"]
    #[inline(always)]
    pub fn cof(&self) -> COF_R {
        COF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Active Edge detection flag"]
    #[inline(always)]
    pub fn aedf(&self) -> AEDF_R {
        AEDF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Control Field 0 received data"]
    #[inline(always)]
    pub fn cf0rd(&self) -> CF0RD_R {
        CF0RD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Control Field 1 received data"]
    #[inline(always)]
    pub fn cf1rd(&self) -> CF1RD_R {
        CF1RD_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Simple LIN Status Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xsr0](index.html) module"]
pub struct XSR0_SPEC;
impl crate::RegisterSpec for XSR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xsr0::R](R) reader structure"]
impl crate::Readable for XSR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets XSR0 to value 0"]
impl crate::Resettable for XSR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
