#[doc = "Register `DMOFR` reader"]
pub struct R(crate::R<DMOFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMOFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMOFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMOFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMOFR` writer"]
pub struct W(crate::W<DMOFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMOFR_SPEC>;
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
impl From<crate::W<DMOFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMOFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMOFR` reader - Specifies the offset when offset addition is selected as the address update mode for transfer source or destination."]
pub type DMOFR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DMOFR` writer - Specifies the offset when offset addition is selected as the address update mode for transfer source or destination."]
pub type DMOFR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMOFR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Specifies the offset when offset addition is selected as the address update mode for transfer source or destination."]
    #[inline(always)]
    pub fn dmofr(&self) -> DMOFR_R {
        DMOFR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies the offset when offset addition is selected as the address update mode for transfer source or destination."]
    #[inline(always)]
    #[must_use]
    pub fn dmofr(&mut self) -> DMOFR_W<0> {
        DMOFR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Offset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmofr](index.html) module"]
pub struct DMOFR_SPEC;
impl crate::RegisterSpec for DMOFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmofr::R](R) reader structure"]
impl crate::Readable for DMOFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmofr::W](W) writer structure"]
impl crate::Writable for DMOFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMOFR to value 0"]
impl crate::Resettable for DMOFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
