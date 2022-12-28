#[doc = "Register `GTADTBRA` reader"]
pub struct R(crate::R<GTADTBRA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTADTBRA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTADTBRA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTADTBRA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTADTBRA` writer"]
pub struct W(crate::W<GTADTBRA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTADTBRA_SPEC>;
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
impl From<crate::W<GTADTBRA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTADTBRA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GTADTBRA` reader - A/D Converter Start Request Timing Buffer Register A"]
pub type GTADTBRA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `GTADTBRA` writer - A/D Converter Start Request Timing Buffer Register A"]
pub type GTADTBRA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTADTBRA_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - A/D Converter Start Request Timing Buffer Register A"]
    #[inline(always)]
    pub fn gtadtbra(&self) -> GTADTBRA_R {
        GTADTBRA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - A/D Converter Start Request Timing Buffer Register A"]
    #[inline(always)]
    #[must_use]
    pub fn gtadtbra(&mut self) -> GTADTBRA_W<0> {
        GTADTBRA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Converter Start Request Timing Buffer Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtadtbra](index.html) module"]
pub struct GTADTBRA_SPEC;
impl crate::RegisterSpec for GTADTBRA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtadtbra::R](R) reader structure"]
impl crate::Readable for GTADTBRA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtadtbra::W](W) writer structure"]
impl crate::Writable for GTADTBRA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTADTBRA to value 0xffff_ffff"]
impl crate::Resettable for GTADTBRA_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
