#[doc = "Register `CFDTMPTR%s` reader"]
pub struct R(crate::R<CFDTMPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDTMPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDTMPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDTMPTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDTMPTR%s` writer"]
pub struct W(crate::W<CFDTMPTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDTMPTR_SPEC>;
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
impl From<crate::W<CFDTMPTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDTMPTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMDLC` reader - TX Message Buffer DLC Field"]
pub type TMDLC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TMDLC` writer - TX Message Buffer DLC Field"]
pub type TMDLC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDTMPTR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 28:31 - TX Message Buffer DLC Field"]
    #[inline(always)]
    pub fn tmdlc(&self) -> TMDLC_R {
        TMDLC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - TX Message Buffer DLC Field"]
    #[inline(always)]
    #[must_use]
    pub fn tmdlc(&mut self) -> TMDLC_W<28> {
        TMDLC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX Message Buffer Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdtmptr](index.html) module"]
pub struct CFDTMPTR_SPEC;
impl crate::RegisterSpec for CFDTMPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdtmptr::R](R) reader structure"]
impl crate::Readable for CFDTMPTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdtmptr::W](W) writer structure"]
impl crate::Writable for CFDTMPTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDTMPTR%s to value 0"]
impl crate::Resettable for CFDTMPTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
