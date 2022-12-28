#[doc = "Register `ADEXDR%s` reader"]
pub struct R(crate::R<ADEXDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADEXDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADEXDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADEXDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - A/D conversion data"]
pub type DATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DIAGSR` reader - Self-diagnosis Status"]
pub type DIAGSR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ERR` reader - A/D Conversion Error Status"]
pub type ERR_R = crate::BitReader<ERR_A>;
#[doc = "A/D Conversion Error Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR_A {
    #[doc = "0: No error (the A/D conversion data is valid)"]
    _0 = 0,
    #[doc = "1: Error is detected (the A/D conversion data is not guaranteed)"]
    _1 = 1,
}
impl From<ERR_A> for bool {
    #[inline(always)]
    fn from(variant: ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR_A {
        match self.bits {
            false => ERR_A::_0,
            true => ERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR_A::_1
    }
}
impl R {
    #[doc = "Bits 0:15 - A/D conversion data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:26 - Self-diagnosis Status"]
    #[inline(always)]
    pub fn diagsr(&self) -> DIAGSR_R {
        DIAGSR_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 31 - A/D Conversion Error Status"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "A/D Extended Analog Data Register %s\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adexdr](index.html) module"]
pub struct ADEXDR_SPEC;
impl crate::RegisterSpec for ADEXDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adexdr::R](R) reader structure"]
impl crate::Readable for ADEXDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADEXDR%s to value 0"]
impl crate::Resettable for ADEXDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
