#[doc = "Register `GTCNT` reader"]
pub struct R(crate::R<GTCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTCNT` writer"]
pub struct W(crate::W<GTCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTCNT_SPEC>;
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
impl From<crate::W<GTCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GTCNT` reader - Counter"]
pub type GTCNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `GTCNT` writer - Counter"]
pub type GTCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTCNT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Counter"]
    #[inline(always)]
    pub fn gtcnt(&self) -> GTCNT_R {
        GTCNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counter"]
    #[inline(always)]
    #[must_use]
    pub fn gtcnt(&mut self) -> GTCNT_W<0> {
        GTCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtcnt](index.html) module"]
pub struct GTCNT_SPEC;
impl crate::RegisterSpec for GTCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtcnt::R](R) reader structure"]
impl crate::Readable for GTCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtcnt::W](W) writer structure"]
impl crate::Writable for GTCNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTCNT to value 0"]
impl crate::Resettable for GTCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
