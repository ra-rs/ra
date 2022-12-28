#[doc = "Register `ADFIFODR%s` reader"]
pub struct R(crate::R<ADFIFODR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADFIFODR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADFIFODR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADFIFODR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - A/D Conversion Data"]
pub type DATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CH` reader - A/D Conversion Channel Number"]
pub type CH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ERR` reader - A/D Conversion Data Error Status"]
pub type ERR_R = crate::BitReader<ERR_A>;
#[doc = "A/D Conversion Data Error Status\n\nValue on reset: 0"]
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
    #[doc = "Bits 0:15 - A/D Conversion Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:30 - A/D Conversion Channel Number"]
    #[inline(always)]
    pub fn ch(&self) -> CH_R {
        CH_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - A/D Conversion Data Error Status"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "FIFO Data Register %s\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adfifodr](index.html) module"]
pub struct ADFIFODR_SPEC;
impl crate::RegisterSpec for ADFIFODR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adfifodr::R](R) reader structure"]
impl crate::Readable for ADFIFODR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADFIFODR%s to value 0"]
impl crate::Resettable for ADFIFODR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
