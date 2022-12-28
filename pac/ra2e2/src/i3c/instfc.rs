#[doc = "Register `INSTFC` writer"]
pub struct W(crate::W<INSTFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INSTFC_SPEC>;
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
impl From<crate::W<INSTFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INSTFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Internal Error Force\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INEFC_AW {
    #[doc = "0: Not force a specific interrupt"]
    _0 = 0,
    #[doc = "1: Force a specific interrupt"]
    _1 = 1,
}
impl From<INEFC_AW> for bool {
    #[inline(always)]
    fn from(variant: INEFC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INEFC` writer - Internal Error Force"]
pub type INEFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, INSTFC_SPEC, INEFC_AW, O>;
impl<'a, const O: u8> INEFC_W<'a, O> {
    #[doc = "Not force a specific interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INEFC_AW::_0)
    }
    #[doc = "Force a specific interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INEFC_AW::_1)
    }
}
impl W {
    #[doc = "Bit 10 - Internal Error Force"]
    #[inline(always)]
    #[must_use]
    pub fn inefc(&mut self) -> INEFC_W<10> {
        INEFC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal Status Force Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [instfc](index.html) module"]
pub struct INSTFC_SPEC;
impl crate::RegisterSpec for INSTFC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [instfc::W](W) writer structure"]
impl crate::Writable for INSTFC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INSTFC to value 0"]
impl crate::Resettable for INSTFC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
