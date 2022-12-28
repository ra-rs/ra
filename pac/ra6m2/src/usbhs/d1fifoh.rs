#[doc = "Register `D1FIFOH` reader"]
pub struct R(crate::R<D1FIFOH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D1FIFOH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D1FIFOH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D1FIFOH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D1FIFOH` writer"]
pub struct W(crate::W<D1FIFOH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D1FIFOH_SPEC>;
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
impl From<crate::W<D1FIFOH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D1FIFOH_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "D1FIFO Port Register H\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d1fifoh](index.html) module"]
pub struct D1FIFOH_SPEC;
impl crate::RegisterSpec for D1FIFOH_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [d1fifoh::R](R) reader structure"]
impl crate::Readable for D1FIFOH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d1fifoh::W](W) writer structure"]
impl crate::Writable for D1FIFOH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D1FIFOH to value 0"]
impl crate::Resettable for D1FIFOH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
