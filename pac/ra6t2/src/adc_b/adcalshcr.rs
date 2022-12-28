#[doc = "Register `ADCALSHCR` reader"]
pub struct R(crate::R<ADCALSHCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCALSHCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCALSHCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCALSHCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCALSHCR` writer"]
pub struct W(crate::W<ADCALSHCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCALSHCR_SPEC>;
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
impl From<crate::W<ADCALSHCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCALSHCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALSHSST` reader - Channel-dedicated Sample-and-hold Circuit Self-calibration Sampling Time Configuration"]
pub type CALSHSST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALSHSST` writer - Channel-dedicated Sample-and-hold Circuit Self-calibration Sampling Time Configuration"]
pub type CALSHSST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCALSHCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `CALSHHST` reader - Channel-dedicated Sample-and-hold Circuit Self-calibration Hold Mode Switching Time Configuration"]
pub type CALSHHST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALSHHST` writer - Channel-dedicated Sample-and-hold Circuit Self-calibration Hold Mode Switching Time Configuration"]
pub type CALSHHST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCALSHCR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:7 - Channel-dedicated Sample-and-hold Circuit Self-calibration Sampling Time Configuration"]
    #[inline(always)]
    pub fn calshsst(&self) -> CALSHSST_R {
        CALSHSST_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - Channel-dedicated Sample-and-hold Circuit Self-calibration Hold Mode Switching Time Configuration"]
    #[inline(always)]
    pub fn calshhst(&self) -> CALSHHST_R {
        CALSHHST_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Channel-dedicated Sample-and-hold Circuit Self-calibration Sampling Time Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn calshsst(&mut self) -> CALSHSST_W<0> {
        CALSHSST_W::new(self)
    }
    #[doc = "Bits 16:18 - Channel-dedicated Sample-and-hold Circuit Self-calibration Hold Mode Switching Time Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn calshhst(&mut self) -> CALSHHST_W<16> {
        CALSHHST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel-dedicated Sample-and-hold Circuit Self-calibration State Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcalshcr](index.html) module"]
pub struct ADCALSHCR_SPEC;
impl crate::RegisterSpec for ADCALSHCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcalshcr::R](R) reader structure"]
impl crate::Readable for ADCALSHCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcalshcr::W](W) writer structure"]
impl crate::Writable for ADCALSHCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCALSHCR to value 0x0002_0004"]
impl crate::Resettable for ADCALSHCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_0004;
}
