#[doc = "Register `PL1CTRL2` reader"]
pub struct R(crate::R<PL1CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PL1CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PL1CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PL1CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PL1CTRL2` writer"]
pub struct W(crate::W<PL1CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PL1CTRL2_SPEC>;
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
impl From<crate::W<PL1CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PL1CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HIRDMON` reader - HIRD Value Monitor"]
pub type HIRDMON_R = crate::FieldReader<u8, HIRDMON_A>;
#[doc = "HIRD Value Monitor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HIRDMON_A {
    #[doc = "0: The HIRD field value of the LPM token received last is reflected."]
    _0 = 0,
    #[doc = "1: The HIRD field value of the LPM token received last is reflected."]
    _1 = 1,
}
impl From<HIRDMON_A> for u8 {
    #[inline(always)]
    fn from(variant: HIRDMON_A) -> Self {
        variant as _
    }
}
impl HIRDMON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HIRDMON_A> {
        match self.bits {
            0 => Some(HIRDMON_A::_0),
            1 => Some(HIRDMON_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HIRDMON_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HIRDMON_A::_1
    }
}
#[doc = "Field `HIRDMON` writer - HIRD Value Monitor"]
pub type HIRDMON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, PL1CTRL2_SPEC, u8, HIRDMON_A, 4, O>;
impl<'a, const O: u8> HIRDMON_W<'a, O> {
    #[doc = "The HIRD field value of the LPM token received last is reflected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HIRDMON_A::_0)
    }
    #[doc = "The HIRD field value of the LPM token received last is reflected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HIRDMON_A::_1)
    }
}
#[doc = "Field `RWEMON` reader - RWE Value Monitor"]
pub type RWEMON_R = crate::BitReader<RWEMON_A>;
#[doc = "RWE Value Monitor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWEMON_A {
    #[doc = "0: The RWE bit value of the LPM token received last is reflected."]
    _0 = 0,
    #[doc = "1: The RWE bit value of the LPM token received last is reflected."]
    _1 = 1,
}
impl From<RWEMON_A> for bool {
    #[inline(always)]
    fn from(variant: RWEMON_A) -> Self {
        variant as u8 != 0
    }
}
impl RWEMON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWEMON_A {
        match self.bits {
            false => RWEMON_A::_0,
            true => RWEMON_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWEMON_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWEMON_A::_1
    }
}
#[doc = "Field `RWEMON` writer - RWE Value Monitor"]
pub type RWEMON_W<'a, const O: u8> = crate::BitWriter<'a, u16, PL1CTRL2_SPEC, RWEMON_A, O>;
impl<'a, const O: u8> RWEMON_W<'a, O> {
    #[doc = "The RWE bit value of the LPM token received last is reflected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWEMON_A::_0)
    }
    #[doc = "The RWE bit value of the LPM token received last is reflected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWEMON_A::_1)
    }
}
impl R {
    #[doc = "Bits 8:11 - HIRD Value Monitor"]
    #[inline(always)]
    pub fn hirdmon(&self) -> HIRDMON_R {
        HIRDMON_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - RWE Value Monitor"]
    #[inline(always)]
    pub fn rwemon(&self) -> RWEMON_R {
        RWEMON_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:11 - HIRD Value Monitor"]
    #[inline(always)]
    #[must_use]
    pub fn hirdmon(&mut self) -> HIRDMON_W<8> {
        HIRDMON_W::new(self)
    }
    #[doc = "Bit 12 - RWE Value Monitor"]
    #[inline(always)]
    #[must_use]
    pub fn rwemon(&mut self) -> RWEMON_W<12> {
        RWEMON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Function L1 Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pl1ctrl2](index.html) module"]
pub struct PL1CTRL2_SPEC;
impl crate::RegisterSpec for PL1CTRL2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pl1ctrl2::R](R) reader structure"]
impl crate::Readable for PL1CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pl1ctrl2::W](W) writer structure"]
impl crate::Writable for PL1CTRL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PL1CTRL2 to value 0"]
impl crate::Resettable for PL1CTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
