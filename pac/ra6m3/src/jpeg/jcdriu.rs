#[doc = "Register `JCDRIU` reader"]
pub struct R(crate::R<JCDRIU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JCDRIU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JCDRIU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JCDRIU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JCDRIU` writer"]
pub struct W(crate::W<JCDRIU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JCDRIU_SPEC>;
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
impl From<crate::W<JCDRIU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JCDRIU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DRIU` reader - Upper Bytes of MCUs Preceding RST MarkerWhen both upper and lower bytes are set to 00h, neither a DRI nor an RST marker is placed.NOTE: Read-only in Decompression."]
pub type DRIU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DRIU` writer - Upper Bytes of MCUs Preceding RST MarkerWhen both upper and lower bytes are set to 00h, neither a DRI nor an RST marker is placed.NOTE: Read-only in Decompression."]
pub type DRIU_W<'a, const O: u8> = crate::FieldWriter<'a, u8, JCDRIU_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Upper Bytes of MCUs Preceding RST MarkerWhen both upper and lower bytes are set to 00h, neither a DRI nor an RST marker is placed.NOTE: Read-only in Decompression."]
    #[inline(always)]
    pub fn driu(&self) -> DRIU_R {
        DRIU_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Upper Bytes of MCUs Preceding RST MarkerWhen both upper and lower bytes are set to 00h, neither a DRI nor an RST marker is placed.NOTE: Read-only in Decompression."]
    #[inline(always)]
    #[must_use]
    pub fn driu(&mut self) -> DRIU_W<0> {
        DRIU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG Code DRI Upper Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jcdriu](index.html) module"]
pub struct JCDRIU_SPEC;
impl crate::RegisterSpec for JCDRIU_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [jcdriu::R](R) reader structure"]
impl crate::Readable for JCDRIU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jcdriu::W](W) writer structure"]
impl crate::Writable for JCDRIU_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets JCDRIU to value 0"]
impl crate::Resettable for JCDRIU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
