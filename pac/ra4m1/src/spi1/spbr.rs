#[doc = "Register `SPBR` reader"]
pub struct R(crate::R<SPBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPBR` writer"]
pub struct W(crate::W<SPBR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPBR_SPEC>;
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
impl From<crate::W<SPBR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPBR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPR` reader - SPBR sets the bit rate in master mode."]
pub type SPR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPR` writer - SPBR sets the bit rate in master mode."]
pub type SPR_W<'a, const O: u8> = crate::FieldWriter<'a, u8, SPBR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - SPBR sets the bit rate in master mode."]
    #[inline(always)]
    pub fn spr(&self) -> SPR_R {
        SPR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - SPBR sets the bit rate in master mode."]
    #[inline(always)]
    #[must_use]
    pub fn spr(&mut self) -> SPR_W<0> {
        SPR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Bit Rate Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spbr](index.html) module"]
pub struct SPBR_SPEC;
impl crate::RegisterSpec for SPBR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [spbr::R](R) reader structure"]
impl crate::Readable for SPBR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spbr::W](W) writer structure"]
impl crate::Writable for SPBR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPBR to value 0xff"]
impl crate::Resettable for SPBR_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
