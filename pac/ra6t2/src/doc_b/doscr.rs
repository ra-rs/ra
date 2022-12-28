#[doc = "Register `DOSCR` writer"]
pub struct W(crate::W<DOSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOSCR_SPEC>;
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
impl From<crate::W<DOSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DOPCF Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOPCFCL_AW {
    #[doc = "0: Maintains the DOPCF flag state."]
    _0 = 0,
    #[doc = "1: Clears the DOPCF flag."]
    _1 = 1,
}
impl From<DOPCFCL_AW> for bool {
    #[inline(always)]
    fn from(variant: DOPCFCL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOPCFCL` writer - DOPCF Clear"]
pub type DOPCFCL_W<'a, const O: u8> = crate::BitWriter<'a, u8, DOSCR_SPEC, DOPCFCL_AW, O>;
impl<'a, const O: u8> DOPCFCL_W<'a, O> {
    #[doc = "Maintains the DOPCF flag state."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOPCFCL_AW::_0)
    }
    #[doc = "Clears the DOPCF flag."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOPCFCL_AW::_1)
    }
}
impl W {
    #[doc = "Bit 0 - DOPCF Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dopcfcl(&mut self) -> DOPCFCL_W<0> {
        DOPCFCL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DOC Flag Status Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doscr](index.html) module"]
pub struct DOSCR_SPEC;
impl crate::RegisterSpec for DOSCR_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [doscr::W](W) writer structure"]
impl crate::Writable for DOSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOSCR to value 0"]
impl crate::Resettable for DOSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
