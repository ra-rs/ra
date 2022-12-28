#[doc = "Register `BCKCR` reader"]
pub struct R(crate::R<BCKCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCKCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCKCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCKCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BCKCR` writer"]
pub struct W(crate::W<BCKCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCKCR_SPEC>;
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
impl From<crate::W<BCKCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCKCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BCLKDIV` reader - BCLK Pin Output Select"]
pub type BCLKDIV_R = crate::BitReader<BCLKDIV_A>;
#[doc = "BCLK Pin Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BCLKDIV_A {
    #[doc = "0: BCLK"]
    _0 = 0,
    #[doc = "1: BCLK/2"]
    _1 = 1,
}
impl From<BCLKDIV_A> for bool {
    #[inline(always)]
    fn from(variant: BCLKDIV_A) -> Self {
        variant as u8 != 0
    }
}
impl BCLKDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCLKDIV_A {
        match self.bits {
            false => BCLKDIV_A::_0,
            true => BCLKDIV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BCLKDIV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BCLKDIV_A::_1
    }
}
#[doc = "Field `BCLKDIV` writer - BCLK Pin Output Select"]
pub type BCLKDIV_W<'a, const O: u8> = crate::BitWriter<'a, u8, BCKCR_SPEC, BCLKDIV_A, O>;
impl<'a, const O: u8> BCLKDIV_W<'a, O> {
    #[doc = "BCLK"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BCLKDIV_A::_0)
    }
    #[doc = "BCLK/2"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BCLKDIV_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - BCLK Pin Output Select"]
    #[inline(always)]
    pub fn bclkdiv(&self) -> BCLKDIV_R {
        BCLKDIV_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BCLK Pin Output Select"]
    #[inline(always)]
    #[must_use]
    pub fn bclkdiv(&mut self) -> BCLKDIV_W<0> {
        BCLKDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Bus Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bckcr](index.html) module"]
pub struct BCKCR_SPEC;
impl crate::RegisterSpec for BCKCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [bckcr::R](R) reader structure"]
impl crate::Readable for BCKCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bckcr::W](W) writer structure"]
impl crate::Writable for BCKCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BCKCR to value 0"]
impl crate::Resettable for BCKCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
