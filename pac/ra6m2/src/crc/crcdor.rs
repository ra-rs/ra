#[doc = "Register `CRCDOR` reader"]
pub struct R(crate::R<CRCDOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRCDOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRCDOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRCDOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRCDOR` writer"]
pub struct W(crate::W<CRCDOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRCDOR_SPEC>;
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
impl From<crate::W<CRCDOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRCDOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRCDOR` reader - Calculation output Data (Case of CRC-32, CRC-32C )"]
pub type CRCDOR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CRCDOR` writer - Calculation output Data (Case of CRC-32, CRC-32C )"]
pub type CRCDOR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRCDOR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Calculation output Data (Case of CRC-32, CRC-32C )"]
    #[inline(always)]
    pub fn crcdor(&self) -> CRCDOR_R {
        CRCDOR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Calculation output Data (Case of CRC-32, CRC-32C )"]
    #[inline(always)]
    #[must_use]
    pub fn crcdor(&mut self) -> CRCDOR_W<0> {
        CRCDOR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC Data Output Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcdor](index.html) module"]
pub struct CRCDOR_SPEC;
impl crate::RegisterSpec for CRCDOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crcdor::R](R) reader structure"]
impl crate::Readable for CRCDOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crcdor::W](W) writer structure"]
impl crate::Writable for CRCDOR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRCDOR to value 0"]
impl crate::Resettable for CRCDOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
