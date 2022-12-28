#[doc = "Register `CFDGAFLCFG` reader"]
pub struct R(crate::R<CFDGAFLCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDGAFLCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDGAFLCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDGAFLCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDGAFLCFG` writer"]
pub struct W(crate::W<CFDGAFLCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDGAFLCFG_SPEC>;
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
impl From<crate::W<CFDGAFLCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDGAFLCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RNC0` reader - Rule Number"]
pub type RNC0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RNC0` writer - Rule Number"]
pub type RNC0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDGAFLCFG_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 16:21 - Rule Number"]
    #[inline(always)]
    pub fn rnc0(&self) -> RNC0_R {
        RNC0_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:21 - Rule Number"]
    #[inline(always)]
    #[must_use]
    pub fn rnc0(&mut self) -> RNC0_W<16> {
        RNC0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Acceptance Filter List Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdgaflcfg](index.html) module"]
pub struct CFDGAFLCFG_SPEC;
impl crate::RegisterSpec for CFDGAFLCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdgaflcfg::R](R) reader structure"]
impl crate::Readable for CFDGAFLCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdgaflcfg::W](W) writer structure"]
impl crate::Writable for CFDGAFLCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDGAFLCFG to value 0"]
impl crate::Resettable for CFDGAFLCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
