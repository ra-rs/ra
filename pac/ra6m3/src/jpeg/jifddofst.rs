#[doc = "Register `JIFDDOFST` reader"]
pub struct R(crate::R<JIFDDOFST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JIFDDOFST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JIFDDOFST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JIFDDOFST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JIFDDOFST` writer"]
pub struct W(crate::W<JIFDDOFST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JIFDDOFST_SPEC>;
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
impl From<crate::W<JIFDDOFST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JIFDDOFST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DDMW` reader - Output Image Data Lines Offset (in 8-byte units) The lower three bits should be set to 0."]
pub type DDMW_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DDMW` writer - Output Image Data Lines Offset (in 8-byte units) The lower three bits should be set to 0."]
pub type DDMW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JIFDDOFST_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bits 0:14 - Output Image Data Lines Offset (in 8-byte units) The lower three bits should be set to 0."]
    #[inline(always)]
    pub fn ddmw(&self) -> DDMW_R {
        DDMW_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Output Image Data Lines Offset (in 8-byte units) The lower three bits should be set to 0."]
    #[inline(always)]
    #[must_use]
    pub fn ddmw(&mut self) -> DDMW_W<0> {
        DDMW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG Interface Decompression Line Offset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jifddofst](index.html) module"]
pub struct JIFDDOFST_SPEC;
impl crate::RegisterSpec for JIFDDOFST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jifddofst::R](R) reader structure"]
impl crate::Readable for JIFDDOFST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jifddofst::W](W) writer structure"]
impl crate::Writable for JIFDDOFST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets JIFDDOFST to value 0"]
impl crate::Resettable for JIFDDOFST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
