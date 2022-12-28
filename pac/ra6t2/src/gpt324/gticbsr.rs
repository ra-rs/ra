#[doc = "Register `GTICBSR` reader"]
pub struct R(crate::R<GTICBSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTICBSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTICBSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTICBSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTICBSR` writer"]
pub struct W(crate::W<GTICBSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTICBSR_SPEC>;
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
impl From<crate::W<GTICBSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTICBSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BSGTRGAR` reader - GTETRGA Pin Rising Input Source GTCCRB Input Capture Enable"]
pub type BSGTRGAR_R = crate::BitReader<BSGTRGAR_A>;
#[doc = "GTETRGA Pin Rising Input Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSGTRGAR_A {
    #[doc = "0: GTCCRB input capture disabled on the rising edge of GTETRGA input"]
    _0 = 0,
    #[doc = "1: GTCCRB input capture enabled on the rising edge of GTETRGA input"]
    _1 = 1,
}
impl From<BSGTRGAR_A> for bool {
    #[inline(always)]
    fn from(variant: BSGTRGAR_A) -> Self {
        variant as u8 != 0
    }
}
impl BSGTRGAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSGTRGAR_A {
        match self.bits {
            false => BSGTRGAR_A::_0,
            true => BSGTRGAR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSGTRGAR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSGTRGAR_A::_1
    }
}
#[doc = "Field `BSGTRGAR` writer - GTETRGA Pin Rising Input Source GTCCRB Input Capture Enable"]
pub type BSGTRGAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICBSR_SPEC, BSGTRGAR_A, O>;
impl<'a, const O: u8> BSGTRGAR_W<'a, O> {
    #[doc = "GTCCRB input capture disabled on the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSGTRGAR_A::_0)
    }
    #[doc = "GTCCRB input capture enabled on the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSGTRGAR_A::_1)
    }
}
#[doc = "Field `BSGTRGAF` reader - GTETRGA Pin Falling Input Source GTCCRB Input Capture Enable"]
pub type BSGTRGAF_R = crate::BitReader<BSGTRGAF_A>;
#[doc = "GTETRGA Pin Falling Input Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSGTRGAF_A {
    #[doc = "0: GTCCRB input capture disabled on the falling edge of GTETRGA input"]
    _0 = 0,
    #[doc = "1: GTCCRB input capture enabled on the falling edge of GTETRGA input"]
    _1 = 1,
}
impl From<BSGTRGAF_A> for bool {
    #[inline(always)]
    fn from(variant: BSGTRGAF_A) -> Self {
        variant as u8 != 0
    }
}
impl BSGTRGAF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSGTRGAF_A {
        match self.bits {
            false => BSGTRGAF_A::_0,
            true => BSGTRGAF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSGTRGAF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSGTRGAF_A::_1
    }
}
#[doc = "Field `BSGTRGAF` writer - GTETRGA Pin Falling Input Source GTCCRB Input Capture Enable"]
pub type BSGTRGAF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICBSR_SPEC, BSGTRGAF_A, O>;
impl<'a, const O: u8> BSGTRGAF_W<'a, O> {
    #[doc = "GTCCRB input capture disabled on the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSGTRGAF_A::_0)
    }
    #[doc = "GTCCRB input capture enabled on the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSGTRGAF_A::_1)
    }
}
#[doc = "Field `BSGTRGBR` reader - GTETRGB Pin Rising Input Source GTCCRB Input Capture Enable"]
pub type BSGTRGBR_R = crate::BitReader<BSGTRGBR_A>;
#[doc = "GTETRGB Pin Rising Input Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSGTRGBR_A {
    #[doc = "0: GTCCRB input capture disabled on the rising edge of GTETRGB input"]
    _0 = 0,
    #[doc = "1: GTCCRB input capture enabled on the rising edge of GTETRGB input"]
    _1 = 1,
}
impl From<BSGTRGBR_A> for bool {
    #[inline(always)]
    fn from(variant: BSGTRGBR_A) -> Self {
        variant as u8 != 0
    }
}
impl BSGTRGBR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSGTRGBR_A {
        match self.bits {
            false => BSGTRGBR_A::_0,
            true => BSGTRGBR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSGTRGBR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSGTRGBR_A::_1
    }
}
#[doc = "Field `BSGTRGBR` writer - GTETRGB Pin Rising Input Source GTCCRB Input Capture Enable"]
pub type BSGTRGBR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICBSR_SPEC, BSGTRGBR_A, O>;
impl<'a, const O: u8> BSGTRGBR_W<'a, O> {
    #[doc = "GTCCRB input capture disabled on the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSGTRGBR_A::_0)
    }
    #[doc = "GTCCRB input capture enabled on the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSGTRGBR_A::_1)
    }
}
#[doc = "Field `BSGTRGBF` reader - GTETRGB Pin Falling Input Source GTCCRB Input Capture Enable"]
pub type BSGTRGBF_R = crate::BitReader<BSGTRGBF_A>;
#[doc = "GTETRGB Pin Falling Input Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSGTRGBF_A {
    #[doc = "0: GTCCRB input capture disabled on the falling edge of GTETRGB input"]
    _0 = 0,
    #[doc = "1: GTCCRB input capture enabled on the falling edge of GTETRGB input"]
    _1 = 1,
}
impl From<BSGTRGBF_A> for bool {
    #[inline(always)]
    fn from(variant: BSGTRGBF_A) -> Self {
        variant as u8 != 0
    }
}
impl BSGTRGBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSGTRGBF_A {
        match self.bits {
            false => BSGTRGBF_A::_0,
            true => BSGTRGBF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSGTRGBF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSGTRGBF_A::_1
    }
}
#[doc = "Field `BSGTRGBF` writer - GTETRGB Pin Falling Input Source GTCCRB Input Capture Enable"]
pub type BSGTRGBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICBSR_SPEC, BSGTRGBF_A, O>;
impl<'a, const O: u8> BSGTRGBF_W<'a, O> {
    #[doc = "GTCCRB input capture disabled on the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSGTRGBF_A::_0)
    }
    #[doc = "GTCCRB input capture enabled on the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSGTRGBF_A::_1)
    }
}
#[doc = "Field `BSGTRGCR` reader - GTETRGC Pin Rising Input Source GTCCRB Input Capture Enable"]
pub type BSGTRGCR_R = crate::BitReader<BSGTRGCR_A>;
#[doc = "GTETRGC Pin Rising Input Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSGTRGCR_A {
    #[doc = "0: GTCCRB input capture disabled on the rising edge of GTETRGC input"]
    _0 = 0,
    #[doc = "1: GTCCRB input capture enabled on the rising edge of GTETRGC input"]
    _1 = 1,
}
impl From<BSGTRGCR_A> for bool {
    #[inline(always)]
    fn from(variant: BSGTRGCR_A) -> Self {
        variant as u8 != 0
    }
}
impl BSGTRGCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSGTRGCR_A {
        match self.bits {
            false => BSGTRGCR_A::_0,
            true => BSGTRGCR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSGTRGCR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSGTRGCR_A::_1
    }
}
#[doc = "Field `BSGTRGCR` writer - GTETRGC Pin Rising Input Source GTCCRB Input Capture Enable"]
pub type BSGTRGCR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICBSR_SPEC, BSGTRGCR_A, O>;
impl<'a, const O: u8> BSGTRGCR_W<'a, O> {
    #[doc = "GTCCRB input capture disabled on the rising edge of GTETRGC input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSGTRGCR_A::_0)
    }
    #[doc = "GTCCRB input capture enabled on the rising edge of GTETRGC input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSGTRGCR_A::_1)
    }
}
#[doc = "Field `BSGTRGCF` reader - GTETRGC Pin Falling Input Source GTCCRB Input Capture Enable"]
pub type BSGTRGCF_R = crate::BitReader<BSGTRGCF_A>;
#[doc = "GTETRGC Pin Falling Input Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSGTRGCF_A {
    #[doc = "0: GTCCRB input capture disabled on the falling edge of GTETRGC input"]
    _0 = 0,
    #[doc = "1: GTCCRB input capture enabled on the falling edge of GTETRGC input"]
    _1 = 1,
}
impl From<BSGTRGCF_A> for bool {
    #[inline(always)]
    fn from(variant: BSGTRGCF_A) -> Self {
        variant as u8 != 0
    }
}
impl BSGTRGCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSGTRGCF_A {
        match self.bits {
            false => BSGTRGCF_A::_0,
            true => BSGTRGCF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSGTRGCF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSGTRGCF_A::_1
    }
}
#[doc = "Field `BSGTRGCF` writer - GTETRGC Pin Falling Input Source GTCCRB Input Capture Enable"]
pub type BSGTRGCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICBSR_SPEC, BSGTRGCF_A, O>;
impl<'a, const O: u8> BSGTRGCF_W<'a, O> {
    #[doc = "GTCCRB input capture disabled on the falling edge of GTETRGC input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSGTRGCF_A::_0)
    }
    #[doc = "GTCCRB input capture enabled on the falling edge of GTETRGC input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSGTRGCF_A::_1)
    }
}
#[doc = "Field `BSGTRGDR` reader - GTETRGD Pin Rising Input Source GTCCRB Input Capture Enable"]
pub type BSGTRGDR_R = crate::BitReader<BSGTRGDR_A>;
#[doc = "GTETRGD Pin Rising Input Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSGTRGDR_A {
    #[doc = "0: GTCCRB input capture disabled on the rising edge of GTETRGD input"]
    _0 = 0,
    #[doc = "1: GTCCRB input capture enabled on the rising edge of GTETRGD input"]
    _1 = 1,
}
impl From<BSGTRGDR_A> for bool {
    #[inline(always)]
    fn from(variant: BSGTRGDR_A) -> Self {
        variant as u8 != 0
    }
}
impl BSGTRGDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSGTRGDR_A {
        match self.bits {
            false => BSGTRGDR_A::_0,
            true => BSGTRGDR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSGTRGDR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSGTRGDR_A::_1
    }
}
#[doc = "Field `BSGTRGDR` writer - GTETRGD Pin Rising Input Source GTCCRB Input Capture Enable"]
pub type BSGTRGDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICBSR_SPEC, BSGTRGDR_A, O>;
impl<'a, const O: u8> BSGTRGDR_W<'a, O> {
    #[doc = "GTCCRB input capture disabled on the rising edge of GTETRGD input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSGTRGDR_A::_0)
    }
    #[doc = "GTCCRB input capture enabled on the rising edge of GTETRGD input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSGTRGDR_A::_1)
    }
}
#[doc = "Field `BSGTRGDF` reader - GTETRGD Pin Falling Input Source GTCCRB Input Capture Enable"]
pub type BSGTRGDF_R = crate::BitReader<BSGTRGDF_A>;
#[doc = "GTETRGD Pin Falling Input Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSGTRGDF_A {
    #[doc = "0: GTCCRB input capture disabled on the falling edge of GTETRGD input"]
    _0 = 0,
    #[doc = "1: GTCCRB input capture enabled on the falling edge of GTETRGD input"]
    _1 = 1,
}
impl From<BSGTRGDF_A> for bool {
    #[inline(always)]
    fn from(variant: BSGTRGDF_A) -> Self {
        variant as u8 != 0
    }
}
impl BSGTRGDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSGTRGDF_A {
        match self.bits {
            false => BSGTRGDF_A::_0,
            true => BSGTRGDF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSGTRGDF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSGTRGDF_A::_1
    }
}
#[doc = "Field `BSGTRGDF` writer - GTETRGD Pin Falling Input Source GTCCRB Input Capture Enable"]
pub type BSGTRGDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICBSR_SPEC, BSGTRGDF_A, O>;
impl<'a, const O: u8> BSGTRGDF_W<'a, O> {
    #[doc = "GTCCRB input capture disabled on the falling edge of GTETRGD input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSGTRGDF_A::_0)
    }
    #[doc = "GTCCRB input capture enabled on the falling edge of GTETRGD input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSGTRGDF_A::_1)
    }
}
#[doc = "Field `BSCARBL` reader - GTIOCnA Pin Rising Input during GTIOCnB Value Low Source GTCCRB Input Capture Enable"]
pub type BSCARBL_R = crate::BitReader<BSCARBL_A>;
#[doc = "GTIOCnA Pin Rising Input during GTIOCnB Value Low Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSCARBL_A {
    #[doc = "0: GTCCRB input capture disabled on the rising edge of GTIOCnA input when GTIOCnB input is 0"]
    _0 = 0,
    #[doc = "1: GTCCRB input capture enabled on the rising edge of GTIOCnA input when GTIOCnB input is 0"]
    _1 = 1,
}
impl From<BSCARBL_A> for bool {
    #[inline(always)]
    fn from(variant: BSCARBL_A) -> Self {
        variant as u8 != 0
    }
}
impl BSCARBL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSCARBL_A {
        match self.bits {
            false => BSCARBL_A::_0,
            true => BSCARBL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSCARBL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSCARBL_A::_1
    }
}
#[doc = "Field `BSCARBL` writer - GTIOCnA Pin Rising Input during GTIOCnB Value Low Source GTCCRB Input Capture Enable"]
pub type BSCARBL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICBSR_SPEC, BSCARBL_A, O>;
impl<'a, const O: u8> BSCARBL_W<'a, O> {
    #[doc = "GTCCRB input capture disabled on the rising edge of GTIOCnA input when GTIOCnB input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSCARBL_A::_0)
    }
    #[doc = "GTCCRB input capture enabled on the rising edge of GTIOCnA input when GTIOCnB input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSCARBL_A::_1)
    }
}
#[doc = "Field `BSCARBH` reader - GTIOCnA Pin Rising Input during GTIOCnB Value High Source GTCCRB Input Capture Enable"]
pub type BSCARBH_R = crate::BitReader<BSCARBH_A>;
#[doc = "GTIOCnA Pin Rising Input during GTIOCnB Value High Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSCARBH_A {
    #[doc = "0: GTCCRB input capture disabled on the rising edge of GTIOCnA input when GTIOCnB input is 1"]
    _0 = 0,
    #[doc = "1: GTCCRB input capture enabled on the rising edge of GTIOCnA input when GTIOCnB input is 1"]
    _1 = 1,
}
impl From<BSCARBH_A> for bool {
    #[inline(always)]
    fn from(variant: BSCARBH_A) -> Self {
        variant as u8 != 0
    }
}
impl BSCARBH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSCARBH_A {
        match self.bits {
            false => BSCARBH_A::_0,
            true => BSCARBH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSCARBH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSCARBH_A::_1
    }
}
#[doc = "Field `BSCARBH` writer - GTIOCnA Pin Rising Input during GTIOCnB Value High Source GTCCRB Input Capture Enable"]
pub type BSCARBH_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICBSR_SPEC, BSCARBH_A, O>;
impl<'a, const O: u8> BSCARBH_W<'a, O> {
    #[doc = "GTCCRB input capture disabled on the rising edge of GTIOCnA input when GTIOCnB input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSCARBH_A::_0)
    }
    #[doc = "GTCCRB input capture enabled on the rising edge of GTIOCnA input when GTIOCnB input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSCARBH_A::_1)
    }
}
#[doc = "Field `BSCAFBL` reader - GTIOCnA Pin Falling Input during GTIOCnB Value Low Source GTCCRB Input Capture Enable"]
pub type BSCAFBL_R = crate::BitReader<BSCAFBL_A>;
#[doc = "GTIOCnA Pin Falling Input during GTIOCnB Value Low Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSCAFBL_A {
    #[doc = "0: GTCCRB input capture disabled on the falling edge of GTIOCnA input when GTIOCnB input is 0"]
    _0 = 0,
    #[doc = "1: GTCCRB input capture enabled on the falling edge of GTIOCnA input when GTIOCnB input is 0"]
    _1 = 1,
}
impl From<BSCAFBL_A> for bool {
    #[inline(always)]
    fn from(variant: BSCAFBL_A) -> Self {
        variant as u8 != 0
    }
}
impl BSCAFBL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSCAFBL_A {
        match self.bits {
            false => BSCAFBL_A::_0,
            true => BSCAFBL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSCAFBL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSCAFBL_A::_1
    }
}
#[doc = "Field `BSCAFBL` writer - GTIOCnA Pin Falling Input during GTIOCnB Value Low Source GTCCRB Input Capture Enable"]
pub type BSCAFBL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICBSR_SPEC, BSCAFBL_A, O>;
impl<'a, const O: u8> BSCAFBL_W<'a, O> {
    #[doc = "GTCCRB input capture disabled on the falling edge of GTIOCnA input when GTIOCnB input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSCAFBL_A::_0)
    }
    #[doc = "GTCCRB input capture enabled on the falling edge of GTIOCnA input when GTIOCnB input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSCAFBL_A::_1)
    }
}
#[doc = "Field `BSCAFBH` reader - GTIOCnA Pin Falling Input during GTIOCnB Value High Source GTCCRB Input Capture Enable"]
pub type BSCAFBH_R = crate::BitReader<BSCAFBH_A>;
#[doc = "GTIOCnA Pin Falling Input during GTIOCnB Value High Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSCAFBH_A {
    #[doc = "0: GTCCRB input capture disabled on the falling edge of GTIOCnA input when GTIOCnB input is 1"]
    _0 = 0,
    #[doc = "1: GTCCRB input capture enabled on the falling edge of GTIOCnA input when GTIOCnB input is 1"]
    _1 = 1,
}
impl From<BSCAFBH_A> for bool {
    #[inline(always)]
    fn from(variant: BSCAFBH_A) -> Self {
        variant as u8 != 0
    }
}
impl BSCAFBH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSCAFBH_A {
        match self.bits {
            false => BSCAFBH_A::_0,
            true => BSCAFBH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSCAFBH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSCAFBH_A::_1
    }
}
#[doc = "Field `BSCAFBH` writer - GTIOCnA Pin Falling Input during GTIOCnB Value High Source GTCCRB Input Capture Enable"]
pub type BSCAFBH_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICBSR_SPEC, BSCAFBH_A, O>;
impl<'a, const O: u8> BSCAFBH_W<'a, O> {
    #[doc = "GTCCRB input capture disabled on the falling edge of GTIOCnA input when GTIOCnB input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSCAFBH_A::_0)
    }
    #[doc = "GTCCRB input capture enabled on the falling edge of GTIOCnA input when GTIOCnB input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSCAFBH_A::_1)
    }
}
#[doc = "Field `BSCBRAL` reader - GTIOCnB Pin Rising Input during GTIOCnA Value Low Source GTCCRB Input Capture Enable"]
pub type BSCBRAL_R = crate::BitReader<BSCBRAL_A>;
#[doc = "GTIOCnB Pin Rising Input during GTIOCnA Value Low Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSCBRAL_A {
    #[doc = "0: GTCCRB input capture disabled on the rising edge of GTIOCnB input when GTIOCnA input is 0"]
    _0 = 0,
    #[doc = "1: GTCCRB input capture enabled on the rising edge of GTIOCnB input when GTIOCnA input is 0"]
    _1 = 1,
}
impl From<BSCBRAL_A> for bool {
    #[inline(always)]
    fn from(variant: BSCBRAL_A) -> Self {
        variant as u8 != 0
    }
}
impl BSCBRAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSCBRAL_A {
        match self.bits {
            false => BSCBRAL_A::_0,
            true => BSCBRAL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSCBRAL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSCBRAL_A::_1
    }
}
#[doc = "Field `BSCBRAL` writer - GTIOCnB Pin Rising Input during GTIOCnA Value Low Source GTCCRB Input Capture Enable"]
pub type BSCBRAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICBSR_SPEC, BSCBRAL_A, O>;
impl<'a, const O: u8> BSCBRAL_W<'a, O> {
    #[doc = "GTCCRB input capture disabled on the rising edge of GTIOCnB input when GTIOCnA input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSCBRAL_A::_0)
    }
    #[doc = "GTCCRB input capture enabled on the rising edge of GTIOCnB input when GTIOCnA input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSCBRAL_A::_1)
    }
}
#[doc = "Field `BSCBRAH` reader - GTIOCnB Pin Rising Input during GTIOCnA Value High Source GTCCRB Input Capture Enable"]
pub type BSCBRAH_R = crate::BitReader<BSCBRAH_A>;
#[doc = "GTIOCnB Pin Rising Input during GTIOCnA Value High Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSCBRAH_A {
    #[doc = "0: GTCCRB input capture disabled on the rising edge of GTIOCnB input when GTIOCnA input is 1"]
    _0 = 0,
    #[doc = "1: GTCCRB input capture enabled on the rising edge of GTIOCnB input when GTIOCnA input is 1"]
    _1 = 1,
}
impl From<BSCBRAH_A> for bool {
    #[inline(always)]
    fn from(variant: BSCBRAH_A) -> Self {
        variant as u8 != 0
    }
}
impl BSCBRAH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSCBRAH_A {
        match self.bits {
            false => BSCBRAH_A::_0,
            true => BSCBRAH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSCBRAH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSCBRAH_A::_1
    }
}
#[doc = "Field `BSCBRAH` writer - GTIOCnB Pin Rising Input during GTIOCnA Value High Source GTCCRB Input Capture Enable"]
pub type BSCBRAH_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICBSR_SPEC, BSCBRAH_A, O>;
impl<'a, const O: u8> BSCBRAH_W<'a, O> {
    #[doc = "GTCCRB input capture disabled on the rising edge of GTIOCnB input when GTIOCnA input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSCBRAH_A::_0)
    }
    #[doc = "GTCCRB input capture enabled on the rising edge of GTIOCnB input when GTIOCnA input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSCBRAH_A::_1)
    }
}
#[doc = "Field `BSCBFAL` reader - GTIOCnB Pin Falling Input during GTIOCnA Value Low Source GTCCRB Input Capture Enable"]
pub type BSCBFAL_R = crate::BitReader<BSCBFAL_A>;
#[doc = "GTIOCnB Pin Falling Input during GTIOCnA Value Low Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSCBFAL_A {
    #[doc = "0: GTCCRB input capture disabled on the falling edge of GTIOCnB input when GTIOCnA input is 0"]
    _0 = 0,
    #[doc = "1: GTCCRB input capture enabled on the falling edge of GTIOCnB input when GTIOCnA input is 0"]
    _1 = 1,
}
impl From<BSCBFAL_A> for bool {
    #[inline(always)]
    fn from(variant: BSCBFAL_A) -> Self {
        variant as u8 != 0
    }
}
impl BSCBFAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSCBFAL_A {
        match self.bits {
            false => BSCBFAL_A::_0,
            true => BSCBFAL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSCBFAL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSCBFAL_A::_1
    }
}
#[doc = "Field `BSCBFAL` writer - GTIOCnB Pin Falling Input during GTIOCnA Value Low Source GTCCRB Input Capture Enable"]
pub type BSCBFAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICBSR_SPEC, BSCBFAL_A, O>;
impl<'a, const O: u8> BSCBFAL_W<'a, O> {
    #[doc = "GTCCRB input capture disabled on the falling edge of GTIOCnB input when GTIOCnA input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSCBFAL_A::_0)
    }
    #[doc = "GTCCRB input capture enabled on the falling edge of GTIOCnB input when GTIOCnA input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSCBFAL_A::_1)
    }
}
#[doc = "Field `BSCBFAH` reader - GTIOCnB Pin Falling Input during GTIOCnA Value High Source GTCCRB Input Capture Enable"]
pub type BSCBFAH_R = crate::BitReader<BSCBFAH_A>;
#[doc = "GTIOCnB Pin Falling Input during GTIOCnA Value High Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSCBFAH_A {
    #[doc = "0: GTCCRB input capture disabled on the falling edge of GTIOCnB input when GTIOCnA input is 1"]
    _0 = 0,
    #[doc = "1: GTCCRB input capture enabled on the falling edge of GTIOCnB input when GTIOCnA input is 1"]
    _1 = 1,
}
impl From<BSCBFAH_A> for bool {
    #[inline(always)]
    fn from(variant: BSCBFAH_A) -> Self {
        variant as u8 != 0
    }
}
impl BSCBFAH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSCBFAH_A {
        match self.bits {
            false => BSCBFAH_A::_0,
            true => BSCBFAH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSCBFAH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSCBFAH_A::_1
    }
}
#[doc = "Field `BSCBFAH` writer - GTIOCnB Pin Falling Input during GTIOCnA Value High Source GTCCRB Input Capture Enable"]
pub type BSCBFAH_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICBSR_SPEC, BSCBFAH_A, O>;
impl<'a, const O: u8> BSCBFAH_W<'a, O> {
    #[doc = "GTCCRB input capture disabled on the falling edge of GTIOCnB input when GTIOCnA input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSCBFAH_A::_0)
    }
    #[doc = "GTCCRB input capture enabled on the falling edge of GTIOCnB input when GTIOCnA input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSCBFAH_A::_1)
    }
}
#[doc = "Field `BSELCA` reader - ELC_GPTA Event Source GTCCRB Input Capture Enable"]
pub type BSELCA_R = crate::BitReader<BSELCA_A>;
#[doc = "ELC_GPTA Event Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSELCA_A {
    #[doc = "0: GTCCRB input capture disabled at the ELC_GPTA input"]
    _0 = 0,
    #[doc = "1: GTCCRB input capture enabled at the ELC_GPTA input"]
    _1 = 1,
}
impl From<BSELCA_A> for bool {
    #[inline(always)]
    fn from(variant: BSELCA_A) -> Self {
        variant as u8 != 0
    }
}
impl BSELCA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSELCA_A {
        match self.bits {
            false => BSELCA_A::_0,
            true => BSELCA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSELCA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSELCA_A::_1
    }
}
#[doc = "Field `BSELCA` writer - ELC_GPTA Event Source GTCCRB Input Capture Enable"]
pub type BSELCA_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICBSR_SPEC, BSELCA_A, O>;
impl<'a, const O: u8> BSELCA_W<'a, O> {
    #[doc = "GTCCRB input capture disabled at the ELC_GPTA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSELCA_A::_0)
    }
    #[doc = "GTCCRB input capture enabled at the ELC_GPTA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSELCA_A::_1)
    }
}
#[doc = "Field `BSELCB` reader - ELC_GPTB Event Source GTCCRB Input Capture Enable"]
pub type BSELCB_R = crate::BitReader<BSELCB_A>;
#[doc = "ELC_GPTB Event Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSELCB_A {
    #[doc = "0: GTCCRB input capture disabled at the ELC_GPTB input"]
    _0 = 0,
    #[doc = "1: GTCCRB input capture enabled at the ELC_GPTB input"]
    _1 = 1,
}
impl From<BSELCB_A> for bool {
    #[inline(always)]
    fn from(variant: BSELCB_A) -> Self {
        variant as u8 != 0
    }
}
impl BSELCB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSELCB_A {
        match self.bits {
            false => BSELCB_A::_0,
            true => BSELCB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSELCB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSELCB_A::_1
    }
}
#[doc = "Field `BSELCB` writer - ELC_GPTB Event Source GTCCRB Input Capture Enable"]
pub type BSELCB_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICBSR_SPEC, BSELCB_A, O>;
impl<'a, const O: u8> BSELCB_W<'a, O> {
    #[doc = "GTCCRB input capture disabled at the ELC_GPTB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSELCB_A::_0)
    }
    #[doc = "GTCCRB input capture enabled at the ELC_GPTB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSELCB_A::_1)
    }
}
#[doc = "Field `BSELCC` reader - ELC_GPTC Event Source GTCCRB Input Capture Enable"]
pub type BSELCC_R = crate::BitReader<BSELCC_A>;
#[doc = "ELC_GPTC Event Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSELCC_A {
    #[doc = "0: GTCCRB input capture disabled at the ELC_GPTC input"]
    _0 = 0,
    #[doc = "1: GTCCRB input capture enabled at the ELC_GPTC input"]
    _1 = 1,
}
impl From<BSELCC_A> for bool {
    #[inline(always)]
    fn from(variant: BSELCC_A) -> Self {
        variant as u8 != 0
    }
}
impl BSELCC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSELCC_A {
        match self.bits {
            false => BSELCC_A::_0,
            true => BSELCC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSELCC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSELCC_A::_1
    }
}
#[doc = "Field `BSELCC` writer - ELC_GPTC Event Source GTCCRB Input Capture Enable"]
pub type BSELCC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICBSR_SPEC, BSELCC_A, O>;
impl<'a, const O: u8> BSELCC_W<'a, O> {
    #[doc = "GTCCRB input capture disabled at the ELC_GPTC input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSELCC_A::_0)
    }
    #[doc = "GTCCRB input capture enabled at the ELC_GPTC input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSELCC_A::_1)
    }
}
#[doc = "Field `BSELCD` reader - ELC_GPTD Event Source GTCCRB Input Capture Enable"]
pub type BSELCD_R = crate::BitReader<BSELCD_A>;
#[doc = "ELC_GPTD Event Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSELCD_A {
    #[doc = "0: GTCCRB input capture disabled at the ELC_GPTD input"]
    _0 = 0,
    #[doc = "1: GTCCRB input capture enabled at the ELC_GPTD input"]
    _1 = 1,
}
impl From<BSELCD_A> for bool {
    #[inline(always)]
    fn from(variant: BSELCD_A) -> Self {
        variant as u8 != 0
    }
}
impl BSELCD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSELCD_A {
        match self.bits {
            false => BSELCD_A::_0,
            true => BSELCD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSELCD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSELCD_A::_1
    }
}
#[doc = "Field `BSELCD` writer - ELC_GPTD Event Source GTCCRB Input Capture Enable"]
pub type BSELCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICBSR_SPEC, BSELCD_A, O>;
impl<'a, const O: u8> BSELCD_W<'a, O> {
    #[doc = "GTCCRB input capture disabled at the ELC_GPTD input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSELCD_A::_0)
    }
    #[doc = "GTCCRB input capture enabled at the ELC_GPTD input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSELCD_A::_1)
    }
}
#[doc = "Field `BSELCE` reader - ELC_GPTE Event Source GTCCRB Input Capture Enable"]
pub type BSELCE_R = crate::BitReader<BSELCE_A>;
#[doc = "ELC_GPTE Event Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSELCE_A {
    #[doc = "0: GTCCRB input capture disabled at the ELC_GPTE input"]
    _0 = 0,
    #[doc = "1: GTCCRB input capture enabled at the ELC_GPTE input"]
    _1 = 1,
}
impl From<BSELCE_A> for bool {
    #[inline(always)]
    fn from(variant: BSELCE_A) -> Self {
        variant as u8 != 0
    }
}
impl BSELCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSELCE_A {
        match self.bits {
            false => BSELCE_A::_0,
            true => BSELCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSELCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSELCE_A::_1
    }
}
#[doc = "Field `BSELCE` writer - ELC_GPTE Event Source GTCCRB Input Capture Enable"]
pub type BSELCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICBSR_SPEC, BSELCE_A, O>;
impl<'a, const O: u8> BSELCE_W<'a, O> {
    #[doc = "GTCCRB input capture disabled at the ELC_GPTE input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSELCE_A::_0)
    }
    #[doc = "GTCCRB input capture enabled at the ELC_GPTE input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSELCE_A::_1)
    }
}
#[doc = "Field `BSELCF` reader - ELC_GPTF Event Source GTCCRB Input Capture Enable"]
pub type BSELCF_R = crate::BitReader<BSELCF_A>;
#[doc = "ELC_GPTF Event Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSELCF_A {
    #[doc = "0: GTCCRB input capture disabled at the ELC_GPTF input"]
    _0 = 0,
    #[doc = "1: GTCCRB input capture enabled at the ELC_GPTF input"]
    _1 = 1,
}
impl From<BSELCF_A> for bool {
    #[inline(always)]
    fn from(variant: BSELCF_A) -> Self {
        variant as u8 != 0
    }
}
impl BSELCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSELCF_A {
        match self.bits {
            false => BSELCF_A::_0,
            true => BSELCF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSELCF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSELCF_A::_1
    }
}
#[doc = "Field `BSELCF` writer - ELC_GPTF Event Source GTCCRB Input Capture Enable"]
pub type BSELCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICBSR_SPEC, BSELCF_A, O>;
impl<'a, const O: u8> BSELCF_W<'a, O> {
    #[doc = "GTCCRB input capture disabled at the ELC_GPTF input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSELCF_A::_0)
    }
    #[doc = "GTCCRB input capture enabled at the ELC_GPTF input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSELCF_A::_1)
    }
}
#[doc = "Field `BSELCG` reader - ELC_GPTG Event Source GTCCRB Input Capture Enable"]
pub type BSELCG_R = crate::BitReader<BSELCG_A>;
#[doc = "ELC_GPTG Event Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSELCG_A {
    #[doc = "0: GTCCRB input capture disabled at the ELC_GPTG input"]
    _0 = 0,
    #[doc = "1: GTCCRB input capture enabled at the ELC_GPTG input"]
    _1 = 1,
}
impl From<BSELCG_A> for bool {
    #[inline(always)]
    fn from(variant: BSELCG_A) -> Self {
        variant as u8 != 0
    }
}
impl BSELCG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSELCG_A {
        match self.bits {
            false => BSELCG_A::_0,
            true => BSELCG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSELCG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSELCG_A::_1
    }
}
#[doc = "Field `BSELCG` writer - ELC_GPTG Event Source GTCCRB Input Capture Enable"]
pub type BSELCG_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICBSR_SPEC, BSELCG_A, O>;
impl<'a, const O: u8> BSELCG_W<'a, O> {
    #[doc = "GTCCRB input capture disabled at the ELC_GPTG input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSELCG_A::_0)
    }
    #[doc = "GTCCRB input capture enabled at the ELC_GPTG input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSELCG_A::_1)
    }
}
#[doc = "Field `BSELCH` reader - ELC_GPTH Event Source GTCCRB Input Capture Enable"]
pub type BSELCH_R = crate::BitReader<BSELCH_A>;
#[doc = "ELC_GPTH Event Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSELCH_A {
    #[doc = "0: GTCCRB input capture disabled at the ELC_GPTH input"]
    _0 = 0,
    #[doc = "1: GTCCRB input capture enabled at the ELC_GPTH input"]
    _1 = 1,
}
impl From<BSELCH_A> for bool {
    #[inline(always)]
    fn from(variant: BSELCH_A) -> Self {
        variant as u8 != 0
    }
}
impl BSELCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSELCH_A {
        match self.bits {
            false => BSELCH_A::_0,
            true => BSELCH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSELCH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSELCH_A::_1
    }
}
#[doc = "Field `BSELCH` writer - ELC_GPTH Event Source GTCCRB Input Capture Enable"]
pub type BSELCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICBSR_SPEC, BSELCH_A, O>;
impl<'a, const O: u8> BSELCH_W<'a, O> {
    #[doc = "GTCCRB input capture disabled at the ELC_GPTH input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSELCH_A::_0)
    }
    #[doc = "GTCCRB input capture enabled at the ELC_GPTH input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSELCH_A::_1)
    }
}
#[doc = "Field `BSOC` reader - Other channel Source GTCCRB Input Capture Enable"]
pub type BSOC_R = crate::BitReader<BSOC_A>;
#[doc = "Other channel Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSOC_A {
    #[doc = "0: Disables GTCCRB input capture by other channel factor"]
    _0 = 0,
    #[doc = "1: Enables GTCCRB input capture by other channel factor"]
    _1 = 1,
}
impl From<BSOC_A> for bool {
    #[inline(always)]
    fn from(variant: BSOC_A) -> Self {
        variant as u8 != 0
    }
}
impl BSOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSOC_A {
        match self.bits {
            false => BSOC_A::_0,
            true => BSOC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSOC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSOC_A::_1
    }
}
#[doc = "Field `BSOC` writer - Other channel Source GTCCRB Input Capture Enable"]
pub type BSOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICBSR_SPEC, BSOC_A, O>;
impl<'a, const O: u8> BSOC_W<'a, O> {
    #[doc = "Disables GTCCRB input capture by other channel factor"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSOC_A::_0)
    }
    #[doc = "Enables GTCCRB input capture by other channel factor"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSOC_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - GTETRGA Pin Rising Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bsgtrgar(&self) -> BSGTRGAR_R {
        BSGTRGAR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GTETRGA Pin Falling Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bsgtrgaf(&self) -> BSGTRGAF_R {
        BSGTRGAF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GTETRGB Pin Rising Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bsgtrgbr(&self) -> BSGTRGBR_R {
        BSGTRGBR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GTETRGB Pin Falling Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bsgtrgbf(&self) -> BSGTRGBF_R {
        BSGTRGBF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GTETRGC Pin Rising Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bsgtrgcr(&self) -> BSGTRGCR_R {
        BSGTRGCR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GTETRGC Pin Falling Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bsgtrgcf(&self) -> BSGTRGCF_R {
        BSGTRGCF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GTETRGD Pin Rising Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bsgtrgdr(&self) -> BSGTRGDR_R {
        BSGTRGDR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GTETRGD Pin Falling Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bsgtrgdf(&self) -> BSGTRGDF_R {
        BSGTRGDF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GTIOCnA Pin Rising Input during GTIOCnB Value Low Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscarbl(&self) -> BSCARBL_R {
        BSCARBL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GTIOCnA Pin Rising Input during GTIOCnB Value High Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscarbh(&self) -> BSCARBH_R {
        BSCARBH_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GTIOCnA Pin Falling Input during GTIOCnB Value Low Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscafbl(&self) -> BSCAFBL_R {
        BSCAFBL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GTIOCnA Pin Falling Input during GTIOCnB Value High Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscafbh(&self) -> BSCAFBH_R {
        BSCAFBH_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GTIOCnB Pin Rising Input during GTIOCnA Value Low Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscbral(&self) -> BSCBRAL_R {
        BSCBRAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GTIOCnB Pin Rising Input during GTIOCnA Value High Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscbrah(&self) -> BSCBRAH_R {
        BSCBRAH_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GTIOCnB Pin Falling Input during GTIOCnA Value Low Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscbfal(&self) -> BSCBFAL_R {
        BSCBFAL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GTIOCnB Pin Falling Input during GTIOCnA Value High Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscbfah(&self) -> BSCBFAH_R {
        BSCBFAH_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ELC_GPTA Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bselca(&self) -> BSELCA_R {
        BSELCA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ELC_GPTB Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bselcb(&self) -> BSELCB_R {
        BSELCB_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ELC_GPTC Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bselcc(&self) -> BSELCC_R {
        BSELCC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ELC_GPTD Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bselcd(&self) -> BSELCD_R {
        BSELCD_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ELC_GPTE Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bselce(&self) -> BSELCE_R {
        BSELCE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ELC_GPTF Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bselcf(&self) -> BSELCF_R {
        BSELCF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ELC_GPTG Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bselcg(&self) -> BSELCG_R {
        BSELCG_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ELC_GPTH Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bselch(&self) -> BSELCH_R {
        BSELCH_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Other channel Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bsoc(&self) -> BSOC_R {
        BSOC_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GTETRGA Pin Rising Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bsgtrgar(&mut self) -> BSGTRGAR_W<0> {
        BSGTRGAR_W::new(self)
    }
    #[doc = "Bit 1 - GTETRGA Pin Falling Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bsgtrgaf(&mut self) -> BSGTRGAF_W<1> {
        BSGTRGAF_W::new(self)
    }
    #[doc = "Bit 2 - GTETRGB Pin Rising Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bsgtrgbr(&mut self) -> BSGTRGBR_W<2> {
        BSGTRGBR_W::new(self)
    }
    #[doc = "Bit 3 - GTETRGB Pin Falling Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bsgtrgbf(&mut self) -> BSGTRGBF_W<3> {
        BSGTRGBF_W::new(self)
    }
    #[doc = "Bit 4 - GTETRGC Pin Rising Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bsgtrgcr(&mut self) -> BSGTRGCR_W<4> {
        BSGTRGCR_W::new(self)
    }
    #[doc = "Bit 5 - GTETRGC Pin Falling Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bsgtrgcf(&mut self) -> BSGTRGCF_W<5> {
        BSGTRGCF_W::new(self)
    }
    #[doc = "Bit 6 - GTETRGD Pin Rising Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bsgtrgdr(&mut self) -> BSGTRGDR_W<6> {
        BSGTRGDR_W::new(self)
    }
    #[doc = "Bit 7 - GTETRGD Pin Falling Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bsgtrgdf(&mut self) -> BSGTRGDF_W<7> {
        BSGTRGDF_W::new(self)
    }
    #[doc = "Bit 8 - GTIOCnA Pin Rising Input during GTIOCnB Value Low Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bscarbl(&mut self) -> BSCARBL_W<8> {
        BSCARBL_W::new(self)
    }
    #[doc = "Bit 9 - GTIOCnA Pin Rising Input during GTIOCnB Value High Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bscarbh(&mut self) -> BSCARBH_W<9> {
        BSCARBH_W::new(self)
    }
    #[doc = "Bit 10 - GTIOCnA Pin Falling Input during GTIOCnB Value Low Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bscafbl(&mut self) -> BSCAFBL_W<10> {
        BSCAFBL_W::new(self)
    }
    #[doc = "Bit 11 - GTIOCnA Pin Falling Input during GTIOCnB Value High Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bscafbh(&mut self) -> BSCAFBH_W<11> {
        BSCAFBH_W::new(self)
    }
    #[doc = "Bit 12 - GTIOCnB Pin Rising Input during GTIOCnA Value Low Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bscbral(&mut self) -> BSCBRAL_W<12> {
        BSCBRAL_W::new(self)
    }
    #[doc = "Bit 13 - GTIOCnB Pin Rising Input during GTIOCnA Value High Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bscbrah(&mut self) -> BSCBRAH_W<13> {
        BSCBRAH_W::new(self)
    }
    #[doc = "Bit 14 - GTIOCnB Pin Falling Input during GTIOCnA Value Low Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bscbfal(&mut self) -> BSCBFAL_W<14> {
        BSCBFAL_W::new(self)
    }
    #[doc = "Bit 15 - GTIOCnB Pin Falling Input during GTIOCnA Value High Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bscbfah(&mut self) -> BSCBFAH_W<15> {
        BSCBFAH_W::new(self)
    }
    #[doc = "Bit 16 - ELC_GPTA Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bselca(&mut self) -> BSELCA_W<16> {
        BSELCA_W::new(self)
    }
    #[doc = "Bit 17 - ELC_GPTB Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bselcb(&mut self) -> BSELCB_W<17> {
        BSELCB_W::new(self)
    }
    #[doc = "Bit 18 - ELC_GPTC Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bselcc(&mut self) -> BSELCC_W<18> {
        BSELCC_W::new(self)
    }
    #[doc = "Bit 19 - ELC_GPTD Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bselcd(&mut self) -> BSELCD_W<19> {
        BSELCD_W::new(self)
    }
    #[doc = "Bit 20 - ELC_GPTE Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bselce(&mut self) -> BSELCE_W<20> {
        BSELCE_W::new(self)
    }
    #[doc = "Bit 21 - ELC_GPTF Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bselcf(&mut self) -> BSELCF_W<21> {
        BSELCF_W::new(self)
    }
    #[doc = "Bit 22 - ELC_GPTG Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bselcg(&mut self) -> BSELCG_W<22> {
        BSELCG_W::new(self)
    }
    #[doc = "Bit 23 - ELC_GPTH Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bselch(&mut self) -> BSELCH_W<23> {
        BSELCH_W::new(self)
    }
    #[doc = "Bit 24 - Other channel Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bsoc(&mut self) -> BSOC_W<24> {
        BSOC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Input Capture Source Select Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gticbsr](index.html) module"]
pub struct GTICBSR_SPEC;
impl crate::RegisterSpec for GTICBSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gticbsr::R](R) reader structure"]
impl crate::Readable for GTICBSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gticbsr::W](W) writer structure"]
impl crate::Writable for GTICBSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTICBSR to value 0"]
impl crate::Resettable for GTICBSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
