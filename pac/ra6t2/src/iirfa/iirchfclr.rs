#[doc = "Register `IIRCH%sFCLR` writer"]
pub struct W(crate::W<IIRCHFCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IIRCHFCLR_SPEC>;
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
impl From<crate::W<IIRCHFCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IIRCHFCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel processing completion flag clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPRCFFCLR_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clears the CPRCFF flag of the IIRCHnSTS register."]
    _1 = 1,
}
impl From<CPRCFFCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: CPRCFFCLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPRCFFCLR` writer - Channel processing completion flag clear bit"]
pub type CPRCFFCLR_W<'a, const O: u8> = crate::BitWriter<'a, u8, IIRCHFCLR_SPEC, CPRCFFCLR_AW, O>;
impl<'a, const O: u8> CPRCFFCLR_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPRCFFCLR_AW::_0)
    }
    #[doc = "Clears the CPRCFF flag of the IIRCHnSTS register."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPRCFFCLR_AW::_1)
    }
}
#[doc = "Operation error flag clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CERRFCLR_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clears the CERRF flag of the IIRCHnSTS register."]
    _1 = 1,
}
impl From<CERRFCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: CERRFCLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CERRFCLR` writer - Operation error flag clear bit"]
pub type CERRFCLR_W<'a, const O: u8> = crate::BitWriter<'a, u8, IIRCHFCLR_SPEC, CERRFCLR_AW, O>;
impl<'a, const O: u8> CERRFCLR_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CERRFCLR_AW::_0)
    }
    #[doc = "Clears the CERRF flag of the IIRCHnSTS register."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CERRFCLR_AW::_1)
    }
}
impl W {
    #[doc = "Bit 1 - Channel processing completion flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cprcffclr(&mut self) -> CPRCFFCLR_W<1> {
        CPRCFFCLR_W::new(self)
    }
    #[doc = "Bit 3 - Operation error flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cerrfclr(&mut self) -> CERRFCLR_W<3> {
        CERRFCLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel %s Flag Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iirchfclr](index.html) module"]
pub struct IIRCHFCLR_SPEC;
impl crate::RegisterSpec for IIRCHFCLR_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [iirchfclr::W](W) writer structure"]
impl crate::Writable for IIRCHFCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IIRCH%sFCLR to value 0"]
impl crate::Resettable for IIRCHFCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
