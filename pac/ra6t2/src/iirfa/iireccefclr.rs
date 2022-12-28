#[doc = "Register `IIRECCEFCLR` writer"]
pub struct W(crate::W<IIRECCEFCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IIRECCEFCLR_SPEC>;
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
impl From<crate::W<IIRECCEFCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IIRECCEFCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ECC 1-bit error flag clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ESEFCLR_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clears the ESEF flag of the IIRECCEF register."]
    _1 = 1,
}
impl From<ESEFCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: ESEFCLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ESEFCLR` writer - ECC 1-bit error flag clear bit"]
pub type ESEFCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IIRECCEFCLR_SPEC, ESEFCLR_AW, O>;
impl<'a, const O: u8> ESEFCLR_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ESEFCLR_AW::_0)
    }
    #[doc = "Clears the ESEF flag of the IIRECCEF register."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ESEFCLR_AW::_1)
    }
}
#[doc = "ECC 2-bit error status flag clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDEFCLR_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clears the EDEF flag of the IIRECCEF register."]
    _1 = 1,
}
impl From<EDEFCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: EDEFCLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDEFCLR` writer - ECC 2-bit error status flag clear bit"]
pub type EDEFCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IIRECCEFCLR_SPEC, EDEFCLR_AW, O>;
impl<'a, const O: u8> EDEFCLR_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDEFCLR_AW::_0)
    }
    #[doc = "Clears the EDEF flag of the IIRECCEF register."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDEFCLR_AW::_1)
    }
}
impl W {
    #[doc = "Bit 0 - ECC 1-bit error flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn esefclr(&mut self) -> ESEFCLR_W<0> {
        ESEFCLR_W::new(self)
    }
    #[doc = "Bit 1 - ECC 2-bit error status flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn edefclr(&mut self) -> EDEFCLR_W<1> {
        EDEFCLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ECC Error Flag Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iireccefclr](index.html) module"]
pub struct IIRECCEFCLR_SPEC;
impl crate::RegisterSpec for IIRECCEFCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [iireccefclr::W](W) writer structure"]
impl crate::Writable for IIRECCEFCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IIRECCEFCLR to value 0"]
impl crate::Resettable for IIRECCEFCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
