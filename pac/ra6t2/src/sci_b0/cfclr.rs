#[doc = "Register `CFCLR` writer"]
pub struct W(crate::W<CFCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFCLR_SPEC>;
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
impl From<crate::W<CFCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERSC` writer - ERS clear bit"]
pub type ERSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFCLR_SPEC, bool, O>;
#[doc = "Field `DCMFC` writer - DCMF clear bit"]
pub type DCMFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFCLR_SPEC, bool, O>;
#[doc = "Field `DPERC` writer - DPER clear bit"]
pub type DPERC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFCLR_SPEC, bool, O>;
#[doc = "Field `DFERC` writer - DFER clear bit"]
pub type DFERC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFCLR_SPEC, bool, O>;
#[doc = "Field `ORERC` writer - ORER clear bit"]
pub type ORERC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFCLR_SPEC, bool, O>;
#[doc = "Field `MFFC` writer - MFF clear bit"]
pub type MFFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFCLR_SPEC, bool, O>;
#[doc = "Field `PERC` writer - PER clear bit"]
pub type PERC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFCLR_SPEC, bool, O>;
#[doc = "Field `FERC` writer - FER clear bit"]
pub type FERC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFCLR_SPEC, bool, O>;
#[doc = "Field `TDREC` writer - TDRE clear bit"]
pub type TDREC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFCLR_SPEC, bool, O>;
#[doc = "Field `RDRFC` writer - RDRF clear bit"]
pub type RDRFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFCLR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 4 - ERS clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn ersc(&mut self) -> ERSC_W<4> {
        ERSC_W::new(self)
    }
    #[doc = "Bit 16 - DCMF clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn dcmfc(&mut self) -> DCMFC_W<16> {
        DCMFC_W::new(self)
    }
    #[doc = "Bit 17 - DPER clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn dperc(&mut self) -> DPERC_W<17> {
        DPERC_W::new(self)
    }
    #[doc = "Bit 18 - DFER clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn dferc(&mut self) -> DFERC_W<18> {
        DFERC_W::new(self)
    }
    #[doc = "Bit 24 - ORER clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn orerc(&mut self) -> ORERC_W<24> {
        ORERC_W::new(self)
    }
    #[doc = "Bit 26 - MFF clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn mffc(&mut self) -> MFFC_W<26> {
        MFFC_W::new(self)
    }
    #[doc = "Bit 27 - PER clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn perc(&mut self) -> PERC_W<27> {
        PERC_W::new(self)
    }
    #[doc = "Bit 28 - FER clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn ferc(&mut self) -> FERC_W<28> {
        FERC_W::new(self)
    }
    #[doc = "Bit 29 - TDRE clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn tdrec(&mut self) -> TDREC_W<29> {
        TDREC_W::new(self)
    }
    #[doc = "Bit 31 - RDRF clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn rdrfc(&mut self) -> RDRFC_W<31> {
        RDRFC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Common Flag Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfclr](index.html) module"]
pub struct CFCLR_SPEC;
impl crate::RegisterSpec for CFCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cfclr::W](W) writer structure"]
impl crate::Writable for CFCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFCLR to value 0"]
impl crate::Resettable for CFCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
