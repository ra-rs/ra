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
#[doc = "Field `TPAUSE` reader - Automatic PAUSE Frame Retransmit Setting"]
pub type TPAUSE_R = crate::FieldReader<u16, TPAUSE_A>;
#[doc = "Automatic PAUSE Frame Retransmit Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum TPAUSE_A {
    #[doc = "0: Number of retransmissions is unlimited"]
    _0X0000 = 0,
}
impl From<TPAUSE_A> for u16 {
    #[inline(always)]
    fn from(variant: TPAUSE_A) -> Self {
        variant as _
    }
}
impl TPAUSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TPAUSE_A> {
        match self.bits {
            0 => Some(TPAUSE_A::_0X0000),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X0000`"]
    #[inline(always)]
    pub fn is_0x0000(&self) -> bool {
        *self == TPAUSE_A::_0X0000
    }
}
#[doc = "Field `TPAUSE` writer - Automatic PAUSE Frame Retransmit Setting"]
pub type TPAUSE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TPAUSER_SPEC, u16, TPAUSE_A, 16, O>;
impl<'a, const O: u8> TPAUSE_W<'a, O> {
    #[doc = "Number of retransmissions is unlimited"]
    #[inline(always)]
    pub fn _0x0000(self) -> &'a mut W {
        self.variant(TPAUSE_A::_0X0000)
    }
}
impl R {
    #[doc = "Bits 0:15 - Automatic PAUSE Frame Retransmit Setting"]
    #[inline(always)]
    pub fn tpause(&self) -> TPAUSE_R {
        TPAUSE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Automatic PAUSE Frame Retransmit Setting"]
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
