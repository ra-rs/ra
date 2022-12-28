#[doc = "Register `GTDNSR` reader"]
pub struct R(crate::R<GTDNSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTDNSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTDNSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTDNSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTDNSR` writer"]
pub struct W(crate::W<GTDNSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTDNSR_SPEC>;
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
impl From<crate::W<GTDNSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTDNSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSGTRGAR` reader - GTETRGA Pin Rising Input Source Counter Count Down Enable"]
pub type DSGTRGAR_R = crate::BitReader<DSGTRGAR_A>;
#[doc = "GTETRGA Pin Rising Input Source Counter Count Down Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSGTRGAR_A {
    #[doc = "0: Counter count down disabled on the rising edge of GTETRGA input"]
    _0 = 0,
    #[doc = "1: Counter count down enabled on the rising edge of GTETRGA input"]
    _1 = 1,
}
impl From<DSGTRGAR_A> for bool {
    #[inline(always)]
    fn from(variant: DSGTRGAR_A) -> Self {
        variant as u8 != 0
    }
}
impl DSGTRGAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSGTRGAR_A {
        match self.bits {
            false => DSGTRGAR_A::_0,
            true => DSGTRGAR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSGTRGAR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSGTRGAR_A::_1
    }
}
#[doc = "Field `DSGTRGAR` writer - GTETRGA Pin Rising Input Source Counter Count Down Enable"]
pub type DSGTRGAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTDNSR_SPEC, DSGTRGAR_A, O>;
impl<'a, const O: u8> DSGTRGAR_W<'a, O> {
    #[doc = "Counter count down disabled on the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DSGTRGAR_A::_0)
    }
    #[doc = "Counter count down enabled on the rising edge of GTETRGA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DSGTRGAR_A::_1)
    }
}
#[doc = "Field `DSGTRGAF` reader - GTETRGA Pin Falling Input Source Counter Count Down Enable"]
pub type DSGTRGAF_R = crate::BitReader<DSGTRGAF_A>;
#[doc = "GTETRGA Pin Falling Input Source Counter Count Down Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSGTRGAF_A {
    #[doc = "0: Counter count down disabled on the falling edge of GTETRGA input"]
    _0 = 0,
    #[doc = "1: Counter count down enabled on the falling edge of GTETRGA input"]
    _1 = 1,
}
impl From<DSGTRGAF_A> for bool {
    #[inline(always)]
    fn from(variant: DSGTRGAF_A) -> Self {
        variant as u8 != 0
    }
}
impl DSGTRGAF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSGTRGAF_A {
        match self.bits {
            false => DSGTRGAF_A::_0,
            true => DSGTRGAF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSGTRGAF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSGTRGAF_A::_1
    }
}
#[doc = "Field `DSGTRGAF` writer - GTETRGA Pin Falling Input Source Counter Count Down Enable"]
pub type DSGTRGAF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTDNSR_SPEC, DSGTRGAF_A, O>;
impl<'a, const O: u8> DSGTRGAF_W<'a, O> {
    #[doc = "Counter count down disabled on the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DSGTRGAF_A::_0)
    }
    #[doc = "Counter count down enabled on the falling edge of GTETRGA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DSGTRGAF_A::_1)
    }
}
#[doc = "Field `DSGTRGBR` reader - GTETRGB Pin Rising Input Source Counter Count Down Enable"]
pub type DSGTRGBR_R = crate::BitReader<DSGTRGBR_A>;
#[doc = "GTETRGB Pin Rising Input Source Counter Count Down Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSGTRGBR_A {
    #[doc = "0: Counter count down disabled on the rising edge of GTETRGB input"]
    _0 = 0,
    #[doc = "1: Counter count down enabled on the rising edge of GTETRGB input"]
    _1 = 1,
}
impl From<DSGTRGBR_A> for bool {
    #[inline(always)]
    fn from(variant: DSGTRGBR_A) -> Self {
        variant as u8 != 0
    }
}
impl DSGTRGBR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSGTRGBR_A {
        match self.bits {
            false => DSGTRGBR_A::_0,
            true => DSGTRGBR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSGTRGBR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSGTRGBR_A::_1
    }
}
#[doc = "Field `DSGTRGBR` writer - GTETRGB Pin Rising Input Source Counter Count Down Enable"]
pub type DSGTRGBR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTDNSR_SPEC, DSGTRGBR_A, O>;
impl<'a, const O: u8> DSGTRGBR_W<'a, O> {
    #[doc = "Counter count down disabled on the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DSGTRGBR_A::_0)
    }
    #[doc = "Counter count down enabled on the rising edge of GTETRGB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DSGTRGBR_A::_1)
    }
}
#[doc = "Field `DSGTRGBF` reader - GTETRGB Pin Falling Input Source Counter Count Down Enable"]
pub type DSGTRGBF_R = crate::BitReader<DSGTRGBF_A>;
#[doc = "GTETRGB Pin Falling Input Source Counter Count Down Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSGTRGBF_A {
    #[doc = "0: Counter count down disabled on the falling edge of GTETRGB input"]
    _0 = 0,
    #[doc = "1: Counter count down enabled on the falling edge of GTETRGB input"]
    _1 = 1,
}
impl From<DSGTRGBF_A> for bool {
    #[inline(always)]
    fn from(variant: DSGTRGBF_A) -> Self {
        variant as u8 != 0
    }
}
impl DSGTRGBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSGTRGBF_A {
        match self.bits {
            false => DSGTRGBF_A::_0,
            true => DSGTRGBF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSGTRGBF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSGTRGBF_A::_1
    }
}
#[doc = "Field `DSGTRGBF` writer - GTETRGB Pin Falling Input Source Counter Count Down Enable"]
pub type DSGTRGBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTDNSR_SPEC, DSGTRGBF_A, O>;
impl<'a, const O: u8> DSGTRGBF_W<'a, O> {
    #[doc = "Counter count down disabled on the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DSGTRGBF_A::_0)
    }
    #[doc = "Counter count down enabled on the falling edge of GTETRGB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DSGTRGBF_A::_1)
    }
}
#[doc = "Field `DSCARBL` reader - GTIOCnA Pin Rising Input during GTIOCnB Value Low Source Counter Count Down Enable"]
pub type DSCARBL_R = crate::BitReader<DSCARBL_A>;
#[doc = "GTIOCnA Pin Rising Input during GTIOCnB Value Low Source Counter Count Down Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSCARBL_A {
    #[doc = "0: Counter count down disabled on the rising edge of GTIOCnA input when GTIOCnB input is 0"]
    _0 = 0,
    #[doc = "1: Counter count down enabled on the rising edge of GTIOCnA input when GTIOCnB input is 0"]
    _1 = 1,
}
impl From<DSCARBL_A> for bool {
    #[inline(always)]
    fn from(variant: DSCARBL_A) -> Self {
        variant as u8 != 0
    }
}
impl DSCARBL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSCARBL_A {
        match self.bits {
            false => DSCARBL_A::_0,
            true => DSCARBL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSCARBL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSCARBL_A::_1
    }
}
#[doc = "Field `DSCARBL` writer - GTIOCnA Pin Rising Input during GTIOCnB Value Low Source Counter Count Down Enable"]
pub type DSCARBL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTDNSR_SPEC, DSCARBL_A, O>;
impl<'a, const O: u8> DSCARBL_W<'a, O> {
    #[doc = "Counter count down disabled on the rising edge of GTIOCnA input when GTIOCnB input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DSCARBL_A::_0)
    }
    #[doc = "Counter count down enabled on the rising edge of GTIOCnA input when GTIOCnB input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DSCARBL_A::_1)
    }
}
#[doc = "Field `DSCARBH` reader - GTIOCnA Pin Rising Input during GTIOCnB Value High Source Counter Count Down Enable"]
pub type DSCARBH_R = crate::BitReader<DSCARBH_A>;
#[doc = "GTIOCnA Pin Rising Input during GTIOCnB Value High Source Counter Count Down Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSCARBH_A {
    #[doc = "0: Counter count down disabled on the rising edge of GTIOCnA input when GTIOCnB input is 1"]
    _0 = 0,
    #[doc = "1: Counter count down enabled on the rising edge of GTIOCnA input when GTIOCnB input is 1"]
    _1 = 1,
}
impl From<DSCARBH_A> for bool {
    #[inline(always)]
    fn from(variant: DSCARBH_A) -> Self {
        variant as u8 != 0
    }
}
impl DSCARBH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSCARBH_A {
        match self.bits {
            false => DSCARBH_A::_0,
            true => DSCARBH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSCARBH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSCARBH_A::_1
    }
}
#[doc = "Field `DSCARBH` writer - GTIOCnA Pin Rising Input during GTIOCnB Value High Source Counter Count Down Enable"]
pub type DSCARBH_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTDNSR_SPEC, DSCARBH_A, O>;
impl<'a, const O: u8> DSCARBH_W<'a, O> {
    #[doc = "Counter count down disabled on the rising edge of GTIOCnA input when GTIOCnB input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DSCARBH_A::_0)
    }
    #[doc = "Counter count down enabled on the rising edge of GTIOCnA input when GTIOCnB input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DSCARBH_A::_1)
    }
}
#[doc = "Field `DSCAFBL` reader - GTIOCnA Pin Falling Input during GTIOCnB Value Low Source Counter Count Down Enable"]
pub type DSCAFBL_R = crate::BitReader<DSCAFBL_A>;
#[doc = "GTIOCnA Pin Falling Input during GTIOCnB Value Low Source Counter Count Down Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSCAFBL_A {
    #[doc = "0: Counter count down disabled on the falling edge of GTIOCnA input when GTIOCnB input is 0"]
    _0 = 0,
    #[doc = "1: Counter count down enabled on the falling edge of GTIOCnA input when GTIOCnB input is 0"]
    _1 = 1,
}
impl From<DSCAFBL_A> for bool {
    #[inline(always)]
    fn from(variant: DSCAFBL_A) -> Self {
        variant as u8 != 0
    }
}
impl DSCAFBL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSCAFBL_A {
        match self.bits {
            false => DSCAFBL_A::_0,
            true => DSCAFBL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSCAFBL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSCAFBL_A::_1
    }
}
#[doc = "Field `DSCAFBL` writer - GTIOCnA Pin Falling Input during GTIOCnB Value Low Source Counter Count Down Enable"]
pub type DSCAFBL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTDNSR_SPEC, DSCAFBL_A, O>;
impl<'a, const O: u8> DSCAFBL_W<'a, O> {
    #[doc = "Counter count down disabled on the falling edge of GTIOCnA input when GTIOCnB input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DSCAFBL_A::_0)
    }
    #[doc = "Counter count down enabled on the falling edge of GTIOCnA input when GTIOCnB input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DSCAFBL_A::_1)
    }
}
#[doc = "Field `DSCAFBH` reader - GTIOCnA Pin Falling Input during GTIOCnB Value High Source Counter Count Down Enable"]
pub type DSCAFBH_R = crate::BitReader<DSCAFBH_A>;
#[doc = "GTIOCnA Pin Falling Input during GTIOCnB Value High Source Counter Count Down Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSCAFBH_A {
    #[doc = "0: Counter count down disabled on the falling edge of GTIOCnA input when GTIOCnB input is 1"]
    _0 = 0,
    #[doc = "1: Counter count down enabled on the falling edge of GTIOCnA input when GTIOCnB input is 1"]
    _1 = 1,
}
impl From<DSCAFBH_A> for bool {
    #[inline(always)]
    fn from(variant: DSCAFBH_A) -> Self {
        variant as u8 != 0
    }
}
impl DSCAFBH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSCAFBH_A {
        match self.bits {
            false => DSCAFBH_A::_0,
            true => DSCAFBH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSCAFBH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSCAFBH_A::_1
    }
}
#[doc = "Field `DSCAFBH` writer - GTIOCnA Pin Falling Input during GTIOCnB Value High Source Counter Count Down Enable"]
pub type DSCAFBH_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTDNSR_SPEC, DSCAFBH_A, O>;
impl<'a, const O: u8> DSCAFBH_W<'a, O> {
    #[doc = "Counter count down disabled on the falling edge of GTIOCnA input when GTIOCnB input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DSCAFBH_A::_0)
    }
    #[doc = "Counter count down enabled on the falling edge of GTIOCnA input when GTIOCnB input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DSCAFBH_A::_1)
    }
}
#[doc = "Field `DSCBRAL` reader - GTIOCnB Pin Rising Input during GTIOCnA Value Low Source Counter Count Down Enable"]
pub type DSCBRAL_R = crate::BitReader<DSCBRAL_A>;
#[doc = "GTIOCnB Pin Rising Input during GTIOCnA Value Low Source Counter Count Down Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSCBRAL_A {
    #[doc = "0: Counter count down disabled on the rising edge of GTIOCnB input when GTIOCnA input is 0"]
    _0 = 0,
    #[doc = "1: Counter count down enabled on the rising edge of GTIOCnB input when GTIOCnA input is 0"]
    _1 = 1,
}
impl From<DSCBRAL_A> for bool {
    #[inline(always)]
    fn from(variant: DSCBRAL_A) -> Self {
        variant as u8 != 0
    }
}
impl DSCBRAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSCBRAL_A {
        match self.bits {
            false => DSCBRAL_A::_0,
            true => DSCBRAL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSCBRAL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSCBRAL_A::_1
    }
}
#[doc = "Field `DSCBRAL` writer - GTIOCnB Pin Rising Input during GTIOCnA Value Low Source Counter Count Down Enable"]
pub type DSCBRAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTDNSR_SPEC, DSCBRAL_A, O>;
impl<'a, const O: u8> DSCBRAL_W<'a, O> {
    #[doc = "Counter count down disabled on the rising edge of GTIOCnB input when GTIOCnA input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DSCBRAL_A::_0)
    }
    #[doc = "Counter count down enabled on the rising edge of GTIOCnB input when GTIOCnA input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DSCBRAL_A::_1)
    }
}
#[doc = "Field `DSCBRAH` reader - GTIOCnB Pin Rising Input during GTIOCnA Value High Source Counter Count Down Enable"]
pub type DSCBRAH_R = crate::BitReader<DSCBRAH_A>;
#[doc = "GTIOCnB Pin Rising Input during GTIOCnA Value High Source Counter Count Down Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSCBRAH_A {
    #[doc = "0: Counter count down disabled on the rising edge of GTIOCnB input when GTIOCnA input is 1"]
    _0 = 0,
    #[doc = "1: Counter count down enabled on the rising edge of GTIOCnB input when GTIOCnA input is 1"]
    _1 = 1,
}
impl From<DSCBRAH_A> for bool {
    #[inline(always)]
    fn from(variant: DSCBRAH_A) -> Self {
        variant as u8 != 0
    }
}
impl DSCBRAH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSCBRAH_A {
        match self.bits {
            false => DSCBRAH_A::_0,
            true => DSCBRAH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSCBRAH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSCBRAH_A::_1
    }
}
#[doc = "Field `DSCBRAH` writer - GTIOCnB Pin Rising Input during GTIOCnA Value High Source Counter Count Down Enable"]
pub type DSCBRAH_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTDNSR_SPEC, DSCBRAH_A, O>;
impl<'a, const O: u8> DSCBRAH_W<'a, O> {
    #[doc = "Counter count down disabled on the rising edge of GTIOCnB input when GTIOCnA input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DSCBRAH_A::_0)
    }
    #[doc = "Counter count down enabled on the rising edge of GTIOCnB input when GTIOCnA input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DSCBRAH_A::_1)
    }
}
#[doc = "Field `DSCBFAL` reader - GTIOCnB Pin Falling Input during GTIOCnA Value Low Source Counter Count Down Enable"]
pub type DSCBFAL_R = crate::BitReader<DSCBFAL_A>;
#[doc = "GTIOCnB Pin Falling Input during GTIOCnA Value Low Source Counter Count Down Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSCBFAL_A {
    #[doc = "0: Counter count down disabled on the falling edge of GTIOCnB input when GTIOCnA input is 0"]
    _0 = 0,
    #[doc = "1: Counter count down enabled on the falling edge of GTIOCnB input when GTIOCnA input is 0"]
    _1 = 1,
}
impl From<DSCBFAL_A> for bool {
    #[inline(always)]
    fn from(variant: DSCBFAL_A) -> Self {
        variant as u8 != 0
    }
}
impl DSCBFAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSCBFAL_A {
        match self.bits {
            false => DSCBFAL_A::_0,
            true => DSCBFAL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSCBFAL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSCBFAL_A::_1
    }
}
#[doc = "Field `DSCBFAL` writer - GTIOCnB Pin Falling Input during GTIOCnA Value Low Source Counter Count Down Enable"]
pub type DSCBFAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTDNSR_SPEC, DSCBFAL_A, O>;
impl<'a, const O: u8> DSCBFAL_W<'a, O> {
    #[doc = "Counter count down disabled on the falling edge of GTIOCnB input when GTIOCnA input is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DSCBFAL_A::_0)
    }
    #[doc = "Counter count down enabled on the falling edge of GTIOCnB input when GTIOCnA input is 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DSCBFAL_A::_1)
    }
}
#[doc = "Field `DSCBFAH` reader - GTIOCnB Pin Falling Input during GTIOCnA Value High Source Counter Count Down Enable"]
pub type DSCBFAH_R = crate::BitReader<DSCBFAH_A>;
#[doc = "GTIOCnB Pin Falling Input during GTIOCnA Value High Source Counter Count Down Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSCBFAH_A {
    #[doc = "0: Counter count down disabled on the falling edge of GTIOCnB input when GTIOCnA input is 1"]
    _0 = 0,
    #[doc = "1: Counter count down enabled on the falling edge of GTIOCnB input when GTIOCnA input is 1"]
    _1 = 1,
}
impl From<DSCBFAH_A> for bool {
    #[inline(always)]
    fn from(variant: DSCBFAH_A) -> Self {
        variant as u8 != 0
    }
}
impl DSCBFAH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSCBFAH_A {
        match self.bits {
            false => DSCBFAH_A::_0,
            true => DSCBFAH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSCBFAH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSCBFAH_A::_1
    }
}
#[doc = "Field `DSCBFAH` writer - GTIOCnB Pin Falling Input during GTIOCnA Value High Source Counter Count Down Enable"]
pub type DSCBFAH_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTDNSR_SPEC, DSCBFAH_A, O>;
impl<'a, const O: u8> DSCBFAH_W<'a, O> {
    #[doc = "Counter count down disabled on the falling edge of GTIOCnB input when GTIOCnA input is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DSCBFAH_A::_0)
    }
    #[doc = "Counter count down enabled on the falling edge of GTIOCnB input when GTIOCnA input is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DSCBFAH_A::_1)
    }
}
#[doc = "Field `DSELCA` reader - ELC_GPTA Event Source Counter Count Down Enable"]
pub type DSELCA_R = crate::BitReader<DSELCA_A>;
#[doc = "ELC_GPTA Event Source Counter Count Down Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSELCA_A {
    #[doc = "0: Counter count down disabled at the ELC_GPTA input"]
    _0 = 0,
    #[doc = "1: Counter count down enabled at the ELC_GPTA input"]
    _1 = 1,
}
impl From<DSELCA_A> for bool {
    #[inline(always)]
    fn from(variant: DSELCA_A) -> Self {
        variant as u8 != 0
    }
}
impl DSELCA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSELCA_A {
        match self.bits {
            false => DSELCA_A::_0,
            true => DSELCA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSELCA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSELCA_A::_1
    }
}
#[doc = "Field `DSELCA` writer - ELC_GPTA Event Source Counter Count Down Enable"]
pub type DSELCA_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTDNSR_SPEC, DSELCA_A, O>;
impl<'a, const O: u8> DSELCA_W<'a, O> {
    #[doc = "Counter count down disabled at the ELC_GPTA input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DSELCA_A::_0)
    }
    #[doc = "Counter count down enabled at the ELC_GPTA input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DSELCA_A::_1)
    }
}
#[doc = "Field `DSELCB` reader - ELC_GPTB Event Source Counter Count Down Enable"]
pub type DSELCB_R = crate::BitReader<DSELCB_A>;
#[doc = "ELC_GPTB Event Source Counter Count Down Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSELCB_A {
    #[doc = "0: Counter count down disabled at the ELC_GPTB input"]
    _0 = 0,
    #[doc = "1: Counter count down enabled at the ELC_GPTB input"]
    _1 = 1,
}
impl From<DSELCB_A> for bool {
    #[inline(always)]
    fn from(variant: DSELCB_A) -> Self {
        variant as u8 != 0
    }
}
impl DSELCB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSELCB_A {
        match self.bits {
            false => DSELCB_A::_0,
            true => DSELCB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSELCB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSELCB_A::_1
    }
}
#[doc = "Field `DSELCB` writer - ELC_GPTB Event Source Counter Count Down Enable"]
pub type DSELCB_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTDNSR_SPEC, DSELCB_A, O>;
impl<'a, const O: u8> DSELCB_W<'a, O> {
    #[doc = "Counter count down disabled at the ELC_GPTB input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DSELCB_A::_0)
    }
    #[doc = "Counter count down enabled at the ELC_GPTB input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DSELCB_A::_1)
    }
}
#[doc = "Field `DSELCC` reader - ELC_GPTC Event Source Counter Count Down Enable"]
pub type DSELCC_R = crate::BitReader<DSELCC_A>;
#[doc = "ELC_GPTC Event Source Counter Count Down Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSELCC_A {
    #[doc = "0: Counter count down disabled at the ELC_GPTC input"]
    _0 = 0,
    #[doc = "1: Counter count down enabled at the ELC_GPTC input"]
    _1 = 1,
}
impl From<DSELCC_A> for bool {
    #[inline(always)]
    fn from(variant: DSELCC_A) -> Self {
        variant as u8 != 0
    }
}
impl DSELCC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSELCC_A {
        match self.bits {
            false => DSELCC_A::_0,
            true => DSELCC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSELCC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSELCC_A::_1
    }
}
#[doc = "Field `DSELCC` writer - ELC_GPTC Event Source Counter Count Down Enable"]
pub type DSELCC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTDNSR_SPEC, DSELCC_A, O>;
impl<'a, const O: u8> DSELCC_W<'a, O> {
    #[doc = "Counter count down disabled at the ELC_GPTC input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DSELCC_A::_0)
    }
    #[doc = "Counter count down enabled at the ELC_GPTC input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DSELCC_A::_1)
    }
}
#[doc = "Field `DSELCD` reader - ELC_GPTD Event Source Counter Count Down Enable"]
pub type DSELCD_R = crate::BitReader<DSELCD_A>;
#[doc = "ELC_GPTD Event Source Counter Count Down Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSELCD_A {
    #[doc = "0: Counter count down disabled at the ELC_GPTD input"]
    _0 = 0,
    #[doc = "1: Counter count down enabled at the ELC_GPTD input"]
    _1 = 1,
}
impl From<DSELCD_A> for bool {
    #[inline(always)]
    fn from(variant: DSELCD_A) -> Self {
        variant as u8 != 0
    }
}
impl DSELCD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSELCD_A {
        match self.bits {
            false => DSELCD_A::_0,
            true => DSELCD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSELCD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSELCD_A::_1
    }
}
#[doc = "Field `DSELCD` writer - ELC_GPTD Event Source Counter Count Down Enable"]
pub type DSELCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTDNSR_SPEC, DSELCD_A, O>;
impl<'a, const O: u8> DSELCD_W<'a, O> {
    #[doc = "Counter count down disabled at the ELC_GPTD input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DSELCD_A::_0)
    }
    #[doc = "Counter count down enabled at the ELC_GPTD input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DSELCD_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - GTETRGA Pin Rising Input Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dsgtrgar(&self) -> DSGTRGAR_R {
        DSGTRGAR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GTETRGA Pin Falling Input Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dsgtrgaf(&self) -> DSGTRGAF_R {
        DSGTRGAF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GTETRGB Pin Rising Input Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dsgtrgbr(&self) -> DSGTRGBR_R {
        DSGTRGBR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GTETRGB Pin Falling Input Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dsgtrgbf(&self) -> DSGTRGBF_R {
        DSGTRGBF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - GTIOCnA Pin Rising Input during GTIOCnB Value Low Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dscarbl(&self) -> DSCARBL_R {
        DSCARBL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GTIOCnA Pin Rising Input during GTIOCnB Value High Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dscarbh(&self) -> DSCARBH_R {
        DSCARBH_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GTIOCnA Pin Falling Input during GTIOCnB Value Low Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dscafbl(&self) -> DSCAFBL_R {
        DSCAFBL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GTIOCnA Pin Falling Input during GTIOCnB Value High Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dscafbh(&self) -> DSCAFBH_R {
        DSCAFBH_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GTIOCnB Pin Rising Input during GTIOCnA Value Low Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dscbral(&self) -> DSCBRAL_R {
        DSCBRAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GTIOCnB Pin Rising Input during GTIOCnA Value High Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dscbrah(&self) -> DSCBRAH_R {
        DSCBRAH_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GTIOCnB Pin Falling Input during GTIOCnA Value Low Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dscbfal(&self) -> DSCBFAL_R {
        DSCBFAL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GTIOCnB Pin Falling Input during GTIOCnA Value High Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dscbfah(&self) -> DSCBFAH_R {
        DSCBFAH_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ELC_GPTA Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dselca(&self) -> DSELCA_R {
        DSELCA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ELC_GPTB Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dselcb(&self) -> DSELCB_R {
        DSELCB_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ELC_GPTC Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dselcc(&self) -> DSELCC_R {
        DSELCC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ELC_GPTD Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dselcd(&self) -> DSELCD_R {
        DSELCD_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GTETRGA Pin Rising Input Source Counter Count Down Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dsgtrgar(&mut self) -> DSGTRGAR_W<0> {
        DSGTRGAR_W::new(self)
    }
    #[doc = "Bit 1 - GTETRGA Pin Falling Input Source Counter Count Down Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dsgtrgaf(&mut self) -> DSGTRGAF_W<1> {
        DSGTRGAF_W::new(self)
    }
    #[doc = "Bit 2 - GTETRGB Pin Rising Input Source Counter Count Down Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dsgtrgbr(&mut self) -> DSGTRGBR_W<2> {
        DSGTRGBR_W::new(self)
    }
    #[doc = "Bit 3 - GTETRGB Pin Falling Input Source Counter Count Down Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dsgtrgbf(&mut self) -> DSGTRGBF_W<3> {
        DSGTRGBF_W::new(self)
    }
    #[doc = "Bit 8 - GTIOCnA Pin Rising Input during GTIOCnB Value Low Source Counter Count Down Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dscarbl(&mut self) -> DSCARBL_W<8> {
        DSCARBL_W::new(self)
    }
    #[doc = "Bit 9 - GTIOCnA Pin Rising Input during GTIOCnB Value High Source Counter Count Down Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dscarbh(&mut self) -> DSCARBH_W<9> {
        DSCARBH_W::new(self)
    }
    #[doc = "Bit 10 - GTIOCnA Pin Falling Input during GTIOCnB Value Low Source Counter Count Down Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dscafbl(&mut self) -> DSCAFBL_W<10> {
        DSCAFBL_W::new(self)
    }
    #[doc = "Bit 11 - GTIOCnA Pin Falling Input during GTIOCnB Value High Source Counter Count Down Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dscafbh(&mut self) -> DSCAFBH_W<11> {
        DSCAFBH_W::new(self)
    }
    #[doc = "Bit 12 - GTIOCnB Pin Rising Input during GTIOCnA Value Low Source Counter Count Down Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dscbral(&mut self) -> DSCBRAL_W<12> {
        DSCBRAL_W::new(self)
    }
    #[doc = "Bit 13 - GTIOCnB Pin Rising Input during GTIOCnA Value High Source Counter Count Down Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dscbrah(&mut self) -> DSCBRAH_W<13> {
        DSCBRAH_W::new(self)
    }
    #[doc = "Bit 14 - GTIOCnB Pin Falling Input during GTIOCnA Value Low Source Counter Count Down Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dscbfal(&mut self) -> DSCBFAL_W<14> {
        DSCBFAL_W::new(self)
    }
    #[doc = "Bit 15 - GTIOCnB Pin Falling Input during GTIOCnA Value High Source Counter Count Down Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dscbfah(&mut self) -> DSCBFAH_W<15> {
        DSCBFAH_W::new(self)
    }
    #[doc = "Bit 16 - ELC_GPTA Event Source Counter Count Down Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dselca(&mut self) -> DSELCA_W<16> {
        DSELCA_W::new(self)
    }
    #[doc = "Bit 17 - ELC_GPTB Event Source Counter Count Down Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dselcb(&mut self) -> DSELCB_W<17> {
        DSELCB_W::new(self)
    }
    #[doc = "Bit 18 - ELC_GPTC Event Source Counter Count Down Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dselcc(&mut self) -> DSELCC_W<18> {
        DSELCC_W::new(self)
    }
    #[doc = "Bit 19 - ELC_GPTD Event Source Counter Count Down Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dselcd(&mut self) -> DSELCD_W<19> {
        DSELCD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Down Count Source Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtdnsr](index.html) module"]
pub struct GTDNSR_SPEC;
impl crate::RegisterSpec for GTDNSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtdnsr::R](R) reader structure"]
impl crate::Readable for GTDNSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtdnsr::W](W) writer structure"]
impl crate::Writable for GTDNSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTDNSR to value 0"]
impl crate::Resettable for GTDNSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
