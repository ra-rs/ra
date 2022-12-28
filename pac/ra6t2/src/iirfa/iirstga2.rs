#[doc = "Register `IIRSTG%sA2` reader"]
pub struct R(crate::R<IIRSTGA2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IIRSTGA2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IIRSTGA2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IIRSTGA2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IIRSTG%sA2` writer"]
pub struct W(crate::W<IIRSTGA2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IIRSTGA2_SPEC>;
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
impl From<crate::W<IIRSTGA2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IIRSTGA2_SPEC>) -> Self {
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
#[doc = "Stage %s Coefficient a2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iirstga2](index.html) module"]
pub struct IIRSTGA2_SPEC;
impl crate::RegisterSpec for IIRSTGA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iirstga2::R](R) reader structure"]
impl crate::Readable for IIRSTGA2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iirstga2::W](W) writer structure"]
impl crate::Writable for IIRSTGA2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IIRSTG%sA2 to value 0"]
impl crate::Resettable for IIRSTGA2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
