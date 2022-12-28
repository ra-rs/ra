#[doc = "Register `MSTPCRD` reader"]
pub struct R(crate::R<MSTPCRD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSTPCRD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSTPCRD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSTPCRD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSTPCRD` writer"]
pub struct W(crate::W<MSTPCRD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSTPCRD_SPEC>;
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
impl From<crate::W<MSTPCRD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSTPCRD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSTPD2` reader - Low Power Asynchronous General Purpose Timer 1 Module Stop"]
pub type MSTPD2_R = crate::BitReader<MSTPD2_A>;
#[doc = "Low Power Asynchronous General Purpose Timer 1 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPD2_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPD2_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPD2_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPD2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPD2_A {
        match self.bits {
            false => MSTPD2_A::_0,
            true => MSTPD2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPD2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPD2_A::_1
    }
}
#[doc = "Field `MSTPD2` writer - Low Power Asynchronous General Purpose Timer 1 Module Stop"]
pub type MSTPD2_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRD_SPEC, MSTPD2_A, O>;
impl<'a, const O: u8> MSTPD2_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPD2_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPD2_A::_1)
    }
}
#[doc = "Field `MSTPD3` reader - Low Power Asynchronous General Purpose Timer 0 Module Stop"]
pub type MSTPD3_R = crate::BitReader<MSTPD3_A>;
#[doc = "Low Power Asynchronous General Purpose Timer 0 Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPD3_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPD3_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPD3_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPD3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPD3_A {
        match self.bits {
            false => MSTPD3_A::_0,
            true => MSTPD3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPD3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPD3_A::_1
    }
}
#[doc = "Field `MSTPD3` writer - Low Power Asynchronous General Purpose Timer 0 Module Stop"]
pub type MSTPD3_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRD_SPEC, MSTPD3_A, O>;
impl<'a, const O: u8> MSTPD3_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPD3_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPD3_A::_1)
    }
}
#[doc = "Field `MSTPD5` reader - General PWM Timer 32n Module Stop"]
pub type MSTPD5_R = crate::BitReader<MSTPD5_A>;
#[doc = "General PWM Timer 32n Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPD5_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPD5_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPD5_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPD5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPD5_A {
        match self.bits {
            false => MSTPD5_A::_0,
            true => MSTPD5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPD5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPD5_A::_1
    }
}
#[doc = "Field `MSTPD5` writer - General PWM Timer 32n Module Stop"]
pub type MSTPD5_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRD_SPEC, MSTPD5_A, O>;
impl<'a, const O: u8> MSTPD5_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPD5_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPD5_A::_1)
    }
}
#[doc = "Field `MSTPD6` reader - General PWM Timer 164 to 169 and PWM Delay Generation Circuit Module Stop"]
pub type MSTPD6_R = crate::BitReader<MSTPD6_A>;
#[doc = "General PWM Timer 164 to 169 and PWM Delay Generation Circuit Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPD6_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPD6_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPD6_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPD6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPD6_A {
        match self.bits {
            false => MSTPD6_A::_0,
            true => MSTPD6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPD6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPD6_A::_1
    }
}
#[doc = "Field `MSTPD6` writer - General PWM Timer 164 to 169 and PWM Delay Generation Circuit Module Stop"]
pub type MSTPD6_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRD_SPEC, MSTPD6_A, O>;
impl<'a, const O: u8> MSTPD6_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPD6_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPD6_A::_1)
    }
}
#[doc = "Field `MSTPD14` reader - Port Output Enable for GPT Module Stop"]
pub type MSTPD14_R = crate::BitReader<MSTPD14_A>;
#[doc = "Port Output Enable for GPT Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPD14_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPD14_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPD14_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPD14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPD14_A {
        match self.bits {
            false => MSTPD14_A::_0,
            true => MSTPD14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPD14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPD14_A::_1
    }
}
#[doc = "Field `MSTPD14` writer - Port Output Enable for GPT Module Stop"]
pub type MSTPD14_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRD_SPEC, MSTPD14_A, O>;
impl<'a, const O: u8> MSTPD14_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPD14_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPD14_A::_1)
    }
}
#[doc = "Field `MSTPD16` reader - 12-bit A/D Converter Module Stop"]
pub type MSTPD16_R = crate::BitReader<MSTPD16_A>;
#[doc = "12-bit A/D Converter Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPD16_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPD16_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPD16_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPD16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPD16_A {
        match self.bits {
            false => MSTPD16_A::_0,
            true => MSTPD16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPD16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPD16_A::_1
    }
}
#[doc = "Field `MSTPD16` writer - 12-bit A/D Converter Module Stop"]
pub type MSTPD16_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRD_SPEC, MSTPD16_A, O>;
impl<'a, const O: u8> MSTPD16_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPD16_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPD16_A::_1)
    }
}
#[doc = "Field `MSTPD20` reader - 12-bit D/A Converter Module Stop"]
pub type MSTPD20_R = crate::BitReader<MSTPD20_A>;
#[doc = "12-bit D/A Converter Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPD20_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPD20_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPD20_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPD20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPD20_A {
        match self.bits {
            false => MSTPD20_A::_0,
            true => MSTPD20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPD20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPD20_A::_1
    }
}
#[doc = "Field `MSTPD20` writer - 12-bit D/A Converter Module Stop"]
pub type MSTPD20_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRD_SPEC, MSTPD20_A, O>;
impl<'a, const O: u8> MSTPD20_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPD20_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPD20_A::_1)
    }
}
#[doc = "Field `MSTPD29` reader - Low-Power Analog Comparator Module Stop"]
pub type MSTPD29_R = crate::BitReader<MSTPD29_A>;
#[doc = "Low-Power Analog Comparator Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPD29_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPD29_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPD29_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPD29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPD29_A {
        match self.bits {
            false => MSTPD29_A::_0,
            true => MSTPD29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPD29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPD29_A::_1
    }
}
#[doc = "Field `MSTPD29` writer - Low-Power Analog Comparator Module Stop"]
pub type MSTPD29_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRD_SPEC, MSTPD29_A, O>;
impl<'a, const O: u8> MSTPD29_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPD29_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPD29_A::_1)
    }
}
impl R {
    #[doc = "Bit 2 - Low Power Asynchronous General Purpose Timer 1 Module Stop"]
    #[inline(always)]
    pub fn mstpd2(&self) -> MSTPD2_R {
        MSTPD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Low Power Asynchronous General Purpose Timer 0 Module Stop"]
    #[inline(always)]
    pub fn mstpd3(&self) -> MSTPD3_R {
        MSTPD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - General PWM Timer 32n Module Stop"]
    #[inline(always)]
    pub fn mstpd5(&self) -> MSTPD5_R {
        MSTPD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - General PWM Timer 164 to 169 and PWM Delay Generation Circuit Module Stop"]
    #[inline(always)]
    pub fn mstpd6(&self) -> MSTPD6_R {
        MSTPD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 14 - Port Output Enable for GPT Module Stop"]
    #[inline(always)]
    pub fn mstpd14(&self) -> MSTPD14_R {
        MSTPD14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 12-bit A/D Converter Module Stop"]
    #[inline(always)]
    pub fn mstpd16(&self) -> MSTPD16_R {
        MSTPD16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - 12-bit D/A Converter Module Stop"]
    #[inline(always)]
    pub fn mstpd20(&self) -> MSTPD20_R {
        MSTPD20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 29 - Low-Power Analog Comparator Module Stop"]
    #[inline(always)]
    pub fn mstpd29(&self) -> MSTPD29_R {
        MSTPD29_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Low Power Asynchronous General Purpose Timer 1 Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpd2(&mut self) -> MSTPD2_W<2> {
        MSTPD2_W::new(self)
    }
    #[doc = "Bit 3 - Low Power Asynchronous General Purpose Timer 0 Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpd3(&mut self) -> MSTPD3_W<3> {
        MSTPD3_W::new(self)
    }
    #[doc = "Bit 5 - General PWM Timer 32n Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpd5(&mut self) -> MSTPD5_W<5> {
        MSTPD5_W::new(self)
    }
    #[doc = "Bit 6 - General PWM Timer 164 to 169 and PWM Delay Generation Circuit Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpd6(&mut self) -> MSTPD6_W<6> {
        MSTPD6_W::new(self)
    }
    #[doc = "Bit 14 - Port Output Enable for GPT Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpd14(&mut self) -> MSTPD14_W<14> {
        MSTPD14_W::new(self)
    }
    #[doc = "Bit 16 - 12-bit A/D Converter Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpd16(&mut self) -> MSTPD16_W<16> {
        MSTPD16_W::new(self)
    }
    #[doc = "Bit 20 - 12-bit D/A Converter Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpd20(&mut self) -> MSTPD20_W<20> {
        MSTPD20_W::new(self)
    }
    #[doc = "Bit 29 - Low-Power Analog Comparator Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpd29(&mut self) -> MSTPD29_W<29> {
        MSTPD29_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Module Stop Control Register D\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mstpcrd](index.html) module"]
pub struct MSTPCRD_SPEC;
impl crate::RegisterSpec for MSTPCRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mstpcrd::R](R) reader structure"]
impl crate::Readable for MSTPCRD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mstpcrd::W](W) writer structure"]
impl crate::Writable for MSTPCRD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSTPCRD to value 0xffff_ffff"]
impl crate::Resettable for MSTPCRD_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
