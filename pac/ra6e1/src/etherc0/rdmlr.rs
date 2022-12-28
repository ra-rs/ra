#[doc = "Register `RDMLR` reader"]
pub struct R(crate::R<RDMLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDMLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDMLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDMLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RDMLR` writer"]
pub struct W(crate::W<RDMLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RDMLR_SPEC>;
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
impl From<crate::W<RDMLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RDMLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RMD` reader - Random Number Generation Counter"]
pub type RMD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RMD` writer - Random Number Generation Counter"]
pub type RMD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RDMLR_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:19 - Random Number Generation Counter"]
    #[inline(always)]
    pub fn rmd(&self) -> RMD_R {
        RMD_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - Random Number Generation Counter"]
    #[inline(always)]
    #[must_use]
    pub fn rmd(&mut self) -> RMD_W<0> {
        RMD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Random Number Generation Counter Upper Limit Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdmlr](index.html) module"]
pub struct RDMLR_SPEC;
impl crate::RegisterSpec for RDMLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rdmlr::R](R) reader structure"]
impl crate::Readable for RDMLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rdmlr::W](W) writer structure"]
impl crate::Writable for RDMLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RDMLR to value 0"]
impl crate::Resettable for RDMLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
