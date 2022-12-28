#[doc = "Register `SPFCR` reader"]
pub struct R(crate::R<SPFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPFCR` writer"]
pub struct W(crate::W<SPFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPFCR_SPEC>;
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
impl From<crate::W<SPFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPFRST` writer - SPI FIFO clear"]
pub type SPFRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPFCR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - SPI FIFO clear"]
    #[inline(always)]
    #[must_use]
    pub fn spfrst(&mut self) -> SPFRST_W<0> {
        SPFRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI FIFO Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spfcr](index.html) module"]
pub struct SPFCR_SPEC;
impl crate::RegisterSpec for SPFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spfcr::R](R) reader structure"]
impl crate::Readable for SPFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spfcr::W](W) writer structure"]
impl crate::Writable for SPFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPFCR to value 0"]
impl crate::Resettable for SPFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
