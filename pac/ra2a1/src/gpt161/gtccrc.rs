#[doc = "Register `GTCCRC` reader"]
pub struct R(crate::R<GTCCRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTCCRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTCCRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTCCRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTCCRC` writer"]
pub struct W(crate::W<GTCCRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTCCRC_SPEC>;
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
impl From<crate::W<GTCCRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTCCRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GTCCRC` reader - Compare Capture Register C"]
pub type GTCCRC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `GTCCRC` writer - Compare Capture Register C"]
pub type GTCCRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTCCRC_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Compare Capture Register C"]
    #[inline(always)]
    pub fn gtccrc(&self) -> GTCCRC_R {
        GTCCRC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Compare Capture Register C"]
    #[inline(always)]
    #[must_use]
    pub fn gtccrc(&mut self) -> GTCCRC_W<0> {
        GTCCRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Compare Capture Register C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtccrc](index.html) module"]
pub struct GTCCRC_SPEC;
impl crate::RegisterSpec for GTCCRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtccrc::R](R) reader structure"]
impl crate::Readable for GTCCRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtccrc::W](W) writer structure"]
impl crate::Writable for GTCCRC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTCCRC to value 0xffff"]
impl crate::Resettable for GTCCRC_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
