#[doc = "Register `BCFRR` reader"]
pub struct R(crate::R<BCFRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCFRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCFRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCFRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BCFRR` writer"]
pub struct W(crate::W<BCFRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCFRR_SPEC>;
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
impl From<crate::W<BCFRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCFRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BCF` reader - "]
pub type BCF_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BCF` writer - "]
pub type BCF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BCFRR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn bcf(&self) -> BCF_R {
        BCF_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn bcf(&mut self) -> BCF_W<0> {
        BCF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Broadcast Frame Receive Count Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcfrr](index.html) module"]
pub struct BCFRR_SPEC;
impl crate::RegisterSpec for BCFRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bcfrr::R](R) reader structure"]
impl crate::Readable for BCFRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bcfrr::W](W) writer structure"]
impl crate::Writable for BCFRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BCFRR to value 0"]
impl crate::Resettable for BCFRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
