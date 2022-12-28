#[doc = "Register `ACTR` reader"]
pub struct R(crate::R<ACTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACTR` writer"]
pub struct W(crate::W<ACTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACTR_SPEC>;
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
impl From<crate::W<ACTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTP` reader - Automatic calibration cycle time setting"]
pub type CTP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CTP` writer - Automatic calibration cycle time setting"]
pub type CTP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACTR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Automatic calibration cycle time setting"]
    #[inline(always)]
    pub fn ctp(&self) -> CTP_R {
        CTP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Automatic calibration cycle time setting"]
    #[inline(always)]
    #[must_use]
    pub fn ctp(&mut self) -> CTP_W<0> {
        CTP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Auto-Calibration Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [actr](index.html) module"]
pub struct ACTR_SPEC;
impl crate::RegisterSpec for ACTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [actr::R](R) reader structure"]
impl crate::Readable for ACTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [actr::W](W) writer structure"]
impl crate::Writable for ACTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACTR to value 0x1000_0000"]
impl crate::Resettable for ACTR_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000_0000;
}
