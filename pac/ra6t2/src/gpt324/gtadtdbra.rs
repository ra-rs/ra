#[doc = "Register `GTADTDBRA` reader"]
pub struct R(crate::R<GTADTDBRA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTADTDBRA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTADTDBRA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTADTDBRA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTADTDBRA` writer"]
pub struct W(crate::W<GTADTDBRA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTADTDBRA_SPEC>;
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
impl From<crate::W<GTADTDBRA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTADTDBRA_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Conversion Start Request Timing Double-Buffer Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtadtdbra](index.html) module"]
pub struct GTADTDBRA_SPEC;
impl crate::RegisterSpec for GTADTDBRA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtadtdbra::R](R) reader structure"]
impl crate::Readable for GTADTDBRA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtadtdbra::W](W) writer structure"]
impl crate::Writable for GTADTDBRA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTADTDBRA to value 0xffff_ffff"]
impl crate::Resettable for GTADTDBRA_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
