#[doc = "Register `FEADDR` reader"]
pub struct R(crate::R<FEADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FEADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FEADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FEADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FEADDR` writer"]
pub struct W(crate::W<FEADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FEADDR_SPEC>;
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
impl From<crate::W<FEADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FEADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FEADDR` reader - End Address for FACI Command Processing"]
pub type FEADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FEADDR` writer - End Address for FACI Command Processing"]
pub type FEADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FEADDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - End Address for FACI Command Processing"]
    #[inline(always)]
    pub fn feaddr(&self) -> FEADDR_R {
        FEADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - End Address for FACI Command Processing"]
    #[inline(always)]
    #[must_use]
    pub fn feaddr(&mut self) -> FEADDR_W<0> {
        FEADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FACI Command End Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [feaddr](index.html) module"]
pub struct FEADDR_SPEC;
impl crate::RegisterSpec for FEADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [feaddr::R](R) reader structure"]
impl crate::Readable for FEADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [feaddr::W](W) writer structure"]
impl crate::Writable for FEADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FEADDR to value 0"]
impl crate::Resettable for FEADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
