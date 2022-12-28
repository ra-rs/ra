#[doc = "Register `CRCDIR_BY` reader"]
pub struct R(crate::R<CRCDIR_BY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRCDIR_BY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRCDIR_BY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRCDIR_BY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRCDIR_BY` writer"]
pub struct W(crate::W<CRCDIR_BY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRCDIR_BY_SPEC>;
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
impl From<crate::W<CRCDIR_BY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRCDIR_BY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRCDIR_BY` reader - Calculation input Data ( Case of CRC-8, CRC-16 or CRC-CCITT )"]
pub type CRCDIR_BY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CRCDIR_BY` writer - Calculation input Data ( Case of CRC-8, CRC-16 or CRC-CCITT )"]
pub type CRCDIR_BY_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CRCDIR_BY_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Calculation input Data ( Case of CRC-8, CRC-16 or CRC-CCITT )"]
    #[inline(always)]
    pub fn crcdir_by(&self) -> CRCDIR_BY_R {
        CRCDIR_BY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Calculation input Data ( Case of CRC-8, CRC-16 or CRC-CCITT )"]
    #[inline(always)]
    #[must_use]
    pub fn crcdir_by(&mut self) -> CRCDIR_BY_W<0> {
        CRCDIR_BY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC Data Input Register (byte access)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcdir_by](index.html) module"]
pub struct CRCDIR_BY_SPEC;
impl crate::RegisterSpec for CRCDIR_BY_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [crcdir_by::R](R) reader structure"]
impl crate::Readable for CRCDIR_BY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crcdir_by::W](W) writer structure"]
impl crate::Writable for CRCDIR_BY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRCDIR_BY to value 0"]
impl crate::Resettable for CRCDIR_BY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
