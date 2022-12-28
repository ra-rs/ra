#[doc = "Register `GTADTDBRB` reader"]
pub struct R(crate::R<GTADTDBRB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTADTDBRB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTADTDBRB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTADTDBRB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTADTDBRB` writer"]
pub struct W(crate::W<GTADTDBRB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTADTDBRB_SPEC>;
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
impl From<crate::W<GTADTDBRB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTADTDBRB_SPEC>) -> Self {
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
#[doc = "A/D Conversion Start Request Timing Double-Buffer Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtadtdbrb](index.html) module"]
pub struct GTADTDBRB_SPEC;
impl crate::RegisterSpec for GTADTDBRB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtadtdbrb::R](R) reader structure"]
impl crate::Readable for GTADTDBRB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtadtdbrb::W](W) writer structure"]
impl crate::Writable for GTADTDBRB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTADTDBRB to value 0xffff_ffff"]
impl crate::Resettable for GTADTDBRB_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
