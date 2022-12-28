#[doc = "Register `JIFESOFST` reader"]
pub struct R(crate::R<JIFESOFST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JIFESOFST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JIFESOFST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JIFESOFST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JIFESOFST` writer"]
pub struct W(crate::W<JIFESOFST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JIFESOFST_SPEC>;
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
impl From<crate::W<JIFESOFST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JIFESOFST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ESMW` reader - Input Image Data Lines Offset(in 8-byte units)The lower three bits should be set to 0."]
pub type ESMW_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ESMW` writer - Input Image Data Lines Offset(in 8-byte units)The lower three bits should be set to 0."]
pub type ESMW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JIFESOFST_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bits 0:14 - Input Image Data Lines Offset(in 8-byte units)The lower three bits should be set to 0."]
    #[inline(always)]
    pub fn esmw(&self) -> ESMW_R {
        ESMW_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Input Image Data Lines Offset(in 8-byte units)The lower three bits should be set to 0."]
    #[inline(always)]
    #[must_use]
    pub fn esmw(&mut self) -> ESMW_W<0> {
        ESMW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG Interface Compression Line Offset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jifesofst](index.html) module"]
pub struct JIFESOFST_SPEC;
impl crate::RegisterSpec for JIFESOFST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jifesofst::R](R) reader structure"]
impl crate::Readable for JIFESOFST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jifesofst::W](W) writer structure"]
impl crate::Writable for JIFESOFST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets JIFESOFST to value 0"]
impl crate::Resettable for JIFESOFST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
