#[doc = "Register `RFLR` reader"]
pub struct R(crate::R<RFLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RFLR` writer"]
pub struct W(crate::W<RFLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFLR_SPEC>;
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
impl From<crate::W<RFLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFL` reader - Receive Frame Maximum Length"]
pub type RFL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RFL` writer - Receive Frame Maximum Length"]
pub type RFL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RFLR_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - Receive Frame Maximum Length"]
    #[inline(always)]
    pub fn rfl(&self) -> RFL_R {
        RFL_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Receive Frame Maximum Length"]
    #[inline(always)]
    #[must_use]
    pub fn rfl(&mut self) -> RFL_W<0> {
        RFL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Frame Maximum Length Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rflr](index.html) module"]
pub struct RFLR_SPEC;
impl crate::RegisterSpec for RFLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rflr::R](R) reader structure"]
impl crate::Readable for RFLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rflr::W](W) writer structure"]
impl crate::Writable for RFLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RFLR to value 0"]
impl crate::Resettable for RFLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
