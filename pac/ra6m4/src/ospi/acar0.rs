#[doc = "Register `ACAR0` reader"]
pub struct R(crate::R<ACAR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACAR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACAR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACAR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACAR0` writer"]
pub struct W(crate::W<ACAR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACAR0_SPEC>;
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
impl From<crate::W<ACAR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACAR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAD0` reader - Automatic calibration address"]
pub type CAD0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CAD0` writer - Automatic calibration address"]
pub type CAD0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACAR0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Automatic calibration address"]
    #[inline(always)]
    pub fn cad0(&self) -> CAD0_R {
        CAD0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Automatic calibration address"]
    #[inline(always)]
    #[must_use]
    pub fn cad0(&mut self) -> CAD0_W<0> {
        CAD0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Auto-Calibration Address Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acar0](index.html) module"]
pub struct ACAR0_SPEC;
impl crate::RegisterSpec for ACAR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acar0::R](R) reader structure"]
impl crate::Readable for ACAR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acar0::W](W) writer structure"]
impl crate::Writable for ACAR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACAR0 to value 0"]
impl crate::Resettable for ACAR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
