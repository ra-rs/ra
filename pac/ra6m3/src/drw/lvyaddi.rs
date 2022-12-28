#[doc = "Register `LVYADDI` writer"]
pub struct W(crate::W<LVYADDI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LVYADDI_SPEC>;
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
impl From<crate::W<LVYADDI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LVYADDI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LVYADDI` writer - V limiter y-axis increment integer part"]
pub type LVYADDI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LVYADDI_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - V limiter y-axis increment integer part"]
    #[inline(always)]
    #[must_use]
    pub fn lvyaddi(&mut self) -> LVYADDI_W<0> {
        LVYADDI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "V Limiter Y-Axis Increment Integer Part Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lvyaddi](index.html) module"]
pub struct LVYADDI_SPEC;
impl crate::RegisterSpec for LVYADDI_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [lvyaddi::W](W) writer structure"]
impl crate::Writable for LVYADDI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LVYADDI to value 0"]
impl crate::Resettable for LVYADDI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
