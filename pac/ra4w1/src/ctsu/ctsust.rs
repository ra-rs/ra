#[doc = "Register `CTSUST` reader"]
pub struct R(crate::R<CTSUST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTSUST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTSUST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTSUST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTSUST` writer"]
pub struct W(crate::W<CTSUST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTSUST_SPEC>;
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
impl From<crate::W<CTSUST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTSUST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTSUSTC` reader - CTSU Measurement Status Counter"]
pub type CTSUSTC_R = crate::FieldReader<u8, CTSUSTC_A>;
#[doc = "CTSU Measurement Status Counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTSUSTC_A {
    #[doc = "0: Status 0"]
    _000 = 0,
    #[doc = "1: Status 1"]
    _001 = 1,
    #[doc = "2: Status 2"]
    _010 = 2,
    #[doc = "3: Status 3"]
    _011 = 3,
    #[doc = "4: Status 4"]
    _100 = 4,
    #[doc = "5: Status 5"]
    _101 = 5,
}
impl From<CTSUSTC_A> for u8 {
    #[inline(always)]
    fn from(variant: CTSUSTC_A) -> Self {
        variant as _
    }
}
impl CTSUSTC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CTSUSTC_A> {
        match self.bits {
            0 => Some(CTSUSTC_A::_000),
            1 => Some(CTSUSTC_A::_001),
            2 => Some(CTSUSTC_A::_010),
            3 => Some(CTSUSTC_A::_011),
            4 => Some(CTSUSTC_A::_100),
            5 => Some(CTSUSTC_A::_101),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CTSUSTC_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == CTSUSTC_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == CTSUSTC_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == CTSUSTC_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == CTSUSTC_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == CTSUSTC_A::_101
    }
}
#[doc = "Field `CTSUSTC` writer - CTSU Measurement Status Counter"]
pub type CTSUSTC_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CTSUST_SPEC, u8, CTSUSTC_A, 3, O>;
impl<'a, const O: u8> CTSUSTC_W<'a, O> {
    #[doc = "Status 0"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(CTSUSTC_A::_000)
    }
    #[doc = "Status 1"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(CTSUSTC_A::_001)
    }
    #[doc = "Status 2"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(CTSUSTC_A::_010)
    }
    #[doc = "Status 3"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(CTSUSTC_A::_011)
    }
    #[doc = "Status 4"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(CTSUSTC_A::_100)
    }
    #[doc = "Status 5"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(CTSUSTC_A::_101)
    }
}
#[doc = "Field `CTSUDTSR` reader - CTSU Data Transfer Status Flag"]
pub type CTSUDTSR_R = crate::BitReader<CTSUDTSR_A>;
#[doc = "CTSU Data Transfer Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSUDTSR_A {
    #[doc = "0: Measurement result has been read"]
    _0 = 0,
    #[doc = "1: Measurement result has not been read"]
    _1 = 1,
}
impl From<CTSUDTSR_A> for bool {
    #[inline(always)]
    fn from(variant: CTSUDTSR_A) -> Self {
        variant as u8 != 0
    }
}
impl CTSUDTSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSUDTSR_A {
        match self.bits {
            false => CTSUDTSR_A::_0,
            true => CTSUDTSR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUDTSR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUDTSR_A::_1
    }
}
#[doc = "Field `CTSUSOVF` reader - CTSU Sensor Counter Overflow Flag"]
pub type CTSUSOVF_R = crate::BitReader<CTSUSOVF_A>;
#[doc = "CTSU Sensor Counter Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSUSOVF_A {
    #[doc = "0: No overflow"]
    _0 = 0,
    #[doc = "1: An overflow"]
    _1 = 1,
}
impl From<CTSUSOVF_A> for bool {
    #[inline(always)]
    fn from(variant: CTSUSOVF_A) -> Self {
        variant as u8 != 0
    }
}
impl CTSUSOVF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSUSOVF_A {
        match self.bits {
            false => CTSUSOVF_A::_0,
            true => CTSUSOVF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUSOVF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUSOVF_A::_1
    }
}
#[doc = "Field `CTSUSOVF` writer - CTSU Sensor Counter Overflow Flag"]
pub type CTSUSOVF_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTSUST_SPEC, CTSUSOVF_A, O>;
impl<'a, const O: u8> CTSUSOVF_W<'a, O> {
    #[doc = "No overflow"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTSUSOVF_A::_0)
    }
    #[doc = "An overflow"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTSUSOVF_A::_1)
    }
}
#[doc = "Field `CTSUROVF` reader - CTSU Reference Counter Overflow Flag"]
pub type CTSUROVF_R = crate::BitReader<CTSUROVF_A>;
#[doc = "CTSU Reference Counter Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSUROVF_A {
    #[doc = "0: No overflow"]
    _0 = 0,
    #[doc = "1: An overflow"]
    _1 = 1,
}
impl From<CTSUROVF_A> for bool {
    #[inline(always)]
    fn from(variant: CTSUROVF_A) -> Self {
        variant as u8 != 0
    }
}
impl CTSUROVF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSUROVF_A {
        match self.bits {
            false => CTSUROVF_A::_0,
            true => CTSUROVF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUROVF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUROVF_A::_1
    }
}
#[doc = "Field `CTSUROVF` writer - CTSU Reference Counter Overflow Flag"]
pub type CTSUROVF_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTSUST_SPEC, CTSUROVF_A, O>;
impl<'a, const O: u8> CTSUROVF_W<'a, O> {
    #[doc = "No overflow"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTSUROVF_A::_0)
    }
    #[doc = "An overflow"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTSUROVF_A::_1)
    }
}
#[doc = "Field `CTSUPS` reader - CTSU Mutual Capacitance Status Flag"]
pub type CTSUPS_R = crate::BitReader<CTSUPS_A>;
#[doc = "CTSU Mutual Capacitance Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSUPS_A {
    #[doc = "0: First measurement"]
    _0 = 0,
    #[doc = "1: Second measurement"]
    _1 = 1,
}
impl From<CTSUPS_A> for bool {
    #[inline(always)]
    fn from(variant: CTSUPS_A) -> Self {
        variant as u8 != 0
    }
}
impl CTSUPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSUPS_A {
        match self.bits {
            false => CTSUPS_A::_0,
            true => CTSUPS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUPS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUPS_A::_1
    }
}
impl R {
    #[doc = "Bits 0:2 - CTSU Measurement Status Counter"]
    #[inline(always)]
    pub fn ctsustc(&self) -> CTSUSTC_R {
        CTSUSTC_R::new(self.bits & 7)
    }
    #[doc = "Bit 4 - CTSU Data Transfer Status Flag"]
    #[inline(always)]
    pub fn ctsudtsr(&self) -> CTSUDTSR_R {
        CTSUDTSR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CTSU Sensor Counter Overflow Flag"]
    #[inline(always)]
    pub fn ctsusovf(&self) -> CTSUSOVF_R {
        CTSUSOVF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CTSU Reference Counter Overflow Flag"]
    #[inline(always)]
    pub fn ctsurovf(&self) -> CTSUROVF_R {
        CTSUROVF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CTSU Mutual Capacitance Status Flag"]
    #[inline(always)]
    pub fn ctsups(&self) -> CTSUPS_R {
        CTSUPS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - CTSU Measurement Status Counter"]
    #[inline(always)]
    #[must_use]
    pub fn ctsustc(&mut self) -> CTSUSTC_W<0> {
        CTSUSTC_W::new(self)
    }
    #[doc = "Bit 5 - CTSU Sensor Counter Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctsusovf(&mut self) -> CTSUSOVF_W<5> {
        CTSUSOVF_W::new(self)
    }
    #[doc = "Bit 6 - CTSU Reference Counter Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctsurovf(&mut self) -> CTSUROVF_W<6> {
        CTSUROVF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTSU Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctsust](index.html) module"]
pub struct CTSUST_SPEC;
impl crate::RegisterSpec for CTSUST_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctsust::R](R) reader structure"]
impl crate::Readable for CTSUST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctsust::W](W) writer structure"]
impl crate::Writable for CTSUST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTSUST to value 0"]
impl crate::Resettable for CTSUST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
