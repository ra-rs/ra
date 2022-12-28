#[doc = "Register `SISR` reader"]
pub struct R(crate::R<SISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IICACKR` reader - ACK Reception Data Flag"]
pub type IICACKR_R = crate::BitReader<IICACKR_A>;
#[doc = "ACK Reception Data Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IICACKR_A {
    #[doc = "0: ACK received"]
    _0 = 0,
    #[doc = "1: NACK received"]
    _1 = 1,
}
impl From<IICACKR_A> for bool {
    #[inline(always)]
    fn from(variant: IICACKR_A) -> Self {
        variant as u8 != 0
    }
}
impl IICACKR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IICACKR_A {
        match self.bits {
            false => IICACKR_A::_0,
            true => IICACKR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IICACKR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IICACKR_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - ACK Reception Data Flag"]
    #[inline(always)]
    pub fn iicackr(&self) -> IICACKR_R {
        IICACKR_R::new((self.bits & 1) != 0)
    }
}
#[doc = "I2C Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sisr](index.html) module"]
pub struct SISR_SPEC;
impl crate::RegisterSpec for SISR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sisr::R](R) reader structure"]
impl crate::Readable for SISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SISR to value 0"]
impl crate::Resettable for SISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
