#[doc = "Register `JIFESA` reader"]
pub struct R(crate::R<JIFESA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JIFESA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JIFESA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JIFESA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JIFESA` writer"]
pub struct W(crate::W<JIFESA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JIFESA_SPEC>;
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
impl From<crate::W<JIFESA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JIFESA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ESA` reader - Input Image Data Source Address (in 8-byte units) The lower three bits should be set to 0."]
pub type ESA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ESA` writer - Input Image Data Source Address (in 8-byte units) The lower three bits should be set to 0."]
pub type ESA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JIFESA_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Input Image Data Source Address (in 8-byte units) The lower three bits should be set to 0."]
    #[inline(always)]
    pub fn esa(&self) -> ESA_R {
        ESA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Input Image Data Source Address (in 8-byte units) The lower three bits should be set to 0."]
    #[inline(always)]
    #[must_use]
    pub fn esa(&mut self) -> ESA_W<0> {
        ESA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG Interface Compression Source Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jifesa](index.html) module"]
pub struct JIFESA_SPEC;
impl crate::RegisterSpec for JIFESA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jifesa::R](R) reader structure"]
impl crate::Readable for JIFESA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jifesa::W](W) writer structure"]
impl crate::Writable for JIFESA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets JIFESA to value 0"]
impl crate::Resettable for JIFESA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
