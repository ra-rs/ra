#[doc = "Register `ISR` reader"]
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
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
#[doc = "Field `IICSTIF` reader - Issuing of Start, Restart, or Stop Condition Completed Flag"]
pub type IICSTIF_R = crate::BitReader<IICSTIF_A>;
#[doc = "Issuing of Start, Restart, or Stop Condition Completed Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IICSTIF_A {
    #[doc = "0: There are no requests for generating conditions or a condition is being generated."]
    _0 = 0,
    #[doc = "1: A start, restart, or stop condition is completely generated."]
    _1 = 1,
}
impl From<IICSTIF_A> for bool {
    #[inline(always)]
    fn from(variant: IICSTIF_A) -> Self {
        variant as u8 != 0
    }
}
impl IICSTIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IICSTIF_A {
        match self.bits {
            false => IICSTIF_A::_0,
            true => IICSTIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IICSTIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IICSTIF_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - ACK Reception Data Flag"]
    #[inline(always)]
    pub fn iicackr(&self) -> IICACKR_R {
        IICACKR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Issuing of Start, Restart, or Stop Condition Completed Flag"]
    #[inline(always)]
    pub fn iicstif(&self) -> IICSTIF_R {
        IICSTIF_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Simple IIC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
