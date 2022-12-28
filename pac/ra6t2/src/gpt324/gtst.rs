#[doc = "Register `GTST` reader"]
pub struct R(crate::R<GTST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTST` writer"]
pub struct W(crate::W<GTST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTST_SPEC>;
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
impl From<crate::W<GTST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCFA` reader - Input Capture/Compare Match Flag A"]
pub type TCFA_R = crate::BitReader<TCFA_A>;
#[doc = "Input Capture/Compare Match Flag A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCFA_A {
    #[doc = "0: No input capture/compare match of GTCCRA is generated"]
    _0 = 0,
    #[doc = "1: An input capture/compare match of GTCCRA is generated"]
    _1 = 1,
}
impl From<TCFA_A> for bool {
    #[inline(always)]
    fn from(variant: TCFA_A) -> Self {
        variant as u8 != 0
    }
}
impl TCFA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCFA_A {
        match self.bits {
            false => TCFA_A::_0,
            true => TCFA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCFA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCFA_A::_1
    }
}
#[doc = "Field `TCFA` writer - Input Capture/Compare Match Flag A"]
pub type TCFA_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTST_SPEC, TCFA_A, O>;
impl<'a, const O: u8> TCFA_W<'a, O> {
    #[doc = "No input capture/compare match of GTCCRA is generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCFA_A::_0)
    }
    #[doc = "An input capture/compare match of GTCCRA is generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCFA_A::_1)
    }
}
#[doc = "Field `TCFB` reader - Input Capture/Compare Match Flag B"]
pub type TCFB_R = crate::BitReader<TCFB_A>;
#[doc = "Input Capture/Compare Match Flag B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCFB_A {
    #[doc = "0: No input capture/compare match of GTCCRB is generated"]
    _0 = 0,
    #[doc = "1: An input capture/compare match of GTCCRB is generated"]
    _1 = 1,
}
impl From<TCFB_A> for bool {
    #[inline(always)]
    fn from(variant: TCFB_A) -> Self {
        variant as u8 != 0
    }
}
impl TCFB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCFB_A {
        match self.bits {
            false => TCFB_A::_0,
            true => TCFB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCFB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCFB_A::_1
    }
}
#[doc = "Field `TCFB` writer - Input Capture/Compare Match Flag B"]
pub type TCFB_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTST_SPEC, TCFB_A, O>;
impl<'a, const O: u8> TCFB_W<'a, O> {
    #[doc = "No input capture/compare match of GTCCRB is generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCFB_A::_0)
    }
    #[doc = "An input capture/compare match of GTCCRB is generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCFB_A::_1)
    }
}
#[doc = "Field `TCFC` reader - Input Compare Match Flag C"]
pub type TCFC_R = crate::BitReader<TCFC_A>;
#[doc = "Input Compare Match Flag C\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCFC_A {
    #[doc = "0: No compare match of GTCCRC is generated"]
    _0 = 0,
    #[doc = "1: A compare match of GTCCRC is generated"]
    _1 = 1,
}
impl From<TCFC_A> for bool {
    #[inline(always)]
    fn from(variant: TCFC_A) -> Self {
        variant as u8 != 0
    }
}
impl TCFC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCFC_A {
        match self.bits {
            false => TCFC_A::_0,
            true => TCFC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCFC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCFC_A::_1
    }
}
#[doc = "Field `TCFC` writer - Input Compare Match Flag C"]
pub type TCFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTST_SPEC, TCFC_A, O>;
impl<'a, const O: u8> TCFC_W<'a, O> {
    #[doc = "No compare match of GTCCRC is generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCFC_A::_0)
    }
    #[doc = "A compare match of GTCCRC is generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCFC_A::_1)
    }
}
#[doc = "Field `TCFD` reader - Input Compare Match Flag D"]
pub type TCFD_R = crate::BitReader<TCFD_A>;
#[doc = "Input Compare Match Flag D\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCFD_A {
    #[doc = "0: No compare match of GTCCRD is generated"]
    _0 = 0,
    #[doc = "1: A compare match of GTCCRD is generated"]
    _1 = 1,
}
impl From<TCFD_A> for bool {
    #[inline(always)]
    fn from(variant: TCFD_A) -> Self {
        variant as u8 != 0
    }
}
impl TCFD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCFD_A {
        match self.bits {
            false => TCFD_A::_0,
            true => TCFD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCFD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCFD_A::_1
    }
}
#[doc = "Field `TCFD` writer - Input Compare Match Flag D"]
pub type TCFD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTST_SPEC, TCFD_A, O>;
impl<'a, const O: u8> TCFD_W<'a, O> {
    #[doc = "No compare match of GTCCRD is generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCFD_A::_0)
    }
    #[doc = "A compare match of GTCCRD is generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCFD_A::_1)
    }
}
#[doc = "Field `TCFE` reader - Input Compare Match Flag E"]
pub type TCFE_R = crate::BitReader<TCFE_A>;
#[doc = "Input Compare Match Flag E\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCFE_A {
    #[doc = "0: No compare match of GTCCRE is generated"]
    _0 = 0,
    #[doc = "1: A compare match of GTCCRE is generated"]
    _1 = 1,
}
impl From<TCFE_A> for bool {
    #[inline(always)]
    fn from(variant: TCFE_A) -> Self {
        variant as u8 != 0
    }
}
impl TCFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCFE_A {
        match self.bits {
            false => TCFE_A::_0,
            true => TCFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCFE_A::_1
    }
}
#[doc = "Field `TCFE` writer - Input Compare Match Flag E"]
pub type TCFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTST_SPEC, TCFE_A, O>;
impl<'a, const O: u8> TCFE_W<'a, O> {
    #[doc = "No compare match of GTCCRE is generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCFE_A::_0)
    }
    #[doc = "A compare match of GTCCRE is generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCFE_A::_1)
    }
}
#[doc = "Field `TCFF` reader - Input Compare Match Flag F"]
pub type TCFF_R = crate::BitReader<TCFF_A>;
#[doc = "Input Compare Match Flag F\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCFF_A {
    #[doc = "0: No compare match of GTCCRF is generated"]
    _0 = 0,
    #[doc = "1: A compare match of GTCCRF is generated"]
    _1 = 1,
}
impl From<TCFF_A> for bool {
    #[inline(always)]
    fn from(variant: TCFF_A) -> Self {
        variant as u8 != 0
    }
}
impl TCFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCFF_A {
        match self.bits {
            false => TCFF_A::_0,
            true => TCFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCFF_A::_1
    }
}
#[doc = "Field `TCFF` writer - Input Compare Match Flag F"]
pub type TCFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTST_SPEC, TCFF_A, O>;
impl<'a, const O: u8> TCFF_W<'a, O> {
    #[doc = "No compare match of GTCCRF is generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCFF_A::_0)
    }
    #[doc = "A compare match of GTCCRF is generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCFF_A::_1)
    }
}
#[doc = "Field `TCFPO` reader - Overflow Flag"]
pub type TCFPO_R = crate::BitReader<TCFPO_A>;
#[doc = "Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCFPO_A {
    #[doc = "0: No overflow (crest) occurred"]
    _0 = 0,
    #[doc = "1: An overflow (crest) occurred"]
    _1 = 1,
}
impl From<TCFPO_A> for bool {
    #[inline(always)]
    fn from(variant: TCFPO_A) -> Self {
        variant as u8 != 0
    }
}
impl TCFPO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCFPO_A {
        match self.bits {
            false => TCFPO_A::_0,
            true => TCFPO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCFPO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCFPO_A::_1
    }
}
#[doc = "Field `TCFPO` writer - Overflow Flag"]
pub type TCFPO_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTST_SPEC, TCFPO_A, O>;
impl<'a, const O: u8> TCFPO_W<'a, O> {
    #[doc = "No overflow (crest) occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCFPO_A::_0)
    }
    #[doc = "An overflow (crest) occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCFPO_A::_1)
    }
}
#[doc = "Field `TCFPU` reader - Underflow Flag"]
pub type TCFPU_R = crate::BitReader<TCFPU_A>;
#[doc = "Underflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCFPU_A {
    #[doc = "0: No underflow (trough) occurred"]
    _0 = 0,
    #[doc = "1: An underflow (trough) occurred"]
    _1 = 1,
}
impl From<TCFPU_A> for bool {
    #[inline(always)]
    fn from(variant: TCFPU_A) -> Self {
        variant as u8 != 0
    }
}
impl TCFPU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCFPU_A {
        match self.bits {
            false => TCFPU_A::_0,
            true => TCFPU_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCFPU_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCFPU_A::_1
    }
}
#[doc = "Field `TCFPU` writer - Underflow Flag"]
pub type TCFPU_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTST_SPEC, TCFPU_A, O>;
impl<'a, const O: u8> TCFPU_W<'a, O> {
    #[doc = "No underflow (trough) occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCFPU_A::_0)
    }
    #[doc = "An underflow (trough) occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCFPU_A::_1)
    }
}
#[doc = "Field `ITCNT` reader - GPTn_OVF/GPTn_UDF Interrupt Skipping Count Counter"]
pub type ITCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TUCF` reader - Count Direction Flag"]
pub type TUCF_R = crate::BitReader<TUCF_A>;
#[doc = "Count Direction Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TUCF_A {
    #[doc = "0: GTCNT counter counts downward"]
    _0 = 0,
    #[doc = "1: GTCNT counter counts upward"]
    _1 = 1,
}
impl From<TUCF_A> for bool {
    #[inline(always)]
    fn from(variant: TUCF_A) -> Self {
        variant as u8 != 0
    }
}
impl TUCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TUCF_A {
        match self.bits {
            false => TUCF_A::_0,
            true => TUCF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TUCF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TUCF_A::_1
    }
}
#[doc = "Field `ADTRAUF` reader - GTADTRA Register Compare Match (Up-Counting) A/D Conversion Start Request Flag"]
pub type ADTRAUF_R = crate::BitReader<ADTRAUF_A>;
#[doc = "GTADTRA Register Compare Match (Up-Counting) A/D Conversion Start Request Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADTRAUF_A {
    #[doc = "0: No GTADTRA register compare match has occurred in up-counting."]
    _0 = 0,
    #[doc = "1: A GTADTRA register compare match has occurred in up-counting."]
    _1 = 1,
}
impl From<ADTRAUF_A> for bool {
    #[inline(always)]
    fn from(variant: ADTRAUF_A) -> Self {
        variant as u8 != 0
    }
}
impl ADTRAUF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADTRAUF_A {
        match self.bits {
            false => ADTRAUF_A::_0,
            true => ADTRAUF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADTRAUF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADTRAUF_A::_1
    }
}
#[doc = "Field `ADTRAUF` writer - GTADTRA Register Compare Match (Up-Counting) A/D Conversion Start Request Flag"]
pub type ADTRAUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTST_SPEC, ADTRAUF_A, O>;
impl<'a, const O: u8> ADTRAUF_W<'a, O> {
    #[doc = "No GTADTRA register compare match has occurred in up-counting."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADTRAUF_A::_0)
    }
    #[doc = "A GTADTRA register compare match has occurred in up-counting."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADTRAUF_A::_1)
    }
}
#[doc = "Field `ADTRADF` reader - GTADTRA Register Compare Match (Down-Counting) A/D Conversion Start Request Flag"]
pub type ADTRADF_R = crate::BitReader<ADTRADF_A>;
#[doc = "GTADTRA Register Compare Match (Down-Counting) A/D Conversion Start Request Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADTRADF_A {
    #[doc = "0: No GTADTRA register compare match has occurred in down-counting."]
    _0 = 0,
    #[doc = "1: A GTADTRA register compare match has occurred in down-counting."]
    _1 = 1,
}
impl From<ADTRADF_A> for bool {
    #[inline(always)]
    fn from(variant: ADTRADF_A) -> Self {
        variant as u8 != 0
    }
}
impl ADTRADF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADTRADF_A {
        match self.bits {
            false => ADTRADF_A::_0,
            true => ADTRADF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADTRADF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADTRADF_A::_1
    }
}
#[doc = "Field `ADTRADF` writer - GTADTRA Register Compare Match (Down-Counting) A/D Conversion Start Request Flag"]
pub type ADTRADF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTST_SPEC, ADTRADF_A, O>;
impl<'a, const O: u8> ADTRADF_W<'a, O> {
    #[doc = "No GTADTRA register compare match has occurred in down-counting."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADTRADF_A::_0)
    }
    #[doc = "A GTADTRA register compare match has occurred in down-counting."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADTRADF_A::_1)
    }
}
#[doc = "Field `ADTRBUF` reader - GTADTRB Register Compare Match (Up-Counting) A/D Conversion Start Request Flag"]
pub type ADTRBUF_R = crate::BitReader<ADTRBUF_A>;
#[doc = "GTADTRB Register Compare Match (Up-Counting) A/D Conversion Start Request Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADTRBUF_A {
    #[doc = "0: No GTADTRB register compare match has occurred in up-counting."]
    _0 = 0,
    #[doc = "1: A GTADTRB register compare match has occurred in up-counting."]
    _1 = 1,
}
impl From<ADTRBUF_A> for bool {
    #[inline(always)]
    fn from(variant: ADTRBUF_A) -> Self {
        variant as u8 != 0
    }
}
impl ADTRBUF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADTRBUF_A {
        match self.bits {
            false => ADTRBUF_A::_0,
            true => ADTRBUF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADTRBUF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADTRBUF_A::_1
    }
}
#[doc = "Field `ADTRBUF` writer - GTADTRB Register Compare Match (Up-Counting) A/D Conversion Start Request Flag"]
pub type ADTRBUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTST_SPEC, ADTRBUF_A, O>;
impl<'a, const O: u8> ADTRBUF_W<'a, O> {
    #[doc = "No GTADTRB register compare match has occurred in up-counting."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADTRBUF_A::_0)
    }
    #[doc = "A GTADTRB register compare match has occurred in up-counting."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADTRBUF_A::_1)
    }
}
#[doc = "Field `ADTRBDF` reader - GTADTRB Register Compare Match (Down-Counting) A/D Conversion Start Request Flag"]
pub type ADTRBDF_R = crate::BitReader<ADTRBDF_A>;
#[doc = "GTADTRB Register Compare Match (Down-Counting) A/D Conversion Start Request Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADTRBDF_A {
    #[doc = "0: No GTADTRB register compare match has occurred in down-counting."]
    _0 = 0,
    #[doc = "1: A GTADTRB register compare match has occurred in down-counting."]
    _1 = 1,
}
impl From<ADTRBDF_A> for bool {
    #[inline(always)]
    fn from(variant: ADTRBDF_A) -> Self {
        variant as u8 != 0
    }
}
impl ADTRBDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADTRBDF_A {
        match self.bits {
            false => ADTRBDF_A::_0,
            true => ADTRBDF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADTRBDF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADTRBDF_A::_1
    }
}
#[doc = "Field `ADTRBDF` writer - GTADTRB Register Compare Match (Down-Counting) A/D Conversion Start Request Flag"]
pub type ADTRBDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTST_SPEC, ADTRBDF_A, O>;
impl<'a, const O: u8> ADTRBDF_W<'a, O> {
    #[doc = "No GTADTRB register compare match has occurred in down-counting."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADTRBDF_A::_0)
    }
    #[doc = "A GTADTRB register compare match has occurred in down-counting."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADTRBDF_A::_1)
    }
}
#[doc = "Field `ODF` reader - Output Disable Flag"]
pub type ODF_R = crate::BitReader<ODF_A>;
#[doc = "Output Disable Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ODF_A {
    #[doc = "0: No output disable request is generated"]
    _0 = 0,
    #[doc = "1: An output disable request is generated"]
    _1 = 1,
}
impl From<ODF_A> for bool {
    #[inline(always)]
    fn from(variant: ODF_A) -> Self {
        variant as u8 != 0
    }
}
impl ODF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ODF_A {
        match self.bits {
            false => ODF_A::_0,
            true => ODF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ODF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ODF_A::_1
    }
}
#[doc = "Field `DTEF` reader - Dead Time Error Flag"]
pub type DTEF_R = crate::BitReader<DTEF_A>;
#[doc = "Dead Time Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTEF_A {
    #[doc = "0: No dead time error has occurred."]
    _0 = 0,
    #[doc = "1: A dead time error has occurred."]
    _1 = 1,
}
impl From<DTEF_A> for bool {
    #[inline(always)]
    fn from(variant: DTEF_A) -> Self {
        variant as u8 != 0
    }
}
impl DTEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTEF_A {
        match self.bits {
            false => DTEF_A::_0,
            true => DTEF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTEF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTEF_A::_1
    }
}
#[doc = "Field `OABHF` reader - Same Time Output Level High Flag"]
pub type OABHF_R = crate::BitReader<OABHF_A>;
#[doc = "Same Time Output Level High Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OABHF_A {
    #[doc = "0: No simultaneous generation of 1 both for the GTIOCA and GTIOCB pins has occurred."]
    _0 = 0,
    #[doc = "1: A simultaneous generation of 1 both for the GTIOCA and GTIOCB pins has occurred."]
    _1 = 1,
}
impl From<OABHF_A> for bool {
    #[inline(always)]
    fn from(variant: OABHF_A) -> Self {
        variant as u8 != 0
    }
}
impl OABHF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OABHF_A {
        match self.bits {
            false => OABHF_A::_0,
            true => OABHF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OABHF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OABHF_A::_1
    }
}
#[doc = "Field `OABLF` reader - Same Time Output Level Low Flag"]
pub type OABLF_R = crate::BitReader<OABLF_A>;
#[doc = "Same Time Output Level Low Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OABLF_A {
    #[doc = "0: No simultaneous generation of 0 both for the GTIOCA and GTIOCB pins has occurred."]
    _0 = 0,
    #[doc = "1: A simultaneous generation of 0 both for the GTIOCA and GTIOCB pins has occurred."]
    _1 = 1,
}
impl From<OABLF_A> for bool {
    #[inline(always)]
    fn from(variant: OABLF_A) -> Self {
        variant as u8 != 0
    }
}
impl OABLF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OABLF_A {
        match self.bits {
            false => OABLF_A::_0,
            true => OABLF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OABLF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OABLF_A::_1
    }
}
#[doc = "Field `PCF` reader - Period Count Function Finish Flag"]
pub type PCF_R = crate::BitReader<PCF_A>;
#[doc = "Period Count Function Finish Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCF_A {
    #[doc = "0: No period count function finish has occurred"]
    _0 = 0,
    #[doc = "1: A period count function finish has occurred"]
    _1 = 1,
}
impl From<PCF_A> for bool {
    #[inline(always)]
    fn from(variant: PCF_A) -> Self {
        variant as u8 != 0
    }
}
impl PCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCF_A {
        match self.bits {
            false => PCF_A::_0,
            true => PCF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PCF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PCF_A::_1
    }
}
#[doc = "Field `PCF` writer - Period Count Function Finish Flag"]
pub type PCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTST_SPEC, PCF_A, O>;
impl<'a, const O: u8> PCF_W<'a, O> {
    #[doc = "No period count function finish has occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PCF_A::_0)
    }
    #[doc = "A period count function finish has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PCF_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Input Capture/Compare Match Flag A"]
    #[inline(always)]
    pub fn tcfa(&self) -> TCFA_R {
        TCFA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Input Capture/Compare Match Flag B"]
    #[inline(always)]
    pub fn tcfb(&self) -> TCFB_R {
        TCFB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Input Compare Match Flag C"]
    #[inline(always)]
    pub fn tcfc(&self) -> TCFC_R {
        TCFC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Input Compare Match Flag D"]
    #[inline(always)]
    pub fn tcfd(&self) -> TCFD_R {
        TCFD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Input Compare Match Flag E"]
    #[inline(always)]
    pub fn tcfe(&self) -> TCFE_R {
        TCFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Input Compare Match Flag F"]
    #[inline(always)]
    pub fn tcff(&self) -> TCFF_R {
        TCFF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Overflow Flag"]
    #[inline(always)]
    pub fn tcfpo(&self) -> TCFPO_R {
        TCFPO_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Underflow Flag"]
    #[inline(always)]
    pub fn tcfpu(&self) -> TCFPU_R {
        TCFPU_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - GPTn_OVF/GPTn_UDF Interrupt Skipping Count Counter"]
    #[inline(always)]
    pub fn itcnt(&self) -> ITCNT_R {
        ITCNT_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - Count Direction Flag"]
    #[inline(always)]
    pub fn tucf(&self) -> TUCF_R {
        TUCF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - GTADTRA Register Compare Match (Up-Counting) A/D Conversion Start Request Flag"]
    #[inline(always)]
    pub fn adtrauf(&self) -> ADTRAUF_R {
        ADTRAUF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - GTADTRA Register Compare Match (Down-Counting) A/D Conversion Start Request Flag"]
    #[inline(always)]
    pub fn adtradf(&self) -> ADTRADF_R {
        ADTRADF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GTADTRB Register Compare Match (Up-Counting) A/D Conversion Start Request Flag"]
    #[inline(always)]
    pub fn adtrbuf(&self) -> ADTRBUF_R {
        ADTRBUF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GTADTRB Register Compare Match (Down-Counting) A/D Conversion Start Request Flag"]
    #[inline(always)]
    pub fn adtrbdf(&self) -> ADTRBDF_R {
        ADTRBDF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Output Disable Flag"]
    #[inline(always)]
    pub fn odf(&self) -> ODF_R {
        ODF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Dead Time Error Flag"]
    #[inline(always)]
    pub fn dtef(&self) -> DTEF_R {
        DTEF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Same Time Output Level High Flag"]
    #[inline(always)]
    pub fn oabhf(&self) -> OABHF_R {
        OABHF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Same Time Output Level Low Flag"]
    #[inline(always)]
    pub fn oablf(&self) -> OABLF_R {
        OABLF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Period Count Function Finish Flag"]
    #[inline(always)]
    pub fn pcf(&self) -> PCF_R {
        PCF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Input Capture/Compare Match Flag A"]
    #[inline(always)]
    #[must_use]
    pub fn tcfa(&mut self) -> TCFA_W<0> {
        TCFA_W::new(self)
    }
    #[doc = "Bit 1 - Input Capture/Compare Match Flag B"]
    #[inline(always)]
    #[must_use]
    pub fn tcfb(&mut self) -> TCFB_W<1> {
        TCFB_W::new(self)
    }
    #[doc = "Bit 2 - Input Compare Match Flag C"]
    #[inline(always)]
    #[must_use]
    pub fn tcfc(&mut self) -> TCFC_W<2> {
        TCFC_W::new(self)
    }
    #[doc = "Bit 3 - Input Compare Match Flag D"]
    #[inline(always)]
    #[must_use]
    pub fn tcfd(&mut self) -> TCFD_W<3> {
        TCFD_W::new(self)
    }
    #[doc = "Bit 4 - Input Compare Match Flag E"]
    #[inline(always)]
    #[must_use]
    pub fn tcfe(&mut self) -> TCFE_W<4> {
        TCFE_W::new(self)
    }
    #[doc = "Bit 5 - Input Compare Match Flag F"]
    #[inline(always)]
    #[must_use]
    pub fn tcff(&mut self) -> TCFF_W<5> {
        TCFF_W::new(self)
    }
    #[doc = "Bit 6 - Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tcfpo(&mut self) -> TCFPO_W<6> {
        TCFPO_W::new(self)
    }
    #[doc = "Bit 7 - Underflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tcfpu(&mut self) -> TCFPU_W<7> {
        TCFPU_W::new(self)
    }
    #[doc = "Bit 16 - GTADTRA Register Compare Match (Up-Counting) A/D Conversion Start Request Flag"]
    #[inline(always)]
    #[must_use]
    pub fn adtrauf(&mut self) -> ADTRAUF_W<16> {
        ADTRAUF_W::new(self)
    }
    #[doc = "Bit 17 - GTADTRA Register Compare Match (Down-Counting) A/D Conversion Start Request Flag"]
    #[inline(always)]
    #[must_use]
    pub fn adtradf(&mut self) -> ADTRADF_W<17> {
        ADTRADF_W::new(self)
    }
    #[doc = "Bit 18 - GTADTRB Register Compare Match (Up-Counting) A/D Conversion Start Request Flag"]
    #[inline(always)]
    #[must_use]
    pub fn adtrbuf(&mut self) -> ADTRBUF_W<18> {
        ADTRBUF_W::new(self)
    }
    #[doc = "Bit 19 - GTADTRB Register Compare Match (Down-Counting) A/D Conversion Start Request Flag"]
    #[inline(always)]
    #[must_use]
    pub fn adtrbdf(&mut self) -> ADTRBDF_W<19> {
        ADTRBDF_W::new(self)
    }
    #[doc = "Bit 31 - Period Count Function Finish Flag"]
    #[inline(always)]
    #[must_use]
    pub fn pcf(&mut self) -> PCF_W<31> {
        PCF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtst](index.html) module"]
pub struct GTST_SPEC;
impl crate::RegisterSpec for GTST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtst::R](R) reader structure"]
impl crate::Readable for GTST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtst::W](W) writer structure"]
impl crate::Writable for GTST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTST to value 0x8000"]
impl crate::Resettable for GTST_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
