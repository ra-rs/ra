#[doc = "Register `SPDR` reader"]
pub struct R(crate::R<SPDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPDR` writer"]
pub struct W(crate::W<SPDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPDR_SPEC>;
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
impl From<crate::W<SPDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPDR` reader - SPDR is the interface with the buffers that hold data for transmission and reception by the RSPI. When accessing in word (SPDCR.SPLW=1), access SPDR."]
pub type SPDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SPDR` writer - SPDR is the interface with the buffers that hold data for transmission and reception by the RSPI. When accessing in word (SPDCR.SPLW=1), access SPDR."]
pub type SPDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - SPDR is the interface with the buffers that hold data for transmission and reception by the RSPI. When accessing in word (SPDCR.SPLW=1), access SPDR."]
    #[inline(always)]
    pub fn spdr(&self) -> SPDR_R {
        SPDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SPDR is the interface with the buffers that hold data for transmission and reception by the RSPI. When accessing in word (SPDCR.SPLW=1), access SPDR."]
    #[inline(always)]
    #[must_use]
    pub fn spdr(&mut self) -> SPDR_W<0> {
        SPDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spdr](index.html) module"]
pub struct SPDR_SPEC;
impl crate::RegisterSpec for SPDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spdr::R](R) reader structure"]
impl crate::Readable for SPDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spdr::W](W) writer structure"]
impl crate::Writable for SPDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPDR to value 0"]
impl crate::Resettable for SPDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
