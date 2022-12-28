#[doc = "Register `GTPDBR` reader"]
pub struct R(crate::R<GTPDBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTPDBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTPDBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTPDBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTPDBR` writer"]
pub struct W(crate::W<GTPDBR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTPDBR_SPEC>;
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
impl From<crate::W<GTPDBR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTPDBR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GTPDBR` reader - Cycle Setting Double-Buffer Register"]
pub type GTPDBR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `GTPDBR` writer - Cycle Setting Double-Buffer Register"]
pub type GTPDBR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTPDBR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Cycle Setting Double-Buffer Register"]
    #[inline(always)]
    pub fn gtpdbr(&self) -> GTPDBR_R {
        GTPDBR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Cycle Setting Double-Buffer Register"]
    #[inline(always)]
    #[must_use]
    pub fn gtpdbr(&mut self) -> GTPDBR_W<0> {
        GTPDBR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Cycle Setting Double-Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtpdbr](index.html) module"]
pub struct GTPDBR_SPEC;
impl crate::RegisterSpec for GTPDBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtpdbr::R](R) reader structure"]
impl crate::Readable for GTPDBR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtpdbr::W](W) writer structure"]
impl crate::Writable for GTPDBR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTPDBR to value 0xffff_ffff"]
impl crate::Resettable for GTPDBR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
