#[doc = "Register `CTSUCHTRC3` reader"]
pub struct R(crate::R<CTSUCHTRC3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSUCHTRC3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSUCHTRC3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSUCHTRC3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTSUCHTRC3` writer"]
pub struct W(crate::W<CTSUCHTRC3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTSUCHTRC3_SPEC>;
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
impl From<crate::W<CTSUCHTRC3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTSUCHTRC3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTSUCHRC3` reader - CTSU Channel Transmit/Receive Control 3"]
pub type CTSUCHRC3_R = crate::FieldReader<u8, CTSUCHRC3_A>;
#[doc = "CTSU Channel Transmit/Receive Control 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTSUCHRC3_A {
    #[doc = "0: Reception"]
    _0 = 0,
    #[doc = "1: Transmission"]
    _1 = 1,
}
impl From<CTSUCHRC3_A> for u8 {
    #[inline(always)]
    fn from(variant: CTSUCHRC3_A) -> Self {
        variant as _
    }
}
impl CTSUCHRC3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CTSUCHRC3_A> {
        match self.bits {
            0 => Some(CTSUCHRC3_A::_0),
            1 => Some(CTSUCHRC3_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUCHRC3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUCHRC3_A::_1
    }
}
#[doc = "Field `CTSUCHRC3` writer - CTSU Channel Transmit/Receive Control 3"]
pub type CTSUCHRC3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, CTSUCHTRC3_SPEC, u8, CTSUCHRC3_A, 8, O>;
impl<'a, const O: u8> CTSUCHRC3_W<'a, O> {
    #[doc = "Reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTSUCHRC3_A::_0)
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTSUCHRC3_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - CTSU Channel Transmit/Receive Control 3"]
    #[inline(always)]
    pub fn ctsuchrc3(&self) -> CTSUCHRC3_R {
        CTSUCHRC3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - CTSU Channel Transmit/Receive Control 3"]
    #[inline(always)]
    #[must_use]
    pub fn ctsuchrc3(&mut self) -> CTSUCHRC3_W<0> {
        CTSUCHRC3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTSU Channel Transmit/Receive Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsuchtrc3](index.html) module"]
pub struct CTSUCHTRC3_SPEC;
impl crate::RegisterSpec for CTSUCHTRC3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctsuchtrc3::R](R) reader structure"]
impl crate::Readable for CTSUCHTRC3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctsuchtrc3::W](W) writer structure"]
impl crate::Writable for CTSUCHTRC3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTSUCHTRC3 to value 0"]
impl crate::Resettable for CTSUCHTRC3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
