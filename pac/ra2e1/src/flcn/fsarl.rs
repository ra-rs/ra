#[doc = "Register `FSARL` reader"]
pub struct R(crate::R<FSARL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSARL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSARL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSARL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSARL` writer"]
pub struct W(crate::W<FSARL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSARL_SPEC>;
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
impl From<crate::W<FSARL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSARL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FSARL` reader - Flash Processing Start Address L"]
pub type FSARL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FSARL` writer - Flash Processing Start Address L"]
pub type FSARL_W<'a, const O: u8> = crate::FieldWriter<'a, u16, FSARL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Flash Processing Start Address L"]
    #[inline(always)]
    pub fn fsarl(&self) -> FSARL_R {
        FSARL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Flash Processing Start Address L"]
    #[inline(always)]
    pub fn fsarl(&mut self) -> FSARL_W<0> {
        FSARL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Processing Start Address Register L\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsarl](index.html) module"]
pub struct FSARL_SPEC;
impl crate::RegisterSpec for FSARL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [fsarl::R](R) reader structure"]
impl crate::Readable for FSARL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsarl::W](W) writer structure"]
impl crate::Writable for FSARL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FSARL to value 0"]
impl crate::Resettable for FSARL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
