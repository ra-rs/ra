#[doc = "Register `JIFEDA` reader"]
pub struct R(crate::R<JIFEDA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JIFEDA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JIFEDA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JIFEDA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JIFEDA` writer"]
pub struct W(crate::W<JIFEDA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JIFEDA_SPEC>;
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
impl From<crate::W<JIFEDA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JIFEDA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EDA` reader - Input Image Data Lines Offset (in 8-byte units) The lower three bits should be set to 0."]
pub type EDA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EDA` writer - Input Image Data Lines Offset (in 8-byte units) The lower three bits should be set to 0."]
pub type EDA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JIFEDA_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Input Image Data Lines Offset (in 8-byte units) The lower three bits should be set to 0."]
    #[inline(always)]
    pub fn eda(&self) -> EDA_R {
        EDA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Input Image Data Lines Offset (in 8-byte units) The lower three bits should be set to 0."]
    #[inline(always)]
    #[must_use]
    pub fn eda(&mut self) -> EDA_W<0> {
        EDA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG Interface Compression Destination Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jifeda](index.html) module"]
pub struct JIFEDA_SPEC;
impl crate::RegisterSpec for JIFEDA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jifeda::R](R) reader structure"]
impl crate::Readable for JIFEDA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jifeda::W](W) writer structure"]
impl crate::Writable for JIFEDA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets JIFEDA to value 0"]
impl crate::Resettable for JIFEDA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
