#[doc = "Register `SEG%s` reader"]
pub struct R(crate::R<SEG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEG%s` writer"]
pub struct W(crate::W<SEG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEG_SPEC>;
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
impl From<crate::W<SEG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEG` reader - LCD Display Data"]
pub type SEG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEG` writer - LCD Display Data"]
pub type SEG_W<'a, const O: u8> = crate::FieldWriter<'a, u8, SEG_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - LCD Display Data"]
    #[inline(always)]
    pub fn seg(&self) -> SEG_R {
        SEG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - LCD Display Data"]
    #[inline(always)]
    #[must_use]
    pub fn seg(&mut self) -> SEG_W<0> {
        SEG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD Display Data Register %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seg](index.html) module"]
pub struct SEG_SPEC;
impl crate::RegisterSpec for SEG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [seg::R](R) reader structure"]
impl crate::Readable for SEG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seg::W](W) writer structure"]
impl crate::Writable for SEG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEG%s to value 0"]
impl crate::Resettable for SEG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
