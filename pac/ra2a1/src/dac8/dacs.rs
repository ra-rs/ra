#[doc = "Register `DACS%s` reader"]
pub struct R(crate::R<DACS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DACS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DACS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DACS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DACS%s` writer"]
pub struct W(crate::W<DACS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DACS_SPEC>;
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
impl From<crate::W<DACS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DACS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DACS` reader - DACS D/A conversion store data"]
pub type DACS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DACS` writer - DACS D/A conversion store data"]
pub type DACS_W<'a, const O: u8> = crate::FieldWriter<'a, u8, DACS_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - DACS D/A conversion store data"]
    #[inline(always)]
    pub fn dacs(&self) -> DACS_R {
        DACS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - DACS D/A conversion store data"]
    #[inline(always)]
    #[must_use]
    pub fn dacs(&mut self) -> DACS_W<0> {
        DACS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "D/A Conversion Value Setting Register %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacs](index.html) module"]
pub struct DACS_SPEC;
impl crate::RegisterSpec for DACS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dacs::R](R) reader structure"]
impl crate::Readable for DACS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dacs::W](W) writer structure"]
impl crate::Writable for DACS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DACS%s to value 0"]
impl crate::Resettable for DACS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
