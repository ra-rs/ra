#[doc = "Register `CFDGLOCKK` writer"]
pub struct W(crate::W<CFDGLOCKK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDGLOCKK_SPEC>;
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
impl From<crate::W<CFDGLOCKK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDGLOCKK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCK` writer - Lock Key"]
pub type LOCK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDGLOCKK_SPEC, u16, u16, 16, O>;
impl W {
    #[doc = "Bits 0:15 - Lock Key"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<0> {
        LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Lock Key Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdglockk](index.html) module"]
pub struct CFDGLOCKK_SPEC;
impl crate::RegisterSpec for CFDGLOCKK_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cfdglockk::W](W) writer structure"]
impl crate::Writable for CFDGLOCKK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDGLOCKK to value 0"]
impl crate::Resettable for CFDGLOCKK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
