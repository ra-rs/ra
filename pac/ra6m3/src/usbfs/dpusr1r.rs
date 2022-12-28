#[doc = "Register `DPUSR1R` reader"]
pub struct R(crate::R<DPUSR1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPUSR1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPUSR1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPUSR1R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPUSR1R` writer"]
pub struct W(crate::W<DPUSR1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPUSR1R_SPEC>;
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
impl From<crate::W<DPUSR1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPUSR1R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DPINTE0` reader - USB DP Interrupt Enable/Clear"]
pub type DPINTE0_R = crate::BitReader<DPINTE0_A>;
#[doc = "USB DP Interrupt Enable/Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPINTE0_A {
    #[doc = "0: Recovery from deep software standby mode is disabled."]
    _0 = 0,
    #[doc = "1: Recovery from deep software standby mode is enabled."]
    _1 = 1,
}
impl From<DPINTE0_A> for bool {
    #[inline(always)]
    fn from(variant: DPINTE0_A) -> Self {
        variant as u8 != 0
    }
}
impl DPINTE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPINTE0_A {
        match self.bits {
            false => DPINTE0_A::_0,
            true => DPINTE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPINTE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPINTE0_A::_1
    }
}
#[doc = "Field `DPINTE0` writer - USB DP Interrupt Enable/Clear"]
pub type DPINTE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPUSR1R_SPEC, DPINTE0_A, O>;
impl<'a, const O: u8> DPINTE0_W<'a, O> {
    #[doc = "Recovery from deep software standby mode is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPINTE0_A::_0)
    }
    #[doc = "Recovery from deep software standby mode is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPINTE0_A::_1)
    }
}
#[doc = "Field `DMINTE0` reader - USB DM Interrupt Enable/Clear"]
pub type DMINTE0_R = crate::BitReader<DMINTE0_A>;
#[doc = "USB DM Interrupt Enable/Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMINTE0_A {
    #[doc = "0: Recovery from deep software standby mode is disabled."]
    _0 = 0,
    #[doc = "1: Recovery from deep software standby mode is enabled."]
    _1 = 1,
}
impl From<DMINTE0_A> for bool {
    #[inline(always)]
    fn from(variant: DMINTE0_A) -> Self {
        variant as u8 != 0
    }
}
impl DMINTE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMINTE0_A {
        match self.bits {
            false => DMINTE0_A::_0,
            true => DMINTE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMINTE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMINTE0_A::_1
    }
}
#[doc = "Field `DMINTE0` writer - USB DM Interrupt Enable/Clear"]
pub type DMINTE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPUSR1R_SPEC, DMINTE0_A, O>;
impl<'a, const O: u8> DMINTE0_W<'a, O> {
    #[doc = "Recovery from deep software standby mode is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMINTE0_A::_0)
    }
    #[doc = "Recovery from deep software standby mode is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMINTE0_A::_1)
    }
}
#[doc = "Field `DOVRCRAE0` reader - USB OVRCURA Interrupt Enable/Clear"]
pub type DOVRCRAE0_R = crate::BitReader<DOVRCRAE0_A>;
#[doc = "USB OVRCURA Interrupt Enable/Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOVRCRAE0_A {
    #[doc = "0: Recovery from deep software standby mode is disabled."]
    _0 = 0,
    #[doc = "1: Recovery from deep software standby mode is enabled."]
    _1 = 1,
}
impl From<DOVRCRAE0_A> for bool {
    #[inline(always)]
    fn from(variant: DOVRCRAE0_A) -> Self {
        variant as u8 != 0
    }
}
impl DOVRCRAE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOVRCRAE0_A {
        match self.bits {
            false => DOVRCRAE0_A::_0,
            true => DOVRCRAE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DOVRCRAE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DOVRCRAE0_A::_1
    }
}
#[doc = "Field `DOVRCRAE0` writer - USB OVRCURA Interrupt Enable/Clear"]
pub type DOVRCRAE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPUSR1R_SPEC, DOVRCRAE0_A, O>;
impl<'a, const O: u8> DOVRCRAE0_W<'a, O> {
    #[doc = "Recovery from deep software standby mode is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOVRCRAE0_A::_0)
    }
    #[doc = "Recovery from deep software standby mode is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOVRCRAE0_A::_1)
    }
}
#[doc = "Field `DOVRCRBE0` reader - USB OVRCURB Interrupt Enable/Clear"]
pub type DOVRCRBE0_R = crate::BitReader<DOVRCRBE0_A>;
#[doc = "USB OVRCURB Interrupt Enable/Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOVRCRBE0_A {
    #[doc = "0: Recovery from deep software standby mode is disabled."]
    _0 = 0,
    #[doc = "1: Recovery from deep software standby mode is enabled."]
    _1 = 1,
}
impl From<DOVRCRBE0_A> for bool {
    #[inline(always)]
    fn from(variant: DOVRCRBE0_A) -> Self {
        variant as u8 != 0
    }
}
impl DOVRCRBE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOVRCRBE0_A {
        match self.bits {
            false => DOVRCRBE0_A::_0,
            true => DOVRCRBE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DOVRCRBE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DOVRCRBE0_A::_1
    }
}
#[doc = "Field `DOVRCRBE0` writer - USB OVRCURB Interrupt Enable/Clear"]
pub type DOVRCRBE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPUSR1R_SPEC, DOVRCRBE0_A, O>;
impl<'a, const O: u8> DOVRCRBE0_W<'a, O> {
    #[doc = "Recovery from deep software standby mode is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOVRCRBE0_A::_0)
    }
    #[doc = "Recovery from deep software standby mode is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOVRCRBE0_A::_1)
    }
}
#[doc = "Field `DVBSE0` reader - USB VBUS Interrupt Enable/Clear"]
pub type DVBSE0_R = crate::BitReader<DVBSE0_A>;
#[doc = "USB VBUS Interrupt Enable/Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DVBSE0_A {
    #[doc = "0: Recovery from deep software standby mode is disabled."]
    _0 = 0,
    #[doc = "1: Recovery from deep software standby mode is enabled."]
    _1 = 1,
}
impl From<DVBSE0_A> for bool {
    #[inline(always)]
    fn from(variant: DVBSE0_A) -> Self {
        variant as u8 != 0
    }
}
impl DVBSE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DVBSE0_A {
        match self.bits {
            false => DVBSE0_A::_0,
            true => DVBSE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DVBSE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DVBSE0_A::_1
    }
}
#[doc = "Field `DVBSE0` writer - USB VBUS Interrupt Enable/Clear"]
pub type DVBSE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPUSR1R_SPEC, DVBSE0_A, O>;
impl<'a, const O: u8> DVBSE0_W<'a, O> {
    #[doc = "Recovery from deep software standby mode is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DVBSE0_A::_0)
    }
    #[doc = "Recovery from deep software standby mode is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DVBSE0_A::_1)
    }
}
#[doc = "Field `DPINT0` reader - USB DP Interrupt Source Recovery"]
pub type DPINT0_R = crate::BitReader<DPINT0_A>;
#[doc = "USB DP Interrupt Source Recovery\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPINT0_A {
    #[doc = "0: The system has not returned from deep software standby mode."]
    _0 = 0,
    #[doc = "1: The system has returned from deep software standby mode."]
    _1 = 1,
}
impl From<DPINT0_A> for bool {
    #[inline(always)]
    fn from(variant: DPINT0_A) -> Self {
        variant as u8 != 0
    }
}
impl DPINT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPINT0_A {
        match self.bits {
            false => DPINT0_A::_0,
            true => DPINT0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPINT0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPINT0_A::_1
    }
}
#[doc = "Field `DMINT0` reader - USB DM Interrupt Source Recovery"]
pub type DMINT0_R = crate::BitReader<DMINT0_A>;
#[doc = "USB DM Interrupt Source Recovery\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMINT0_A {
    #[doc = "0: The system has not returned from deep software standby mode."]
    _0 = 0,
    #[doc = "1: The system has returned from deep software standby mode."]
    _1 = 1,
}
impl From<DMINT0_A> for bool {
    #[inline(always)]
    fn from(variant: DMINT0_A) -> Self {
        variant as u8 != 0
    }
}
impl DMINT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMINT0_A {
        match self.bits {
            false => DMINT0_A::_0,
            true => DMINT0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMINT0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMINT0_A::_1
    }
}
#[doc = "Field `DOVRCRA0` reader - USB OVRCURA Interrupt Source Recovery"]
pub type DOVRCRA0_R = crate::BitReader<DOVRCRA0_A>;
#[doc = "USB OVRCURA Interrupt Source Recovery\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOVRCRA0_A {
    #[doc = "0: The system has not returned from deep software standby mode."]
    _0 = 0,
    #[doc = "1: The system has returned from deep software standby mode."]
    _1 = 1,
}
impl From<DOVRCRA0_A> for bool {
    #[inline(always)]
    fn from(variant: DOVRCRA0_A) -> Self {
        variant as u8 != 0
    }
}
impl DOVRCRA0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOVRCRA0_A {
        match self.bits {
            false => DOVRCRA0_A::_0,
            true => DOVRCRA0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DOVRCRA0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DOVRCRA0_A::_1
    }
}
#[doc = "Field `DOVRCRB0` reader - USB OVRCURB Interrupt Source Recovery"]
pub type DOVRCRB0_R = crate::BitReader<DOVRCRB0_A>;
#[doc = "USB OVRCURB Interrupt Source Recovery\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOVRCRB0_A {
    #[doc = "0: The system has not returned from deep software standby mode."]
    _0 = 0,
    #[doc = "1: The system has returned from deep software standby mode."]
    _1 = 1,
}
impl From<DOVRCRB0_A> for bool {
    #[inline(always)]
    fn from(variant: DOVRCRB0_A) -> Self {
        variant as u8 != 0
    }
}
impl DOVRCRB0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOVRCRB0_A {
        match self.bits {
            false => DOVRCRB0_A::_0,
            true => DOVRCRB0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DOVRCRB0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DOVRCRB0_A::_1
    }
}
#[doc = "Field `DVBINT0` reader - USB VBUS Interrupt Source Recovery"]
pub type DVBINT0_R = crate::BitReader<DVBINT0_A>;
#[doc = "USB VBUS Interrupt Source Recovery\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DVBINT0_A {
    #[doc = "0: The system has not returned from deep software standby mode."]
    _0 = 0,
    #[doc = "1: The system has returned from deep software standby mode."]
    _1 = 1,
}
impl From<DVBINT0_A> for bool {
    #[inline(always)]
    fn from(variant: DVBINT0_A) -> Self {
        variant as u8 != 0
    }
}
impl DVBINT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DVBINT0_A {
        match self.bits {
            false => DVBINT0_A::_0,
            true => DVBINT0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DVBINT0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DVBINT0_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - USB DP Interrupt Enable/Clear"]
    #[inline(always)]
    pub fn dpinte0(&self) -> DPINTE0_R {
        DPINTE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB DM Interrupt Enable/Clear"]
    #[inline(always)]
    pub fn dminte0(&self) -> DMINTE0_R {
        DMINTE0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - USB OVRCURA Interrupt Enable/Clear"]
    #[inline(always)]
    pub fn dovrcrae0(&self) -> DOVRCRAE0_R {
        DOVRCRAE0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USB OVRCURB Interrupt Enable/Clear"]
    #[inline(always)]
    pub fn dovrcrbe0(&self) -> DOVRCRBE0_R {
        DOVRCRBE0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - USB VBUS Interrupt Enable/Clear"]
    #[inline(always)]
    pub fn dvbse0(&self) -> DVBSE0_R {
        DVBSE0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - USB DP Interrupt Source Recovery"]
    #[inline(always)]
    pub fn dpint0(&self) -> DPINT0_R {
        DPINT0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - USB DM Interrupt Source Recovery"]
    #[inline(always)]
    pub fn dmint0(&self) -> DMINT0_R {
        DMINT0_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - USB OVRCURA Interrupt Source Recovery"]
    #[inline(always)]
    pub fn dovrcra0(&self) -> DOVRCRA0_R {
        DOVRCRA0_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - USB OVRCURB Interrupt Source Recovery"]
    #[inline(always)]
    pub fn dovrcrb0(&self) -> DOVRCRB0_R {
        DOVRCRB0_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - USB VBUS Interrupt Source Recovery"]
    #[inline(always)]
    pub fn dvbint0(&self) -> DVBINT0_R {
        DVBINT0_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB DP Interrupt Enable/Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dpinte0(&mut self) -> DPINTE0_W<0> {
        DPINTE0_W::new(self)
    }
    #[doc = "Bit 1 - USB DM Interrupt Enable/Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dminte0(&mut self) -> DMINTE0_W<1> {
        DMINTE0_W::new(self)
    }
    #[doc = "Bit 4 - USB OVRCURA Interrupt Enable/Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dovrcrae0(&mut self) -> DOVRCRAE0_W<4> {
        DOVRCRAE0_W::new(self)
    }
    #[doc = "Bit 5 - USB OVRCURB Interrupt Enable/Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dovrcrbe0(&mut self) -> DOVRCRBE0_W<5> {
        DOVRCRBE0_W::new(self)
    }
    #[doc = "Bit 7 - USB VBUS Interrupt Enable/Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dvbse0(&mut self) -> DVBSE0_W<7> {
        DVBSE0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Deep Software Standby USB Suspend/Resume Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpusr1r](index.html) module"]
pub struct DPUSR1R_SPEC;
impl crate::RegisterSpec for DPUSR1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dpusr1r::R](R) reader structure"]
impl crate::Readable for DPUSR1R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dpusr1r::W](W) writer structure"]
impl crate::Writable for DPUSR1R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPUSR1R to value 0"]
impl crate::Resettable for DPUSR1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
