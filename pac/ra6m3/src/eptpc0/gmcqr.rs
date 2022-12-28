#[doc = "Register `GMCQR` reader"]
pub struct R(crate::R<GMCQR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMCQR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMCQR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMCQR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GMCQR` writer"]
pub struct W(crate::W<GMCQR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GMCQR_SPEC>;
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
impl From<crate::W<GMCQR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GMCQR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GMCQR` reader - These bits are used to set the value of the grandmasterClockQuality fields of Announce messages. The correspondence between bits and the grandmasterClockQuality fields is as listed below.b31 to b24: clockClassb23 to b16: clockAccuracyb15 to b0: offsetScaledLogVariance"]
pub type GMCQR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `GMCQR` writer - These bits are used to set the value of the grandmasterClockQuality fields of Announce messages. The correspondence between bits and the grandmasterClockQuality fields is as listed below.b31 to b24: clockClassb23 to b16: clockAccuracyb15 to b0: offsetScaledLogVariance"]
pub type GMCQR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GMCQR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - These bits are used to set the value of the grandmasterClockQuality fields of Announce messages. The correspondence between bits and the grandmasterClockQuality fields is as listed below.b31 to b24: clockClassb23 to b16: clockAccuracyb15 to b0: offsetScaledLogVariance"]
    #[inline(always)]
    pub fn gmcqr(&self) -> GMCQR_R {
        GMCQR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - These bits are used to set the value of the grandmasterClockQuality fields of Announce messages. The correspondence between bits and the grandmasterClockQuality fields is as listed below.b31 to b24: clockClassb23 to b16: clockAccuracyb15 to b0: offsetScaledLogVariance"]
    #[inline(always)]
    #[must_use]
    pub fn gmcqr(&mut self) -> GMCQR_W<0> {
        GMCQR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "grandmasterClockQuality Field Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmcqr](index.html) module"]
pub struct GMCQR_SPEC;
impl crate::RegisterSpec for GMCQR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmcqr::R](R) reader structure"]
impl crate::Readable for GMCQR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gmcqr::W](W) writer structure"]
impl crate::Writable for GMCQR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GMCQR to value 0"]
impl crate::Resettable for GMCQR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
