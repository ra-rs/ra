#[doc = "Register `DAM` reader"]
pub struct R(crate::R<DAM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAM` writer"]
pub struct W(crate::W<DAM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAM_SPEC>;
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
impl From<crate::W<DAM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAMD0` reader - D/A operation mode select 0"]
pub type DAMD0_R = crate::BitReader<DAMD0_A>;
#[doc = "D/A operation mode select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAMD0_A {
    #[doc = "0: Channel 0 for normal operation mode"]
    _0 = 0,
    #[doc = "1: Channel 0 for real-time output mode(event link)"]
    _1 = 1,
}
impl From<DAMD0_A> for bool {
    #[inline(always)]
    fn from(variant: DAMD0_A) -> Self {
        variant as u8 != 0
    }
}
impl DAMD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAMD0_A {
        match self.bits {
            false => DAMD0_A::_0,
            true => DAMD0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DAMD0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DAMD0_A::_1
    }
}
#[doc = "Field `DAMD0` writer - D/A operation mode select 0"]
pub type DAMD0_W<'a, const O: u8> = crate::BitWriter<'a, u8, DAM_SPEC, DAMD0_A, O>;
impl<'a, const O: u8> DAMD0_W<'a, O> {
    #[doc = "Channel 0 for normal operation mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DAMD0_A::_0)
    }
    #[doc = "Channel 0 for real-time output mode(event link)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DAMD0_A::_1)
    }
}
#[doc = "Field `DAMD1` reader - D/A operation mode select 1"]
pub type DAMD1_R = crate::BitReader<DAMD1_A>;
#[doc = "D/A operation mode select 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAMD1_A {
    #[doc = "0: Channel 1 for normal operation mode"]
    _0 = 0,
    #[doc = "1: Channel 1 for real-time output mode(event link)"]
    _1 = 1,
}
impl From<DAMD1_A> for bool {
    #[inline(always)]
    fn from(variant: DAMD1_A) -> Self {
        variant as u8 != 0
    }
}
impl DAMD1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAMD1_A {
        match self.bits {
            false => DAMD1_A::_0,
            true => DAMD1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DAMD1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DAMD1_A::_1
    }
}
#[doc = "Field `DAMD1` writer - D/A operation mode select 1"]
pub type DAMD1_W<'a, const O: u8> = crate::BitWriter<'a, u8, DAM_SPEC, DAMD1_A, O>;
impl<'a, const O: u8> DAMD1_W<'a, O> {
    #[doc = "Channel 1 for normal operation mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DAMD1_A::_0)
    }
    #[doc = "Channel 1 for real-time output mode(event link)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DAMD1_A::_1)
    }
}
#[doc = "Field `DACE0` reader - D/A operation enable 0"]
pub type DACE0_R = crate::BitReader<DACE0_A>;
#[doc = "D/A operation enable 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DACE0_A {
    #[doc = "0: D/A conversion disabled for channel 0"]
    _0 = 0,
    #[doc = "1: D/A conversion enabled for channel 0"]
    _1 = 1,
}
impl From<DACE0_A> for bool {
    #[inline(always)]
    fn from(variant: DACE0_A) -> Self {
        variant as u8 != 0
    }
}
impl DACE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACE0_A {
        match self.bits {
            false => DACE0_A::_0,
            true => DACE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DACE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DACE0_A::_1
    }
}
#[doc = "Field `DACE0` writer - D/A operation enable 0"]
pub type DACE0_W<'a, const O: u8> = crate::BitWriter<'a, u8, DAM_SPEC, DACE0_A, O>;
impl<'a, const O: u8> DACE0_W<'a, O> {
    #[doc = "D/A conversion disabled for channel 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACE0_A::_0)
    }
    #[doc = "D/A conversion enabled for channel 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACE0_A::_1)
    }
}
#[doc = "Field `DACE1` reader - D/A operation enable 1"]
pub type DACE1_R = crate::BitReader<DACE1_A>;
#[doc = "D/A operation enable 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DACE1_A {
    #[doc = "0: D/A conversion disabled for channel 1"]
    _0 = 0,
    #[doc = "1: D/A conversion enabled for channel 1"]
    _1 = 1,
}
impl From<DACE1_A> for bool {
    #[inline(always)]
    fn from(variant: DACE1_A) -> Self {
        variant as u8 != 0
    }
}
impl DACE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACE1_A {
        match self.bits {
            false => DACE1_A::_0,
            true => DACE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DACE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DACE1_A::_1
    }
}
#[doc = "Field `DACE1` writer - D/A operation enable 1"]
pub type DACE1_W<'a, const O: u8> = crate::BitWriter<'a, u8, DAM_SPEC, DACE1_A, O>;
impl<'a, const O: u8> DACE1_W<'a, O> {
    #[doc = "D/A conversion disabled for channel 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACE1_A::_0)
    }
    #[doc = "D/A conversion enabled for channel 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACE1_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - D/A operation mode select 0"]
    #[inline(always)]
    pub fn damd0(&self) -> DAMD0_R {
        DAMD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - D/A operation mode select 1"]
    #[inline(always)]
    pub fn damd1(&self) -> DAMD1_R {
        DAMD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - D/A operation enable 0"]
    #[inline(always)]
    pub fn dace0(&self) -> DACE0_R {
        DACE0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - D/A operation enable 1"]
    #[inline(always)]
    pub fn dace1(&self) -> DACE1_R {
        DACE1_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - D/A operation mode select 0"]
    #[inline(always)]
    #[must_use]
    pub fn damd0(&mut self) -> DAMD0_W<0> {
        DAMD0_W::new(self)
    }
    #[doc = "Bit 1 - D/A operation mode select 1"]
    #[inline(always)]
    #[must_use]
    pub fn damd1(&mut self) -> DAMD1_W<1> {
        DAMD1_W::new(self)
    }
    #[doc = "Bit 4 - D/A operation enable 0"]
    #[inline(always)]
    #[must_use]
    pub fn dace0(&mut self) -> DACE0_W<4> {
        DACE0_W::new(self)
    }
    #[doc = "Bit 5 - D/A operation enable 1"]
    #[inline(always)]
    #[must_use]
    pub fn dace1(&mut self) -> DACE1_W<5> {
        DACE1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "D/A Converter Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dam](index.html) module"]
pub struct DAM_SPEC;
impl crate::RegisterSpec for DAM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dam::R](R) reader structure"]
impl crate::Readable for DAM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dam::W](W) writer structure"]
impl crate::Writable for DAM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAM to value 0"]
impl crate::Resettable for DAM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
