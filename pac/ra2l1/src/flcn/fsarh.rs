#[doc = "Register `FSARH` reader"]
pub struct R(crate::R<FSARH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSARH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSARH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSARH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSARH` writer"]
pub struct W(crate::W<FSARH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSARH_SPEC>;
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
impl From<crate::W<FSARH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSARH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FSARH` reader - Flash Processing Start Address H"]
pub type FSARH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FSARH` writer - Flash Processing Start Address H"]
pub type FSARH_W<'a, const O: u8> = crate::FieldWriter<'a, u16, FSARH_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Flash Processing Start Address H"]
    #[inline(always)]
    pub fn fsarh(&self) -> FSARH_R {
        FSARH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Flash Processing Start Address H"]
    #[inline(always)]
    #[must_use]
    pub fn fsarh(&mut self) -> FSARH_W<0> {
        FSARH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Processing Start Address Register H\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsarh](index.html) module"]
pub struct FSARH_SPEC;
impl crate::RegisterSpec for FSARH_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [fsarh::R](R) reader structure"]
impl crate::Readable for FSARH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsarh::W](W) writer structure"]
impl crate::Writable for FSARH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSARH to value 0"]
impl crate::Resettable for FSARH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
