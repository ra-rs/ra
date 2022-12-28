#[doc = "Register `CRCDIR` reader"]
pub struct R(crate::R<CRCDIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRCDIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRCDIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRCDIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRCDIR` writer"]
pub struct W(crate::W<CRCDIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRCDIR_SPEC>;
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
impl From<crate::W<CRCDIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRCDIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRCDIR` reader - Calculation input Data (Case of CRC-32, CRC-32C )"]
pub type CRCDIR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CRCDIR` writer - Calculation input Data (Case of CRC-32, CRC-32C )"]
pub type CRCDIR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRCDIR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Calculation input Data (Case of CRC-32, CRC-32C )"]
    #[inline(always)]
    pub fn crcdir(&self) -> CRCDIR_R {
        CRCDIR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Calculation input Data (Case of CRC-32, CRC-32C )"]
    #[inline(always)]
    #[must_use]
    pub fn crcdir(&mut self) -> CRCDIR_W<0> {
        CRCDIR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC Data Input Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcdir](index.html) module"]
pub struct CRCDIR_SPEC;
impl crate::RegisterSpec for CRCDIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crcdir::R](R) reader structure"]
impl crate::Readable for CRCDIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crcdir::W](W) writer structure"]
impl crate::Writable for CRCDIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRCDIR to value 0"]
impl crate::Resettable for CRCDIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
