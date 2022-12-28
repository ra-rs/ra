#[doc = "Register `SCISPICKDIVCR` reader"]
pub struct R(crate::R<SCISPICKDIVCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCISPICKDIVCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCISPICKDIVCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCISPICKDIVCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCISPICKDIVCR` writer"]
pub struct W(crate::W<SCISPICKDIVCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCISPICKDIVCR_SPEC>;
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
impl From<crate::W<SCISPICKDIVCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCISPICKDIVCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCISPICKDIV` reader - SCI SPI Clock (SCISPICLK) Division Select"]
pub type SCISPICKDIV_R = crate::FieldReader<u8, SCISPICKDIV_A>;
#[doc = "SCI SPI Clock (SCISPICLK) Division Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SCISPICKDIV_A {
    #[doc = "0: â\u{88}\u{95} 1 (value after reset)"]
    _000 = 0,
    #[doc = "1: â\u{88}\u{95} 2"]
    _001 = 1,
    #[doc = "2: â\u{88}\u{95} 4"]
    _010 = 2,
    #[doc = "3: â\u{88}\u{95} 6"]
    _011 = 3,
    #[doc = "4: â\u{88}\u{95} 8"]
    _100 = 4,
}
impl From<SCISPICKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: SCISPICKDIV_A) -> Self {
        variant as _
    }
}
impl SCISPICKDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SCISPICKDIV_A> {
        match self.bits {
            0 => Some(SCISPICKDIV_A::_000),
            1 => Some(SCISPICKDIV_A::_001),
            2 => Some(SCISPICKDIV_A::_010),
            3 => Some(SCISPICKDIV_A::_011),
            4 => Some(SCISPICKDIV_A::_100),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SCISPICKDIV_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SCISPICKDIV_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SCISPICKDIV_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == SCISPICKDIV_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == SCISPICKDIV_A::_100
    }
}
#[doc = "Field `SCISPICKDIV` writer - SCI SPI Clock (SCISPICLK) Division Select"]
pub type SCISPICKDIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, SCISPICKDIVCR_SPEC, u8, SCISPICKDIV_A, 3, O>;
impl<'a, const O: u8> SCISPICKDIV_W<'a, O> {
    #[doc = "â\u{88}\u{95} 1 (value after reset)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(SCISPICKDIV_A::_000)
    }
    #[doc = "â\u{88}\u{95} 2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(SCISPICKDIV_A::_001)
    }
    #[doc = "â\u{88}\u{95} 4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(SCISPICKDIV_A::_010)
    }
    #[doc = "â\u{88}\u{95} 6"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(SCISPICKDIV_A::_011)
    }
    #[doc = "â\u{88}\u{95} 8"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(SCISPICKDIV_A::_100)
    }
}
impl R {
    #[doc = "Bits 0:2 - SCI SPI Clock (SCISPICLK) Division Select"]
    #[inline(always)]
    pub fn scispickdiv(&self) -> SCISPICKDIV_R {
        SCISPICKDIV_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - SCI SPI Clock (SCISPICLK) Division Select"]
    #[inline(always)]
    #[must_use]
    pub fn scispickdiv(&mut self) -> SCISPICKDIV_W<0> {
        SCISPICKDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCI SPI Clock Division Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scispickdivcr](index.html) module"]
pub struct SCISPICKDIVCR_SPEC;
impl crate::RegisterSpec for SCISPICKDIVCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [scispickdivcr::R](R) reader structure"]
impl crate::Readable for SCISPICKDIVCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scispickdivcr::W](W) writer structure"]
impl crate::Writable for SCISPICKDIVCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCISPICKDIVCR to value 0"]
impl crate::Resettable for SCISPICKDIVCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
