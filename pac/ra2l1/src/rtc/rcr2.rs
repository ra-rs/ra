#[doc = "Register `RCR2` reader"]
pub struct R(crate::R<RCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCR2` writer"]
pub struct W(crate::W<RCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCR2_SPEC>;
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
impl From<crate::W<RCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` reader - Start"]
pub type START_R = crate::BitReader<START_A>;
#[doc = "Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum START_A {
    #[doc = "0: Stop prescaler and time counter"]
    _0 = 0,
    #[doc = "1: Operate prescaler and time counter normally"]
    _1 = 1,
}
impl From<START_A> for bool {
    #[inline(always)]
    fn from(variant: START_A) -> Self {
        variant as u8 != 0
    }
}
impl START_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> START_A {
        match self.bits {
            false => START_A::_0,
            true => START_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == START_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == START_A::_1
    }
}
#[doc = "Field `START` writer - Start"]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u8, RCR2_SPEC, START_A, O>;
impl<'a, const O: u8> START_W<'a, O> {
    #[doc = "Stop prescaler and time counter"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(START_A::_0)
    }
    #[doc = "Operate prescaler and time counter normally"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(START_A::_1)
    }
}
#[doc = "Field `RESET` reader - RTC Software Reset"]
pub type RESET_R = crate::BitReader<RESET_A>;
#[doc = "RTC Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESET_A {
    #[doc = "0: In writing: Invalid (writing 0 has no effect). In reading: Normal time operation in progress, or an RTC software reset has completed."]
    _0 = 0,
    #[doc = "1: In writing: Initialize the prescaler and target registers for RTC software reset. In reading: RTC software reset in progress."]
    _1 = 1,
}
impl From<RESET_A> for bool {
    #[inline(always)]
    fn from(variant: RESET_A) -> Self {
        variant as u8 != 0
    }
}
impl RESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESET_A {
        match self.bits {
            false => RESET_A::_0,
            true => RESET_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RESET_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RESET_A::_1
    }
}
#[doc = "Field `RESET` writer - RTC Software Reset"]
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, u8, RCR2_SPEC, RESET_A, O>;
impl<'a, const O: u8> RESET_W<'a, O> {
    #[doc = "In writing: Invalid (writing 0 has no effect). In reading: Normal time operation in progress, or an RTC software reset has completed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RESET_A::_0)
    }
    #[doc = "In writing: Initialize the prescaler and target registers for RTC software reset. In reading: RTC software reset in progress."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RESET_A::_1)
    }
}
#[doc = "Field `ADJ30` reader - 30-Second Adjustment"]
pub type ADJ30_R = crate::BitReader<ADJ30_A>;
#[doc = "30-Second Adjustment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADJ30_A {
    #[doc = "0: In writing: Invalid (writing 0 has no effect). In reading: Normal time operation in progress, or 30-second adjustment has completed."]
    _0 = 0,
    #[doc = "1: In writing: Execute 30-second adjustment. In reading: 30-second adjustment in progress."]
    _1 = 1,
}
impl From<ADJ30_A> for bool {
    #[inline(always)]
    fn from(variant: ADJ30_A) -> Self {
        variant as u8 != 0
    }
}
impl ADJ30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADJ30_A {
        match self.bits {
            false => ADJ30_A::_0,
            true => ADJ30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADJ30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADJ30_A::_1
    }
}
#[doc = "Field `ADJ30` writer - 30-Second Adjustment"]
pub type ADJ30_W<'a, const O: u8> = crate::BitWriter<'a, u8, RCR2_SPEC, ADJ30_A, O>;
impl<'a, const O: u8> ADJ30_W<'a, O> {
    #[doc = "In writing: Invalid (writing 0 has no effect). In reading: Normal time operation in progress, or 30-second adjustment has completed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADJ30_A::_0)
    }
    #[doc = "In writing: Execute 30-second adjustment. In reading: 30-second adjustment in progress."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADJ30_A::_1)
    }
}
#[doc = "Field `RTCOE` reader - RTCOUT Output Enable"]
pub type RTCOE_R = crate::BitReader<RTCOE_A>;
#[doc = "RTCOUT Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCOE_A {
    #[doc = "0: Disable RTCOUT output"]
    _0 = 0,
    #[doc = "1: Enable RTCOUT output"]
    _1 = 1,
}
impl From<RTCOE_A> for bool {
    #[inline(always)]
    fn from(variant: RTCOE_A) -> Self {
        variant as u8 != 0
    }
}
impl RTCOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCOE_A {
        match self.bits {
            false => RTCOE_A::_0,
            true => RTCOE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RTCOE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTCOE_A::_1
    }
}
#[doc = "Field `RTCOE` writer - RTCOUT Output Enable"]
pub type RTCOE_W<'a, const O: u8> = crate::BitWriter<'a, u8, RCR2_SPEC, RTCOE_A, O>;
impl<'a, const O: u8> RTCOE_W<'a, O> {
    #[doc = "Disable RTCOUT output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTCOE_A::_0)
    }
    #[doc = "Enable RTCOUT output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTCOE_A::_1)
    }
}
#[doc = "Field `AADJE` reader - Automatic Adjustment Enable"]
pub type AADJE_R = crate::BitReader<AADJE_A>;
#[doc = "Automatic Adjustment Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AADJE_A {
    #[doc = "0: Disable automatic adjustment"]
    _0 = 0,
    #[doc = "1: Enable automatic adjustment"]
    _1 = 1,
}
impl From<AADJE_A> for bool {
    #[inline(always)]
    fn from(variant: AADJE_A) -> Self {
        variant as u8 != 0
    }
}
impl AADJE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AADJE_A {
        match self.bits {
            false => AADJE_A::_0,
            true => AADJE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AADJE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AADJE_A::_1
    }
}
#[doc = "Field `AADJE` writer - Automatic Adjustment Enable"]
pub type AADJE_W<'a, const O: u8> = crate::BitWriter<'a, u8, RCR2_SPEC, AADJE_A, O>;
impl<'a, const O: u8> AADJE_W<'a, O> {
    #[doc = "Disable automatic adjustment"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AADJE_A::_0)
    }
    #[doc = "Enable automatic adjustment"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AADJE_A::_1)
    }
}
#[doc = "Field `AADJP` reader - Automatic Adjustment Period Select"]
pub type AADJP_R = crate::BitReader<AADJP_A>;
#[doc = "Automatic Adjustment Period Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AADJP_A {
    #[doc = "0: In normal operation mode, adjust RADJ.ADJ\\[5:0\\]
setting from the count value of the prescaler every minute. In low-consumption clock mode, adjust RADJ.ADJ\\[5:0\\]
setting from the count value of the 64-Hz counter every day."]
    _0 = 0,
    #[doc = "1: In normal operation mode, adjust RADJ.ADJ\\[5:0\\]
setting from the count value of the prescaler every 10 seconds. In low-consumption clock mode, adjust RADJ.ADJ\\[5:0\\]
setting from the count value of the 64-Hz counter every hour."]
    _1 = 1,
}
impl From<AADJP_A> for bool {
    #[inline(always)]
    fn from(variant: AADJP_A) -> Self {
        variant as u8 != 0
    }
}
impl AADJP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AADJP_A {
        match self.bits {
            false => AADJP_A::_0,
            true => AADJP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AADJP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AADJP_A::_1
    }
}
#[doc = "Field `AADJP` writer - Automatic Adjustment Period Select"]
pub type AADJP_W<'a, const O: u8> = crate::BitWriter<'a, u8, RCR2_SPEC, AADJP_A, O>;
impl<'a, const O: u8> AADJP_W<'a, O> {
    #[doc = "In normal operation mode, adjust RADJ.ADJ\\[5:0\\]
setting from the count value of the prescaler every minute. In low-consumption clock mode, adjust RADJ.ADJ\\[5:0\\]
setting from the count value of the 64-Hz counter every day."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AADJP_A::_0)
    }
    #[doc = "In normal operation mode, adjust RADJ.ADJ\\[5:0\\]
setting from the count value of the prescaler every 10 seconds. In low-consumption clock mode, adjust RADJ.ADJ\\[5:0\\]
setting from the count value of the 64-Hz counter every hour."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AADJP_A::_1)
    }
}
#[doc = "Field `HR24` reader - Hours Mode"]
pub type HR24_R = crate::BitReader<HR24_A>;
#[doc = "Hours Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HR24_A {
    #[doc = "0: Operate RTC in 12-hour mode"]
    _0 = 0,
    #[doc = "1: Operate RTC in 24-hour mode"]
    _1 = 1,
}
impl From<HR24_A> for bool {
    #[inline(always)]
    fn from(variant: HR24_A) -> Self {
        variant as u8 != 0
    }
}
impl HR24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HR24_A {
        match self.bits {
            false => HR24_A::_0,
            true => HR24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HR24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HR24_A::_1
    }
}
#[doc = "Field `HR24` writer - Hours Mode"]
pub type HR24_W<'a, const O: u8> = crate::BitWriter<'a, u8, RCR2_SPEC, HR24_A, O>;
impl<'a, const O: u8> HR24_W<'a, O> {
    #[doc = "Operate RTC in 12-hour mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HR24_A::_0)
    }
    #[doc = "Operate RTC in 24-hour mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HR24_A::_1)
    }
}
#[doc = "Field `CNTMD` reader - Count Mode Select"]
pub type CNTMD_R = crate::BitReader<CNTMD_A>;
#[doc = "Count Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CNTMD_A {
    #[doc = "0: Calendar count mode"]
    _0 = 0,
    #[doc = "1: Binary count mode"]
    _1 = 1,
}
impl From<CNTMD_A> for bool {
    #[inline(always)]
    fn from(variant: CNTMD_A) -> Self {
        variant as u8 != 0
    }
}
impl CNTMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTMD_A {
        match self.bits {
            false => CNTMD_A::_0,
            true => CNTMD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CNTMD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CNTMD_A::_1
    }
}
#[doc = "Field `CNTMD` writer - Count Mode Select"]
pub type CNTMD_W<'a, const O: u8> = crate::BitWriter<'a, u8, RCR2_SPEC, CNTMD_A, O>;
impl<'a, const O: u8> CNTMD_W<'a, O> {
    #[doc = "Calendar count mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNTMD_A::_0)
    }
    #[doc = "Binary count mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CNTMD_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Start"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTC Software Reset"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 30-Second Adjustment"]
    #[inline(always)]
    pub fn adj30(&self) -> ADJ30_R {
        ADJ30_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTCOUT Output Enable"]
    #[inline(always)]
    pub fn rtcoe(&self) -> RTCOE_R {
        RTCOE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Automatic Adjustment Enable"]
    #[inline(always)]
    pub fn aadje(&self) -> AADJE_R {
        AADJE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Automatic Adjustment Period Select"]
    #[inline(always)]
    pub fn aadjp(&self) -> AADJP_R {
        AADJP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Hours Mode"]
    #[inline(always)]
    pub fn hr24(&self) -> HR24_R {
        HR24_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Count Mode Select"]
    #[inline(always)]
    pub fn cntmd(&self) -> CNTMD_R {
        CNTMD_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<0> {
        START_W::new(self)
    }
    #[doc = "Bit 1 - RTC Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<1> {
        RESET_W::new(self)
    }
    #[doc = "Bit 2 - 30-Second Adjustment"]
    #[inline(always)]
    #[must_use]
    pub fn adj30(&mut self) -> ADJ30_W<2> {
        ADJ30_W::new(self)
    }
    #[doc = "Bit 3 - RTCOUT Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtcoe(&mut self) -> RTCOE_W<3> {
        RTCOE_W::new(self)
    }
    #[doc = "Bit 4 - Automatic Adjustment Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aadje(&mut self) -> AADJE_W<4> {
        AADJE_W::new(self)
    }
    #[doc = "Bit 5 - Automatic Adjustment Period Select"]
    #[inline(always)]
    #[must_use]
    pub fn aadjp(&mut self) -> AADJP_W<5> {
        AADJP_W::new(self)
    }
    #[doc = "Bit 6 - Hours Mode"]
    #[inline(always)]
    #[must_use]
    pub fn hr24(&mut self) -> HR24_W<6> {
        HR24_W::new(self)
    }
    #[doc = "Bit 7 - Count Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn cntmd(&mut self) -> CNTMD_W<7> {
        CNTMD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Control Register 2 (in Calendar Count Mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcr2](index.html) module"]
pub struct RCR2_SPEC;
impl crate::RegisterSpec for RCR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rcr2::R](R) reader structure"]
impl crate::Readable for RCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcr2::W](W) writer structure"]
impl crate::Writable for RCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCR2 to value 0"]
impl crate::Resettable for RCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
