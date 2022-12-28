#[doc = "Register `AMPC` reader"]
pub struct R(crate::R<AMPC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AMPC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AMPC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AMPC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AMPC` writer"]
pub struct W(crate::W<AMPC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AMPC_SPEC>;
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
impl From<crate::W<AMPC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AMPC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AMPE0` reader - Operation control of operational amplifier(UNIT0)"]
pub type AMPE0_R = crate::BitReader<AMPE0_A>;
#[doc = "Operation control of operational amplifier(UNIT0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPE0_A {
    #[doc = "0: Operation amplifier is stopped."]
    _0 = 0,
    #[doc = "1: Software trigger mode: Operation of operational amplifier is enabled Operation of the operational amplifier reference current circuit is also enabled regardless of the IREFE bit se An activation trigger mode or An activation and A/D trigger mode: Wait for AGT is enabled."]
    _1 = 1,
}
impl From<AMPE0_A> for bool {
    #[inline(always)]
    fn from(variant: AMPE0_A) -> Self {
        variant as u8 != 0
    }
}
impl AMPE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMPE0_A {
        match self.bits {
            false => AMPE0_A::_0,
            true => AMPE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPE0_A::_1
    }
}
#[doc = "Field `AMPE0` writer - Operation control of operational amplifier(UNIT0)"]
pub type AMPE0_W<'a, const O: u8> = crate::BitWriter<'a, u8, AMPC_SPEC, AMPE0_A, O>;
impl<'a, const O: u8> AMPE0_W<'a, O> {
    #[doc = "Operation amplifier is stopped."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AMPE0_A::_0)
    }
    #[doc = "Software trigger mode: Operation of operational amplifier is enabled Operation of the operational amplifier reference current circuit is also enabled regardless of the IREFE bit se An activation trigger mode or An activation and A/D trigger mode: Wait for AGT is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AMPE0_A::_1)
    }
}
#[doc = "Field `AMPE1` reader - Operation control of operational amplifier(UNIT1)"]
pub type AMPE1_R = crate::BitReader<AMPE1_A>;
#[doc = "Operation control of operational amplifier(UNIT1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPE1_A {
    #[doc = "0: Operation amplifier is stopped."]
    _0 = 0,
    #[doc = "1: Software trigger mode: Operation of operational amplifier is enabled Operation of the operational amplifier reference current circuit is also enabled regardless of the IREFE bit se An activation trigger mode or An activation and A/D trigger mode: Wait for An activation is enabled."]
    _1 = 1,
}
impl From<AMPE1_A> for bool {
    #[inline(always)]
    fn from(variant: AMPE1_A) -> Self {
        variant as u8 != 0
    }
}
impl AMPE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMPE1_A {
        match self.bits {
            false => AMPE1_A::_0,
            true => AMPE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPE1_A::_1
    }
}
#[doc = "Field `AMPE1` writer - Operation control of operational amplifier(UNIT1)"]
pub type AMPE1_W<'a, const O: u8> = crate::BitWriter<'a, u8, AMPC_SPEC, AMPE1_A, O>;
impl<'a, const O: u8> AMPE1_W<'a, O> {
    #[doc = "Operation amplifier is stopped."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AMPE1_A::_0)
    }
    #[doc = "Software trigger mode: Operation of operational amplifier is enabled Operation of the operational amplifier reference current circuit is also enabled regardless of the IREFE bit se An activation trigger mode or An activation and A/D trigger mode: Wait for An activation is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AMPE1_A::_1)
    }
}
#[doc = "Field `AMPE2` reader - Operation control of operational amplifier(UNIT2)"]
pub type AMPE2_R = crate::BitReader<AMPE2_A>;
#[doc = "Operation control of operational amplifier(UNIT2)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPE2_A {
    #[doc = "0: Operation amplifier is stopped."]
    _0 = 0,
    #[doc = "1: Software trigger mode: Operation of operational amplifier is enabled Operation of the operational amplifier reference current circuit is also enabled regardless of the IREFE bit se An activation trigger mode or An activation and A/D trigger mode: Wait for An activation is enabled."]
    _1 = 1,
}
impl From<AMPE2_A> for bool {
    #[inline(always)]
    fn from(variant: AMPE2_A) -> Self {
        variant as u8 != 0
    }
}
impl AMPE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMPE2_A {
        match self.bits {
            false => AMPE2_A::_0,
            true => AMPE2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPE2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPE2_A::_1
    }
}
#[doc = "Field `AMPE2` writer - Operation control of operational amplifier(UNIT2)"]
pub type AMPE2_W<'a, const O: u8> = crate::BitWriter<'a, u8, AMPC_SPEC, AMPE2_A, O>;
impl<'a, const O: u8> AMPE2_W<'a, O> {
    #[doc = "Operation amplifier is stopped."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AMPE2_A::_0)
    }
    #[doc = "Software trigger mode: Operation of operational amplifier is enabled Operation of the operational amplifier reference current circuit is also enabled regardless of the IREFE bit se An activation trigger mode or An activation and A/D trigger mode: Wait for An activation is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AMPE2_A::_1)
    }
}
#[doc = "Field `AMPE3` reader - Operation control of operational amplifier(UNIT3)"]
pub type AMPE3_R = crate::BitReader<AMPE3_A>;
#[doc = "Operation control of operational amplifier(UNIT3)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPE3_A {
    #[doc = "0: Operation amplifier is stopped."]
    _0 = 0,
    #[doc = "1: Software trigger mode: Operation of operational amplifier is enabled Operation of the operational amplifier reference current circuit is also enabled regardless of the IREFE bit se An activation trigger mode or An activation and A/D trigger mode: Wait for An activation is enabled."]
    _1 = 1,
}
impl From<AMPE3_A> for bool {
    #[inline(always)]
    fn from(variant: AMPE3_A) -> Self {
        variant as u8 != 0
    }
}
impl AMPE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMPE3_A {
        match self.bits {
            false => AMPE3_A::_0,
            true => AMPE3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPE3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPE3_A::_1
    }
}
#[doc = "Field `AMPE3` writer - Operation control of operational amplifier(UNIT3)"]
pub type AMPE3_W<'a, const O: u8> = crate::BitWriter<'a, u8, AMPC_SPEC, AMPE3_A, O>;
impl<'a, const O: u8> AMPE3_W<'a, O> {
    #[doc = "Operation amplifier is stopped."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AMPE3_A::_0)
    }
    #[doc = "Software trigger mode: Operation of operational amplifier is enabled Operation of the operational amplifier reference current circuit is also enabled regardless of the IREFE bit se An activation trigger mode or An activation and A/D trigger mode: Wait for An activation is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AMPE3_A::_1)
    }
}
#[doc = "Field `IREFE` reader - Operation control of operational amplifier reference current circuit"]
pub type IREFE_R = crate::BitReader<IREFE_A>;
#[doc = "Operation control of operational amplifier reference current circuit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IREFE_A {
    #[doc = "0: Operational amplifier reference current circuit is stopped."]
    _0 = 0,
    #[doc = "1: Operation of operational amplifier reference current circuit is enabled."]
    _1 = 1,
}
impl From<IREFE_A> for bool {
    #[inline(always)]
    fn from(variant: IREFE_A) -> Self {
        variant as u8 != 0
    }
}
impl IREFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IREFE_A {
        match self.bits {
            false => IREFE_A::_0,
            true => IREFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IREFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IREFE_A::_1
    }
}
#[doc = "Field `IREFE` writer - Operation control of operational amplifier reference current circuit"]
pub type IREFE_W<'a, const O: u8> = crate::BitWriter<'a, u8, AMPC_SPEC, IREFE_A, O>;
impl<'a, const O: u8> IREFE_W<'a, O> {
    #[doc = "Operational amplifier reference current circuit is stopped."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IREFE_A::_0)
    }
    #[doc = "Operation of operational amplifier reference current circuit is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IREFE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Operation control of operational amplifier(UNIT0)"]
    #[inline(always)]
    pub fn ampe0(&self) -> AMPE0_R {
        AMPE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Operation control of operational amplifier(UNIT1)"]
    #[inline(always)]
    pub fn ampe1(&self) -> AMPE1_R {
        AMPE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Operation control of operational amplifier(UNIT2)"]
    #[inline(always)]
    pub fn ampe2(&self) -> AMPE2_R {
        AMPE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Operation control of operational amplifier(UNIT3)"]
    #[inline(always)]
    pub fn ampe3(&self) -> AMPE3_R {
        AMPE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Operation control of operational amplifier reference current circuit"]
    #[inline(always)]
    pub fn irefe(&self) -> IREFE_R {
        IREFE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Operation control of operational amplifier(UNIT0)"]
    #[inline(always)]
    #[must_use]
    pub fn ampe0(&mut self) -> AMPE0_W<0> {
        AMPE0_W::new(self)
    }
    #[doc = "Bit 1 - Operation control of operational amplifier(UNIT1)"]
    #[inline(always)]
    #[must_use]
    pub fn ampe1(&mut self) -> AMPE1_W<1> {
        AMPE1_W::new(self)
    }
    #[doc = "Bit 2 - Operation control of operational amplifier(UNIT2)"]
    #[inline(always)]
    #[must_use]
    pub fn ampe2(&mut self) -> AMPE2_W<2> {
        AMPE2_W::new(self)
    }
    #[doc = "Bit 3 - Operation control of operational amplifier(UNIT3)"]
    #[inline(always)]
    #[must_use]
    pub fn ampe3(&mut self) -> AMPE3_W<3> {
        AMPE3_W::new(self)
    }
    #[doc = "Bit 7 - Operation control of operational amplifier reference current circuit"]
    #[inline(always)]
    #[must_use]
    pub fn irefe(&mut self) -> IREFE_W<7> {
        IREFE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Operational amplifier control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ampc](index.html) module"]
pub struct AMPC_SPEC;
impl crate::RegisterSpec for AMPC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ampc::R](R) reader structure"]
impl crate::Readable for AMPC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ampc::W](W) writer structure"]
impl crate::Writable for AMPC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AMPC to value 0"]
impl crate::Resettable for AMPC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
