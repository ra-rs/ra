#[doc = "Register `POSR` writer"]
pub struct W(crate::W<POSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POSR_SPEC>;
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
impl From<crate::W<POSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pmn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum POSR_AW {
    #[doc = "0: No affect to output"]
    _0 = 0,
    #[doc = "1: High output."]
    _1 = 1,
}
impl From<POSR_AW> for u16 {
    #[inline(always)]
    fn from(variant: POSR_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `POSR` writer - Pmn Output Set"]
pub type POSR_W<'a, const O: u8> = crate::FieldWriter<'a, u16, POSR_SPEC, u16, POSR_AW, 16, O>;
impl<'a, const O: u8> POSR_W<'a, O> {
    #[doc = "No affect to output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POSR_AW::_0)
    }
    #[doc = "High output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POSR_AW::_1)
    }
}
impl W {
    #[doc = "Bits 0:15 - Pmn Output Set"]
    #[inline(always)]
    #[must_use]
    pub fn posr(&mut self) -> POSR_W<0> {
        POSR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output reset register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [posr](index.html) module"]
pub struct POSR_SPEC;
impl crate::RegisterSpec for POSR_SPEC {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [posr::W](W) writer structure"]
impl crate::Writable for POSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets POSR to value 0"]
impl crate::Resettable for POSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
