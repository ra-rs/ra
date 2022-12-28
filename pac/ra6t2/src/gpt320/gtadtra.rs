#[doc = "Register `GTADTRA` reader"]
pub struct R(crate::R<GTADTRA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTADTRA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTADTRA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTADTRA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTADTRA` writer"]
pub struct W(crate::W<GTADTRA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTADTRA_SPEC>;
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
impl From<crate::W<GTADTRA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTADTRA_SPEC>) -> Self {
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
#[doc = "A/D Conversion Start Request Timing Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtadtra](index.html) module"]
pub struct GTADTRA_SPEC;
impl crate::RegisterSpec for GTADTRA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtadtra::R](R) reader structure"]
impl crate::Readable for GTADTRA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtadtra::W](W) writer structure"]
impl crate::Writable for GTADTRA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTADTRA to value 0xffff_ffff"]
impl crate::Resettable for GTADTRA_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
