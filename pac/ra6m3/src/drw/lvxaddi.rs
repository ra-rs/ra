#[doc = "Register `LVXADDI` writer"]
pub struct W(crate::W<LVXADDI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LVXADDI_SPEC>;
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
impl From<crate::W<LVXADDI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LVXADDI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LVXADDI` writer - V limiter x-axis increment integer part"]
pub type LVXADDI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LVXADDI_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - V limiter x-axis increment integer part"]
    #[inline(always)]
    #[must_use]
    pub fn lvxaddi(&mut self) -> LVXADDI_W<0> {
        LVXADDI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "V Limiter X-Axis Increment Integer Part Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lvxaddi](index.html) module"]
pub struct LVXADDI_SPEC;
impl crate::RegisterSpec for LVXADDI_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [lvxaddi::W](W) writer structure"]
impl crate::Writable for LVXADDI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LVXADDI to value 0"]
impl crate::Resettable for LVXADDI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
