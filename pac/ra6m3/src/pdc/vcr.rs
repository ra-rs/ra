#[doc = "Register `VCR` reader"]
pub struct R(crate::R<VCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VCR` writer"]
pub struct W(crate::W<VCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VCR_SPEC>;
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
impl From<crate::W<VCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VST` reader - Vertical Capture Start Line PositionNumber of the line where capture is to start."]
pub type VST_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VST` writer - Vertical Capture Start Line PositionNumber of the line where capture is to start."]
pub type VST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VCR_SPEC, u16, u16, 12, O>;
#[doc = "Field `VSZ` reader - Vertical Capture Size Number of lines to be captured."]
pub type VSZ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VSZ` writer - Vertical Capture Size Number of lines to be captured."]
pub type VSZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VCR_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - Vertical Capture Start Line PositionNumber of the line where capture is to start."]
    #[inline(always)]
    pub fn vst(&self) -> VST_R {
        VST_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Vertical Capture Size Number of lines to be captured."]
    #[inline(always)]
    pub fn vsz(&self) -> VSZ_R {
        VSZ_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Vertical Capture Start Line PositionNumber of the line where capture is to start."]
    #[inline(always)]
    #[must_use]
    pub fn vst(&mut self) -> VST_W<0> {
        VST_W::new(self)
    }
    #[doc = "Bits 16:27 - Vertical Capture Size Number of lines to be captured."]
    #[inline(always)]
    #[must_use]
    pub fn vsz(&mut self) -> VSZ_W<16> {
        VSZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Vertical Capture Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vcr](index.html) module"]
pub struct VCR_SPEC;
impl crate::RegisterSpec for VCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vcr::R](R) reader structure"]
impl crate::Readable for VCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vcr::W](W) writer structure"]
impl crate::Writable for VCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VCR to value 0"]
impl crate::Resettable for VCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
