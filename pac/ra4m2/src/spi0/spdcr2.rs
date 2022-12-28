#[doc = "Register `SPDCR2` reader"]
pub struct R(crate::R<SPDCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPDCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPDCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPDCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPDCR2` writer"]
pub struct W(crate::W<SPDCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPDCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SPDCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPDCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BYSW` reader - Byte Swap Operating Mode Select"]
pub type BYSW_R = crate::BitReader<BYSW_A>;
#[doc = "Byte Swap Operating Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BYSW_A {
    #[doc = "0: Byte Swap OFF"]
    _0 = 0,
    #[doc = "1: Byte Swap ON"]
    _1 = 1,
}
impl From<BYSW_A> for bool {
    #[inline(always)]
    fn from(variant: BYSW_A) -> Self {
        variant as u8 != 0
    }
}
impl BYSW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYSW_A {
        match self.bits {
            false => BYSW_A::_0,
            true => BYSW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BYSW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BYSW_A::_1
    }
}
#[doc = "Field `BYSW` writer - Byte Swap Operating Mode Select"]
pub type BYSW_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPDCR2_SPEC, BYSW_A, O>;
impl<'a, const O: u8> BYSW_W<'a, O> {
    #[doc = "Byte Swap OFF"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BYSW_A::_0)
    }
    #[doc = "Byte Swap ON"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BYSW_A::_1)
    }
}
#[doc = "Field `SINV` reader - Serial Data Invert Bit"]
pub type SINV_R = crate::BitReader<SINV_A>;
#[doc = "Serial Data Invert Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SINV_A {
    #[doc = "0: Not invert serial data"]
    _0 = 0,
    #[doc = "1: Invert serial data"]
    _1 = 1,
}
impl From<SINV_A> for bool {
    #[inline(always)]
    fn from(variant: SINV_A) -> Self {
        variant as u8 != 0
    }
}
impl SINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SINV_A {
        match self.bits {
            false => SINV_A::_0,
            true => SINV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SINV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SINV_A::_1
    }
}
#[doc = "Field `SINV` writer - Serial Data Invert Bit"]
pub type SINV_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPDCR2_SPEC, SINV_A, O>;
impl<'a, const O: u8> SINV_W<'a, O> {
    #[doc = "Not invert serial data"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SINV_A::_0)
    }
    #[doc = "Invert serial data"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SINV_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Byte Swap Operating Mode Select"]
    #[inline(always)]
    pub fn bysw(&self) -> BYSW_R {
        BYSW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Serial Data Invert Bit"]
    #[inline(always)]
    pub fn sinv(&self) -> SINV_R {
        SINV_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Byte Swap Operating Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn bysw(&mut self) -> BYSW_W<0> {
        BYSW_W::new(self)
    }
    #[doc = "Bit 1 - Serial Data Invert Bit"]
    #[inline(always)]
    #[must_use]
    pub fn sinv(&mut self) -> SINV_W<1> {
        SINV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Data Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spdcr2](index.html) module"]
pub struct SPDCR2_SPEC;
impl crate::RegisterSpec for SPDCR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [spdcr2::R](R) reader structure"]
impl crate::Readable for SPDCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spdcr2::W](W) writer structure"]
impl crate::Writable for SPDCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPDCR2 to value 0"]
impl crate::Resettable for SPDCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
