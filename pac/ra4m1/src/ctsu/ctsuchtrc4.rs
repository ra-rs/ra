#[doc = "Register `CTSUCHTRC4` reader"]
pub struct R(crate::R<CTSUCHTRC4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSUCHTRC4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSUCHTRC4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSUCHTRC4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTSUCHTRC4` writer"]
pub struct W(crate::W<CTSUCHTRC4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTSUCHTRC4_SPEC>;
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
impl From<crate::W<CTSUCHTRC4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTSUCHTRC4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTSUCHAC4` reader - CTSU Channel Transmit/Receive Control 4"]
pub type CTSUCHAC4_R = crate::FieldReader<u8, CTSUCHAC4_A>;
#[doc = "CTSU Channel Transmit/Receive Control 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTSUCHAC4_A {
    #[doc = "0: Reception"]
    _0 = 0,
    #[doc = "1: Transmission"]
    _1 = 1,
}
impl From<CTSUCHAC4_A> for u8 {
    #[inline(always)]
    fn from(variant: CTSUCHAC4_A) -> Self {
        variant as _
    }
}
impl CTSUCHAC4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CTSUCHAC4_A> {
        match self.bits {
            0 => Some(CTSUCHAC4_A::_0),
            1 => Some(CTSUCHAC4_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUCHAC4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUCHAC4_A::_1
    }
}
#[doc = "Field `CTSUCHAC4` writer - CTSU Channel Transmit/Receive Control 4"]
pub type CTSUCHAC4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, CTSUCHTRC4_SPEC, u8, CTSUCHAC4_A, 4, O>;
impl<'a, const O: u8> CTSUCHAC4_W<'a, O> {
    #[doc = "Reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTSUCHAC4_A::_0)
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTSUCHAC4_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - CTSU Channel Transmit/Receive Control 4"]
    #[inline(always)]
    pub fn ctsuchac4(&self) -> CTSUCHAC4_R {
        CTSUCHAC4_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - CTSU Channel Transmit/Receive Control 4"]
    #[inline(always)]
    #[must_use]
    pub fn ctsuchac4(&mut self) -> CTSUCHAC4_W<0> {
        CTSUCHAC4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTSU Channel Transmit/Receive Control Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsuchtrc4](index.html) module"]
pub struct CTSUCHTRC4_SPEC;
impl crate::RegisterSpec for CTSUCHTRC4_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctsuchtrc4::R](R) reader structure"]
impl crate::Readable for CTSUCHTRC4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctsuchtrc4::W](W) writer structure"]
impl crate::Writable for CTSUCHTRC4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTSUCHTRC4 to value 0"]
impl crate::Resettable for CTSUCHTRC4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
