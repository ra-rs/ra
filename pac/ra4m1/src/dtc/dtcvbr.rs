#[doc = "Register `DTCVBR` reader"]
pub struct R(crate::R<DTCVBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTCVBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTCVBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTCVBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTCVBR` writer"]
pub struct W(crate::W<DTCVBR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTCVBR_SPEC>;
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
impl From<crate::W<DTCVBR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTCVBR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTCVBR` reader - DTC Vector Base Address. Note: A value cannot be set in the lower-order 10 bits. These bits are fixed to 0."]
pub type DTCVBR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DTCVBR` writer - DTC Vector Base Address. Note: A value cannot be set in the lower-order 10 bits. These bits are fixed to 0."]
pub type DTCVBR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DTCVBR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - DTC Vector Base Address. Note: A value cannot be set in the lower-order 10 bits. These bits are fixed to 0."]
    #[inline(always)]
    pub fn dtcvbr(&self) -> DTCVBR_R {
        DTCVBR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DTC Vector Base Address. Note: A value cannot be set in the lower-order 10 bits. These bits are fixed to 0."]
    #[inline(always)]
    #[must_use]
    pub fn dtcvbr(&mut self) -> DTCVBR_W<0> {
        DTCVBR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DTC Vector Base Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtcvbr](index.html) module"]
pub struct DTCVBR_SPEC;
impl crate::RegisterSpec for DTCVBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtcvbr::R](R) reader structure"]
impl crate::Readable for DTCVBR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dtcvbr::W](W) writer structure"]
impl crate::Writable for DTCVBR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DTCVBR to value 0"]
impl crate::Resettable for DTCVBR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
