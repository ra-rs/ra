#[doc = "Register `DPSIER1` reader"]
pub struct R(crate::R<DPSIER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPSIER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPSIER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPSIER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPSIER1` writer"]
pub struct W(crate::W<DPSIER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPSIER1_SPEC>;
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
impl From<crate::W<DPSIER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPSIER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIRQ8E` reader - IRQ8-DS Pin Enable"]
pub type DIRQ8E_R = crate::BitReader<DIRQ8E_A>;
#[doc = "IRQ8-DS Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ8E_A {
    #[doc = "0: Cancelling Deep Software Standby mode is disabled"]
    _0 = 0,
    #[doc = "1: Cancelling Deep Software Standby mode is enabled"]
    _1 = 1,
}
impl From<DIRQ8E_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ8E_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRQ8E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRQ8E_A {
        match self.bits {
            false => DIRQ8E_A::_0,
            true => DIRQ8E_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ8E_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ8E_A::_1
    }
}
#[doc = "Field `DIRQ8E` writer - IRQ8-DS Pin Enable"]
pub type DIRQ8E_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIER1_SPEC, DIRQ8E_A, O>;
impl<'a, const O: u8> DIRQ8E_W<'a, O> {
    #[doc = "Cancelling Deep Software Standby mode is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIRQ8E_A::_0)
    }
    #[doc = "Cancelling Deep Software Standby mode is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIRQ8E_A::_1)
    }
}
#[doc = "Field `DIRQ9E` reader - IRQ9-DS Pin Enable"]
pub type DIRQ9E_R = crate::BitReader<DIRQ9E_A>;
#[doc = "IRQ9-DS Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ9E_A {
    #[doc = "0: Cancelling Deep Software Standby mode is disabled"]
    _0 = 0,
    #[doc = "1: Cancelling Deep Software Standby mode is enabled"]
    _1 = 1,
}
impl From<DIRQ9E_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ9E_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRQ9E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRQ9E_A {
        match self.bits {
            false => DIRQ9E_A::_0,
            true => DIRQ9E_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ9E_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ9E_A::_1
    }
}
#[doc = "Field `DIRQ9E` writer - IRQ9-DS Pin Enable"]
pub type DIRQ9E_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIER1_SPEC, DIRQ9E_A, O>;
impl<'a, const O: u8> DIRQ9E_W<'a, O> {
    #[doc = "Cancelling Deep Software Standby mode is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIRQ9E_A::_0)
    }
    #[doc = "Cancelling Deep Software Standby mode is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIRQ9E_A::_1)
    }
}
#[doc = "Field `DIRQ10E` reader - IRQ10-DS Pin Enable"]
pub type DIRQ10E_R = crate::BitReader<DIRQ10E_A>;
#[doc = "IRQ10-DS Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ10E_A {
    #[doc = "0: Cancelling Deep Software Standby mode is disabled"]
    _0 = 0,
    #[doc = "1: Cancelling Deep Software Standby mode is enabled"]
    _1 = 1,
}
impl From<DIRQ10E_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ10E_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRQ10E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRQ10E_A {
        match self.bits {
            false => DIRQ10E_A::_0,
            true => DIRQ10E_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ10E_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ10E_A::_1
    }
}
#[doc = "Field `DIRQ10E` writer - IRQ10-DS Pin Enable"]
pub type DIRQ10E_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIER1_SPEC, DIRQ10E_A, O>;
impl<'a, const O: u8> DIRQ10E_W<'a, O> {
    #[doc = "Cancelling Deep Software Standby mode is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIRQ10E_A::_0)
    }
    #[doc = "Cancelling Deep Software Standby mode is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIRQ10E_A::_1)
    }
}
#[doc = "Field `DIRQ11E` reader - IRQ11-DS Pin Enable"]
pub type DIRQ11E_R = crate::BitReader<DIRQ11E_A>;
#[doc = "IRQ11-DS Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ11E_A {
    #[doc = "0: Cancelling Deep Software Standby mode is disabled"]
    _0 = 0,
    #[doc = "1: Cancelling Deep Software Standby mode is enabled"]
    _1 = 1,
}
impl From<DIRQ11E_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ11E_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRQ11E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRQ11E_A {
        match self.bits {
            false => DIRQ11E_A::_0,
            true => DIRQ11E_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ11E_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ11E_A::_1
    }
}
#[doc = "Field `DIRQ11E` writer - IRQ11-DS Pin Enable"]
pub type DIRQ11E_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIER1_SPEC, DIRQ11E_A, O>;
impl<'a, const O: u8> DIRQ11E_W<'a, O> {
    #[doc = "Cancelling Deep Software Standby mode is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIRQ11E_A::_0)
    }
    #[doc = "Cancelling Deep Software Standby mode is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIRQ11E_A::_1)
    }
}
#[doc = "Field `DIRQ12E` reader - IRQ12-DS Pin Enable"]
pub type DIRQ12E_R = crate::BitReader<DIRQ12E_A>;
#[doc = "IRQ12-DS Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ12E_A {
    #[doc = "0: Cancelling Deep Software Standby mode is disabled"]
    _0 = 0,
    #[doc = "1: Cancelling Deep Software Standby mode is enabled"]
    _1 = 1,
}
impl From<DIRQ12E_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ12E_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRQ12E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRQ12E_A {
        match self.bits {
            false => DIRQ12E_A::_0,
            true => DIRQ12E_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ12E_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ12E_A::_1
    }
}
#[doc = "Field `DIRQ12E` writer - IRQ12-DS Pin Enable"]
pub type DIRQ12E_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIER1_SPEC, DIRQ12E_A, O>;
impl<'a, const O: u8> DIRQ12E_W<'a, O> {
    #[doc = "Cancelling Deep Software Standby mode is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIRQ12E_A::_0)
    }
    #[doc = "Cancelling Deep Software Standby mode is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIRQ12E_A::_1)
    }
}
#[doc = "Field `DIRQ14E` reader - IRQ14-DS Pin Enable"]
pub type DIRQ14E_R = crate::BitReader<DIRQ14E_A>;
#[doc = "IRQ14-DS Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ14E_A {
    #[doc = "0: Cancelling Deep Software Standby mode is disabled"]
    _0 = 0,
    #[doc = "1: Cancelling Deep Software Standby mode is enabled"]
    _1 = 1,
}
impl From<DIRQ14E_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ14E_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRQ14E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRQ14E_A {
        match self.bits {
            false => DIRQ14E_A::_0,
            true => DIRQ14E_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ14E_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ14E_A::_1
    }
}
#[doc = "Field `DIRQ14E` writer - IRQ14-DS Pin Enable"]
pub type DIRQ14E_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIER1_SPEC, DIRQ14E_A, O>;
impl<'a, const O: u8> DIRQ14E_W<'a, O> {
    #[doc = "Cancelling Deep Software Standby mode is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIRQ14E_A::_0)
    }
    #[doc = "Cancelling Deep Software Standby mode is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIRQ14E_A::_1)
    }
}
#[doc = "Field `DIRQ15E` reader - IRQ15-DS Pin Enable"]
pub type DIRQ15E_R = crate::BitReader<DIRQ15E_A>;
#[doc = "IRQ15-DS Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ15E_A {
    #[doc = "0: Cancelling Deep Software Standby mode is disabled"]
    _0 = 0,
    #[doc = "1: Cancelling Deep Software Standby mode is enabled"]
    _1 = 1,
}
impl From<DIRQ15E_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ15E_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRQ15E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRQ15E_A {
        match self.bits {
            false => DIRQ15E_A::_0,
            true => DIRQ15E_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ15E_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ15E_A::_1
    }
}
#[doc = "Field `DIRQ15E` writer - IRQ15-DS Pin Enable"]
pub type DIRQ15E_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIER1_SPEC, DIRQ15E_A, O>;
impl<'a, const O: u8> DIRQ15E_W<'a, O> {
    #[doc = "Cancelling Deep Software Standby mode is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIRQ15E_A::_0)
    }
    #[doc = "Cancelling Deep Software Standby mode is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIRQ15E_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - IRQ8-DS Pin Enable"]
    #[inline(always)]
    pub fn dirq8e(&self) -> DIRQ8E_R {
        DIRQ8E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IRQ9-DS Pin Enable"]
    #[inline(always)]
    pub fn dirq9e(&self) -> DIRQ9E_R {
        DIRQ9E_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IRQ10-DS Pin Enable"]
    #[inline(always)]
    pub fn dirq10e(&self) -> DIRQ10E_R {
        DIRQ10E_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IRQ11-DS Pin Enable"]
    #[inline(always)]
    pub fn dirq11e(&self) -> DIRQ11E_R {
        DIRQ11E_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IRQ12-DS Pin Enable"]
    #[inline(always)]
    pub fn dirq12e(&self) -> DIRQ12E_R {
        DIRQ12E_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - IRQ14-DS Pin Enable"]
    #[inline(always)]
    pub fn dirq14e(&self) -> DIRQ14E_R {
        DIRQ14E_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IRQ15-DS Pin Enable"]
    #[inline(always)]
    pub fn dirq15e(&self) -> DIRQ15E_R {
        DIRQ15E_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IRQ8-DS Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dirq8e(&mut self) -> DIRQ8E_W<0> {
        DIRQ8E_W::new(self)
    }
    #[doc = "Bit 1 - IRQ9-DS Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dirq9e(&mut self) -> DIRQ9E_W<1> {
        DIRQ9E_W::new(self)
    }
    #[doc = "Bit 2 - IRQ10-DS Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dirq10e(&mut self) -> DIRQ10E_W<2> {
        DIRQ10E_W::new(self)
    }
    #[doc = "Bit 3 - IRQ11-DS Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dirq11e(&mut self) -> DIRQ11E_W<3> {
        DIRQ11E_W::new(self)
    }
    #[doc = "Bit 4 - IRQ12-DS Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dirq12e(&mut self) -> DIRQ12E_W<4> {
        DIRQ12E_W::new(self)
    }
    #[doc = "Bit 6 - IRQ14-DS Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dirq14e(&mut self) -> DIRQ14E_W<6> {
        DIRQ14E_W::new(self)
    }
    #[doc = "Bit 7 - IRQ15-DS Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dirq15e(&mut self) -> DIRQ15E_W<7> {
        DIRQ15E_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Deep Standby Interrupt Enable Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpsier1](index.html) module"]
pub struct DPSIER1_SPEC;
impl crate::RegisterSpec for DPSIER1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dpsier1::R](R) reader structure"]
impl crate::Readable for DPSIER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dpsier1::W](W) writer structure"]
impl crate::Writable for DPSIER1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPSIER1 to value 0"]
impl crate::Resettable for DPSIER1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
