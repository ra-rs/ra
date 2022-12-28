#[doc = "Register `PORR` writer"]
pub struct W(crate::W<PORR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PORR_SPEC>;
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
impl From<crate::W<PORR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PORR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pmn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PORR_AW {
    #[doc = "0: No affect to output"]
    _0 = 0,
    #[doc = "1: Low output."]
    _1 = 1,
}
impl From<PORR_AW> for u16 {
    #[inline(always)]
    fn from(variant: PORR_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `PORR` writer - Pmn Output Reset"]
pub type PORR_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PORR_SPEC, u16, PORR_AW, 16, O>;
impl<'a, const O: u8> PORR_W<'a, O> {
    #[doc = "No affect to output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORR_AW::_0)
    }
    #[doc = "Low output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORR_AW::_1)
    }
}
impl W {
    #[doc = "Bits 0:15 - Pmn Output Reset"]
    #[inline(always)]
    #[must_use]
    pub fn porr(&mut self) -> PORR_W<0> {
        PORR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output reset register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [porr](index.html) module"]
pub struct PORR_SPEC;
impl crate::RegisterSpec for PORR_SPEC {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [porr::W](W) writer structure"]
impl crate::Writable for PORR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PORR to value 0"]
impl crate::Resettable for PORR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
