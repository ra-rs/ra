#[doc = "Register `FLLCR2` reader"]
pub struct R(crate::R<FLLCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLLCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLLCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLLCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLLCR2` writer"]
pub struct W(crate::W<FLLCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLLCR2_SPEC>;
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
impl From<crate::W<FLLCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLLCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLLCNTL` reader - FLL Multiplication Control"]
pub type FLLCNTL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FLLCNTL` writer - FLL Multiplication Control"]
pub type FLLCNTL_W<'a, const O: u8> = crate::FieldWriter<'a, u16, FLLCR2_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 0:10 - FLL Multiplication Control"]
    #[inline(always)]
    pub fn fllcntl(&self) -> FLLCNTL_R {
        FLLCNTL_R::new(self.bits & 0x07ff)
    }
}
impl W {
    #[doc = "Bits 0:10 - FLL Multiplication Control"]
    #[inline(always)]
    #[must_use]
    pub fn fllcntl(&mut self) -> FLLCNTL_W<0> {
        FLLCNTL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLL Control Register2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fllcr2](index.html) module"]
pub struct FLLCR2_SPEC;
impl crate::RegisterSpec for FLLCR2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [fllcr2::R](R) reader structure"]
impl crate::Readable for FLLCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fllcr2::W](W) writer structure"]
impl crate::Writable for FLLCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLLCR2 to value 0"]
impl crate::Resettable for FLLCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
