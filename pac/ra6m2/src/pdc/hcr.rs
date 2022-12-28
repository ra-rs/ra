#[doc = "Register `HCR` reader"]
pub struct R(crate::R<HCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCR` writer"]
pub struct W(crate::W<HCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCR_SPEC>;
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
impl From<crate::W<HCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HST` reader - Horizontal Capture Start Byte Position Horizontal position in bytes where capture is to start."]
pub type HST_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HST` writer - Horizontal Capture Start Byte Position Horizontal position in bytes where capture is to start."]
pub type HST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCR_SPEC, u16, u16, 12, O>;
#[doc = "Field `HSZ` reader - Horizontal Capture Size Number of bytes to capture horizontally."]
pub type HSZ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HSZ` writer - Horizontal Capture Size Number of bytes to capture horizontally."]
pub type HSZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCR_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - Horizontal Capture Start Byte Position Horizontal position in bytes where capture is to start."]
    #[inline(always)]
    pub fn hst(&self) -> HST_R {
        HST_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Horizontal Capture Size Number of bytes to capture horizontally."]
    #[inline(always)]
    pub fn hsz(&self) -> HSZ_R {
        HSZ_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Horizontal Capture Start Byte Position Horizontal position in bytes where capture is to start."]
    #[inline(always)]
    #[must_use]
    pub fn hst(&mut self) -> HST_W<0> {
        HST_W::new(self)
    }
    #[doc = "Bits 16:27 - Horizontal Capture Size Number of bytes to capture horizontally."]
    #[inline(always)]
    #[must_use]
    pub fn hsz(&mut self) -> HSZ_W<16> {
        HSZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Horizontal Capture Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcr](index.html) module"]
pub struct HCR_SPEC;
impl crate::RegisterSpec for HCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcr::R](R) reader structure"]
impl crate::Readable for HCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcr::W](W) writer structure"]
impl crate::Writable for HCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCR to value 0"]
impl crate::Resettable for HCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
