#[doc = "Register `LVSTARTF` writer"]
pub struct W(crate::W<LVSTARTF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LVSTARTF_SPEC>;
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
impl From<crate::W<LVSTARTF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LVSTARTF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LVSTARTF` writer - V limiter start value fractional part"]
pub type LVSTARTF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LVSTARTF_SPEC, u16, u16, 16, O>;
impl W {
    #[doc = "Bits 0:15 - V limiter start value fractional part"]
    #[inline(always)]
    #[must_use]
    pub fn lvstartf(&mut self) -> LVSTARTF_W<0> {
        LVSTARTF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "V Limiter Start Value Fractional Part Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lvstartf](index.html) module"]
pub struct LVSTARTF_SPEC;
impl crate::RegisterSpec for LVSTARTF_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [lvstartf::W](W) writer structure"]
impl crate::Writable for LVSTARTF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LVSTARTF to value 0"]
impl crate::Resettable for LVSTARTF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
