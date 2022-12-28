#[doc = "Register `GTCCRE` reader"]
pub struct R(crate::R<GTCCRE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTCCRE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTCCRE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTCCRE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTCCRE` writer"]
pub struct W(crate::W<GTCCRE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTCCRE_SPEC>;
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
impl From<crate::W<GTCCRE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTCCRE_SPEC>) -> Self {
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
#[doc = "General PWM Timer Compare Capture Register E\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtccre](index.html) module"]
pub struct GTCCRE_SPEC;
impl crate::RegisterSpec for GTCCRE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtccre::R](R) reader structure"]
impl crate::Readable for GTCCRE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtccre::W](W) writer structure"]
impl crate::Writable for GTCCRE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTCCRE to value 0xffff"]
impl crate::Resettable for GTCCRE_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
