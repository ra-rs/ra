#[doc = "Register `FEAMH` reader"]
pub struct R(crate::R<FEAMH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FEAMH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FEAMH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FEAMH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FEAMH` writer"]
pub struct W(crate::W<FEAMH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FEAMH_SPEC>;
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
impl From<crate::W<FEAMH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FEAMH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FEAMH` reader - Flash Error Address Monitor Register H"]
pub type FEAMH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FEAMH` writer - Flash Error Address Monitor Register H"]
pub type FEAMH_W<'a, const O: u8> = crate::FieldWriter<'a, u16, FEAMH_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Flash Error Address Monitor Register H"]
    #[inline(always)]
    pub fn feamh(&self) -> FEAMH_R {
        FEAMH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Flash Error Address Monitor Register H"]
    #[inline(always)]
    pub fn feamh(&mut self) -> FEAMH_W<0> {
        FEAMH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Error Address Monitor Register H\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [feamh](index.html) module"]
pub struct FEAMH_SPEC;
impl crate::RegisterSpec for FEAMH_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [feamh::R](R) reader structure"]
impl crate::Readable for FEAMH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [feamh::W](W) writer structure"]
impl crate::Writable for FEAMH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FEAMH to value 0"]
impl crate::Resettable for FEAMH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
