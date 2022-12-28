#[doc = "Register `JIFDDA` reader"]
pub struct R(crate::R<JIFDDA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JIFDDA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JIFDDA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JIFDDA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JIFDDA` writer"]
pub struct W(crate::W<JIFDDA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JIFDDA_SPEC>;
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
impl From<crate::W<JIFDDA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JIFDDA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DDA` reader - Output Image Data Destination Address (in 8-byte units) The lower three bits should be set to 0."]
pub type DDA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DDA` writer - Output Image Data Destination Address (in 8-byte units) The lower three bits should be set to 0."]
pub type DDA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JIFDDA_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Output Image Data Destination Address (in 8-byte units) The lower three bits should be set to 0."]
    #[inline(always)]
    pub fn dda(&self) -> DDA_R {
        DDA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Output Image Data Destination Address (in 8-byte units) The lower three bits should be set to 0."]
    #[inline(always)]
    #[must_use]
    pub fn dda(&mut self) -> DDA_W<0> {
        DDA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG Interface Decompression Destination Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jifdda](index.html) module"]
pub struct JIFDDA_SPEC;
impl crate::RegisterSpec for JIFDDA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jifdda::R](R) reader structure"]
impl crate::Readable for JIFDDA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jifdda::W](W) writer structure"]
impl crate::Writable for JIFDDA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets JIFDDA to value 0"]
impl crate::Resettable for JIFDDA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
