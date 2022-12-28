#[doc = "Register `CFDRFISTS` reader"]
pub struct R(crate::R<CFDRFISTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDRFISTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDRFISTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDRFISTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RFXIF` reader - RX FIFO\\[x\\]
Interrupt Flag Status"]
pub type RFXIF_R = crate::FieldReader<u8, RFXIF_A>;
#[doc = "RX FIFO\\[x\\]
Interrupt Flag Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RFXIF_A {
    #[doc = "0: Corresponding RX FIFO Interrupt flag not set"]
    _0 = 0,
    #[doc = "1: Corresponding RX FIFO Interrupt flag set"]
    _1 = 1,
}
impl From<RFXIF_A> for u8 {
    #[inline(always)]
    fn from(variant: RFXIF_A) -> Self {
        variant as _
    }
}
impl RFXIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RFXIF_A> {
        match self.bits {
            0 => Some(RFXIF_A::_0),
            1 => Some(RFXIF_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFXIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFXIF_A::_1
    }
}
impl R {
    #[doc = "Bits 0:1 - RX FIFO\\[x\\]
Interrupt Flag Status"]
    #[inline(always)]
    pub fn rfxif(&self) -> RFXIF_R {
        RFXIF_R::new((self.bits & 3) as u8)
    }
}
#[doc = "RX FIFO Interrupt Flag Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdrfists](index.html) module"]
pub struct CFDRFISTS_SPEC;
impl crate::RegisterSpec for CFDRFISTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdrfists::R](R) reader structure"]
impl crate::Readable for CFDRFISTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CFDRFISTS to value 0"]
impl crate::Resettable for CFDRFISTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
