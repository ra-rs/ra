#[doc = "Register `DPSIFR2` reader"]
pub struct R(crate::R<DPSIFR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPSIFR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPSIFR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPSIFR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPSIFR2` writer"]
pub struct W(crate::W<DPSIFR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPSIFR2_SPEC>;
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
impl From<crate::W<DPSIFR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPSIFR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLVD1IF` reader - LVD1 Deep Standby Cancel Flag"]
pub type DLVD1IF_R = crate::BitReader<DLVD1IF_A>;
#[doc = "LVD1 Deep Standby Cancel Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLVD1IF_A {
    #[doc = "0: The cancel request is not generated"]
    _0 = 0,
    #[doc = "1: The cancel request is generated"]
    _1 = 1,
}
impl From<DLVD1IF_A> for bool {
    #[inline(always)]
    fn from(variant: DLVD1IF_A) -> Self {
        variant as u8 != 0
    }
}
impl DLVD1IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLVD1IF_A {
        match self.bits {
            false => DLVD1IF_A::_0,
            true => DLVD1IF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLVD1IF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLVD1IF_A::_1
    }
}
#[doc = "Field `DLVD1IF` writer - LVD1 Deep Standby Cancel Flag"]
pub type DLVD1IF_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIFR2_SPEC, DLVD1IF_A, O>;
impl<'a, const O: u8> DLVD1IF_W<'a, O> {
    #[doc = "The cancel request is not generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DLVD1IF_A::_0)
    }
    #[doc = "The cancel request is generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DLVD1IF_A::_1)
    }
}
#[doc = "Field `DLVD2IF` reader - LVD2 Deep Standby Cancel Flag"]
pub type DLVD2IF_R = crate::BitReader<DLVD2IF_A>;
#[doc = "LVD2 Deep Standby Cancel Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLVD2IF_A {
    #[doc = "0: The cancel request is not generated"]
    _0 = 0,
    #[doc = "1: The cancel request is generated"]
    _1 = 1,
}
impl From<DLVD2IF_A> for bool {
    #[inline(always)]
    fn from(variant: DLVD2IF_A) -> Self {
        variant as u8 != 0
    }
}
impl DLVD2IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLVD2IF_A {
        match self.bits {
            false => DLVD2IF_A::_0,
            true => DLVD2IF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLVD2IF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLVD2IF_A::_1
    }
}
#[doc = "Field `DLVD2IF` writer - LVD2 Deep Standby Cancel Flag"]
pub type DLVD2IF_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIFR2_SPEC, DLVD2IF_A, O>;
impl<'a, const O: u8> DLVD2IF_W<'a, O> {
    #[doc = "The cancel request is not generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DLVD2IF_A::_0)
    }
    #[doc = "The cancel request is generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DLVD2IF_A::_1)
    }
}
#[doc = "Field `DRTCIIF` reader - RTC Interval Interrupt Deep Standby Cancel Flag"]
pub type DRTCIIF_R = crate::BitReader<DRTCIIF_A>;
#[doc = "RTC Interval Interrupt Deep Standby Cancel Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRTCIIF_A {
    #[doc = "0: The cancel request is not generated"]
    _0 = 0,
    #[doc = "1: The cancel request is generated"]
    _1 = 1,
}
impl From<DRTCIIF_A> for bool {
    #[inline(always)]
    fn from(variant: DRTCIIF_A) -> Self {
        variant as u8 != 0
    }
}
impl DRTCIIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRTCIIF_A {
        match self.bits {
            false => DRTCIIF_A::_0,
            true => DRTCIIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DRTCIIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DRTCIIF_A::_1
    }
}
#[doc = "Field `DRTCIIF` writer - RTC Interval Interrupt Deep Standby Cancel Flag"]
pub type DRTCIIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIFR2_SPEC, DRTCIIF_A, O>;
impl<'a, const O: u8> DRTCIIF_W<'a, O> {
    #[doc = "The cancel request is not generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DRTCIIF_A::_0)
    }
    #[doc = "The cancel request is generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DRTCIIF_A::_1)
    }
}
#[doc = "Field `DRTCAIF` reader - RTC Alarm Interrupt Deep Standby Cancel Flag"]
pub type DRTCAIF_R = crate::BitReader<DRTCAIF_A>;
#[doc = "RTC Alarm Interrupt Deep Standby Cancel Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRTCAIF_A {
    #[doc = "0: The cancel request is not generated"]
    _0 = 0,
    #[doc = "1: The cancel request is generated"]
    _1 = 1,
}
impl From<DRTCAIF_A> for bool {
    #[inline(always)]
    fn from(variant: DRTCAIF_A) -> Self {
        variant as u8 != 0
    }
}
impl DRTCAIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRTCAIF_A {
        match self.bits {
            false => DRTCAIF_A::_0,
            true => DRTCAIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DRTCAIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DRTCAIF_A::_1
    }
}
#[doc = "Field `DRTCAIF` writer - RTC Alarm Interrupt Deep Standby Cancel Flag"]
pub type DRTCAIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIFR2_SPEC, DRTCAIF_A, O>;
impl<'a, const O: u8> DRTCAIF_W<'a, O> {
    #[doc = "The cancel request is not generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DRTCAIF_A::_0)
    }
    #[doc = "The cancel request is generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DRTCAIF_A::_1)
    }
}
#[doc = "Field `DNMIF` reader - NMI Pin Deep Standby Cancel Flag"]
pub type DNMIF_R = crate::BitReader<DNMIF_A>;
#[doc = "NMI Pin Deep Standby Cancel Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DNMIF_A {
    #[doc = "0: The cancel request is not generated"]
    _0 = 0,
    #[doc = "1: The cancel request is generated"]
    _1 = 1,
}
impl From<DNMIF_A> for bool {
    #[inline(always)]
    fn from(variant: DNMIF_A) -> Self {
        variant as u8 != 0
    }
}
impl DNMIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DNMIF_A {
        match self.bits {
            false => DNMIF_A::_0,
            true => DNMIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DNMIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DNMIF_A::_1
    }
}
#[doc = "Field `DNMIF` writer - NMI Pin Deep Standby Cancel Flag"]
pub type DNMIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIFR2_SPEC, DNMIF_A, O>;
impl<'a, const O: u8> DNMIF_W<'a, O> {
    #[doc = "The cancel request is not generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DNMIF_A::_0)
    }
    #[doc = "The cancel request is generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DNMIF_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - LVD1 Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dlvd1if(&self) -> DLVD1IF_R {
        DLVD1IF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LVD2 Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dlvd2if(&self) -> DLVD2IF_R {
        DLVD2IF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RTC Interval Interrupt Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn drtciif(&self) -> DRTCIIF_R {
        DRTCIIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTC Alarm Interrupt Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn drtcaif(&self) -> DRTCAIF_R {
        DRTCAIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NMI Pin Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dnmif(&self) -> DNMIF_R {
        DNMIF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LVD1 Deep Standby Cancel Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dlvd1if(&mut self) -> DLVD1IF_W<0> {
        DLVD1IF_W::new(self)
    }
    #[doc = "Bit 1 - LVD2 Deep Standby Cancel Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dlvd2if(&mut self) -> DLVD2IF_W<1> {
        DLVD2IF_W::new(self)
    }
    #[doc = "Bit 2 - RTC Interval Interrupt Deep Standby Cancel Flag"]
    #[inline(always)]
    #[must_use]
    pub fn drtciif(&mut self) -> DRTCIIF_W<2> {
        DRTCIIF_W::new(self)
    }
    #[doc = "Bit 3 - RTC Alarm Interrupt Deep Standby Cancel Flag"]
    #[inline(always)]
    #[must_use]
    pub fn drtcaif(&mut self) -> DRTCAIF_W<3> {
        DRTCAIF_W::new(self)
    }
    #[doc = "Bit 4 - NMI Pin Deep Standby Cancel Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dnmif(&mut self) -> DNMIF_W<4> {
        DNMIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Deep Standby Interrupt Flag Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpsifr2](index.html) module"]
pub struct DPSIFR2_SPEC;
impl crate::RegisterSpec for DPSIFR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dpsifr2::R](R) reader structure"]
impl crate::Readable for DPSIFR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dpsifr2::W](W) writer structure"]
impl crate::Writable for DPSIFR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPSIFR2 to value 0"]
impl crate::Resettable for DPSIFR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
