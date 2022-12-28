#[doc = "Register `MSTPCRC` reader"]
pub struct R(crate::R<MSTPCRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSTPCRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSTPCRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSTPCRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSTPCRC` writer"]
pub struct W(crate::W<MSTPCRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSTPCRC_SPEC>;
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
impl From<crate::W<MSTPCRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSTPCRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSTPC0` reader - Clock Frequency Accuracy Measurement Circuit Module Stop"]
pub type MSTPC0_R = crate::BitReader<MSTPC0_A>;
#[doc = "Clock Frequency Accuracy Measurement Circuit Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPC0_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPC0_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPC0_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPC0_A {
        match self.bits {
            false => MSTPC0_A::_0,
            true => MSTPC0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPC0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPC0_A::_1
    }
}
#[doc = "Field `MSTPC0` writer - Clock Frequency Accuracy Measurement Circuit Module Stop"]
pub type MSTPC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRC_SPEC, MSTPC0_A, O>;
impl<'a, const O: u8> MSTPC0_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPC0_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPC0_A::_1)
    }
}
#[doc = "Field `MSTPC1` reader - Cyclic Redundancy Check Calculator Module Stop"]
pub type MSTPC1_R = crate::BitReader<MSTPC1_A>;
#[doc = "Cyclic Redundancy Check Calculator Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPC1_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPC1_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPC1_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPC1_A {
        match self.bits {
            false => MSTPC1_A::_0,
            true => MSTPC1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPC1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPC1_A::_1
    }
}
#[doc = "Field `MSTPC1` writer - Cyclic Redundancy Check Calculator Module Stop"]
pub type MSTPC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRC_SPEC, MSTPC1_A, O>;
impl<'a, const O: u8> MSTPC1_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPC1_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPC1_A::_1)
    }
}
#[doc = "Field `MSTPC3` reader - Capacitive Sensing Unit 2 Module Stop"]
pub type MSTPC3_R = crate::BitReader<MSTPC3_A>;
#[doc = "Capacitive Sensing Unit 2 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPC3_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPC3_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPC3_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPC3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPC3_A {
        match self.bits {
            false => MSTPC3_A::_0,
            true => MSTPC3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPC3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPC3_A::_1
    }
}
#[doc = "Field `MSTPC3` writer - Capacitive Sensing Unit 2 Module Stop"]
pub type MSTPC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRC_SPEC, MSTPC3_A, O>;
impl<'a, const O: u8> MSTPC3_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPC3_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPC3_A::_1)
    }
}
#[doc = "Field `MSTPC13` reader - Data Operation Circuit Module Stop"]
pub type MSTPC13_R = crate::BitReader<MSTPC13_A>;
#[doc = "Data Operation Circuit Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPC13_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPC13_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPC13_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPC13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPC13_A {
        match self.bits {
            false => MSTPC13_A::_0,
            true => MSTPC13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPC13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPC13_A::_1
    }
}
#[doc = "Field `MSTPC13` writer - Data Operation Circuit Module Stop"]
pub type MSTPC13_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRC_SPEC, MSTPC13_A, O>;
impl<'a, const O: u8> MSTPC13_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPC13_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPC13_A::_1)
    }
}
#[doc = "Field `MSTPC14` reader - Event Link Controller Module Stop"]
pub type MSTPC14_R = crate::BitReader<MSTPC14_A>;
#[doc = "Event Link Controller Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPC14_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPC14_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPC14_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPC14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPC14_A {
        match self.bits {
            false => MSTPC14_A::_0,
            true => MSTPC14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPC14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPC14_A::_1
    }
}
#[doc = "Field `MSTPC14` writer - Event Link Controller Module Stop"]
pub type MSTPC14_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRC_SPEC, MSTPC14_A, O>;
impl<'a, const O: u8> MSTPC14_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPC14_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPC14_A::_1)
    }
}
#[doc = "Field `MSTPC28` reader - Random Number Generator Module Stop"]
pub type MSTPC28_R = crate::BitReader<MSTPC28_A>;
#[doc = "Random Number Generator Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPC28_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPC28_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPC28_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPC28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPC28_A {
        match self.bits {
            false => MSTPC28_A::_0,
            true => MSTPC28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPC28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPC28_A::_1
    }
}
#[doc = "Field `MSTPC28` writer - Random Number Generator Module Stop"]
pub type MSTPC28_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRC_SPEC, MSTPC28_A, O>;
impl<'a, const O: u8> MSTPC28_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPC28_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPC28_A::_1)
    }
}
#[doc = "Field `MSTPC31` reader - AES Module Stop"]
pub type MSTPC31_R = crate::BitReader<MSTPC31_A>;
#[doc = "AES Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPC31_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPC31_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPC31_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPC31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPC31_A {
        match self.bits {
            false => MSTPC31_A::_0,
            true => MSTPC31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPC31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPC31_A::_1
    }
}
#[doc = "Field `MSTPC31` writer - AES Module Stop"]
pub type MSTPC31_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRC_SPEC, MSTPC31_A, O>;
impl<'a, const O: u8> MSTPC31_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPC31_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPC31_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Clock Frequency Accuracy Measurement Circuit Module Stop"]
    #[inline(always)]
    pub fn mstpc0(&self) -> MSTPC0_R {
        MSTPC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Cyclic Redundancy Check Calculator Module Stop"]
    #[inline(always)]
    pub fn mstpc1(&self) -> MSTPC1_R {
        MSTPC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Capacitive Sensing Unit 2 Module Stop"]
    #[inline(always)]
    pub fn mstpc3(&self) -> MSTPC3_R {
        MSTPC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 13 - Data Operation Circuit Module Stop"]
    #[inline(always)]
    pub fn mstpc13(&self) -> MSTPC13_R {
        MSTPC13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Event Link Controller Module Stop"]
    #[inline(always)]
    pub fn mstpc14(&self) -> MSTPC14_R {
        MSTPC14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 28 - Random Number Generator Module Stop"]
    #[inline(always)]
    pub fn mstpc28(&self) -> MSTPC28_R {
        MSTPC28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - AES Module Stop"]
    #[inline(always)]
    pub fn mstpc31(&self) -> MSTPC31_R {
        MSTPC31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Frequency Accuracy Measurement Circuit Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpc0(&mut self) -> MSTPC0_W<0> {
        MSTPC0_W::new(self)
    }
    #[doc = "Bit 1 - Cyclic Redundancy Check Calculator Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpc1(&mut self) -> MSTPC1_W<1> {
        MSTPC1_W::new(self)
    }
    #[doc = "Bit 3 - Capacitive Sensing Unit 2 Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpc3(&mut self) -> MSTPC3_W<3> {
        MSTPC3_W::new(self)
    }
    #[doc = "Bit 13 - Data Operation Circuit Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpc13(&mut self) -> MSTPC13_W<13> {
        MSTPC13_W::new(self)
    }
    #[doc = "Bit 14 - Event Link Controller Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpc14(&mut self) -> MSTPC14_W<14> {
        MSTPC14_W::new(self)
    }
    #[doc = "Bit 28 - Random Number Generator Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpc28(&mut self) -> MSTPC28_W<28> {
        MSTPC28_W::new(self)
    }
    #[doc = "Bit 31 - AES Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpc31(&mut self) -> MSTPC31_W<31> {
        MSTPC31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Module Stop Control Register C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mstpcrc](index.html) module"]
pub struct MSTPCRC_SPEC;
impl crate::RegisterSpec for MSTPCRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mstpcrc::R](R) reader structure"]
impl crate::Readable for MSTPCRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mstpcrc::W](W) writer structure"]
impl crate::Writable for MSTPCRC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSTPCRC to value 0xffff_ffff"]
impl crate::Resettable for MSTPCRC_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
