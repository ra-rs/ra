#[doc = "Register `ADCMPTBR%s` reader"]
pub struct R(crate::R<ADCMPTBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCMPTBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCMPTBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCMPTBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCMPTBR%s` writer"]
pub struct W(crate::W<ADCMPTBR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCMPTBR_SPEC>;
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
impl From<crate::W<ADCMPTBR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCMPTBR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPTBL` reader - Compare Match Table n : Low-side level"]
pub type CMPTBL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CMPTBL` writer - Compare Match Table n : Low-side level"]
pub type CMPTBL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCMPTBR_SPEC, u16, u16, 16, O>;
#[doc = "Field `CMPTBH` reader - Compare Match Table n : High-side level"]
pub type CMPTBH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CMPTBH` writer - Compare Match Table n : High-side level"]
pub type CMPTBH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCMPTBR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Compare Match Table n : Low-side level"]
    #[inline(always)]
    pub fn cmptbl(&self) -> CMPTBL_R {
        CMPTBL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Compare Match Table n : High-side level"]
    #[inline(always)]
    pub fn cmptbh(&self) -> CMPTBH_R {
        CMPTBH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Compare Match Table n : Low-side level"]
    #[inline(always)]
    #[must_use]
    pub fn cmptbl(&mut self) -> CMPTBL_W<0> {
        CMPTBL_W::new(self)
    }
    #[doc = "Bits 16:31 - Compare Match Table n : High-side level"]
    #[inline(always)]
    #[must_use]
    pub fn cmptbh(&mut self) -> CMPTBH_W<16> {
        CMPTBH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Compare Match Table Register %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcmptbr](index.html) module"]
pub struct ADCMPTBR_SPEC;
impl crate::RegisterSpec for ADCMPTBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcmptbr::R](R) reader structure"]
impl crate::Readable for ADCMPTBR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcmptbr::W](W) writer structure"]
impl crate::Writable for ADCMPTBR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCMPTBR%s to value 0xffff_0000"]
impl crate::Resettable for ADCMPTBR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_0000;
}
