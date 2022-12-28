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
#[doc = "Field `MSTPC1` reader - Cyclic Redundancy Check Module Stop"]
pub type MSTPC1_R = crate::BitReader<MSTPC1_A>;
#[doc = "Cyclic Redundancy Check Module Stop\n\nValue on reset: 1"]
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
#[doc = "Field `MSTPC1` writer - Cyclic Redundancy Check Module Stop"]
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
#[doc = "Field `MSTPC20` reader - Trigonometric Function Unit Module Stop"]
pub type MSTPC20_R = crate::BitReader<MSTPC20_A>;
#[doc = "Trigonometric Function Unit Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPC20_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPC20_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPC20_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPC20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPC20_A {
        match self.bits {
            false => MSTPC20_A::_0,
            true => MSTPC20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPC20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPC20_A::_1
    }
}
#[doc = "Field `MSTPC20` writer - Trigonometric Function Unit Module Stop"]
pub type MSTPC20_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRC_SPEC, MSTPC20_A, O>;
impl<'a, const O: u8> MSTPC20_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPC20_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPC20_A::_1)
    }
}
#[doc = "Field `MSTPC21` reader - IIR Filter Accelerator Module Stop"]
pub type MSTPC21_R = crate::BitReader<MSTPC21_A>;
#[doc = "IIR Filter Accelerator Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPC21_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPC21_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPC21_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPC21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPC21_A {
        match self.bits {
            false => MSTPC21_A::_0,
            true => MSTPC21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPC21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPC21_A::_1
    }
}
#[doc = "Field `MSTPC21` writer - IIR Filter Accelerator Module Stop"]
pub type MSTPC21_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRC_SPEC, MSTPC21_A, O>;
impl<'a, const O: u8> MSTPC21_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPC21_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPC21_A::_1)
    }
}
#[doc = "Field `MSTPC27` reader - CANFD Module Stop"]
pub type MSTPC27_R = crate::BitReader<MSTPC27_A>;
#[doc = "CANFD Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPC27_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPC27_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPC27_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPC27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPC27_A {
        match self.bits {
            false => MSTPC27_A::_0,
            true => MSTPC27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPC27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPC27_A::_1
    }
}
#[doc = "Field `MSTPC27` writer - CANFD Module Stop"]
pub type MSTPC27_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRC_SPEC, MSTPC27_A, O>;
impl<'a, const O: u8> MSTPC27_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPC27_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPC27_A::_1)
    }
}
#[doc = "Field `MSTPC31` reader - Secure Cryptographic Engine Module Stop"]
pub type MSTPC31_R = crate::BitReader<MSTPC31_A>;
#[doc = "Secure Cryptographic Engine Module Stop\n\nValue on reset: 1"]
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
#[doc = "Field `MSTPC31` writer - Secure Cryptographic Engine Module Stop"]
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
    #[doc = "Bit 1 - Cyclic Redundancy Check Module Stop"]
    #[inline(always)]
    pub fn mstpc1(&self) -> MSTPC1_R {
        MSTPC1_R::new(((self.bits >> 1) & 1) != 0)
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
    #[doc = "Bit 20 - Trigonometric Function Unit Module Stop"]
    #[inline(always)]
    pub fn mstpc20(&self) -> MSTPC20_R {
        MSTPC20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - IIR Filter Accelerator Module Stop"]
    #[inline(always)]
    pub fn mstpc21(&self) -> MSTPC21_R {
        MSTPC21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 27 - CANFD Module Stop"]
    #[inline(always)]
    pub fn mstpc27(&self) -> MSTPC27_R {
        MSTPC27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 31 - Secure Cryptographic Engine Module Stop"]
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
    #[doc = "Bit 1 - Cyclic Redundancy Check Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpc1(&mut self) -> MSTPC1_W<1> {
        MSTPC1_W::new(self)
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
    #[doc = "Bit 20 - Trigonometric Function Unit Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpc20(&mut self) -> MSTPC20_W<20> {
        MSTPC20_W::new(self)
    }
    #[doc = "Bit 21 - IIR Filter Accelerator Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpc21(&mut self) -> MSTPC21_W<21> {
        MSTPC21_W::new(self)
    }
    #[doc = "Bit 27 - CANFD Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpc27(&mut self) -> MSTPC27_W<27> {
        MSTPC27_W::new(self)
    }
    #[doc = "Bit 31 - Secure Cryptographic Engine Module Stop"]
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
