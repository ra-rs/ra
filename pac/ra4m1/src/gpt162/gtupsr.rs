#[doc = "Register `GTUPSR` reader"]
pub struct R(crate::R<GTUPSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTUPSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTUPSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTUPSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTUPSR` writer"]
pub struct W(crate::W<GTUPSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTUPSR_SPEC>;
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
impl From<crate::W<GTUPSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTUPSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USGTRGAR` reader - GTETRGA Pin Rising Input Source Counter Count Up Enable"]
pub type USGTRGAR_R = crate::BitReader<USGTRGAR_A>;
#[doc = "GTETRGA Pin Rising Input Source Counter Count Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USGTRGAR_A {
    #[doc = "0: Counter count up is disable at the rising edge of GTETRGA input"]
    _0 = 0,
    #[doc = "1: Counter count up is enable at the rising edge of GTETRGA input"]
    _1 = 1,
}
impl From<USGTRGAR_A> for bool {
    #[inline(always)]
    fn from(variant: USGTRGAR_A) -> Self {
        variant as u8 != 0
    }
}
impl USGTRGAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USGTRGAR_A {
        match self.bits {
            false => USGTRGAR_A::_0,
            true => USGTRGAR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USGTRGAR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USGTRGAR_A::_1
    }
}
#[doc = "Field `USGTRGAR` writer - GTETRGA Pin Rising Input Source Counter Count Up Enable"]
pub type USGTRGAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTUPSR_SPEC, USGTRGAR_A, O>;
impl<'a, const O: u8> USGTRGAR_W<'a, O> {
    #[doc = "Counter count up is disable at the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USGTRGAR_A::_0)
    }
    #[doc = "Counter count up is enable at the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USGTRGAR_A::_1)
    }
}
#[doc = "Field `USGTRGAF` reader - GTETRGA Pin Falling Input Source Counter Count Up Enable"]
pub type USGTRGAF_R = crate::BitReader<USGTRGAF_A>;
#[doc = "GTETRGA Pin Falling Input Source Counter Count Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USGTRGAF_A {
    #[doc = "0: Counter count up is disable at the falling edge of GTETRGA input"]
    _0 = 0,
    #[doc = "1: Counter count up is enable at the falling edge of GTETRGA input"]
    _1 = 1,
}
impl From<USGTRGAF_A> for bool {
    #[inline(always)]
    fn from(variant: USGTRGAF_A) -> Self {
        variant as u8 != 0
    }
}
impl USGTRGAF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USGTRGAF_A {
        match self.bits {
            false => USGTRGAF_A::_0,
            true => USGTRGAF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USGTRGAF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USGTRGAF_A::_1
    }
}
#[doc = "Field `USGTRGAF` writer - GTETRGA Pin Falling Input Source Counter Count Up Enable"]
pub type USGTRGAF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTUPSR_SPEC, USGTRGAF_A, O>;
impl<'a, const O: u8> USGTRGAF_W<'a, O> {
    #[doc = "Counter count up is disable at the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USGTRGAF_A::_0)
    }
    #[doc = "Counter count up is enable at the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USGTRGAF_A::_1)
    }
}
#[doc = "Field `USGTRGBR` reader - GTETRGB Pin Rising Input Source Counter Count Up Enable"]
pub type USGTRGBR_R = crate::BitReader<USGTRGBR_A>;
#[doc = "GTETRGB Pin Rising Input Source Counter Count Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USGTRGBR_A {
    #[doc = "0: Counter count up is disable at the rising edge of GTETRGB input"]
    _0 = 0,
    #[doc = "1: Counter count up is enable at the rising edge of GTETRGB input"]
    _1 = 1,
}
impl From<USGTRGBR_A> for bool {
    #[inline(always)]
    fn from(variant: USGTRGBR_A) -> Self {
        variant as u8 != 0
    }
}
impl USGTRGBR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USGTRGBR_A {
        match self.bits {
            false => USGTRGBR_A::_0,
            true => USGTRGBR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USGTRGBR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USGTRGBR_A::_1
    }
}
#[doc = "Field `USGTRGBR` writer - GTETRGB Pin Rising Input Source Counter Count Up Enable"]
pub type USGTRGBR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTUPSR_SPEC, USGTRGBR_A, O>;
impl<'a, const O: u8> USGTRGBR_W<'a, O> {
    #[doc = "Counter count up is disable at the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USGTRGBR_A::_0)
    }
    #[doc = "Counter count up is enable at the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USGTRGBR_A::_1)
    }
}
#[doc = "Field `USGTRGBF` reader - GTETRGB Pin Falling Input Source Counter Count Up Enable"]
pub type USGTRGBF_R = crate::BitReader<USGTRGBF_A>;
#[doc = "GTETRGB Pin Falling Input Source Counter Count Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USGTRGBF_A {
    #[doc = "0: Counter count up is disable at the falling edge of GTETRGB input"]
    _0 = 0,
    #[doc = "1: Counter count up is enable at the falling edge of GTETRGB input"]
    _1 = 1,
}
impl From<USGTRGBF_A> for bool {
    #[inline(always)]
    fn from(variant: USGTRGBF_A) -> Self {
        variant as u8 != 0
    }
}
impl USGTRGBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USGTRGBF_A {
        match self.bits {
            false => USGTRGBF_A::_0,
            true => USGTRGBF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USGTRGBF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USGTRGBF_A::_1
    }
}
#[doc = "Field `USGTRGBF` writer - GTETRGB Pin Falling Input Source Counter Count Up Enable"]
pub type USGTRGBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTUPSR_SPEC, USGTRGBF_A, O>;
impl<'a, const O: u8> USGTRGBF_W<'a, O> {
    #[doc = "Counter count up is disable at the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USGTRGBF_A::_0)
    }
    #[doc = "Counter count up is enable at the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USGTRGBF_A::_1)
    }
}
#[doc = "Field `USCARBL` reader - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Count Up Enable"]
pub type USCARBL_R = crate::BitReader<USCARBL_A>;
#[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Count Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USCARBL_A {
    #[doc = "0: Counter count up is disable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    _0 = 0,
    #[doc = "1: Counter count up is enable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    _1 = 1,
}
impl From<USCARBL_A> for bool {
    #[inline(always)]
    fn from(variant: USCARBL_A) -> Self {
        variant as u8 != 0
    }
}
impl USCARBL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USCARBL_A {
        match self.bits {
            false => USCARBL_A::_0,
            true => USCARBL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USCARBL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USCARBL_A::_1
    }
}
#[doc = "Field `USCARBL` writer - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Count Up Enable"]
pub type USCARBL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTUPSR_SPEC, USCARBL_A, O>;
impl<'a, const O: u8> USCARBL_W<'a, O> {
    #[doc = "Counter count up is disable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USCARBL_A::_0)
    }
    #[doc = "Counter count up is enable at the rising edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USCARBL_A::_1)
    }
}
#[doc = "Field `USCARBH` reader - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Count Up Enable"]
pub type USCARBH_R = crate::BitReader<USCARBH_A>;
#[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Count Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USCARBH_A {
    #[doc = "0: Counter count up is disable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    _0 = 0,
    #[doc = "1: Counter count up is enable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    _1 = 1,
}
impl From<USCARBH_A> for bool {
    #[inline(always)]
    fn from(variant: USCARBH_A) -> Self {
        variant as u8 != 0
    }
}
impl USCARBH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USCARBH_A {
        match self.bits {
            false => USCARBH_A::_0,
            true => USCARBH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USCARBH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USCARBH_A::_1
    }
}
#[doc = "Field `USCARBH` writer - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Count Up Enable"]
pub type USCARBH_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTUPSR_SPEC, USCARBH_A, O>;
impl<'a, const O: u8> USCARBH_W<'a, O> {
    #[doc = "Counter count up is disable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USCARBH_A::_0)
    }
    #[doc = "Counter count up is enable at the rising edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USCARBH_A::_1)
    }
}
#[doc = "Field `USCAFBL` reader - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Count Up Enable"]
pub type USCAFBL_R = crate::BitReader<USCAFBL_A>;
#[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Count Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USCAFBL_A {
    #[doc = "0: Counter count up is disable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    _0 = 0,
    #[doc = "1: Counter count up is enable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    _1 = 1,
}
impl From<USCAFBL_A> for bool {
    #[inline(always)]
    fn from(variant: USCAFBL_A) -> Self {
        variant as u8 != 0
    }
}
impl USCAFBL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USCAFBL_A {
        match self.bits {
            false => USCAFBL_A::_0,
            true => USCAFBL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USCAFBL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USCAFBL_A::_1
    }
}
#[doc = "Field `USCAFBL` writer - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Count Up Enable"]
pub type USCAFBL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTUPSR_SPEC, USCAFBL_A, O>;
impl<'a, const O: u8> USCAFBL_W<'a, O> {
    #[doc = "Counter count up is disable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USCAFBL_A::_0)
    }
    #[doc = "Counter count up is enable at the falling edge of GTIOCA input when GTIOCB input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USCAFBL_A::_1)
    }
}
#[doc = "Field `USCAFBH` reader - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Count Up Enable"]
pub type USCAFBH_R = crate::BitReader<USCAFBH_A>;
#[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Count Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USCAFBH_A {
    #[doc = "0: Counter count up is disable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    _0 = 0,
    #[doc = "1: Counter count up is enable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    _1 = 1,
}
impl From<USCAFBH_A> for bool {
    #[inline(always)]
    fn from(variant: USCAFBH_A) -> Self {
        variant as u8 != 0
    }
}
impl USCAFBH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USCAFBH_A {
        match self.bits {
            false => USCAFBH_A::_0,
            true => USCAFBH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USCAFBH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USCAFBH_A::_1
    }
}
#[doc = "Field `USCAFBH` writer - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Count Up Enable"]
pub type USCAFBH_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTUPSR_SPEC, USCAFBH_A, O>;
impl<'a, const O: u8> USCAFBH_W<'a, O> {
    #[doc = "Counter count up is disable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USCAFBH_A::_0)
    }
    #[doc = "Counter count up is enable at the falling edge of GTIOCA input when GTIOCB input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USCAFBH_A::_1)
    }
}
#[doc = "Field `USCBRAL` reader - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Count Up Enable"]
pub type USCBRAL_R = crate::BitReader<USCBRAL_A>;
#[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Count Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USCBRAL_A {
    #[doc = "0: Counter count up is disable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    _0 = 0,
    #[doc = "1: Counter count up is enable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    _1 = 1,
}
impl From<USCBRAL_A> for bool {
    #[inline(always)]
    fn from(variant: USCBRAL_A) -> Self {
        variant as u8 != 0
    }
}
impl USCBRAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USCBRAL_A {
        match self.bits {
            false => USCBRAL_A::_0,
            true => USCBRAL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USCBRAL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USCBRAL_A::_1
    }
}
#[doc = "Field `USCBRAL` writer - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Count Up Enable"]
pub type USCBRAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTUPSR_SPEC, USCBRAL_A, O>;
impl<'a, const O: u8> USCBRAL_W<'a, O> {
    #[doc = "Counter count up is disable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USCBRAL_A::_0)
    }
    #[doc = "Counter count up is enable at the rising edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USCBRAL_A::_1)
    }
}
#[doc = "Field `USCBRAH` reader - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Count Up Enable"]
pub type USCBRAH_R = crate::BitReader<USCBRAH_A>;
#[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Count Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USCBRAH_A {
    #[doc = "0: Counter count up is disable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    _0 = 0,
    #[doc = "1: Counter count up is enable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    _1 = 1,
}
impl From<USCBRAH_A> for bool {
    #[inline(always)]
    fn from(variant: USCBRAH_A) -> Self {
        variant as u8 != 0
    }
}
impl USCBRAH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USCBRAH_A {
        match self.bits {
            false => USCBRAH_A::_0,
            true => USCBRAH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USCBRAH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USCBRAH_A::_1
    }
}
#[doc = "Field `USCBRAH` writer - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Count Up Enable"]
pub type USCBRAH_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTUPSR_SPEC, USCBRAH_A, O>;
impl<'a, const O: u8> USCBRAH_W<'a, O> {
    #[doc = "Counter count up is disable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USCBRAH_A::_0)
    }
    #[doc = "Counter count up is enable at the rising edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USCBRAH_A::_1)
    }
}
#[doc = "Field `USCBFAL` reader - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Count Up Enable"]
pub type USCBFAL_R = crate::BitReader<USCBFAL_A>;
#[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Count Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USCBFAL_A {
    #[doc = "0: Counter count up is disable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    _0 = 0,
    #[doc = "1: Counter count up is enable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    _1 = 1,
}
impl From<USCBFAL_A> for bool {
    #[inline(always)]
    fn from(variant: USCBFAL_A) -> Self {
        variant as u8 != 0
    }
}
impl USCBFAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USCBFAL_A {
        match self.bits {
            false => USCBFAL_A::_0,
            true => USCBFAL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USCBFAL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USCBFAL_A::_1
    }
}
#[doc = "Field `USCBFAL` writer - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Count Up Enable"]
pub type USCBFAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTUPSR_SPEC, USCBFAL_A, O>;
impl<'a, const O: u8> USCBFAL_W<'a, O> {
    #[doc = "Counter count up is disable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USCBFAL_A::_0)
    }
    #[doc = "Counter count up is enable at the falling edge of GTIOCB input when GTIOCA input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USCBFAL_A::_1)
    }
}
#[doc = "Field `USCBFAH` reader - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Count Up Enable"]
pub type USCBFAH_R = crate::BitReader<USCBFAH_A>;
#[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Count Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USCBFAH_A {
    #[doc = "0: Counter count up is disable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    _0 = 0,
    #[doc = "1: Counter count up is enable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    _1 = 1,
}
impl From<USCBFAH_A> for bool {
    #[inline(always)]
    fn from(variant: USCBFAH_A) -> Self {
        variant as u8 != 0
    }
}
impl USCBFAH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USCBFAH_A {
        match self.bits {
            false => USCBFAH_A::_0,
            true => USCBFAH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USCBFAH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USCBFAH_A::_1
    }
}
#[doc = "Field `USCBFAH` writer - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Count Up Enable"]
pub type USCBFAH_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTUPSR_SPEC, USCBFAH_A, O>;
impl<'a, const O: u8> USCBFAH_W<'a, O> {
    #[doc = "Counter count up is disable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USCBFAH_A::_0)
    }
    #[doc = "Counter count up is enable at the falling edge of GTIOCB input when GTIOCA input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USCBFAH_A::_1)
    }
}
#[doc = "Field `USELCA` reader - ELC_GPTA Event Source Counter Count Up Enable"]
pub type USELCA_R = crate::BitReader<USELCA_A>;
#[doc = "ELC_GPTA Event Source Counter Count Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USELCA_A {
    #[doc = "0: Counter count up is disable at the ELC_GPTA input"]
    _0 = 0,
    #[doc = "1: Counter count up is enable at the ELC_GPTA input"]
    _1 = 1,
}
impl From<USELCA_A> for bool {
    #[inline(always)]
    fn from(variant: USELCA_A) -> Self {
        variant as u8 != 0
    }
}
impl USELCA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USELCA_A {
        match self.bits {
            false => USELCA_A::_0,
            true => USELCA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USELCA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USELCA_A::_1
    }
}
#[doc = "Field `USELCA` writer - ELC_GPTA Event Source Counter Count Up Enable"]
pub type USELCA_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTUPSR_SPEC, USELCA_A, O>;
impl<'a, const O: u8> USELCA_W<'a, O> {
    #[doc = "Counter count up is disable at the ELC_GPTA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USELCA_A::_0)
    }
    #[doc = "Counter count up is enable at the ELC_GPTA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USELCA_A::_1)
    }
}
#[doc = "Field `USELCB` reader - ELC_GPTB Event Source Counter Count Up Enable"]
pub type USELCB_R = crate::BitReader<USELCB_A>;
#[doc = "ELC_GPTB Event Source Counter Count Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USELCB_A {
    #[doc = "0: Counter count up is disable at the ELC_GPTB input"]
    _0 = 0,
    #[doc = "1: Counter count up is enable at the ELC_GPTB input"]
    _1 = 1,
}
impl From<USELCB_A> for bool {
    #[inline(always)]
    fn from(variant: USELCB_A) -> Self {
        variant as u8 != 0
    }
}
impl USELCB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USELCB_A {
        match self.bits {
            false => USELCB_A::_0,
            true => USELCB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USELCB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USELCB_A::_1
    }
}
#[doc = "Field `USELCB` writer - ELC_GPTB Event Source Counter Count Up Enable"]
pub type USELCB_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTUPSR_SPEC, USELCB_A, O>;
impl<'a, const O: u8> USELCB_W<'a, O> {
    #[doc = "Counter count up is disable at the ELC_GPTB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USELCB_A::_0)
    }
    #[doc = "Counter count up is enable at the ELC_GPTB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USELCB_A::_1)
    }
}
#[doc = "Field `USELCC` reader - ELC_GPTC Event Source Counter Count Up Enable"]
pub type USELCC_R = crate::BitReader<USELCC_A>;
#[doc = "ELC_GPTC Event Source Counter Count Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USELCC_A {
    #[doc = "0: Counter count up is disable at the ELC_GPTC input"]
    _0 = 0,
    #[doc = "1: Counter count up is enable at the ELC_GPTC input"]
    _1 = 1,
}
impl From<USELCC_A> for bool {
    #[inline(always)]
    fn from(variant: USELCC_A) -> Self {
        variant as u8 != 0
    }
}
impl USELCC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USELCC_A {
        match self.bits {
            false => USELCC_A::_0,
            true => USELCC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USELCC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USELCC_A::_1
    }
}
#[doc = "Field `USELCC` writer - ELC_GPTC Event Source Counter Count Up Enable"]
pub type USELCC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTUPSR_SPEC, USELCC_A, O>;
impl<'a, const O: u8> USELCC_W<'a, O> {
    #[doc = "Counter count up is disable at the ELC_GPTC input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USELCC_A::_0)
    }
    #[doc = "Counter count up is enable at the ELC_GPTC input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USELCC_A::_1)
    }
}
#[doc = "Field `USELCD` reader - ELC_GPTD Event Source Counter Count Up Enable"]
pub type USELCD_R = crate::BitReader<USELCD_A>;
#[doc = "ELC_GPTD Event Source Counter Count Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USELCD_A {
    #[doc = "0: Counter count up is disable at the ELC_GPTD input"]
    _0 = 0,
    #[doc = "1: Counter count up is enable at the ELC_GPTD input"]
    _1 = 1,
}
impl From<USELCD_A> for bool {
    #[inline(always)]
    fn from(variant: USELCD_A) -> Self {
        variant as u8 != 0
    }
}
impl USELCD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USELCD_A {
        match self.bits {
            false => USELCD_A::_0,
            true => USELCD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USELCD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USELCD_A::_1
    }
}
#[doc = "Field `USELCD` writer - ELC_GPTD Event Source Counter Count Up Enable"]
pub type USELCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTUPSR_SPEC, USELCD_A, O>;
impl<'a, const O: u8> USELCD_W<'a, O> {
    #[doc = "Counter count up is disable at the ELC_GPTD input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USELCD_A::_0)
    }
    #[doc = "Counter count up is enable at the ELC_GPTD input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USELCD_A::_1)
    }
}
#[doc = "Field `USELCE` reader - ELC_GPTE Event Source Counter Count Up Enable"]
pub type USELCE_R = crate::BitReader<USELCE_A>;
#[doc = "ELC_GPTE Event Source Counter Count Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USELCE_A {
    #[doc = "0: Counter count up is disable at the ELC_GPTE input"]
    _0 = 0,
    #[doc = "1: Counter count up is enable at the ELC_GPTE input"]
    _1 = 1,
}
impl From<USELCE_A> for bool {
    #[inline(always)]
    fn from(variant: USELCE_A) -> Self {
        variant as u8 != 0
    }
}
impl USELCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USELCE_A {
        match self.bits {
            false => USELCE_A::_0,
            true => USELCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USELCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USELCE_A::_1
    }
}
#[doc = "Field `USELCE` writer - ELC_GPTE Event Source Counter Count Up Enable"]
pub type USELCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTUPSR_SPEC, USELCE_A, O>;
impl<'a, const O: u8> USELCE_W<'a, O> {
    #[doc = "Counter count up is disable at the ELC_GPTE input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USELCE_A::_0)
    }
    #[doc = "Counter count up is enable at the ELC_GPTE input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USELCE_A::_1)
    }
}
#[doc = "Field `USELCF` reader - ELC_GPTF Event Source Counter Count Up Enable"]
pub type USELCF_R = crate::BitReader<USELCF_A>;
#[doc = "ELC_GPTF Event Source Counter Count Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USELCF_A {
    #[doc = "0: Counter count up is disable at the ELC_GPTF input"]
    _0 = 0,
    #[doc = "1: Counter count up is enable at the ELC_GPTF input"]
    _1 = 1,
}
impl From<USELCF_A> for bool {
    #[inline(always)]
    fn from(variant: USELCF_A) -> Self {
        variant as u8 != 0
    }
}
impl USELCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USELCF_A {
        match self.bits {
            false => USELCF_A::_0,
            true => USELCF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USELCF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USELCF_A::_1
    }
}
#[doc = "Field `USELCF` writer - ELC_GPTF Event Source Counter Count Up Enable"]
pub type USELCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTUPSR_SPEC, USELCF_A, O>;
impl<'a, const O: u8> USELCF_W<'a, O> {
    #[doc = "Counter count up is disable at the ELC_GPTF input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USELCF_A::_0)
    }
    #[doc = "Counter count up is enable at the ELC_GPTF input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USELCF_A::_1)
    }
}
#[doc = "Field `USELCG` reader - ELC_GPTG Event Source Counter Count Up Enable"]
pub type USELCG_R = crate::BitReader<USELCG_A>;
#[doc = "ELC_GPTG Event Source Counter Count Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USELCG_A {
    #[doc = "0: Counter count up is disable at the ELC_GPTG input"]
    _0 = 0,
    #[doc = "1: Counter count up is enable at the ELC_GPTG input"]
    _1 = 1,
}
impl From<USELCG_A> for bool {
    #[inline(always)]
    fn from(variant: USELCG_A) -> Self {
        variant as u8 != 0
    }
}
impl USELCG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USELCG_A {
        match self.bits {
            false => USELCG_A::_0,
            true => USELCG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USELCG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USELCG_A::_1
    }
}
#[doc = "Field `USELCG` writer - ELC_GPTG Event Source Counter Count Up Enable"]
pub type USELCG_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTUPSR_SPEC, USELCG_A, O>;
impl<'a, const O: u8> USELCG_W<'a, O> {
    #[doc = "Counter count up is disable at the ELC_GPTG input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USELCG_A::_0)
    }
    #[doc = "Counter count up is enable at the ELC_GPTG input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USELCG_A::_1)
    }
}
#[doc = "Field `USELCH` reader - ELC_GPTH Event Source Counter Count Up Enable"]
pub type USELCH_R = crate::BitReader<USELCH_A>;
#[doc = "ELC_GPTH Event Source Counter Count Up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USELCH_A {
    #[doc = "0: Counter count up is disable at the ELC_GPTH input"]
    _0 = 0,
    #[doc = "1: Counter count up is enable at the ELC_GPTH input"]
    _1 = 1,
}
impl From<USELCH_A> for bool {
    #[inline(always)]
    fn from(variant: USELCH_A) -> Self {
        variant as u8 != 0
    }
}
impl USELCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USELCH_A {
        match self.bits {
            false => USELCH_A::_0,
            true => USELCH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USELCH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USELCH_A::_1
    }
}
#[doc = "Field `USELCH` writer - ELC_GPTH Event Source Counter Count Up Enable"]
pub type USELCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTUPSR_SPEC, USELCH_A, O>;
impl<'a, const O: u8> USELCH_W<'a, O> {
    #[doc = "Counter count up is disable at the ELC_GPTH input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USELCH_A::_0)
    }
    #[doc = "Counter count up is enable at the ELC_GPTH input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USELCH_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - GTETRGA Pin Rising Input Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn usgtrgar(&self) -> USGTRGAR_R {
        USGTRGAR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GTETRGA Pin Falling Input Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn usgtrgaf(&self) -> USGTRGAF_R {
        USGTRGAF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GTETRGB Pin Rising Input Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn usgtrgbr(&self) -> USGTRGBR_R {
        USGTRGBR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GTETRGB Pin Falling Input Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn usgtrgbf(&self) -> USGTRGBF_R {
        USGTRGBF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uscarbl(&self) -> USCARBL_R {
        USCARBL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uscarbh(&self) -> USCARBH_R {
        USCARBH_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uscafbl(&self) -> USCAFBL_R {
        USCAFBL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uscafbh(&self) -> USCAFBH_R {
        USCAFBH_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uscbral(&self) -> USCBRAL_R {
        USCBRAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uscbrah(&self) -> USCBRAH_R {
        USCBRAH_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uscbfal(&self) -> USCBFAL_R {
        USCBFAL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uscbfah(&self) -> USCBFAH_R {
        USCBFAH_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ELC_GPTA Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uselca(&self) -> USELCA_R {
        USELCA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ELC_GPTB Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uselcb(&self) -> USELCB_R {
        USELCB_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ELC_GPTC Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uselcc(&self) -> USELCC_R {
        USELCC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ELC_GPTD Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uselcd(&self) -> USELCD_R {
        USELCD_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ELC_GPTE Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uselce(&self) -> USELCE_R {
        USELCE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ELC_GPTF Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uselcf(&self) -> USELCF_R {
        USELCF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ELC_GPTG Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uselcg(&self) -> USELCG_R {
        USELCG_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ELC_GPTH Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uselch(&self) -> USELCH_R {
        USELCH_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GTETRGA Pin Rising Input Source Counter Count Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usgtrgar(&mut self) -> USGTRGAR_W<0> {
        USGTRGAR_W::new(self)
    }
    #[doc = "Bit 1 - GTETRGA Pin Falling Input Source Counter Count Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usgtrgaf(&mut self) -> USGTRGAF_W<1> {
        USGTRGAF_W::new(self)
    }
    #[doc = "Bit 2 - GTETRGB Pin Rising Input Source Counter Count Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usgtrgbr(&mut self) -> USGTRGBR_W<2> {
        USGTRGBR_W::new(self)
    }
    #[doc = "Bit 3 - GTETRGB Pin Falling Input Source Counter Count Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usgtrgbf(&mut self) -> USGTRGBF_W<3> {
        USGTRGBF_W::new(self)
    }
    #[doc = "Bit 8 - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Count Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uscarbl(&mut self) -> USCARBL_W<8> {
        USCARBL_W::new(self)
    }
    #[doc = "Bit 9 - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Count Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uscarbh(&mut self) -> USCARBH_W<9> {
        USCARBH_W::new(self)
    }
    #[doc = "Bit 10 - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Count Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uscafbl(&mut self) -> USCAFBL_W<10> {
        USCAFBL_W::new(self)
    }
    #[doc = "Bit 11 - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Count Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uscafbh(&mut self) -> USCAFBH_W<11> {
        USCAFBH_W::new(self)
    }
    #[doc = "Bit 12 - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Count Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uscbral(&mut self) -> USCBRAL_W<12> {
        USCBRAL_W::new(self)
    }
    #[doc = "Bit 13 - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Count Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uscbrah(&mut self) -> USCBRAH_W<13> {
        USCBRAH_W::new(self)
    }
    #[doc = "Bit 14 - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Count Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uscbfal(&mut self) -> USCBFAL_W<14> {
        USCBFAL_W::new(self)
    }
    #[doc = "Bit 15 - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Count Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uscbfah(&mut self) -> USCBFAH_W<15> {
        USCBFAH_W::new(self)
    }
    #[doc = "Bit 16 - ELC_GPTA Event Source Counter Count Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uselca(&mut self) -> USELCA_W<16> {
        USELCA_W::new(self)
    }
    #[doc = "Bit 17 - ELC_GPTB Event Source Counter Count Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uselcb(&mut self) -> USELCB_W<17> {
        USELCB_W::new(self)
    }
    #[doc = "Bit 18 - ELC_GPTC Event Source Counter Count Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uselcc(&mut self) -> USELCC_W<18> {
        USELCC_W::new(self)
    }
    #[doc = "Bit 19 - ELC_GPTD Event Source Counter Count Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uselcd(&mut self) -> USELCD_W<19> {
        USELCD_W::new(self)
    }
    #[doc = "Bit 20 - ELC_GPTE Event Source Counter Count Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uselce(&mut self) -> USELCE_W<20> {
        USELCE_W::new(self)
    }
    #[doc = "Bit 21 - ELC_GPTF Event Source Counter Count Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uselcf(&mut self) -> USELCF_W<21> {
        USELCF_W::new(self)
    }
    #[doc = "Bit 22 - ELC_GPTG Event Source Counter Count Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uselcg(&mut self) -> USELCG_W<22> {
        USELCG_W::new(self)
    }
    #[doc = "Bit 23 - ELC_GPTH Event Source Counter Count Up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uselch(&mut self) -> USELCH_W<23> {
        USELCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Up Count Source Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtupsr](index.html) module"]
pub struct GTUPSR_SPEC;
impl crate::RegisterSpec for GTUPSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtupsr::R](R) reader structure"]
impl crate::Readable for GTUPSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtupsr::W](W) writer structure"]
impl crate::Writable for GTUPSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTUPSR to value 0"]
impl crate::Resettable for GTUPSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
