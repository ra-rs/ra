#[doc = "Register `GTICASR` reader"]
pub struct R(crate::R<GTICASR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTICASR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTICASR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTICASR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTICASR` writer"]
pub struct W(crate::W<GTICASR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTICASR_SPEC>;
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
impl From<crate::W<GTICASR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTICASR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ASGTRGAR` reader - GTETRGA Pin Rising Input Source GTCCRA Input Capture Enable"]
pub type ASGTRGAR_R = crate::BitReader<ASGTRGAR_A>;
#[doc = "GTETRGA Pin Rising Input Source GTCCRA Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASGTRGAR_A {
    #[doc = "0: GTCCRA input capture disabled on the rising edge of GTETRGA input"]
    _0 = 0,
    #[doc = "1: GTCCRA input capture enabled on the rising edge of GTETRGA input"]
    _1 = 1,
}
impl From<ASGTRGAR_A> for bool {
    #[inline(always)]
    fn from(variant: ASGTRGAR_A) -> Self {
        variant as u8 != 0
    }
}
impl ASGTRGAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASGTRGAR_A {
        match self.bits {
            false => ASGTRGAR_A::_0,
            true => ASGTRGAR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASGTRGAR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASGTRGAR_A::_1
    }
}
#[doc = "Field `ASGTRGAR` writer - GTETRGA Pin Rising Input Source GTCCRA Input Capture Enable"]
pub type ASGTRGAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICASR_SPEC, ASGTRGAR_A, O>;
impl<'a, const O: u8> ASGTRGAR_W<'a, O> {
    #[doc = "GTCCRA input capture disabled on the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ASGTRGAR_A::_0)
    }
    #[doc = "GTCCRA input capture enabled on the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ASGTRGAR_A::_1)
    }
}
#[doc = "Field `ASGTRGAF` reader - GTETRGA Pin Falling Input Source GTCCRA Input Capture Enable"]
pub type ASGTRGAF_R = crate::BitReader<ASGTRGAF_A>;
#[doc = "GTETRGA Pin Falling Input Source GTCCRA Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASGTRGAF_A {
    #[doc = "0: GTCCRA input capture disabled on the falling edge of GTETRGA input"]
    _0 = 0,
    #[doc = "1: GTCCRA input capture enabled on the falling edge of GTETRGA input"]
    _1 = 1,
}
impl From<ASGTRGAF_A> for bool {
    #[inline(always)]
    fn from(variant: ASGTRGAF_A) -> Self {
        variant as u8 != 0
    }
}
impl ASGTRGAF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASGTRGAF_A {
        match self.bits {
            false => ASGTRGAF_A::_0,
            true => ASGTRGAF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASGTRGAF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASGTRGAF_A::_1
    }
}
#[doc = "Field `ASGTRGAF` writer - GTETRGA Pin Falling Input Source GTCCRA Input Capture Enable"]
pub type ASGTRGAF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICASR_SPEC, ASGTRGAF_A, O>;
impl<'a, const O: u8> ASGTRGAF_W<'a, O> {
    #[doc = "GTCCRA input capture disabled on the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ASGTRGAF_A::_0)
    }
    #[doc = "GTCCRA input capture enabled on the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ASGTRGAF_A::_1)
    }
}
#[doc = "Field `ASGTRGBR` reader - GTETRGB Pin Rising Input Source GTCCRA Input Capture Enable"]
pub type ASGTRGBR_R = crate::BitReader<ASGTRGBR_A>;
#[doc = "GTETRGB Pin Rising Input Source GTCCRA Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASGTRGBR_A {
    #[doc = "0: GTCCRA input capture disabled on the rising edge of GTETRGB input"]
    _0 = 0,
    #[doc = "1: GTCCRA input capture enabled on the rising edge of GTETRGB input"]
    _1 = 1,
}
impl From<ASGTRGBR_A> for bool {
    #[inline(always)]
    fn from(variant: ASGTRGBR_A) -> Self {
        variant as u8 != 0
    }
}
impl ASGTRGBR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASGTRGBR_A {
        match self.bits {
            false => ASGTRGBR_A::_0,
            true => ASGTRGBR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASGTRGBR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASGTRGBR_A::_1
    }
}
#[doc = "Field `ASGTRGBR` writer - GTETRGB Pin Rising Input Source GTCCRA Input Capture Enable"]
pub type ASGTRGBR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICASR_SPEC, ASGTRGBR_A, O>;
impl<'a, const O: u8> ASGTRGBR_W<'a, O> {
    #[doc = "GTCCRA input capture disabled on the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ASGTRGBR_A::_0)
    }
    #[doc = "GTCCRA input capture enabled on the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ASGTRGBR_A::_1)
    }
}
#[doc = "Field `ASGTRGBF` reader - GTETRGB Pin Falling Input Source GTCCRA Input Capture Enable"]
pub type ASGTRGBF_R = crate::BitReader<ASGTRGBF_A>;
#[doc = "GTETRGB Pin Falling Input Source GTCCRA Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASGTRGBF_A {
    #[doc = "0: GTCCRA input capture disabled on the falling edge of GTETRGB input"]
    _0 = 0,
    #[doc = "1: GTCCRA input capture enabled on the falling edge of GTETRGB input"]
    _1 = 1,
}
impl From<ASGTRGBF_A> for bool {
    #[inline(always)]
    fn from(variant: ASGTRGBF_A) -> Self {
        variant as u8 != 0
    }
}
impl ASGTRGBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASGTRGBF_A {
        match self.bits {
            false => ASGTRGBF_A::_0,
            true => ASGTRGBF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASGTRGBF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASGTRGBF_A::_1
    }
}
#[doc = "Field `ASGTRGBF` writer - GTETRGB Pin Falling Input Source GTCCRA Input Capture Enable"]
pub type ASGTRGBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICASR_SPEC, ASGTRGBF_A, O>;
impl<'a, const O: u8> ASGTRGBF_W<'a, O> {
    #[doc = "GTCCRA input capture disabled on the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ASGTRGBF_A::_0)
    }
    #[doc = "GTCCRA input capture enabled on the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ASGTRGBF_A::_1)
    }
}
#[doc = "Field `ASCARBL` reader - GTIOCnA Pin Rising Input during GTIOCnB Value Low Source GTCCRA Input Capture Enable"]
pub type ASCARBL_R = crate::BitReader<ASCARBL_A>;
#[doc = "GTIOCnA Pin Rising Input during GTIOCnB Value Low Source GTCCRA Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASCARBL_A {
    #[doc = "0: GTCCRA input capture disabled on the rising edge of GTIOCnA input when GTIOCnB input is 0"]
    _0 = 0,
    #[doc = "1: GTCCRA input capture enabled on the rising edge of GTIOCnA input when GTIOCnB input is 0"]
    _1 = 1,
}
impl From<ASCARBL_A> for bool {
    #[inline(always)]
    fn from(variant: ASCARBL_A) -> Self {
        variant as u8 != 0
    }
}
impl ASCARBL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASCARBL_A {
        match self.bits {
            false => ASCARBL_A::_0,
            true => ASCARBL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASCARBL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASCARBL_A::_1
    }
}
#[doc = "Field `ASCARBL` writer - GTIOCnA Pin Rising Input during GTIOCnB Value Low Source GTCCRA Input Capture Enable"]
pub type ASCARBL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICASR_SPEC, ASCARBL_A, O>;
impl<'a, const O: u8> ASCARBL_W<'a, O> {
    #[doc = "GTCCRA input capture disabled on the rising edge of GTIOCnA input when GTIOCnB input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ASCARBL_A::_0)
    }
    #[doc = "GTCCRA input capture enabled on the rising edge of GTIOCnA input when GTIOCnB input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ASCARBL_A::_1)
    }
}
#[doc = "Field `ASCARBH` reader - GTIOCnA Pin Rising Input during GTIOCnB Value High Source GTCCRA Input Capture Enable"]
pub type ASCARBH_R = crate::BitReader<ASCARBH_A>;
#[doc = "GTIOCnA Pin Rising Input during GTIOCnB Value High Source GTCCRA Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASCARBH_A {
    #[doc = "0: GTCCRA input capture disabled on the rising edge of GTIOCnA input when GTIOCnB input is 1"]
    _0 = 0,
    #[doc = "1: GTCCRA input capture enabled on the rising edge of GTIOCnA input when GTIOCnB input is 1"]
    _1 = 1,
}
impl From<ASCARBH_A> for bool {
    #[inline(always)]
    fn from(variant: ASCARBH_A) -> Self {
        variant as u8 != 0
    }
}
impl ASCARBH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASCARBH_A {
        match self.bits {
            false => ASCARBH_A::_0,
            true => ASCARBH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASCARBH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASCARBH_A::_1
    }
}
#[doc = "Field `ASCARBH` writer - GTIOCnA Pin Rising Input during GTIOCnB Value High Source GTCCRA Input Capture Enable"]
pub type ASCARBH_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICASR_SPEC, ASCARBH_A, O>;
impl<'a, const O: u8> ASCARBH_W<'a, O> {
    #[doc = "GTCCRA input capture disabled on the rising edge of GTIOCnA input when GTIOCnB input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ASCARBH_A::_0)
    }
    #[doc = "GTCCRA input capture enabled on the rising edge of GTIOCnA input when GTIOCnB input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ASCARBH_A::_1)
    }
}
#[doc = "Field `ASCAFBL` reader - GTIOCnA Pin Falling Input during GTIOCnB Value Low Source GTCCRA Input Capture Enable"]
pub type ASCAFBL_R = crate::BitReader<ASCAFBL_A>;
#[doc = "GTIOCnA Pin Falling Input during GTIOCnB Value Low Source GTCCRA Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASCAFBL_A {
    #[doc = "0: GTCCRA input capture disabled on the falling edge of GTIOCnA input when GTIOCnB input is 0"]
    _0 = 0,
    #[doc = "1: GTCCRA input capture enabled on the falling edge of GTIOCnA input when GTIOCnB input is 0"]
    _1 = 1,
}
impl From<ASCAFBL_A> for bool {
    #[inline(always)]
    fn from(variant: ASCAFBL_A) -> Self {
        variant as u8 != 0
    }
}
impl ASCAFBL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASCAFBL_A {
        match self.bits {
            false => ASCAFBL_A::_0,
            true => ASCAFBL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASCAFBL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASCAFBL_A::_1
    }
}
#[doc = "Field `ASCAFBL` writer - GTIOCnA Pin Falling Input during GTIOCnB Value Low Source GTCCRA Input Capture Enable"]
pub type ASCAFBL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICASR_SPEC, ASCAFBL_A, O>;
impl<'a, const O: u8> ASCAFBL_W<'a, O> {
    #[doc = "GTCCRA input capture disabled on the falling edge of GTIOCnA input when GTIOCnB input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ASCAFBL_A::_0)
    }
    #[doc = "GTCCRA input capture enabled on the falling edge of GTIOCnA input when GTIOCnB input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ASCAFBL_A::_1)
    }
}
#[doc = "Field `ASCAFBH` reader - GTIOCnA Pin Falling Input during GTIOCnB Value High Source GTCCRA Input Capture Enable"]
pub type ASCAFBH_R = crate::BitReader<ASCAFBH_A>;
#[doc = "GTIOCnA Pin Falling Input during GTIOCnB Value High Source GTCCRA Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASCAFBH_A {
    #[doc = "0: GTCCRA input capture disabled on the falling edge of GTIOCnA input when GTIOCnB input is 1"]
    _0 = 0,
    #[doc = "1: GTCCRA input capture enabled on the falling edge of GTIOCnA input when GTIOCnB input is 1"]
    _1 = 1,
}
impl From<ASCAFBH_A> for bool {
    #[inline(always)]
    fn from(variant: ASCAFBH_A) -> Self {
        variant as u8 != 0
    }
}
impl ASCAFBH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASCAFBH_A {
        match self.bits {
            false => ASCAFBH_A::_0,
            true => ASCAFBH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASCAFBH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASCAFBH_A::_1
    }
}
#[doc = "Field `ASCAFBH` writer - GTIOCnA Pin Falling Input during GTIOCnB Value High Source GTCCRA Input Capture Enable"]
pub type ASCAFBH_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICASR_SPEC, ASCAFBH_A, O>;
impl<'a, const O: u8> ASCAFBH_W<'a, O> {
    #[doc = "GTCCRA input capture disabled on the falling edge of GTIOCnA input when GTIOCnB input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ASCAFBH_A::_0)
    }
    #[doc = "GTCCRA input capture enabled on the falling edge of GTIOCnA input when GTIOCnB input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ASCAFBH_A::_1)
    }
}
#[doc = "Field `ASCBRAL` reader - GTIOCnB Pin Rising Input during GTIOCnA Value Low Source GTCCRA Input Capture Enable"]
pub type ASCBRAL_R = crate::BitReader<ASCBRAL_A>;
#[doc = "GTIOCnB Pin Rising Input during GTIOCnA Value Low Source GTCCRA Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASCBRAL_A {
    #[doc = "0: GTCCRA input capture disabled on the rising edge of GTIOCnB input when GTIOCnA input is 0"]
    _0 = 0,
    #[doc = "1: GTCCRA input capture enabled on the rising edge of GTIOCnB input when GTIOCnA input is 0"]
    _1 = 1,
}
impl From<ASCBRAL_A> for bool {
    #[inline(always)]
    fn from(variant: ASCBRAL_A) -> Self {
        variant as u8 != 0
    }
}
impl ASCBRAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASCBRAL_A {
        match self.bits {
            false => ASCBRAL_A::_0,
            true => ASCBRAL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASCBRAL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASCBRAL_A::_1
    }
}
#[doc = "Field `ASCBRAL` writer - GTIOCnB Pin Rising Input during GTIOCnA Value Low Source GTCCRA Input Capture Enable"]
pub type ASCBRAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICASR_SPEC, ASCBRAL_A, O>;
impl<'a, const O: u8> ASCBRAL_W<'a, O> {
    #[doc = "GTCCRA input capture disabled on the rising edge of GTIOCnB input when GTIOCnA input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ASCBRAL_A::_0)
    }
    #[doc = "GTCCRA input capture enabled on the rising edge of GTIOCnB input when GTIOCnA input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ASCBRAL_A::_1)
    }
}
#[doc = "Field `ASCBRAH` reader - GTIOCnB Pin Rising Input during GTIOCnA Value High Source GTCCRA Input Capture Enable"]
pub type ASCBRAH_R = crate::BitReader<ASCBRAH_A>;
#[doc = "GTIOCnB Pin Rising Input during GTIOCnA Value High Source GTCCRA Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASCBRAH_A {
    #[doc = "0: GTCCRA input capture disabled on the rising edge of GTIOCnB input when GTIOCnA input is 1"]
    _0 = 0,
    #[doc = "1: GTCCRA input capture enabled on the rising edge of GTIOCnB input when GTIOCnA input is 1"]
    _1 = 1,
}
impl From<ASCBRAH_A> for bool {
    #[inline(always)]
    fn from(variant: ASCBRAH_A) -> Self {
        variant as u8 != 0
    }
}
impl ASCBRAH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASCBRAH_A {
        match self.bits {
            false => ASCBRAH_A::_0,
            true => ASCBRAH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASCBRAH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASCBRAH_A::_1
    }
}
#[doc = "Field `ASCBRAH` writer - GTIOCnB Pin Rising Input during GTIOCnA Value High Source GTCCRA Input Capture Enable"]
pub type ASCBRAH_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICASR_SPEC, ASCBRAH_A, O>;
impl<'a, const O: u8> ASCBRAH_W<'a, O> {
    #[doc = "GTCCRA input capture disabled on the rising edge of GTIOCnB input when GTIOCnA input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ASCBRAH_A::_0)
    }
    #[doc = "GTCCRA input capture enabled on the rising edge of GTIOCnB input when GTIOCnA input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ASCBRAH_A::_1)
    }
}
#[doc = "Field `ASCBFAL` reader - GTIOCnB Pin Falling Input during GTIOCnA Value Low Source GTCCRA Input Capture Enable"]
pub type ASCBFAL_R = crate::BitReader<ASCBFAL_A>;
#[doc = "GTIOCnB Pin Falling Input during GTIOCnA Value Low Source GTCCRA Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASCBFAL_A {
    #[doc = "0: GTCCRA input capture disabled on the falling edge of GTIOCnB input when GTIOCnA input is 0"]
    _0 = 0,
    #[doc = "1: GTCCRA input capture enabled on the falling edge of GTIOCnB input when GTIOCnA input is 0"]
    _1 = 1,
}
impl From<ASCBFAL_A> for bool {
    #[inline(always)]
    fn from(variant: ASCBFAL_A) -> Self {
        variant as u8 != 0
    }
}
impl ASCBFAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASCBFAL_A {
        match self.bits {
            false => ASCBFAL_A::_0,
            true => ASCBFAL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASCBFAL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASCBFAL_A::_1
    }
}
#[doc = "Field `ASCBFAL` writer - GTIOCnB Pin Falling Input during GTIOCnA Value Low Source GTCCRA Input Capture Enable"]
pub type ASCBFAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICASR_SPEC, ASCBFAL_A, O>;
impl<'a, const O: u8> ASCBFAL_W<'a, O> {
    #[doc = "GTCCRA input capture disabled on the falling edge of GTIOCnB input when GTIOCnA input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ASCBFAL_A::_0)
    }
    #[doc = "GTCCRA input capture enabled on the falling edge of GTIOCnB input when GTIOCnA input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ASCBFAL_A::_1)
    }
}
#[doc = "Field `ASCBFAH` reader - GTIOCnB Pin Falling Input during GTIOCnA Value High Source GTCCRA Input Capture Enable"]
pub type ASCBFAH_R = crate::BitReader<ASCBFAH_A>;
#[doc = "GTIOCnB Pin Falling Input during GTIOCnA Value High Source GTCCRA Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASCBFAH_A {
    #[doc = "0: GTCCRA input capture disabled on the falling edge of GTIOCnB input when GTIOCnA input is 1"]
    _0 = 0,
    #[doc = "1: GTCCRA input capture enabled on the falling edge of GTIOCnB input when GTIOCnA input is 1"]
    _1 = 1,
}
impl From<ASCBFAH_A> for bool {
    #[inline(always)]
    fn from(variant: ASCBFAH_A) -> Self {
        variant as u8 != 0
    }
}
impl ASCBFAH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASCBFAH_A {
        match self.bits {
            false => ASCBFAH_A::_0,
            true => ASCBFAH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASCBFAH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASCBFAH_A::_1
    }
}
#[doc = "Field `ASCBFAH` writer - GTIOCnB Pin Falling Input during GTIOCnA Value High Source GTCCRA Input Capture Enable"]
pub type ASCBFAH_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICASR_SPEC, ASCBFAH_A, O>;
impl<'a, const O: u8> ASCBFAH_W<'a, O> {
    #[doc = "GTCCRA input capture disabled on the falling edge of GTIOCnB input when GTIOCnA input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ASCBFAH_A::_0)
    }
    #[doc = "GTCCRA input capture enabled on the falling edge of GTIOCnB input when GTIOCnA input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ASCBFAH_A::_1)
    }
}
#[doc = "Field `ASELCA` reader - ELC_GPTA Event Source GTCCRA Input Capture Enable"]
pub type ASELCA_R = crate::BitReader<ASELCA_A>;
#[doc = "ELC_GPTA Event Source GTCCRA Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASELCA_A {
    #[doc = "0: GTCCRA input capture disabled at the ELC_GPTA input"]
    _0 = 0,
    #[doc = "1: GTCCRA input capture enabled at the ELC_GPTA input"]
    _1 = 1,
}
impl From<ASELCA_A> for bool {
    #[inline(always)]
    fn from(variant: ASELCA_A) -> Self {
        variant as u8 != 0
    }
}
impl ASELCA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASELCA_A {
        match self.bits {
            false => ASELCA_A::_0,
            true => ASELCA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASELCA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASELCA_A::_1
    }
}
#[doc = "Field `ASELCA` writer - ELC_GPTA Event Source GTCCRA Input Capture Enable"]
pub type ASELCA_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICASR_SPEC, ASELCA_A, O>;
impl<'a, const O: u8> ASELCA_W<'a, O> {
    #[doc = "GTCCRA input capture disabled at the ELC_GPTA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ASELCA_A::_0)
    }
    #[doc = "GTCCRA input capture enabled at the ELC_GPTA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ASELCA_A::_1)
    }
}
#[doc = "Field `ASELCB` reader - ELC_GPTB Event Source GTCCRA Input Capture Enable"]
pub type ASELCB_R = crate::BitReader<ASELCB_A>;
#[doc = "ELC_GPTB Event Source GTCCRA Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASELCB_A {
    #[doc = "0: GTCCRA input capture disabled at the ELC_GPTB input"]
    _0 = 0,
    #[doc = "1: GTCCRA input capture enabled at the ELC_GPTB input"]
    _1 = 1,
}
impl From<ASELCB_A> for bool {
    #[inline(always)]
    fn from(variant: ASELCB_A) -> Self {
        variant as u8 != 0
    }
}
impl ASELCB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASELCB_A {
        match self.bits {
            false => ASELCB_A::_0,
            true => ASELCB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASELCB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASELCB_A::_1
    }
}
#[doc = "Field `ASELCB` writer - ELC_GPTB Event Source GTCCRA Input Capture Enable"]
pub type ASELCB_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICASR_SPEC, ASELCB_A, O>;
impl<'a, const O: u8> ASELCB_W<'a, O> {
    #[doc = "GTCCRA input capture disabled at the ELC_GPTB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ASELCB_A::_0)
    }
    #[doc = "GTCCRA input capture enabled at the ELC_GPTB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ASELCB_A::_1)
    }
}
#[doc = "Field `ASELCC` reader - ELC_GPTC Event Source GTCCRA Input Capture Enable"]
pub type ASELCC_R = crate::BitReader<ASELCC_A>;
#[doc = "ELC_GPTC Event Source GTCCRA Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASELCC_A {
    #[doc = "0: GTCCRA input capture disabled at the ELC_GPTC input"]
    _0 = 0,
    #[doc = "1: GTCCRA input capture enabled at the ELC_GPTC input"]
    _1 = 1,
}
impl From<ASELCC_A> for bool {
    #[inline(always)]
    fn from(variant: ASELCC_A) -> Self {
        variant as u8 != 0
    }
}
impl ASELCC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASELCC_A {
        match self.bits {
            false => ASELCC_A::_0,
            true => ASELCC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASELCC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASELCC_A::_1
    }
}
#[doc = "Field `ASELCC` writer - ELC_GPTC Event Source GTCCRA Input Capture Enable"]
pub type ASELCC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICASR_SPEC, ASELCC_A, O>;
impl<'a, const O: u8> ASELCC_W<'a, O> {
    #[doc = "GTCCRA input capture disabled at the ELC_GPTC input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ASELCC_A::_0)
    }
    #[doc = "GTCCRA input capture enabled at the ELC_GPTC input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ASELCC_A::_1)
    }
}
#[doc = "Field `ASELCD` reader - ELC_GPTD Event Source GTCCRA Input Capture Enable"]
pub type ASELCD_R = crate::BitReader<ASELCD_A>;
#[doc = "ELC_GPTD Event Source GTCCRA Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASELCD_A {
    #[doc = "0: GTCCRA input capture disabled at the ELC_GPTD input"]
    _0 = 0,
    #[doc = "1: GTCCRA input capture enabled at the ELC_GPTD input"]
    _1 = 1,
}
impl From<ASELCD_A> for bool {
    #[inline(always)]
    fn from(variant: ASELCD_A) -> Self {
        variant as u8 != 0
    }
}
impl ASELCD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASELCD_A {
        match self.bits {
            false => ASELCD_A::_0,
            true => ASELCD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASELCD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASELCD_A::_1
    }
}
#[doc = "Field `ASELCD` writer - ELC_GPTD Event Source GTCCRA Input Capture Enable"]
pub type ASELCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTICASR_SPEC, ASELCD_A, O>;
impl<'a, const O: u8> ASELCD_W<'a, O> {
    #[doc = "GTCCRA input capture disabled at the ELC_GPTD input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ASELCD_A::_0)
    }
    #[doc = "GTCCRA input capture enabled at the ELC_GPTD input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ASELCD_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - GTETRGA Pin Rising Input Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn asgtrgar(&self) -> ASGTRGAR_R {
        ASGTRGAR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GTETRGA Pin Falling Input Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn asgtrgaf(&self) -> ASGTRGAF_R {
        ASGTRGAF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GTETRGB Pin Rising Input Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn asgtrgbr(&self) -> ASGTRGBR_R {
        ASGTRGBR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GTETRGB Pin Falling Input Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn asgtrgbf(&self) -> ASGTRGBF_R {
        ASGTRGBF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - GTIOCnA Pin Rising Input during GTIOCnB Value Low Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn ascarbl(&self) -> ASCARBL_R {
        ASCARBL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GTIOCnA Pin Rising Input during GTIOCnB Value High Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn ascarbh(&self) -> ASCARBH_R {
        ASCARBH_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GTIOCnA Pin Falling Input during GTIOCnB Value Low Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn ascafbl(&self) -> ASCAFBL_R {
        ASCAFBL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GTIOCnA Pin Falling Input during GTIOCnB Value High Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn ascafbh(&self) -> ASCAFBH_R {
        ASCAFBH_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GTIOCnB Pin Rising Input during GTIOCnA Value Low Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn ascbral(&self) -> ASCBRAL_R {
        ASCBRAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GTIOCnB Pin Rising Input during GTIOCnA Value High Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn ascbrah(&self) -> ASCBRAH_R {
        ASCBRAH_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GTIOCnB Pin Falling Input during GTIOCnA Value Low Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn ascbfal(&self) -> ASCBFAL_R {
        ASCBFAL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GTIOCnB Pin Falling Input during GTIOCnA Value High Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn ascbfah(&self) -> ASCBFAH_R {
        ASCBFAH_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ELC_GPTA Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn aselca(&self) -> ASELCA_R {
        ASELCA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ELC_GPTB Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn aselcb(&self) -> ASELCB_R {
        ASELCB_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ELC_GPTC Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn aselcc(&self) -> ASELCC_R {
        ASELCC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ELC_GPTD Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn aselcd(&self) -> ASELCD_R {
        ASELCD_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GTETRGA Pin Rising Input Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn asgtrgar(&mut self) -> ASGTRGAR_W<0> {
        ASGTRGAR_W::new(self)
    }
    #[doc = "Bit 1 - GTETRGA Pin Falling Input Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn asgtrgaf(&mut self) -> ASGTRGAF_W<1> {
        ASGTRGAF_W::new(self)
    }
    #[doc = "Bit 2 - GTETRGB Pin Rising Input Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn asgtrgbr(&mut self) -> ASGTRGBR_W<2> {
        ASGTRGBR_W::new(self)
    }
    #[doc = "Bit 3 - GTETRGB Pin Falling Input Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn asgtrgbf(&mut self) -> ASGTRGBF_W<3> {
        ASGTRGBF_W::new(self)
    }
    #[doc = "Bit 8 - GTIOCnA Pin Rising Input during GTIOCnB Value Low Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ascarbl(&mut self) -> ASCARBL_W<8> {
        ASCARBL_W::new(self)
    }
    #[doc = "Bit 9 - GTIOCnA Pin Rising Input during GTIOCnB Value High Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ascarbh(&mut self) -> ASCARBH_W<9> {
        ASCARBH_W::new(self)
    }
    #[doc = "Bit 10 - GTIOCnA Pin Falling Input during GTIOCnB Value Low Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ascafbl(&mut self) -> ASCAFBL_W<10> {
        ASCAFBL_W::new(self)
    }
    #[doc = "Bit 11 - GTIOCnA Pin Falling Input during GTIOCnB Value High Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ascafbh(&mut self) -> ASCAFBH_W<11> {
        ASCAFBH_W::new(self)
    }
    #[doc = "Bit 12 - GTIOCnB Pin Rising Input during GTIOCnA Value Low Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ascbral(&mut self) -> ASCBRAL_W<12> {
        ASCBRAL_W::new(self)
    }
    #[doc = "Bit 13 - GTIOCnB Pin Rising Input during GTIOCnA Value High Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ascbrah(&mut self) -> ASCBRAH_W<13> {
        ASCBRAH_W::new(self)
    }
    #[doc = "Bit 14 - GTIOCnB Pin Falling Input during GTIOCnA Value Low Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ascbfal(&mut self) -> ASCBFAL_W<14> {
        ASCBFAL_W::new(self)
    }
    #[doc = "Bit 15 - GTIOCnB Pin Falling Input during GTIOCnA Value High Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ascbfah(&mut self) -> ASCBFAH_W<15> {
        ASCBFAH_W::new(self)
    }
    #[doc = "Bit 16 - ELC_GPTA Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aselca(&mut self) -> ASELCA_W<16> {
        ASELCA_W::new(self)
    }
    #[doc = "Bit 17 - ELC_GPTB Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aselcb(&mut self) -> ASELCB_W<17> {
        ASELCB_W::new(self)
    }
    #[doc = "Bit 18 - ELC_GPTC Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aselcc(&mut self) -> ASELCC_W<18> {
        ASELCC_W::new(self)
    }
    #[doc = "Bit 19 - ELC_GPTD Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aselcd(&mut self) -> ASELCD_W<19> {
        ASELCD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Input Capture Source Select Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gticasr](index.html) module"]
pub struct GTICASR_SPEC;
impl crate::RegisterSpec for GTICASR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gticasr::R](R) reader structure"]
impl crate::Readable for GTICASR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gticasr::W](W) writer structure"]
impl crate::Writable for GTICASR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTICASR to value 0"]
impl crate::Resettable for GTICASR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
