#[doc = "Register `FPR` reader"]
pub struct R(crate::R<FPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FPR` writer"]
pub struct W(crate::W<FPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPR_SPEC>;
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
impl From<crate::W<FPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FPR` reader - Protection Unlock"]
pub type FPR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FPR` writer - Protection Unlock"]
pub type FPR_W<'a, const O: u8> = crate::FieldWriter<'a, u8, FPR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Protection Unlock"]
    #[inline(always)]
    pub fn fpr(&self) -> FPR_R {
        FPR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Protection Unlock"]
    #[inline(always)]
    #[must_use]
    pub fn fpr(&mut self) -> FPR_W<0> {
        FPR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Protection Unlock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpr](index.html) module"]
pub struct FPR_SPEC;
impl crate::RegisterSpec for FPR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fpr::R](R) reader structure"]
impl crate::Readable for FPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fpr::W](W) writer structure"]
impl crate::Writable for FPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FPR to value 0"]
impl crate::Resettable for FPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
