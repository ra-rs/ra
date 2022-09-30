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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `BSCARBL` reader - GTIOCnA Pin Rising Input during GTIOCnB Value Low Source GTCCRB Input Capture Enable"]
pub type BSCARBL_R = crate::BitReader<BSCARBL_A>;
#[doc = "GTIOCnA Pin Rising Input during GTIOCnB Value Low Source GTCCRB Input Capture Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
}
impl W {
    #[doc = "Bit 0 - GTETRGA Pin Rising Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bsgtrgar(&mut self) -> BSGTRGAR_W<0> {
        BSGTRGAR_W::new(self)
    }
    #[doc = "Bit 1 - GTETRGA Pin Falling Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bsgtrgaf(&mut self) -> BSGTRGAF_W<1> {
        BSGTRGAF_W::new(self)
    }
    #[doc = "Bit 2 - GTETRGB Pin Rising Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bsgtrgbr(&mut self) -> BSGTRGBR_W<2> {
        BSGTRGBR_W::new(self)
    }
    #[doc = "Bit 3 - GTETRGB Pin Falling Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bsgtrgbf(&mut self) -> BSGTRGBF_W<3> {
        BSGTRGBF_W::new(self)
    }
    #[doc = "Bit 8 - GTIOCnA Pin Rising Input during GTIOCnB Value Low Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscarbl(&mut self) -> BSCARBL_W<8> {
        BSCARBL_W::new(self)
    }
    #[doc = "Bit 9 - GTIOCnA Pin Rising Input during GTIOCnB Value High Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscarbh(&mut self) -> BSCARBH_W<9> {
        BSCARBH_W::new(self)
    }
    #[doc = "Bit 10 - GTIOCnA Pin Falling Input during GTIOCnB Value Low Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscafbl(&mut self) -> BSCAFBL_W<10> {
        BSCAFBL_W::new(self)
    }
    #[doc = "Bit 11 - GTIOCnA Pin Falling Input during GTIOCnB Value High Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscafbh(&mut self) -> BSCAFBH_W<11> {
        BSCAFBH_W::new(self)
    }
    #[doc = "Bit 12 - GTIOCnB Pin Rising Input during GTIOCnA Value Low Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscbral(&mut self) -> BSCBRAL_W<12> {
        BSCBRAL_W::new(self)
    }
    #[doc = "Bit 13 - GTIOCnB Pin Rising Input during GTIOCnA Value High Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscbrah(&mut self) -> BSCBRAH_W<13> {
        BSCBRAH_W::new(self)
    }
    #[doc = "Bit 14 - GTIOCnB Pin Falling Input during GTIOCnA Value Low Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscbfal(&mut self) -> BSCBFAL_W<14> {
        BSCBFAL_W::new(self)
    }
    #[doc = "Bit 15 - GTIOCnB Pin Falling Input during GTIOCnA Value High Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscbfah(&mut self) -> BSCBFAH_W<15> {
        BSCBFAH_W::new(self)
    }
    #[doc = "Bit 16 - ELC_GPTA Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bselca(&mut self) -> BSELCA_W<16> {
        BSELCA_W::new(self)
    }
    #[doc = "Bit 17 - ELC_GPTB Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bselcb(&mut self) -> BSELCB_W<17> {
        BSELCB_W::new(self)
    }
    #[doc = "Bit 18 - ELC_GPTC Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bselcc(&mut self) -> BSELCC_W<18> {
        BSELCC_W::new(self)
    }
    #[doc = "Bit 19 - ELC_GPTD Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bselcd(&mut self) -> BSELCD_W<19> {
        BSELCD_W::new(self)
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
}
#[doc = "`reset()` method sets GTICBSR to value 0"]
impl crate::Resettable for GTICBSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
