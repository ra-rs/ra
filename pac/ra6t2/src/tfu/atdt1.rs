#[doc = "Register `ATDT1` reader"]
pub struct R(crate::R<ATDT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ATDT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ATDT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ATDT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ATDT1` writer"]
pub struct W(crate::W<ATDT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ATDT1_SPEC>;
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
impl From<crate::W<ATDT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ATDT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ATDT1` reader - Arctangent Data Register 1 (single-precision floating-point)"]
pub type ATDT1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ATDT1` writer - Arctangent Data Register 1 (single-precision floating-point)"]
pub type ATDT1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ATDT1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Arctangent Data Register 1 (single-precision floating-point)"]
    #[inline(always)]
    pub fn atdt1(&self) -> ATDT1_R {
        ATDT1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Arctangent Data Register 1 (single-precision floating-point)"]
    #[inline(always)]
    #[must_use]
    pub fn atdt1(&mut self) -> ATDT1_W<0> {
        ATDT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Arctangent Data Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [atdt1](index.html) module"]
pub struct ATDT1_SPEC;
impl crate::RegisterSpec for ATDT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [atdt1::R](R) reader structure"]
impl crate::Readable for ATDT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [atdt1::W](W) writer structure"]
impl crate::Writable for ATDT1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ATDT1 to value 0"]
impl crate::Resettable for ATDT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
