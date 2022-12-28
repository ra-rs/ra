#[doc = "Register `JIFDSA` reader"]
pub struct R(crate::R<JIFDSA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JIFDSA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JIFDSA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JIFDSA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JIFDSA` writer"]
pub struct W(crate::W<JIFDSA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JIFDSA_SPEC>;
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
impl From<crate::W<JIFDSA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JIFDSA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSA` reader - Input Coded Data Source AddressInput Coded Data Source Address (in 8-byte units) The lower three bits should be set to 0."]
pub type DSA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DSA` writer - Input Coded Data Source AddressInput Coded Data Source Address (in 8-byte units) The lower three bits should be set to 0."]
pub type DSA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JIFDSA_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Input Coded Data Source AddressInput Coded Data Source Address (in 8-byte units) The lower three bits should be set to 0."]
    #[inline(always)]
    pub fn dsa(&self) -> DSA_R {
        DSA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Input Coded Data Source AddressInput Coded Data Source Address (in 8-byte units) The lower three bits should be set to 0."]
    #[inline(always)]
    #[must_use]
    pub fn dsa(&mut self) -> DSA_W<0> {
        DSA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG Interface Decompression Source Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jifdsa](index.html) module"]
pub struct JIFDSA_SPEC;
impl crate::RegisterSpec for JIFDSA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jifdsa::R](R) reader structure"]
impl crate::Readable for JIFDSA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jifdsa::W](W) writer structure"]
impl crate::Writable for JIFDSA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets JIFDSA to value 0"]
impl crate::Resettable for JIFDSA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
