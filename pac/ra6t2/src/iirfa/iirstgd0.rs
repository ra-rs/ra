#[doc = "Register `IIRSTG%sD0` reader"]
pub struct R(crate::R<IIRSTGD0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IIRSTGD0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IIRSTGD0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IIRSTGD0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IIRSTG%sD0` writer"]
pub struct W(crate::W<IIRSTGD0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IIRSTGD0_SPEC>;
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
impl From<crate::W<IIRSTGD0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IIRSTGD0_SPEC>) -> Self {
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
#[doc = "Stage %s Delay Data D0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iirstgd0](index.html) module"]
pub struct IIRSTGD0_SPEC;
impl crate::RegisterSpec for IIRSTGD0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iirstgd0::R](R) reader structure"]
impl crate::Readable for IIRSTGD0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iirstgd0::W](W) writer structure"]
impl crate::Writable for IIRSTGD0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IIRSTG%sD0 to value 0"]
impl crate::Resettable for IIRSTGD0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
