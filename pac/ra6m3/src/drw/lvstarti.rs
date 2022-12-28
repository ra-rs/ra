#[doc = "Register `LVSTARTI` writer"]
pub struct W(crate::W<LVSTARTI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LVSTARTI_SPEC>;
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
impl From<crate::W<LVSTARTI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LVSTARTI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LVSTARTI` writer - V limiter start value integer part"]
pub type LVSTARTI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LVSTARTI_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - V limiter start value integer part"]
    #[inline(always)]
    #[must_use]
    pub fn lvstarti(&mut self) -> LVSTARTI_W<0> {
        LVSTARTI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "V Limiter Start Value Integer Part Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lvstarti](index.html) module"]
pub struct LVSTARTI_SPEC;
impl crate::RegisterSpec for LVSTARTI_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [lvstarti::W](W) writer structure"]
impl crate::Writable for LVSTARTI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LVSTARTI to value 0"]
impl crate::Resettable for LVSTARTI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
