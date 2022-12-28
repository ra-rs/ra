#[doc = "Register `ADCLKENR` reader"]
pub struct R(crate::R<ADCLKENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCLKENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCLKENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCLKENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCLKENR` writer"]
pub struct W(crate::W<ADCLKENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCLKENR_SPEC>;
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
impl From<crate::W<ADCLKENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCLKENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKEN` reader - ADCLK Operating Enable"]
pub type CLKEN_R = crate::BitReader<CLKEN_A>;
#[doc = "ADCLK Operating Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKEN_A {
    #[doc = "0: Stop ADCLK"]
    _0 = 0,
    #[doc = "1: Supply ADCLK"]
    _1 = 1,
}
impl From<CLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: CLKEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKEN_A {
        match self.bits {
            false => CLKEN_A::_0,
            true => CLKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CLKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CLKEN_A::_1
    }
}
#[doc = "Field `CLKEN` writer - ADCLK Operating Enable"]
pub type CLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCLKENR_SPEC, CLKEN_A, O>;
impl<'a, const O: u8> CLKEN_W<'a, O> {
    #[doc = "Stop ADCLK"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKEN_A::_0)
    }
    #[doc = "Supply ADCLK"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - ADCLK Operating Enable"]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADCLK Operating Enable"]
    #[inline(always)]
    #[must_use]
    pub fn clken(&mut self) -> CLKEN_W<0> {
        CLKEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Conversion Clock Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adclkenr](index.html) module"]
pub struct ADCLKENR_SPEC;
impl crate::RegisterSpec for ADCLKENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adclkenr::R](R) reader structure"]
impl crate::Readable for ADCLKENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adclkenr::W](W) writer structure"]
impl crate::Writable for ADCLKENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCLKENR to value 0"]
impl crate::Resettable for ADCLKENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
