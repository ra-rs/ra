#[doc = "Register `PHYSLEW` reader"]
pub struct R(crate::R<PHYSLEW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PHYSLEW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PHYSLEW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PHYSLEW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PHYSLEW` writer"]
pub struct W(crate::W<PHYSLEW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PHYSLEW_SPEC>;
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
impl From<crate::W<PHYSLEW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PHYSLEW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLEWR00` reader - Receiver Cross Point Adjustment 00"]
pub type SLEWR00_R = crate::BitReader<SLEWR00_A>;
#[doc = "Receiver Cross Point Adjustment 00\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLEWR00_A {
    #[doc = "0: Reserved"]
    _0 = 0,
    #[doc = "1: Host or device controller mode."]
    _1 = 1,
}
impl From<SLEWR00_A> for bool {
    #[inline(always)]
    fn from(variant: SLEWR00_A) -> Self {
        variant as u8 != 0
    }
}
impl SLEWR00_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEWR00_A {
        match self.bits {
            false => SLEWR00_A::_0,
            true => SLEWR00_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLEWR00_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLEWR00_A::_1
    }
}
#[doc = "Field `SLEWR00` writer - Receiver Cross Point Adjustment 00"]
pub type SLEWR00_W<'a, const O: u8> = crate::BitWriter<'a, u32, PHYSLEW_SPEC, SLEWR00_A, O>;
impl<'a, const O: u8> SLEWR00_W<'a, O> {
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLEWR00_A::_0)
    }
    #[doc = "Host or device controller mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLEWR00_A::_1)
    }
}
#[doc = "Field `SLEWR01` reader - Receiver Cross Point Adjustment 01"]
pub type SLEWR01_R = crate::BitReader<SLEWR01_A>;
#[doc = "Receiver Cross Point Adjustment 01\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLEWR01_A {
    #[doc = "0: Reserved"]
    _0 = 0,
    #[doc = "1: Host or device controller mode."]
    _1 = 1,
}
impl From<SLEWR01_A> for bool {
    #[inline(always)]
    fn from(variant: SLEWR01_A) -> Self {
        variant as u8 != 0
    }
}
impl SLEWR01_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEWR01_A {
        match self.bits {
            false => SLEWR01_A::_0,
            true => SLEWR01_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLEWR01_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLEWR01_A::_1
    }
}
#[doc = "Field `SLEWR01` writer - Receiver Cross Point Adjustment 01"]
pub type SLEWR01_W<'a, const O: u8> = crate::BitWriter<'a, u32, PHYSLEW_SPEC, SLEWR01_A, O>;
impl<'a, const O: u8> SLEWR01_W<'a, O> {
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLEWR01_A::_0)
    }
    #[doc = "Host or device controller mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLEWR01_A::_1)
    }
}
#[doc = "Field `SLEWF00` reader - Receiver Cross Point Adjustment 00"]
pub type SLEWF00_R = crate::BitReader<SLEWF00_A>;
#[doc = "Receiver Cross Point Adjustment 00\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLEWF00_A {
    #[doc = "0: Reserved"]
    _0 = 0,
    #[doc = "1: Host or device controller mode."]
    _1 = 1,
}
impl From<SLEWF00_A> for bool {
    #[inline(always)]
    fn from(variant: SLEWF00_A) -> Self {
        variant as u8 != 0
    }
}
impl SLEWF00_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEWF00_A {
        match self.bits {
            false => SLEWF00_A::_0,
            true => SLEWF00_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLEWF00_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLEWF00_A::_1
    }
}
#[doc = "Field `SLEWF00` writer - Receiver Cross Point Adjustment 00"]
pub type SLEWF00_W<'a, const O: u8> = crate::BitWriter<'a, u32, PHYSLEW_SPEC, SLEWF00_A, O>;
impl<'a, const O: u8> SLEWF00_W<'a, O> {
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLEWF00_A::_0)
    }
    #[doc = "Host or device controller mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLEWF00_A::_1)
    }
}
#[doc = "Field `SLEWF01` reader - Receiver Cross Point Adjustment 01"]
pub type SLEWF01_R = crate::BitReader<SLEWF01_A>;
#[doc = "Receiver Cross Point Adjustment 01\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLEWF01_A {
    #[doc = "0: Reserved"]
    _0 = 0,
    #[doc = "1: Host or device controller mode."]
    _1 = 1,
}
impl From<SLEWF01_A> for bool {
    #[inline(always)]
    fn from(variant: SLEWF01_A) -> Self {
        variant as u8 != 0
    }
}
impl SLEWF01_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEWF01_A {
        match self.bits {
            false => SLEWF01_A::_0,
            true => SLEWF01_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLEWF01_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLEWF01_A::_1
    }
}
#[doc = "Field `SLEWF01` writer - Receiver Cross Point Adjustment 01"]
pub type SLEWF01_W<'a, const O: u8> = crate::BitWriter<'a, u32, PHYSLEW_SPEC, SLEWF01_A, O>;
impl<'a, const O: u8> SLEWF01_W<'a, O> {
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLEWF01_A::_0)
    }
    #[doc = "Host or device controller mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLEWF01_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Receiver Cross Point Adjustment 00"]
    #[inline(always)]
    pub fn slewr00(&self) -> SLEWR00_R {
        SLEWR00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receiver Cross Point Adjustment 01"]
    #[inline(always)]
    pub fn slewr01(&self) -> SLEWR01_R {
        SLEWR01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receiver Cross Point Adjustment 00"]
    #[inline(always)]
    pub fn slewf00(&self) -> SLEWF00_R {
        SLEWF00_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receiver Cross Point Adjustment 01"]
    #[inline(always)]
    pub fn slewf01(&self) -> SLEWF01_R {
        SLEWF01_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receiver Cross Point Adjustment 00"]
    #[inline(always)]
    #[must_use]
    pub fn slewr00(&mut self) -> SLEWR00_W<0> {
        SLEWR00_W::new(self)
    }
    #[doc = "Bit 1 - Receiver Cross Point Adjustment 01"]
    #[inline(always)]
    #[must_use]
    pub fn slewr01(&mut self) -> SLEWR01_W<1> {
        SLEWR01_W::new(self)
    }
    #[doc = "Bit 2 - Receiver Cross Point Adjustment 00"]
    #[inline(always)]
    #[must_use]
    pub fn slewf00(&mut self) -> SLEWF00_W<2> {
        SLEWF00_W::new(self)
    }
    #[doc = "Bit 3 - Receiver Cross Point Adjustment 01"]
    #[inline(always)]
    #[must_use]
    pub fn slewf01(&mut self) -> SLEWF01_W<3> {
        SLEWF01_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PHY Cross Point Adjustment Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [physlew](index.html) module"]
pub struct PHYSLEW_SPEC;
impl crate::RegisterSpec for PHYSLEW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [physlew::R](R) reader structure"]
impl crate::Readable for PHYSLEW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [physlew::W](W) writer structure"]
impl crate::Writable for PHYSLEW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PHYSLEW to value 0x0e"]
impl crate::Resettable for PHYSLEW_SPEC {
    const RESET_VALUE: Self::Ux = 0x0e;
}
