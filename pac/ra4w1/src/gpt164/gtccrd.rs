#[doc = "Register `GTCCRD` reader"]
pub struct R(crate::R<GTCCRD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTCCRD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTCCRD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTCCRD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTCCRD` writer"]
pub struct W(crate::W<GTCCRD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTCCRD_SPEC>;
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
impl From<crate::W<GTCCRD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTCCRD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GTCCRD` reader - Compare Capture Register D"]
pub type GTCCRD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `GTCCRD` writer - Compare Capture Register D"]
pub type GTCCRD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTCCRD_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Compare Capture Register D"]
    #[inline(always)]
    pub fn gtccrd(&self) -> GTCCRD_R {
        GTCCRD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Compare Capture Register D"]
    #[inline(always)]
    #[must_use]
    pub fn gtccrd(&mut self) -> GTCCRD_W<0> {
        GTCCRD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Compare Capture Register D\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtccrd](index.html) module"]
pub struct GTCCRD_SPEC;
impl crate::RegisterSpec for GTCCRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtccrd::R](R) reader structure"]
impl crate::Readable for GTCCRD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtccrd::W](W) writer structure"]
impl crate::Writable for GTCCRD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTCCRD to value 0xffff"]
impl crate::Resettable for GTCCRD_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
