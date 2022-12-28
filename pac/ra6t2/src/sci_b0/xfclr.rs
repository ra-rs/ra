#[doc = "Register `XFCLR` writer"]
pub struct W(crate::W<XFCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XFCLR_SPEC>;
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
impl From<crate::W<XFCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XFCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BFOC` writer - BFOF clear bit"]
pub type BFOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, XFCLR_SPEC, bool, O>;
#[doc = "Field `BCDC` writer - BCDF clear bit"]
pub type BCDC_W<'a, const O: u8> = crate::BitWriter<'a, u32, XFCLR_SPEC, bool, O>;
#[doc = "Field `BFDC` writer - BFDF clear bit"]
pub type BFDC_W<'a, const O: u8> = crate::BitWriter<'a, u32, XFCLR_SPEC, bool, O>;
#[doc = "Field `CF0MC` writer - CF0MF clear bit"]
pub type CF0MC_W<'a, const O: u8> = crate::BitWriter<'a, u32, XFCLR_SPEC, bool, O>;
#[doc = "Field `CF1MC` writer - CF1MF clear bit"]
pub type CF1MC_W<'a, const O: u8> = crate::BitWriter<'a, u32, XFCLR_SPEC, bool, O>;
#[doc = "Field `PIBDC` writer - PIBDF clear bit"]
pub type PIBDC_W<'a, const O: u8> = crate::BitWriter<'a, u32, XFCLR_SPEC, bool, O>;
#[doc = "Field `COFC` writer - COFF clear bit"]
pub type COFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, XFCLR_SPEC, bool, O>;
#[doc = "Field `AEDC` writer - AEDF clear bit"]
pub type AEDC_W<'a, const O: u8> = crate::BitWriter<'a, u32, XFCLR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 8 - BFOF clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn bfoc(&mut self) -> BFOC_W<8> {
        BFOC_W::new(self)
    }
    #[doc = "Bit 9 - BCDF clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn bcdc(&mut self) -> BCDC_W<9> {
        BCDC_W::new(self)
    }
    #[doc = "Bit 10 - BFDF clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn bfdc(&mut self) -> BFDC_W<10> {
        BFDC_W::new(self)
    }
    #[doc = "Bit 11 - CF0MF clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cf0mc(&mut self) -> CF0MC_W<11> {
        CF0MC_W::new(self)
    }
    #[doc = "Bit 12 - CF1MF clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cf1mc(&mut self) -> CF1MC_W<12> {
        CF1MC_W::new(self)
    }
    #[doc = "Bit 13 - PIBDF clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn pibdc(&mut self) -> PIBDC_W<13> {
        PIBDC_W::new(self)
    }
    #[doc = "Bit 14 - COFF clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cofc(&mut self) -> COFC_W<14> {
        COFC_W::new(self)
    }
    #[doc = "Bit 15 - AEDF clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn aedc(&mut self) -> AEDC_W<15> {
        AEDC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Simple LIN Flag Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xfclr](index.html) module"]
pub struct XFCLR_SPEC;
impl crate::RegisterSpec for XFCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [xfclr::W](W) writer structure"]
impl crate::Writable for XFCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XFCLR to value 0"]
impl crate::Resettable for XFCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
