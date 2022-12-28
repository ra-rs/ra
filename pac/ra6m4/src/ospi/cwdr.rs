#[doc = "Register `CWDR` writer"]
pub struct W(crate::W<CWDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CWDR_SPEC>;
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
impl From<crate::W<CWDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CWDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WD0` writer - Write data 0"]
pub type WD0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CWDR_SPEC, u8, u8, 8, O>;
#[doc = "Field `WD1` writer - Write data 1"]
pub type WD1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CWDR_SPEC, u8, u8, 8, O>;
#[doc = "Field `WD2` writer - Write data 2"]
pub type WD2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CWDR_SPEC, u8, u8, 8, O>;
#[doc = "Field `WD3` writer - Write data 3"]
pub type WD3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CWDR_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bits 0:7 - Write data 0"]
    #[inline(always)]
    #[must_use]
    pub fn wd0(&mut self) -> WD0_W<0> {
        WD0_W::new(self)
    }
    #[doc = "Bits 8:15 - Write data 1"]
    #[inline(always)]
    #[must_use]
    pub fn wd1(&mut self) -> WD1_W<8> {
        WD1_W::new(self)
    }
    #[doc = "Bits 16:23 - Write data 2"]
    #[inline(always)]
    #[must_use]
    pub fn wd2(&mut self) -> WD2_W<16> {
        WD2_W::new(self)
    }
    #[doc = "Bits 24:31 - Write data 3"]
    #[inline(always)]
    #[must_use]
    pub fn wd3(&mut self) -> WD3_W<24> {
        WD3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure Write Data Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cwdr](index.html) module"]
pub struct CWDR_SPEC;
impl crate::RegisterSpec for CWDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cwdr::W](W) writer structure"]
impl crate::Writable for CWDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CWDR to value 0"]
impl crate::Resettable for CWDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
