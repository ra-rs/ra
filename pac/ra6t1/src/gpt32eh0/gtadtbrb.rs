#[doc = "Register `GTADTBRB` reader"]
pub struct R(crate::R<GTADTBRB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTADTBRB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTADTBRB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTADTBRB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTADTBRB` writer"]
pub struct W(crate::W<GTADTBRB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTADTBRB_SPEC>;
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
impl From<crate::W<GTADTBRB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTADTBRB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GTADTBRB` reader - A/D Converter Start Request Timing Buffer Register B"]
pub type GTADTBRB_R = crate::FieldReader<u32, u32>;
#[doc = "Field `GTADTBRB` writer - A/D Converter Start Request Timing Buffer Register B"]
pub type GTADTBRB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTADTBRB_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - A/D Converter Start Request Timing Buffer Register B"]
    #[inline(always)]
    pub fn gtadtbrb(&self) -> GTADTBRB_R {
        GTADTBRB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - A/D Converter Start Request Timing Buffer Register B"]
    #[inline(always)]
    #[must_use]
    pub fn gtadtbrb(&mut self) -> GTADTBRB_W<0> {
        GTADTBRB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Converter Start Request Timing Buffer Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtadtbrb](index.html) module"]
pub struct GTADTBRB_SPEC;
impl crate::RegisterSpec for GTADTBRB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtadtbrb::R](R) reader structure"]
impl crate::Readable for GTADTBRB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtadtbrb::W](W) writer structure"]
impl crate::Writable for GTADTBRB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTADTBRB to value 0xffff_ffff"]
impl crate::Resettable for GTADTBRB_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
