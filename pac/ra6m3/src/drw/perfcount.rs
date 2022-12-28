#[doc = "Register `PERFCOUNT%s` reader"]
pub struct R(crate::R<PERFCOUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERFCOUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERFCOUNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERFCOUNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERFCOUNT%s` writer"]
pub struct W(crate::W<PERFCOUNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERFCOUNT_SPEC>;
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
impl From<crate::W<PERFCOUNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERFCOUNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERFCOUNT` reader - Counter value.The counter is reset by writing PERFCOUNT = 0000 0000H."]
pub type PERFCOUNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PERFCOUNT` writer - Counter value.The counter is reset by writing PERFCOUNT = 0000 0000H."]
pub type PERFCOUNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PERFCOUNT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Counter value.The counter is reset by writing PERFCOUNT = 0000 0000H."]
    #[inline(always)]
    pub fn perfcount(&self) -> PERFCOUNT_R {
        PERFCOUNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counter value.The counter is reset by writing PERFCOUNT = 0000 0000H."]
    #[inline(always)]
    #[must_use]
    pub fn perfcount(&mut self) -> PERFCOUNT_W<0> {
        PERFCOUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Performance Counter %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perfcount](index.html) module"]
pub struct PERFCOUNT_SPEC;
impl crate::RegisterSpec for PERFCOUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perfcount::R](R) reader structure"]
impl crate::Readable for PERFCOUNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perfcount::W](W) writer structure"]
impl crate::Writable for PERFCOUNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERFCOUNT%s to value 0"]
impl crate::Resettable for PERFCOUNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
