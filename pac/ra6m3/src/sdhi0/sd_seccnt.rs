#[doc = "Register `SD_SECCNT` reader"]
pub struct R(crate::R<SD_SECCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_SECCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_SECCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_SECCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SD_SECCNT` writer"]
pub struct W(crate::W<SD_SECCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_SECCNT_SPEC>;
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
impl From<crate::W<SD_SECCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_SECCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SD_SECCNT` reader - Number of Transfer BlocksNOTE: Do not change the value of this bit when the CBSY bit in SD_INFO2 is set to 1."]
pub type SD_SECCNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SD_SECCNT` writer - Number of Transfer BlocksNOTE: Do not change the value of this bit when the CBSY bit in SD_INFO2 is set to 1."]
pub type SD_SECCNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SD_SECCNT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Number of Transfer BlocksNOTE: Do not change the value of this bit when the CBSY bit in SD_INFO2 is set to 1."]
    #[inline(always)]
    pub fn sd_seccnt(&self) -> SD_SECCNT_R {
        SD_SECCNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of Transfer BlocksNOTE: Do not change the value of this bit when the CBSY bit in SD_INFO2 is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn sd_seccnt(&mut self) -> SD_SECCNT_W<0> {
        SD_SECCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Block Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_seccnt](index.html) module"]
pub struct SD_SECCNT_SPEC;
impl crate::RegisterSpec for SD_SECCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sd_seccnt::R](R) reader structure"]
impl crate::Readable for SD_SECCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_seccnt::W](W) writer structure"]
impl crate::Writable for SD_SECCNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SD_SECCNT to value 0"]
impl crate::Resettable for SD_SECCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
