#[doc = "Register `FPCKAR` reader"]
pub struct R(crate::R<FPCKAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPCKAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPCKAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPCKAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FPCKAR` writer"]
pub struct W(crate::W<FPCKAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPCKAR_SPEC>;
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
impl From<crate::W<FPCKAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPCKAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCKA` reader - Flash Sequencer Operating Clock Notification"]
pub type PCKA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCKA` writer - Flash Sequencer Operating Clock Notification"]
pub type PCKA_W<'a, const O: u8> = crate::FieldWriter<'a, u16, FPCKAR_SPEC, u8, u8, 8, O>;
#[doc = "Field `KEY` writer - Key Code"]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u16, FPCKAR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Flash Sequencer Operating Clock Notification"]
    #[inline(always)]
    pub fn pcka(&self) -> PCKA_R {
        PCKA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Flash Sequencer Operating Clock Notification"]
    #[inline(always)]
    #[must_use]
    pub fn pcka(&mut self) -> PCKA_W<0> {
        PCKA_W::new(self)
    }
    #[doc = "Bits 8:15 - Key Code"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<8> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Sequencer Processing Clock Notification Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpckar](index.html) module"]
pub struct FPCKAR_SPEC;
impl crate::RegisterSpec for FPCKAR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [fpckar::R](R) reader structure"]
impl crate::Readable for FPCKAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fpckar::W](W) writer structure"]
impl crate::Writable for FPCKAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FPCKAR to value 0x32"]
impl crate::Resettable for FPCKAR_SPEC {
    const RESET_VALUE: Self::Ux = 0x32;
}
