#[doc = "Register `CTSUSST` reader"]
pub struct R(crate::R<CTSUSST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSUSST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSUSST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSUSST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTSUSST` writer"]
pub struct W(crate::W<CTSUSST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTSUSST_SPEC>;
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
impl From<crate::W<CTSUSST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTSUSST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTSUSST` reader - CTSU Sensor Stabilization Wait ControlNOTE: The value of these bits should be fixed to 00010000b."]
pub type CTSUSST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTSUSST` writer - CTSU Sensor Stabilization Wait ControlNOTE: The value of these bits should be fixed to 00010000b."]
pub type CTSUSST_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CTSUSST_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - CTSU Sensor Stabilization Wait ControlNOTE: The value of these bits should be fixed to 00010000b."]
    #[inline(always)]
    pub fn ctsusst(&self) -> CTSUSST_R {
        CTSUSST_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - CTSU Sensor Stabilization Wait ControlNOTE: The value of these bits should be fixed to 00010000b."]
    #[inline(always)]
    #[must_use]
    pub fn ctsusst(&mut self) -> CTSUSST_W<0> {
        CTSUSST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTSU Sensor Stabilization Wait Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsusst](index.html) module"]
pub struct CTSUSST_SPEC;
impl crate::RegisterSpec for CTSUSST_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctsusst::R](R) reader structure"]
impl crate::Readable for CTSUSST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctsusst::W](W) writer structure"]
impl crate::Writable for CTSUSST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTSUSST to value 0"]
impl crate::Resettable for CTSUSST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
