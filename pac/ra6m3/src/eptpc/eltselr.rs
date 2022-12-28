#[doc = "Register `ELTSELR` reader"]
pub struct R(crate::R<ELTSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ELTSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ELTSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ELTSELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ELTSELR` writer"]
pub struct W(crate::W<ELTSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ELTSELR_SPEC>;
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
impl From<crate::W<ELTSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ELTSELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ELTDIS0` reader - Pulse Output Timer 0 Event Generation Disable"]
pub type ELTDIS0_R = crate::BitReader<ELTDIS0_A>;
#[doc = "Pulse Output Timer 0 Event Generation Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELTDIS0_A {
    #[doc = "0: Pulse output timer 0 is used for the generation of event signals for the ELC."]
    _0 = 0,
    #[doc = "1: Pulse output timer 0 is not used for the generation of event signals for the ELC."]
    _1 = 1,
}
impl From<ELTDIS0_A> for bool {
    #[inline(always)]
    fn from(variant: ELTDIS0_A) -> Self {
        variant as u8 != 0
    }
}
impl ELTDIS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELTDIS0_A {
        match self.bits {
            false => ELTDIS0_A::_0,
            true => ELTDIS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELTDIS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELTDIS0_A::_1
    }
}
#[doc = "Field `ELTDIS0` writer - Pulse Output Timer 0 Event Generation Disable"]
pub type ELTDIS0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELTSELR_SPEC, ELTDIS0_A, O>;
impl<'a, const O: u8> ELTDIS0_W<'a, O> {
    #[doc = "Pulse output timer 0 is used for the generation of event signals for the ELC."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ELTDIS0_A::_0)
    }
    #[doc = "Pulse output timer 0 is not used for the generation of event signals for the ELC."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ELTDIS0_A::_1)
    }
}
#[doc = "Field `ELTDIS1` reader - Pulse Output Timer 1 Event Generation Disable"]
pub type ELTDIS1_R = crate::BitReader<ELTDIS1_A>;
#[doc = "Pulse Output Timer 1 Event Generation Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELTDIS1_A {
    #[doc = "0: Pulse output timer 1 is used for the generation of event signals for the ELC."]
    _0 = 0,
    #[doc = "1: Pulse output timer 1 is not used for the generation of event signals for the ELC."]
    _1 = 1,
}
impl From<ELTDIS1_A> for bool {
    #[inline(always)]
    fn from(variant: ELTDIS1_A) -> Self {
        variant as u8 != 0
    }
}
impl ELTDIS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELTDIS1_A {
        match self.bits {
            false => ELTDIS1_A::_0,
            true => ELTDIS1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELTDIS1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELTDIS1_A::_1
    }
}
#[doc = "Field `ELTDIS1` writer - Pulse Output Timer 1 Event Generation Disable"]
pub type ELTDIS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELTSELR_SPEC, ELTDIS1_A, O>;
impl<'a, const O: u8> ELTDIS1_W<'a, O> {
    #[doc = "Pulse output timer 1 is used for the generation of event signals for the ELC."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ELTDIS1_A::_0)
    }
    #[doc = "Pulse output timer 1 is not used for the generation of event signals for the ELC."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ELTDIS1_A::_1)
    }
}
#[doc = "Field `ELTDIS2` reader - Pulse Output Timer 2 Event Generation Disable"]
pub type ELTDIS2_R = crate::BitReader<ELTDIS2_A>;
#[doc = "Pulse Output Timer 2 Event Generation Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELTDIS2_A {
    #[doc = "0: Pulse output timer 2 is used for the generation of event signals for the ELC."]
    _0 = 0,
    #[doc = "1: Pulse output timer 2 is not used for the generation of event signals for the ELC."]
    _1 = 1,
}
impl From<ELTDIS2_A> for bool {
    #[inline(always)]
    fn from(variant: ELTDIS2_A) -> Self {
        variant as u8 != 0
    }
}
impl ELTDIS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELTDIS2_A {
        match self.bits {
            false => ELTDIS2_A::_0,
            true => ELTDIS2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELTDIS2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELTDIS2_A::_1
    }
}
#[doc = "Field `ELTDIS2` writer - Pulse Output Timer 2 Event Generation Disable"]
pub type ELTDIS2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELTSELR_SPEC, ELTDIS2_A, O>;
impl<'a, const O: u8> ELTDIS2_W<'a, O> {
    #[doc = "Pulse output timer 2 is used for the generation of event signals for the ELC."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ELTDIS2_A::_0)
    }
    #[doc = "Pulse output timer 2 is not used for the generation of event signals for the ELC."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ELTDIS2_A::_1)
    }
}
#[doc = "Field `ELTDIS3` reader - Pulse Output Timer 3 Event Generation Disable"]
pub type ELTDIS3_R = crate::BitReader<ELTDIS3_A>;
#[doc = "Pulse Output Timer 3 Event Generation Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELTDIS3_A {
    #[doc = "0: Pulse output timer 3 is used for the generation of event signals for the ELC."]
    _0 = 0,
    #[doc = "1: Pulse output timer 3 is not used for the generation of event signals for the ELC."]
    _1 = 1,
}
impl From<ELTDIS3_A> for bool {
    #[inline(always)]
    fn from(variant: ELTDIS3_A) -> Self {
        variant as u8 != 0
    }
}
impl ELTDIS3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELTDIS3_A {
        match self.bits {
            false => ELTDIS3_A::_0,
            true => ELTDIS3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELTDIS3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELTDIS3_A::_1
    }
}
#[doc = "Field `ELTDIS3` writer - Pulse Output Timer 3 Event Generation Disable"]
pub type ELTDIS3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELTSELR_SPEC, ELTDIS3_A, O>;
impl<'a, const O: u8> ELTDIS3_W<'a, O> {
    #[doc = "Pulse output timer 3 is used for the generation of event signals for the ELC."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ELTDIS3_A::_0)
    }
    #[doc = "Pulse output timer 3 is not used for the generation of event signals for the ELC."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ELTDIS3_A::_1)
    }
}
#[doc = "Field `ELTDIS4` reader - Pulse Output Timer 4 Event Generation Disable"]
pub type ELTDIS4_R = crate::BitReader<ELTDIS4_A>;
#[doc = "Pulse Output Timer 4 Event Generation Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELTDIS4_A {
    #[doc = "0: Pulse output timer 4 is used for the generation of event signals for the ELC."]
    _0 = 0,
    #[doc = "1: Pulse output timer 4 is not used for the generation of event signals for the ELC."]
    _1 = 1,
}
impl From<ELTDIS4_A> for bool {
    #[inline(always)]
    fn from(variant: ELTDIS4_A) -> Self {
        variant as u8 != 0
    }
}
impl ELTDIS4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELTDIS4_A {
        match self.bits {
            false => ELTDIS4_A::_0,
            true => ELTDIS4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELTDIS4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELTDIS4_A::_1
    }
}
#[doc = "Field `ELTDIS4` writer - Pulse Output Timer 4 Event Generation Disable"]
pub type ELTDIS4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELTSELR_SPEC, ELTDIS4_A, O>;
impl<'a, const O: u8> ELTDIS4_W<'a, O> {
    #[doc = "Pulse output timer 4 is used for the generation of event signals for the ELC."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ELTDIS4_A::_0)
    }
    #[doc = "Pulse output timer 4 is not used for the generation of event signals for the ELC."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ELTDIS4_A::_1)
    }
}
#[doc = "Field `ELTDIS5` reader - Pulse Output Timer 5 Event Generation Disable"]
pub type ELTDIS5_R = crate::BitReader<ELTDIS5_A>;
#[doc = "Pulse Output Timer 5 Event Generation Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELTDIS5_A {
    #[doc = "0: Pulse output timer 5 is used for the generation of event signals for the ELC."]
    _0 = 0,
    #[doc = "1: Pulse output timer 5 is not used for the generation of event signals for the ELC."]
    _1 = 1,
}
impl From<ELTDIS5_A> for bool {
    #[inline(always)]
    fn from(variant: ELTDIS5_A) -> Self {
        variant as u8 != 0
    }
}
impl ELTDIS5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELTDIS5_A {
        match self.bits {
            false => ELTDIS5_A::_0,
            true => ELTDIS5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELTDIS5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELTDIS5_A::_1
    }
}
#[doc = "Field `ELTDIS5` writer - Pulse Output Timer 5 Event Generation Disable"]
pub type ELTDIS5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELTSELR_SPEC, ELTDIS5_A, O>;
impl<'a, const O: u8> ELTDIS5_W<'a, O> {
    #[doc = "Pulse output timer 5 is used for the generation of event signals for the ELC."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ELTDIS5_A::_0)
    }
    #[doc = "Pulse output timer 5 is not used for the generation of event signals for the ELC."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ELTDIS5_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Pulse Output Timer 0 Event Generation Disable"]
    #[inline(always)]
    pub fn eltdis0(&self) -> ELTDIS0_R {
        ELTDIS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pulse Output Timer 1 Event Generation Disable"]
    #[inline(always)]
    pub fn eltdis1(&self) -> ELTDIS1_R {
        ELTDIS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pulse Output Timer 2 Event Generation Disable"]
    #[inline(always)]
    pub fn eltdis2(&self) -> ELTDIS2_R {
        ELTDIS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pulse Output Timer 3 Event Generation Disable"]
    #[inline(always)]
    pub fn eltdis3(&self) -> ELTDIS3_R {
        ELTDIS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pulse Output Timer 4 Event Generation Disable"]
    #[inline(always)]
    pub fn eltdis4(&self) -> ELTDIS4_R {
        ELTDIS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pulse Output Timer 5 Event Generation Disable"]
    #[inline(always)]
    pub fn eltdis5(&self) -> ELTDIS5_R {
        ELTDIS5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pulse Output Timer 0 Event Generation Disable"]
    #[inline(always)]
    #[must_use]
    pub fn eltdis0(&mut self) -> ELTDIS0_W<0> {
        ELTDIS0_W::new(self)
    }
    #[doc = "Bit 1 - Pulse Output Timer 1 Event Generation Disable"]
    #[inline(always)]
    #[must_use]
    pub fn eltdis1(&mut self) -> ELTDIS1_W<1> {
        ELTDIS1_W::new(self)
    }
    #[doc = "Bit 2 - Pulse Output Timer 2 Event Generation Disable"]
    #[inline(always)]
    #[must_use]
    pub fn eltdis2(&mut self) -> ELTDIS2_W<2> {
        ELTDIS2_W::new(self)
    }
    #[doc = "Bit 3 - Pulse Output Timer 3 Event Generation Disable"]
    #[inline(always)]
    #[must_use]
    pub fn eltdis3(&mut self) -> ELTDIS3_W<3> {
        ELTDIS3_W::new(self)
    }
    #[doc = "Bit 4 - Pulse Output Timer 4 Event Generation Disable"]
    #[inline(always)]
    #[must_use]
    pub fn eltdis4(&mut self) -> ELTDIS4_W<4> {
        ELTDIS4_W::new(self)
    }
    #[doc = "Bit 5 - Pulse Output Timer 5 Event Generation Disable"]
    #[inline(always)]
    #[must_use]
    pub fn eltdis5(&mut self) -> ELTDIS5_W<5> {
        ELTDIS5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ELC Output Timer Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eltselr](index.html) module"]
pub struct ELTSELR_SPEC;
impl crate::RegisterSpec for ELTSELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eltselr::R](R) reader structure"]
impl crate::Readable for ELTSELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eltselr::W](W) writer structure"]
impl crate::Writable for ELTSELR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ELTSELR to value 0"]
impl crate::Resettable for ELTSELR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
