#[doc = "Register `LCCR` reader"]
pub struct R(crate::R<LCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCCR` writer"]
pub struct W(crate::W<LCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCCR_SPEC>;
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
impl From<crate::W<LCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCCR` reader - Lost Carrier Counter"]
pub type LCCR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LCCR` writer - Lost Carrier Counter"]
pub type LCCR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LCCR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Lost Carrier Counter"]
    #[inline(always)]
    pub fn lccr(&self) -> LCCR_R {
        LCCR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Lost Carrier Counter"]
    #[inline(always)]
    #[must_use]
    pub fn lccr(&mut self) -> LCCR_W<0> {
        LCCR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Lost Carrier Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lccr](index.html) module"]
pub struct LCCR_SPEC;
impl crate::RegisterSpec for LCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lccr::R](R) reader structure"]
impl crate::Readable for LCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lccr::W](W) writer structure"]
impl crate::Writable for LCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LCCR to value 0"]
impl crate::Resettable for LCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
