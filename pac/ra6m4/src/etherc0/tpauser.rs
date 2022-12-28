#[doc = "Register `TPAUSER` reader"]
pub struct R(crate::R<TPAUSER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TPAUSER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TPAUSER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TPAUSER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TPAUSER` writer"]
pub struct W(crate::W<TPAUSER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TPAUSER_SPEC>;
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
impl From<crate::W<TPAUSER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TPAUSER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TPAUSE` reader - "]
pub type TPAUSE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TPAUSE` writer - "]
pub type TPAUSE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TPAUSER_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn tpause(&self) -> TPAUSE_R {
        TPAUSE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn tpause(&mut self) -> TPAUSE_W<0> {
        TPAUSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PAUSE Frame Retransmit Count Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpauser](index.html) module"]
pub struct TPAUSER_SPEC;
impl crate::RegisterSpec for TPAUSER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tpauser::R](R) reader structure"]
impl crate::Readable for TPAUSER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tpauser::W](W) writer structure"]
impl crate::Writable for TPAUSER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TPAUSER to value 0"]
impl crate::Resettable for TPAUSER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
