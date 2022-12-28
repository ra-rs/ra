#[doc = "Register `BCCTRL1` reader"]
pub struct R(crate::R<BCCTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCCTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCCTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCCTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BCCTRL1` writer"]
pub struct W(crate::W<BCCTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCCTRL1_SPEC>;
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
impl From<crate::W<BCCTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCCTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RPDME` reader - D- Line Pull-down Control"]
pub type RPDME_R = crate::BitReader<RPDME_A>;
#[doc = "D- Line Pull-down Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPDME_A {
    #[doc = "0: Disable D- Line Pull-down"]
    _0 = 0,
    #[doc = "1: Enable D- Line Pull-down"]
    _1 = 1,
}
impl From<RPDME_A> for bool {
    #[inline(always)]
    fn from(variant: RPDME_A) -> Self {
        variant as u8 != 0
    }
}
impl RPDME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RPDME_A {
        match self.bits {
            false => RPDME_A::_0,
            true => RPDME_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RPDME_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RPDME_A::_1
    }
}
#[doc = "Field `RPDME` writer - D- Line Pull-down Control"]
pub type RPDME_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCCTRL1_SPEC, RPDME_A, O>;
impl<'a, const O: u8> RPDME_W<'a, O> {
    #[doc = "Disable D- Line Pull-down"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RPDME_A::_0)
    }
    #[doc = "Enable D- Line Pull-down"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RPDME_A::_1)
    }
}
#[doc = "Field `IDPSRCE` reader - D+ Line IDPSRC Output Control"]
pub type IDPSRCE_R = crate::BitReader<IDPSRCE_A>;
#[doc = "D+ Line IDPSRC Output Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDPSRCE_A {
    #[doc = "0: Stopped"]
    _0 = 0,
    #[doc = "1: 10 µA output"]
    _1 = 1,
}
impl From<IDPSRCE_A> for bool {
    #[inline(always)]
    fn from(variant: IDPSRCE_A) -> Self {
        variant as u8 != 0
    }
}
impl IDPSRCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDPSRCE_A {
        match self.bits {
            false => IDPSRCE_A::_0,
            true => IDPSRCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IDPSRCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IDPSRCE_A::_1
    }
}
#[doc = "Field `IDPSRCE` writer - D+ Line IDPSRC Output Control"]
pub type IDPSRCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCCTRL1_SPEC, IDPSRCE_A, O>;
impl<'a, const O: u8> IDPSRCE_W<'a, O> {
    #[doc = "Stopped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IDPSRCE_A::_0)
    }
    #[doc = "10 µA output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IDPSRCE_A::_1)
    }
}
#[doc = "Field `VDMSRCE` reader - D- Line VDMSRC (0.6 V) Output Control"]
pub type VDMSRCE_R = crate::BitReader<VDMSRCE_A>;
#[doc = "D- Line VDMSRC (0.6 V) Output Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDMSRCE_A {
    #[doc = "0: Stopped"]
    _0 = 0,
    #[doc = "1: 0.6 V output"]
    _1 = 1,
}
impl From<VDMSRCE_A> for bool {
    #[inline(always)]
    fn from(variant: VDMSRCE_A) -> Self {
        variant as u8 != 0
    }
}
impl VDMSRCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDMSRCE_A {
        match self.bits {
            false => VDMSRCE_A::_0,
            true => VDMSRCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VDMSRCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VDMSRCE_A::_1
    }
}
#[doc = "Field `VDMSRCE` writer - D- Line VDMSRC (0.6 V) Output Control"]
pub type VDMSRCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCCTRL1_SPEC, VDMSRCE_A, O>;
impl<'a, const O: u8> VDMSRCE_W<'a, O> {
    #[doc = "Stopped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VDMSRCE_A::_0)
    }
    #[doc = "0.6 V output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VDMSRCE_A::_1)
    }
}
#[doc = "Field `VDPSRCE` reader - D+ Line VDPSRC (0.6 V) Output Control"]
pub type VDPSRCE_R = crate::BitReader<VDPSRCE_A>;
#[doc = "D+ Line VDPSRC (0.6 V) Output Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDPSRCE_A {
    #[doc = "0: Stopped"]
    _0 = 0,
    #[doc = "1: 0.6 V output"]
    _1 = 1,
}
impl From<VDPSRCE_A> for bool {
    #[inline(always)]
    fn from(variant: VDPSRCE_A) -> Self {
        variant as u8 != 0
    }
}
impl VDPSRCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDPSRCE_A {
        match self.bits {
            false => VDPSRCE_A::_0,
            true => VDPSRCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VDPSRCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VDPSRCE_A::_1
    }
}
#[doc = "Field `VDPSRCE` writer - D+ Line VDPSRC (0.6 V) Output Control"]
pub type VDPSRCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCCTRL1_SPEC, VDPSRCE_A, O>;
impl<'a, const O: u8> VDPSRCE_W<'a, O> {
    #[doc = "Stopped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VDPSRCE_A::_0)
    }
    #[doc = "0.6 V output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VDPSRCE_A::_1)
    }
}
#[doc = "Field `PDDETE` reader - D+ Line 0.6 V Input Detection Control"]
pub type PDDETE_R = crate::BitReader<PDDETE_A>;
#[doc = "D+ Line 0.6 V Input Detection Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDDETE_A {
    #[doc = "0: Disable detection"]
    _0 = 0,
    #[doc = "1: Enable detection"]
    _1 = 1,
}
impl From<PDDETE_A> for bool {
    #[inline(always)]
    fn from(variant: PDDETE_A) -> Self {
        variant as u8 != 0
    }
}
impl PDDETE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDDETE_A {
        match self.bits {
            false => PDDETE_A::_0,
            true => PDDETE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDDETE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDDETE_A::_1
    }
}
#[doc = "Field `PDDETE` writer - D+ Line 0.6 V Input Detection Control"]
pub type PDDETE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCCTRL1_SPEC, PDDETE_A, O>;
impl<'a, const O: u8> PDDETE_W<'a, O> {
    #[doc = "Disable detection"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDDETE_A::_0)
    }
    #[doc = "Enable detection"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDDETE_A::_1)
    }
}
#[doc = "Field `CHGDETE` reader - D- Line 0.6 V Input Detection Control"]
pub type CHGDETE_R = crate::BitReader<CHGDETE_A>;
#[doc = "D- Line 0.6 V Input Detection Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHGDETE_A {
    #[doc = "0: Disable detection"]
    _0 = 0,
    #[doc = "1: Enable detection"]
    _1 = 1,
}
impl From<CHGDETE_A> for bool {
    #[inline(always)]
    fn from(variant: CHGDETE_A) -> Self {
        variant as u8 != 0
    }
}
impl CHGDETE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHGDETE_A {
        match self.bits {
            false => CHGDETE_A::_0,
            true => CHGDETE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHGDETE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHGDETE_A::_1
    }
}
#[doc = "Field `CHGDETE` writer - D- Line 0.6 V Input Detection Control"]
pub type CHGDETE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCCTRL1_SPEC, CHGDETE_A, O>;
impl<'a, const O: u8> CHGDETE_W<'a, O> {
    #[doc = "Disable detection"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHGDETE_A::_0)
    }
    #[doc = "Enable detection"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHGDETE_A::_1)
    }
}
#[doc = "Field `PDDETSTS` reader - D+ Line 0.6 V Input Detection Status Flag"]
pub type PDDETSTS_R = crate::BitReader<PDDETSTS_A>;
#[doc = "D+ Line 0.6 V Input Detection Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDDETSTS_A {
    #[doc = "0: Not detected"]
    _0 = 0,
    #[doc = "1: Detected"]
    _1 = 1,
}
impl From<PDDETSTS_A> for bool {
    #[inline(always)]
    fn from(variant: PDDETSTS_A) -> Self {
        variant as u8 != 0
    }
}
impl PDDETSTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDDETSTS_A {
        match self.bits {
            false => PDDETSTS_A::_0,
            true => PDDETSTS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDDETSTS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDDETSTS_A::_1
    }
}
#[doc = "Field `CHGDETSTS` reader - D- Line 0.6 V Input Detection Status Flag"]
pub type CHGDETSTS_R = crate::BitReader<CHGDETSTS_A>;
#[doc = "D- Line 0.6 V Input Detection Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHGDETSTS_A {
    #[doc = "0: Not detected"]
    _0 = 0,
    #[doc = "1: Detected"]
    _1 = 1,
}
impl From<CHGDETSTS_A> for bool {
    #[inline(always)]
    fn from(variant: CHGDETSTS_A) -> Self {
        variant as u8 != 0
    }
}
impl CHGDETSTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHGDETSTS_A {
        match self.bits {
            false => CHGDETSTS_A::_0,
            true => CHGDETSTS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHGDETSTS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHGDETSTS_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - D- Line Pull-down Control"]
    #[inline(always)]
    pub fn rpdme(&self) -> RPDME_R {
        RPDME_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - D+ Line IDPSRC Output Control"]
    #[inline(always)]
    pub fn idpsrce(&self) -> IDPSRCE_R {
        IDPSRCE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - D- Line VDMSRC (0.6 V) Output Control"]
    #[inline(always)]
    pub fn vdmsrce(&self) -> VDMSRCE_R {
        VDMSRCE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - D+ Line VDPSRC (0.6 V) Output Control"]
    #[inline(always)]
    pub fn vdpsrce(&self) -> VDPSRCE_R {
        VDPSRCE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - D+ Line 0.6 V Input Detection Control"]
    #[inline(always)]
    pub fn pddete(&self) -> PDDETE_R {
        PDDETE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - D- Line 0.6 V Input Detection Control"]
    #[inline(always)]
    pub fn chgdete(&self) -> CHGDETE_R {
        CHGDETE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - D+ Line 0.6 V Input Detection Status Flag"]
    #[inline(always)]
    pub fn pddetsts(&self) -> PDDETSTS_R {
        PDDETSTS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - D- Line 0.6 V Input Detection Status Flag"]
    #[inline(always)]
    pub fn chgdetsts(&self) -> CHGDETSTS_R {
        CHGDETSTS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - D- Line Pull-down Control"]
    #[inline(always)]
    #[must_use]
    pub fn rpdme(&mut self) -> RPDME_W<0> {
        RPDME_W::new(self)
    }
    #[doc = "Bit 1 - D+ Line IDPSRC Output Control"]
    #[inline(always)]
    #[must_use]
    pub fn idpsrce(&mut self) -> IDPSRCE_W<1> {
        IDPSRCE_W::new(self)
    }
    #[doc = "Bit 2 - D- Line VDMSRC (0.6 V) Output Control"]
    #[inline(always)]
    #[must_use]
    pub fn vdmsrce(&mut self) -> VDMSRCE_W<2> {
        VDMSRCE_W::new(self)
    }
    #[doc = "Bit 3 - D+ Line VDPSRC (0.6 V) Output Control"]
    #[inline(always)]
    #[must_use]
    pub fn vdpsrce(&mut self) -> VDPSRCE_W<3> {
        VDPSRCE_W::new(self)
    }
    #[doc = "Bit 4 - D+ Line 0.6 V Input Detection Control"]
    #[inline(always)]
    #[must_use]
    pub fn pddete(&mut self) -> PDDETE_W<4> {
        PDDETE_W::new(self)
    }
    #[doc = "Bit 5 - D- Line 0.6 V Input Detection Control"]
    #[inline(always)]
    #[must_use]
    pub fn chgdete(&mut self) -> CHGDETE_W<5> {
        CHGDETE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Battery Charging Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcctrl1](index.html) module"]
pub struct BCCTRL1_SPEC;
impl crate::RegisterSpec for BCCTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bcctrl1::R](R) reader structure"]
impl crate::Readable for BCCTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bcctrl1::W](W) writer structure"]
impl crate::Writable for BCCTRL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BCCTRL1 to value 0"]
impl crate::Resettable for BCCTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
