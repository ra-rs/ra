#[doc = "Register `CFDGTSTCFG` reader"]
pub struct R(crate::R<CFDGTSTCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDGTSTCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDGTSTCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDGTSTCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDGTSTCFG` writer"]
pub struct W(crate::W<CFDGTSTCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDGTSTCFG_SPEC>;
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
impl From<crate::W<CFDGTSTCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDGTSTCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTMPS` reader - RAM Test Mode Page Select"]
pub type RTMPS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTMPS` writer - RAM Test Mode Page Select"]
pub type RTMPS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDGTSTCFG_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 16:19 - RAM Test Mode Page Select"]
    #[inline(always)]
    pub fn rtmps(&self) -> RTMPS_R {
        RTMPS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:19 - RAM Test Mode Page Select"]
    #[inline(always)]
    #[must_use]
    pub fn rtmps(&mut self) -> RTMPS_W<16> {
        RTMPS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Test Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdgtstcfg](index.html) module"]
pub struct CFDGTSTCFG_SPEC;
impl crate::RegisterSpec for CFDGTSTCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdgtstcfg::R](R) reader structure"]
impl crate::Readable for CFDGTSTCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdgtstcfg::W](W) writer structure"]
impl crate::Writable for CFDGTSTCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDGTSTCFG to value 0"]
impl crate::Resettable for CFDGTSTCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
