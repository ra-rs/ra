#[doc = "Register `AMPTRM` reader"]
pub struct R(crate::R<AMPTRM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AMPTRM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AMPTRM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AMPTRM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AMPTRM` writer"]
pub struct W(crate::W<AMPTRM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AMPTRM_SPEC>;
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
impl From<crate::W<AMPTRM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AMPTRM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AMPTRM00` reader - OPAMP function activation/stop trigger control"]
pub type AMPTRM00_R = crate::BitReader<AMPTRM00_A>;
#[doc = "OPAMP function activation/stop trigger control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPTRM00_A {
    #[doc = "0: Software trigger mode(AMPTRM01=0)/Setting prohibited(AMPTRM01=1)."]
    _0 = 0,
    #[doc = "1: An activation trigger mode(AMPTRM01=0)/An activation and A/D trigger mode(AMPTRM01=1)."]
    _1 = 1,
}
impl From<AMPTRM00_A> for bool {
    #[inline(always)]
    fn from(variant: AMPTRM00_A) -> Self {
        variant as u8 != 0
    }
}
impl AMPTRM00_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMPTRM00_A {
        match self.bits {
            false => AMPTRM00_A::_0,
            true => AMPTRM00_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPTRM00_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPTRM00_A::_1
    }
}
#[doc = "Field `AMPTRM00` writer - OPAMP function activation/stop trigger control"]
pub type AMPTRM00_W<'a, const O: u8> = crate::BitWriter<'a, u8, AMPTRM_SPEC, AMPTRM00_A, O>;
impl<'a, const O: u8> AMPTRM00_W<'a, O> {
    #[doc = "Software trigger mode(AMPTRM01=0)/Setting prohibited(AMPTRM01=1)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AMPTRM00_A::_0)
    }
    #[doc = "An activation trigger mode(AMPTRM01=0)/An activation and A/D trigger mode(AMPTRM01=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AMPTRM00_A::_1)
    }
}
#[doc = "Field `AMPTRM01` reader - OPAMP function activation/stop trigger control"]
pub type AMPTRM01_R = crate::BitReader<AMPTRM01_A>;
#[doc = "OPAMP function activation/stop trigger control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPTRM01_A {
    #[doc = "0: Software trigger mode(AMPTRM00=0)/An activation trigger mode(AMPTRM00=1)."]
    _0 = 0,
    #[doc = "1: Setting prohibited(AMPTRM00=0)/An activation and A/D trigger mode(AMPTRM00=1)."]
    _1 = 1,
}
impl From<AMPTRM01_A> for bool {
    #[inline(always)]
    fn from(variant: AMPTRM01_A) -> Self {
        variant as u8 != 0
    }
}
impl AMPTRM01_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMPTRM01_A {
        match self.bits {
            false => AMPTRM01_A::_0,
            true => AMPTRM01_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPTRM01_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPTRM01_A::_1
    }
}
#[doc = "Field `AMPTRM01` writer - OPAMP function activation/stop trigger control"]
pub type AMPTRM01_W<'a, const O: u8> = crate::BitWriter<'a, u8, AMPTRM_SPEC, AMPTRM01_A, O>;
impl<'a, const O: u8> AMPTRM01_W<'a, O> {
    #[doc = "Software trigger mode(AMPTRM00=0)/An activation trigger mode(AMPTRM00=1)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AMPTRM01_A::_0)
    }
    #[doc = "Setting prohibited(AMPTRM00=0)/An activation and A/D trigger mode(AMPTRM00=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AMPTRM01_A::_1)
    }
}
#[doc = "Field `AMPTRM10` reader - OPAMP function activation/stop trigger control"]
pub type AMPTRM10_R = crate::BitReader<AMPTRM10_A>;
#[doc = "OPAMP function activation/stop trigger control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPTRM10_A {
    #[doc = "0: Software trigger mode(AMPTRM11=0)/Setting prohibited(AMPTRM11=1)."]
    _0 = 0,
    #[doc = "1: An activation trigger mode(AMPTRM11=0)/An activation and A/D trigger mode(AMPTRM11=1)."]
    _1 = 1,
}
impl From<AMPTRM10_A> for bool {
    #[inline(always)]
    fn from(variant: AMPTRM10_A) -> Self {
        variant as u8 != 0
    }
}
impl AMPTRM10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMPTRM10_A {
        match self.bits {
            false => AMPTRM10_A::_0,
            true => AMPTRM10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPTRM10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPTRM10_A::_1
    }
}
#[doc = "Field `AMPTRM10` writer - OPAMP function activation/stop trigger control"]
pub type AMPTRM10_W<'a, const O: u8> = crate::BitWriter<'a, u8, AMPTRM_SPEC, AMPTRM10_A, O>;
impl<'a, const O: u8> AMPTRM10_W<'a, O> {
    #[doc = "Software trigger mode(AMPTRM11=0)/Setting prohibited(AMPTRM11=1)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AMPTRM10_A::_0)
    }
    #[doc = "An activation trigger mode(AMPTRM11=0)/An activation and A/D trigger mode(AMPTRM11=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AMPTRM10_A::_1)
    }
}
#[doc = "Field `AMPTRM11` reader - OPAMP function activation/stop trigger control"]
pub type AMPTRM11_R = crate::BitReader<AMPTRM11_A>;
#[doc = "OPAMP function activation/stop trigger control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPTRM11_A {
    #[doc = "0: Software trigger mode(AMPTRM10=0)/An activation trigger mode(AMPTRM10=1)."]
    _0 = 0,
    #[doc = "1: Setting prohibited(AMPTRM10=0)/An activation and A/D trigger mode(AMPTRM10=1)."]
    _1 = 1,
}
impl From<AMPTRM11_A> for bool {
    #[inline(always)]
    fn from(variant: AMPTRM11_A) -> Self {
        variant as u8 != 0
    }
}
impl AMPTRM11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMPTRM11_A {
        match self.bits {
            false => AMPTRM11_A::_0,
            true => AMPTRM11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPTRM11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPTRM11_A::_1
    }
}
#[doc = "Field `AMPTRM11` writer - OPAMP function activation/stop trigger control"]
pub type AMPTRM11_W<'a, const O: u8> = crate::BitWriter<'a, u8, AMPTRM_SPEC, AMPTRM11_A, O>;
impl<'a, const O: u8> AMPTRM11_W<'a, O> {
    #[doc = "Software trigger mode(AMPTRM10=0)/An activation trigger mode(AMPTRM10=1)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AMPTRM11_A::_0)
    }
    #[doc = "Setting prohibited(AMPTRM10=0)/An activation and A/D trigger mode(AMPTRM10=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AMPTRM11_A::_1)
    }
}
#[doc = "Field `AMPTRM20` reader - OPAMP function activation/stop trigger control"]
pub type AMPTRM20_R = crate::BitReader<AMPTRM20_A>;
#[doc = "OPAMP function activation/stop trigger control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPTRM20_A {
    #[doc = "0: Software trigger mode(AMPTRM21=0)/Setting prohibited(AMPTRM21=1)."]
    _0 = 0,
    #[doc = "1: An activation trigger mode(AMPTRM21=0)/An activation and A/D trigger mode(AMPTRM21=1)."]
    _1 = 1,
}
impl From<AMPTRM20_A> for bool {
    #[inline(always)]
    fn from(variant: AMPTRM20_A) -> Self {
        variant as u8 != 0
    }
}
impl AMPTRM20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMPTRM20_A {
        match self.bits {
            false => AMPTRM20_A::_0,
            true => AMPTRM20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPTRM20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPTRM20_A::_1
    }
}
#[doc = "Field `AMPTRM20` writer - OPAMP function activation/stop trigger control"]
pub type AMPTRM20_W<'a, const O: u8> = crate::BitWriter<'a, u8, AMPTRM_SPEC, AMPTRM20_A, O>;
impl<'a, const O: u8> AMPTRM20_W<'a, O> {
    #[doc = "Software trigger mode(AMPTRM21=0)/Setting prohibited(AMPTRM21=1)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AMPTRM20_A::_0)
    }
    #[doc = "An activation trigger mode(AMPTRM21=0)/An activation and A/D trigger mode(AMPTRM21=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AMPTRM20_A::_1)
    }
}
#[doc = "Field `AMPTRM21` reader - OPAMP function activation/stop trigger control"]
pub type AMPTRM21_R = crate::BitReader<AMPTRM21_A>;
#[doc = "OPAMP function activation/stop trigger control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPTRM21_A {
    #[doc = "0: Software trigger mode(AMPTRM20=0)/An activation trigger mode(AMPTRM20=1)."]
    _0 = 0,
    #[doc = "1: Setting prohibited(AMPTRM20=0)/An activation and A/D trigger mode(AMPTRM20=1)."]
    _1 = 1,
}
impl From<AMPTRM21_A> for bool {
    #[inline(always)]
    fn from(variant: AMPTRM21_A) -> Self {
        variant as u8 != 0
    }
}
impl AMPTRM21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMPTRM21_A {
        match self.bits {
            false => AMPTRM21_A::_0,
            true => AMPTRM21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMPTRM21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMPTRM21_A::_1
    }
}
#[doc = "Field `AMPTRM21` writer - OPAMP function activation/stop trigger control"]
pub type AMPTRM21_W<'a, const O: u8> = crate::BitWriter<'a, u8, AMPTRM_SPEC, AMPTRM21_A, O>;
impl<'a, const O: u8> AMPTRM21_W<'a, O> {
    #[doc = "Software trigger mode(AMPTRM20=0)/An activation trigger mode(AMPTRM20=1)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AMPTRM21_A::_0)
    }
    #[doc = "Setting prohibited(AMPTRM20=0)/An activation and A/D trigger mode(AMPTRM20=1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AMPTRM21_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - OPAMP function activation/stop trigger control"]
    #[inline(always)]
    pub fn amptrm00(&self) -> AMPTRM00_R {
        AMPTRM00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OPAMP function activation/stop trigger control"]
    #[inline(always)]
    pub fn amptrm01(&self) -> AMPTRM01_R {
        AMPTRM01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OPAMP function activation/stop trigger control"]
    #[inline(always)]
    pub fn amptrm10(&self) -> AMPTRM10_R {
        AMPTRM10_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OPAMP function activation/stop trigger control"]
    #[inline(always)]
    pub fn amptrm11(&self) -> AMPTRM11_R {
        AMPTRM11_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OPAMP function activation/stop trigger control"]
    #[inline(always)]
    pub fn amptrm20(&self) -> AMPTRM20_R {
        AMPTRM20_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - OPAMP function activation/stop trigger control"]
    #[inline(always)]
    pub fn amptrm21(&self) -> AMPTRM21_R {
        AMPTRM21_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OPAMP function activation/stop trigger control"]
    #[inline(always)]
    #[must_use]
    pub fn amptrm00(&mut self) -> AMPTRM00_W<0> {
        AMPTRM00_W::new(self)
    }
    #[doc = "Bit 1 - OPAMP function activation/stop trigger control"]
    #[inline(always)]
    #[must_use]
    pub fn amptrm01(&mut self) -> AMPTRM01_W<1> {
        AMPTRM01_W::new(self)
    }
    #[doc = "Bit 2 - OPAMP function activation/stop trigger control"]
    #[inline(always)]
    #[must_use]
    pub fn amptrm10(&mut self) -> AMPTRM10_W<2> {
        AMPTRM10_W::new(self)
    }
    #[doc = "Bit 3 - OPAMP function activation/stop trigger control"]
    #[inline(always)]
    #[must_use]
    pub fn amptrm11(&mut self) -> AMPTRM11_W<3> {
        AMPTRM11_W::new(self)
    }
    #[doc = "Bit 4 - OPAMP function activation/stop trigger control"]
    #[inline(always)]
    #[must_use]
    pub fn amptrm20(&mut self) -> AMPTRM20_W<4> {
        AMPTRM20_W::new(self)
    }
    #[doc = "Bit 5 - OPAMP function activation/stop trigger control"]
    #[inline(always)]
    #[must_use]
    pub fn amptrm21(&mut self) -> AMPTRM21_W<5> {
        AMPTRM21_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Operational amplifier trigger mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [amptrm](index.html) module"]
pub struct AMPTRM_SPEC;
impl crate::RegisterSpec for AMPTRM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [amptrm::R](R) reader structure"]
impl crate::Readable for AMPTRM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [amptrm::W](W) writer structure"]
impl crate::Writable for AMPTRM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AMPTRM to value 0"]
impl crate::Resettable for AMPTRM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
