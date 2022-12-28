#[doc = "Register `LVYXADDF` writer"]
pub struct W(crate::W<LVYXADDF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LVYXADDF_SPEC>;
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
impl From<crate::W<LVYXADDF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LVYXADDF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LVXADDF` writer - V xlimiter increment fractional part"]
pub type LVXADDF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LVYXADDF_SPEC, u16, u16, 16, O>;
#[doc = "Field `LVYADDF` writer - V y limiter increment fractional part"]
pub type LVYADDF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LVYXADDF_SPEC, u16, u16, 16, O>;
impl W {
    #[doc = "Bits 0:15 - V xlimiter increment fractional part"]
    #[inline(always)]
    #[must_use]
    pub fn lvxaddf(&mut self) -> LVXADDF_W<0> {
        LVXADDF_W::new(self)
    }
    #[doc = "Bits 16:31 - V y limiter increment fractional part"]
    #[inline(always)]
    #[must_use]
    pub fn lvyaddf(&mut self) -> LVYADDF_W<16> {
        LVYADDF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "V Limiter Increment Fractional Parts Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lvyxaddf](index.html) module"]
pub struct LVYXADDF_SPEC;
impl crate::RegisterSpec for LVYXADDF_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [lvyxaddf::W](W) writer structure"]
impl crate::Writable for LVYXADDF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LVYXADDF to value 0"]
impl crate::Resettable for LVYXADDF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
