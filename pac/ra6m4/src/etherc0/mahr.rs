#[doc = "Register `MAHR` reader"]
pub struct R(crate::R<MAHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAHR` writer"]
pub struct W(crate::W<MAHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAHR_SPEC>;
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
impl From<crate::W<MAHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAHR` reader - MAC Address Upper Bit"]
pub type MAHR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MAHR` writer - MAC Address Upper Bit"]
pub type MAHR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAHR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - MAC Address Upper Bit"]
    #[inline(always)]
    pub fn mahr(&self) -> MAHR_R {
        MAHR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MAC Address Upper Bit"]
    #[inline(always)]
    #[must_use]
    pub fn mahr(&mut self) -> MAHR_W<0> {
        MAHR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAC Address Upper Bit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mahr](index.html) module"]
pub struct MAHR_SPEC;
impl crate::RegisterSpec for MAHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mahr::R](R) reader structure"]
impl crate::Readable for MAHR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mahr::W](W) writer structure"]
impl crate::Writable for MAHR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAHR to value 0"]
impl crate::Resettable for MAHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
