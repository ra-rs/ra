#[doc = "Register `MFCLR` writer"]
pub struct W(crate::W<MFCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MFCLR_SPEC>;
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
impl From<crate::W<MFCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MFCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PFERC` writer - PFER clear bit"]
pub type PFERC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MFCLR_SPEC, bool, O>;
#[doc = "Field `SYERC` writer - SYER clear bit"]
pub type SYERC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MFCLR_SPEC, bool, O>;
#[doc = "Field `SBERC` writer - SBER clear bit"]
pub type SBERC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MFCLR_SPEC, bool, O>;
#[doc = "Field `MERC` writer - MER clear bit"]
pub type MERC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MFCLR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - PFER clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn pferc(&mut self) -> PFERC_W<0> {
        PFERC_W::new(self)
    }
    #[doc = "Bit 1 - SYER clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn syerc(&mut self) -> SYERC_W<1> {
        SYERC_W::new(self)
    }
    #[doc = "Bit 2 - SBER clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn sberc(&mut self) -> SBERC_W<2> {
        SBERC_W::new(self)
    }
    #[doc = "Bit 4 - MER clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn merc(&mut self) -> MERC_W<4> {
        MERC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Manchester Flag Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mfclr](index.html) module"]
pub struct MFCLR_SPEC;
impl crate::RegisterSpec for MFCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [mfclr::W](W) writer structure"]
impl crate::Writable for MFCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MFCLR to value 0"]
impl crate::Resettable for MFCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
