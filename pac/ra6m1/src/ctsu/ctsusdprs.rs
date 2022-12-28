#[doc = "Register `CTSUSDPRS` reader"]
pub struct R(crate::R<CTSUSDPRS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSUSDPRS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSUSDPRS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSUSDPRS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTSUSDPRS` writer"]
pub struct W(crate::W<CTSUSDPRS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTSUSDPRS_SPEC>;
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
impl From<crate::W<CTSUSDPRS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTSUSDPRS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTSUPRRATIO` reader - CTSU Measurement Time and Pulse Count Adjustment Recommended setting: 3 (0011b)"]
pub type CTSUPRRATIO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTSUPRRATIO` writer - CTSU Measurement Time and Pulse Count Adjustment Recommended setting: 3 (0011b)"]
pub type CTSUPRRATIO_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CTSUSDPRS_SPEC, u8, u8, 4, O>;
#[doc = "Field `CTSUPRMODE` reader - CTSU Base Period and Pulse Count Setting"]
pub type CTSUPRMODE_R = crate::FieldReader<u8, CTSUPRMODE_A>;
#[doc = "CTSU Base Period and Pulse Count Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTSUPRMODE_A {
    #[doc = "0: 510 pulses"]
    _00 = 0,
    #[doc = "1: 126 pulses"]
    _01 = 1,
    #[doc = "2: 62 pulses (recommended setting value)"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<CTSUPRMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CTSUPRMODE_A) -> Self {
        variant as _
    }
}
impl CTSUPRMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSUPRMODE_A {
        match self.bits {
            0 => CTSUPRMODE_A::_00,
            1 => CTSUPRMODE_A::_01,
            2 => CTSUPRMODE_A::_10,
            3 => CTSUPRMODE_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CTSUPRMODE_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CTSUPRMODE_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CTSUPRMODE_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CTSUPRMODE_A::_11
    }
}
#[doc = "Field `CTSUPRMODE` writer - CTSU Base Period and Pulse Count Setting"]
pub type CTSUPRMODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, CTSUSDPRS_SPEC, u8, CTSUPRMODE_A, 2, O>;
impl<'a, const O: u8> CTSUPRMODE_W<'a, O> {
    #[doc = "510 pulses"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CTSUPRMODE_A::_00)
    }
    #[doc = "126 pulses"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CTSUPRMODE_A::_01)
    }
    #[doc = "62 pulses (recommended setting value)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CTSUPRMODE_A::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CTSUPRMODE_A::_11)
    }
}
#[doc = "Field `CTSUSOFF` reader - CTSU High-Pass Noise Reduction Function Off Setting"]
pub type CTSUSOFF_R = crate::BitReader<CTSUSOFF_A>;
#[doc = "CTSU High-Pass Noise Reduction Function Off Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSUSOFF_A {
    #[doc = "0: High-pass noise reduction function turned on"]
    _0 = 0,
    #[doc = "1: High-pass noise reduction function turned off"]
    _1 = 1,
}
impl From<CTSUSOFF_A> for bool {
    #[inline(always)]
    fn from(variant: CTSUSOFF_A) -> Self {
        variant as u8 != 0
    }
}
impl CTSUSOFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSUSOFF_A {
        match self.bits {
            false => CTSUSOFF_A::_0,
            true => CTSUSOFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUSOFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUSOFF_A::_1
    }
}
#[doc = "Field `CTSUSOFF` writer - CTSU High-Pass Noise Reduction Function Off Setting"]
pub type CTSUSOFF_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTSUSDPRS_SPEC, CTSUSOFF_A, O>;
impl<'a, const O: u8> CTSUSOFF_W<'a, O> {
    #[doc = "High-pass noise reduction function turned on"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTSUSOFF_A::_0)
    }
    #[doc = "High-pass noise reduction function turned off"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTSUSOFF_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - CTSU Measurement Time and Pulse Count Adjustment Recommended setting: 3 (0011b)"]
    #[inline(always)]
    pub fn ctsuprratio(&self) -> CTSUPRRATIO_R {
        CTSUPRRATIO_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:5 - CTSU Base Period and Pulse Count Setting"]
    #[inline(always)]
    pub fn ctsuprmode(&self) -> CTSUPRMODE_R {
        CTSUPRMODE_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - CTSU High-Pass Noise Reduction Function Off Setting"]
    #[inline(always)]
    pub fn ctsusoff(&self) -> CTSUSOFF_R {
        CTSUSOFF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - CTSU Measurement Time and Pulse Count Adjustment Recommended setting: 3 (0011b)"]
    #[inline(always)]
    #[must_use]
    pub fn ctsuprratio(&mut self) -> CTSUPRRATIO_W<0> {
        CTSUPRRATIO_W::new(self)
    }
    #[doc = "Bits 4:5 - CTSU Base Period and Pulse Count Setting"]
    #[inline(always)]
    #[must_use]
    pub fn ctsuprmode(&mut self) -> CTSUPRMODE_W<4> {
        CTSUPRMODE_W::new(self)
    }
    #[doc = "Bit 6 - CTSU High-Pass Noise Reduction Function Off Setting"]
    #[inline(always)]
    #[must_use]
    pub fn ctsusoff(&mut self) -> CTSUSOFF_W<6> {
        CTSUSOFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTSU Synchronous Noise Reduction Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsusdprs](index.html) module"]
pub struct CTSUSDPRS_SPEC;
impl crate::RegisterSpec for CTSUSDPRS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctsusdprs::R](R) reader structure"]
impl crate::Readable for CTSUSDPRS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctsusdprs::W](W) writer structure"]
impl crate::Writable for CTSUSDPRS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTSUSDPRS to value 0"]
impl crate::Resettable for CTSUSDPRS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
