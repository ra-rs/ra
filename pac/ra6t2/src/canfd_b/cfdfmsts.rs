#[doc = "Register `CFDFMSTS` reader"]
pub struct R(crate::R<CFDFMSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDFMSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDFMSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDFMSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RFXMLT` reader - RX FIFO Message Lost Status"]
pub type RFXMLT_R = crate::FieldReader<u8, RFXMLT_A>;
#[doc = "RX FIFO Message Lost Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RFXMLT_A {
    #[doc = "0: Corresponding FIFO Message Lost flag not set"]
    _0 = 0,
    #[doc = "1: Corresponding FIFO Message Lost flag set"]
    _1 = 1,
}
impl From<RFXMLT_A> for u8 {
    #[inline(always)]
    fn from(variant: RFXMLT_A) -> Self {
        variant as _
    }
}
impl RFXMLT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RFXMLT_A> {
        match self.bits {
            0 => Some(RFXMLT_A::_0),
            1 => Some(RFXMLT_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFXMLT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFXMLT_A::_1
    }
}
#[doc = "Field `CFMLT` reader - Common FIFO Message Lost Status"]
pub type CFMLT_R = crate::BitReader<CFMLT_A>;
#[doc = "Common FIFO Message Lost Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFMLT_A {
    #[doc = "0: Corresponding FIFO Message Lost flag not set"]
    _0 = 0,
    #[doc = "1: Corresponding FIFO Message Lost flag set"]
    _1 = 1,
}
impl From<CFMLT_A> for bool {
    #[inline(always)]
    fn from(variant: CFMLT_A) -> Self {
        variant as u8 != 0
    }
}
impl CFMLT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFMLT_A {
        match self.bits {
            false => CFMLT_A::_0,
            true => CFMLT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFMLT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFMLT_A::_1
    }
}
impl R {
    #[doc = "Bits 0:1 - RX FIFO Message Lost Status"]
    #[inline(always)]
    pub fn rfxmlt(&self) -> RFXMLT_R {
        RFXMLT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - Common FIFO Message Lost Status"]
    #[inline(always)]
    pub fn cfmlt(&self) -> CFMLT_R {
        CFMLT_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "FIFO Message Lost Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdfmsts](index.html) module"]
pub struct CFDFMSTS_SPEC;
impl crate::RegisterSpec for CFDFMSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdfmsts::R](R) reader structure"]
impl crate::Readable for CFDFMSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CFDFMSTS to value 0"]
impl crate::Resettable for CFDFMSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
