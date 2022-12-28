#[doc = "Register `POEGGD` reader"]
pub struct R(crate::R<POEGGD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POEGGD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POEGGD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POEGGD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POEGGD` writer"]
pub struct W(crate::W<POEGGD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POEGGD_SPEC>;
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
impl From<crate::W<POEGGD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POEGGD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIDF` reader - Port Input Detection Flag"]
pub type PIDF_R = crate::BitReader<PIDF_A>;
#[doc = "Port Input Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIDF_A {
    #[doc = "0: The selected input level was not detected on the GTETRGn pin"]
    _0 = 0,
    #[doc = "1: The selected input level was detected on the GTETRGn pin"]
    _1 = 1,
}
impl From<PIDF_A> for bool {
    #[inline(always)]
    fn from(variant: PIDF_A) -> Self {
        variant as u8 != 0
    }
}
impl PIDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIDF_A {
        match self.bits {
            false => PIDF_A::_0,
            true => PIDF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIDF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIDF_A::_1
    }
}
#[doc = "Field `PIDF` writer - Port Input Detection Flag"]
pub type PIDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEGGD_SPEC, PIDF_A, O>;
impl<'a, const O: u8> PIDF_W<'a, O> {
    #[doc = "The selected input level was not detected on the GTETRGn pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIDF_A::_0)
    }
    #[doc = "The selected input level was detected on the GTETRGn pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIDF_A::_1)
    }
}
#[doc = "Field `IOCF` reader - GPT or ACMPHS Output Stop Request Detection Flag"]
pub type IOCF_R = crate::BitReader<IOCF_A>;
#[doc = "GPT or ACMPHS Output Stop Request Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOCF_A {
    #[doc = "0: Neither stopping of GPT output nor a comparator edge was detected"]
    _0 = 0,
    #[doc = "1: Either stopping of GPT output or comparator edge was detected"]
    _1 = 1,
}
impl From<IOCF_A> for bool {
    #[inline(always)]
    fn from(variant: IOCF_A) -> Self {
        variant as u8 != 0
    }
}
impl IOCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOCF_A {
        match self.bits {
            false => IOCF_A::_0,
            true => IOCF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IOCF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IOCF_A::_1
    }
}
#[doc = "Field `IOCF` writer - GPT or ACMPHS Output Stop Request Detection Flag"]
pub type IOCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEGGD_SPEC, IOCF_A, O>;
impl<'a, const O: u8> IOCF_W<'a, O> {
    #[doc = "Neither stopping of GPT output nor a comparator edge was detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IOCF_A::_0)
    }
    #[doc = "Either stopping of GPT output or comparator edge was detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IOCF_A::_1)
    }
}
#[doc = "Field `OSTPF` reader - Oscillation Stop Detection Flag"]
pub type OSTPF_R = crate::BitReader<OSTPF_A>;
#[doc = "Oscillation Stop Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSTPF_A {
    #[doc = "0: Stopping of oscillation was not detected"]
    _0 = 0,
    #[doc = "1: Stopping of oscillation was detected"]
    _1 = 1,
}
impl From<OSTPF_A> for bool {
    #[inline(always)]
    fn from(variant: OSTPF_A) -> Self {
        variant as u8 != 0
    }
}
impl OSTPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSTPF_A {
        match self.bits {
            false => OSTPF_A::_0,
            true => OSTPF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OSTPF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OSTPF_A::_1
    }
}
#[doc = "Field `OSTPF` writer - Oscillation Stop Detection Flag"]
pub type OSTPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEGGD_SPEC, OSTPF_A, O>;
impl<'a, const O: u8> OSTPF_W<'a, O> {
    #[doc = "Stopping of oscillation was not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OSTPF_A::_0)
    }
    #[doc = "Stopping of oscillation was detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OSTPF_A::_1)
    }
}
#[doc = "Field `SSF` reader - Software Stop Flag"]
pub type SSF_R = crate::BitReader<SSF_A>;
#[doc = "Software Stop Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSF_A {
    #[doc = "0: Software has not stopped output"]
    _0 = 0,
    #[doc = "1: Software has stopped output"]
    _1 = 1,
}
impl From<SSF_A> for bool {
    #[inline(always)]
    fn from(variant: SSF_A) -> Self {
        variant as u8 != 0
    }
}
impl SSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSF_A {
        match self.bits {
            false => SSF_A::_0,
            true => SSF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSF_A::_1
    }
}
#[doc = "Field `SSF` writer - Software Stop Flag"]
pub type SSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEGGD_SPEC, SSF_A, O>;
impl<'a, const O: u8> SSF_W<'a, O> {
    #[doc = "Software has not stopped output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSF_A::_0)
    }
    #[doc = "Software has stopped output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSF_A::_1)
    }
}
#[doc = "Field `PIDE` reader - Port Input Detection Enable"]
pub type PIDE_R = crate::BitReader<PIDE_A>;
#[doc = "Port Input Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIDE_A {
    #[doc = "0: Detection of input levels on the corresponding GTETRGn pin is disabled"]
    _0 = 0,
    #[doc = "1: Detection of input levels on the corresponding GTETRGn pin is enabled"]
    _1 = 1,
}
impl From<PIDE_A> for bool {
    #[inline(always)]
    fn from(variant: PIDE_A) -> Self {
        variant as u8 != 0
    }
}
impl PIDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIDE_A {
        match self.bits {
            false => PIDE_A::_0,
            true => PIDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIDE_A::_1
    }
}
#[doc = "Field `PIDE` writer - Port Input Detection Enable"]
pub type PIDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEGGD_SPEC, PIDE_A, O>;
impl<'a, const O: u8> PIDE_W<'a, O> {
    #[doc = "Detection of input levels on the corresponding GTETRGn pin is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIDE_A::_0)
    }
    #[doc = "Detection of input levels on the corresponding GTETRGn pin is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIDE_A::_1)
    }
}
#[doc = "Field `IOCE` reader - GPT Output Stop Request Enable"]
pub type IOCE_R = crate::BitReader<IOCE_A>;
#[doc = "GPT Output Stop Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOCE_A {
    #[doc = "0: Detection of stopping of output from the GPT is disabled"]
    _0 = 0,
    #[doc = "1: Detection of stopping of output from the GPT is enabled"]
    _1 = 1,
}
impl From<IOCE_A> for bool {
    #[inline(always)]
    fn from(variant: IOCE_A) -> Self {
        variant as u8 != 0
    }
}
impl IOCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOCE_A {
        match self.bits {
            false => IOCE_A::_0,
            true => IOCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IOCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IOCE_A::_1
    }
}
#[doc = "Field `IOCE` writer - GPT Output Stop Request Enable"]
pub type IOCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEGGD_SPEC, IOCE_A, O>;
impl<'a, const O: u8> IOCE_W<'a, O> {
    #[doc = "Detection of stopping of output from the GPT is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IOCE_A::_0)
    }
    #[doc = "Detection of stopping of output from the GPT is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IOCE_A::_1)
    }
}
#[doc = "Field `OSTPE` reader - Enable Stopping Output on Stopping of Oscillation"]
pub type OSTPE_R = crate::BitReader<OSTPE_A>;
#[doc = "Enable Stopping Output on Stopping of Oscillation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSTPE_A {
    #[doc = "0: Detection of stopping of oscillation is disabled"]
    _0 = 0,
    #[doc = "1: Detection of stopping of oscillation is enabled"]
    _1 = 1,
}
impl From<OSTPE_A> for bool {
    #[inline(always)]
    fn from(variant: OSTPE_A) -> Self {
        variant as u8 != 0
    }
}
impl OSTPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSTPE_A {
        match self.bits {
            false => OSTPE_A::_0,
            true => OSTPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OSTPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OSTPE_A::_1
    }
}
#[doc = "Field `OSTPE` writer - Enable Stopping Output on Stopping of Oscillation"]
pub type OSTPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEGGD_SPEC, OSTPE_A, O>;
impl<'a, const O: u8> OSTPE_W<'a, O> {
    #[doc = "Detection of stopping of oscillation is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OSTPE_A::_0)
    }
    #[doc = "Detection of stopping of oscillation is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OSTPE_A::_1)
    }
}
#[doc = "Field `CDRE0` reader - ACMPHS0 Enable"]
pub type CDRE0_R = crate::BitReader<CDRE0_A>;
#[doc = "ACMPHS0 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDRE0_A {
    #[doc = "0: Comparator edge detection 0 is disabled"]
    _0 = 0,
    #[doc = "1: Comparator edge detection 0 is enabled"]
    _1 = 1,
}
impl From<CDRE0_A> for bool {
    #[inline(always)]
    fn from(variant: CDRE0_A) -> Self {
        variant as u8 != 0
    }
}
impl CDRE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDRE0_A {
        match self.bits {
            false => CDRE0_A::_0,
            true => CDRE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CDRE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CDRE0_A::_1
    }
}
#[doc = "Field `CDRE0` writer - ACMPHS0 Enable"]
pub type CDRE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEGGD_SPEC, CDRE0_A, O>;
impl<'a, const O: u8> CDRE0_W<'a, O> {
    #[doc = "Comparator edge detection 0 is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CDRE0_A::_0)
    }
    #[doc = "Comparator edge detection 0 is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CDRE0_A::_1)
    }
}
#[doc = "Field `CDRE1` reader - ACMPHS1 Enable"]
pub type CDRE1_R = crate::BitReader<CDRE1_A>;
#[doc = "ACMPHS1 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDRE1_A {
    #[doc = "0: Comparator edge detection 1 is disabled"]
    _0 = 0,
    #[doc = "1: Comparator edge detection 1 is enabled"]
    _1 = 1,
}
impl From<CDRE1_A> for bool {
    #[inline(always)]
    fn from(variant: CDRE1_A) -> Self {
        variant as u8 != 0
    }
}
impl CDRE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDRE1_A {
        match self.bits {
            false => CDRE1_A::_0,
            true => CDRE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CDRE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CDRE1_A::_1
    }
}
#[doc = "Field `CDRE1` writer - ACMPHS1 Enable"]
pub type CDRE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEGGD_SPEC, CDRE1_A, O>;
impl<'a, const O: u8> CDRE1_W<'a, O> {
    #[doc = "Comparator edge detection 1 is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CDRE1_A::_0)
    }
    #[doc = "Comparator edge detection 1 is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CDRE1_A::_1)
    }
}
#[doc = "Field `CDRE2` reader - ACMPHS2 Enable"]
pub type CDRE2_R = crate::BitReader<CDRE2_A>;
#[doc = "ACMPHS2 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDRE2_A {
    #[doc = "0: Comparator edge detection 2 is disabled"]
    _0 = 0,
    #[doc = "1: Comparator edge detection 2 is enabled"]
    _1 = 1,
}
impl From<CDRE2_A> for bool {
    #[inline(always)]
    fn from(variant: CDRE2_A) -> Self {
        variant as u8 != 0
    }
}
impl CDRE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDRE2_A {
        match self.bits {
            false => CDRE2_A::_0,
            true => CDRE2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CDRE2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CDRE2_A::_1
    }
}
#[doc = "Field `CDRE2` writer - ACMPHS2 Enable"]
pub type CDRE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEGGD_SPEC, CDRE2_A, O>;
impl<'a, const O: u8> CDRE2_W<'a, O> {
    #[doc = "Comparator edge detection 2 is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CDRE2_A::_0)
    }
    #[doc = "Comparator edge detection 2 is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CDRE2_A::_1)
    }
}
#[doc = "Field `CDRE3` reader - ACMPHS3 Enable"]
pub type CDRE3_R = crate::BitReader<CDRE3_A>;
#[doc = "ACMPHS3 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDRE3_A {
    #[doc = "0: Comparator edge detection 3 is disabled"]
    _0 = 0,
    #[doc = "1: Comparator edge detection 3 is enabled"]
    _1 = 1,
}
impl From<CDRE3_A> for bool {
    #[inline(always)]
    fn from(variant: CDRE3_A) -> Self {
        variant as u8 != 0
    }
}
impl CDRE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDRE3_A {
        match self.bits {
            false => CDRE3_A::_0,
            true => CDRE3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CDRE3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CDRE3_A::_1
    }
}
#[doc = "Field `CDRE3` writer - ACMPHS3 Enable"]
pub type CDRE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEGGD_SPEC, CDRE3_A, O>;
impl<'a, const O: u8> CDRE3_W<'a, O> {
    #[doc = "Comparator edge detection 3 is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CDRE3_A::_0)
    }
    #[doc = "Comparator edge detection 3 is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CDRE3_A::_1)
    }
}
#[doc = "Field `ST` reader - GTETRGn Input Status Flag"]
pub type ST_R = crate::BitReader<ST_A>;
#[doc = "GTETRGn Input Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ST_A {
    #[doc = "0: The corresponding external trigger for output to the GPT is 0"]
    _0 = 0,
    #[doc = "1: The corresponding external trigger for output to the GPT is 1"]
    _1 = 1,
}
impl From<ST_A> for bool {
    #[inline(always)]
    fn from(variant: ST_A) -> Self {
        variant as u8 != 0
    }
}
impl ST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ST_A {
        match self.bits {
            false => ST_A::_0,
            true => ST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ST_A::_1
    }
}
#[doc = "Field `INV` reader - GTETRGn Input Inverting"]
pub type INV_R = crate::BitReader<INV_A>;
#[doc = "GTETRGn Input Inverting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INV_A {
    #[doc = "0: Input on the GTETRGn pin is not inverted"]
    _0 = 0,
    #[doc = "1: Input on the GTETRGn pin is inverted"]
    _1 = 1,
}
impl From<INV_A> for bool {
    #[inline(always)]
    fn from(variant: INV_A) -> Self {
        variant as u8 != 0
    }
}
impl INV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INV_A {
        match self.bits {
            false => INV_A::_0,
            true => INV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INV_A::_1
    }
}
#[doc = "Field `INV` writer - GTETRGn Input Inverting"]
pub type INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEGGD_SPEC, INV_A, O>;
impl<'a, const O: u8> INV_W<'a, O> {
    #[doc = "Input on the GTETRGn pin is not inverted"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INV_A::_0)
    }
    #[doc = "Input on the GTETRGn pin is inverted"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INV_A::_1)
    }
}
#[doc = "Field `NFEN` reader - Noise filter Enable"]
pub type NFEN_R = crate::BitReader<NFEN_A>;
#[doc = "Noise filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NFEN_A {
    #[doc = "0: Digital noise filter on the GTETRGn pin is disabled"]
    _0 = 0,
    #[doc = "1: Digital noise filter on the GTETRGn pin is enabled"]
    _1 = 1,
}
impl From<NFEN_A> for bool {
    #[inline(always)]
    fn from(variant: NFEN_A) -> Self {
        variant as u8 != 0
    }
}
impl NFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NFEN_A {
        match self.bits {
            false => NFEN_A::_0,
            true => NFEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NFEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NFEN_A::_1
    }
}
#[doc = "Field `NFEN` writer - Noise filter Enable"]
pub type NFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEGGD_SPEC, NFEN_A, O>;
impl<'a, const O: u8> NFEN_W<'a, O> {
    #[doc = "Digital noise filter on the GTETRGn pin is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NFEN_A::_0)
    }
    #[doc = "Digital noise filter on the GTETRGn pin is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NFEN_A::_1)
    }
}
#[doc = "Field `NFCS` reader - Noise filter Clock Select"]
pub type NFCS_R = crate::FieldReader<u8, NFCS_A>;
#[doc = "Noise filter Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NFCS_A {
    #[doc = "0: Samples the input level of GTETRGn pin three times per PCLKB/1 clock"]
    _00 = 0,
    #[doc = "1: Samples the input level of GTETRGn pin three times per PCLKB/8 clock"]
    _01 = 1,
    #[doc = "2: Samples the input level of GTETRGn pin three times per PCLKB/32 clock"]
    _10 = 2,
    #[doc = "3: Samples the input level of GTETRGn pin three times per PCLKB/128 clock"]
    _11 = 3,
}
impl From<NFCS_A> for u8 {
    #[inline(always)]
    fn from(variant: NFCS_A) -> Self {
        variant as _
    }
}
impl NFCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NFCS_A {
        match self.bits {
            0 => NFCS_A::_00,
            1 => NFCS_A::_01,
            2 => NFCS_A::_10,
            3 => NFCS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == NFCS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == NFCS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == NFCS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == NFCS_A::_11
    }
}
#[doc = "Field `NFCS` writer - Noise filter Clock Select"]
pub type NFCS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, POEGGD_SPEC, u8, NFCS_A, 2, O>;
impl<'a, const O: u8> NFCS_W<'a, O> {
    #[doc = "Samples the input level of GTETRGn pin three times per PCLKB/1 clock"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(NFCS_A::_00)
    }
    #[doc = "Samples the input level of GTETRGn pin three times per PCLKB/8 clock"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(NFCS_A::_01)
    }
    #[doc = "Samples the input level of GTETRGn pin three times per PCLKB/32 clock"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(NFCS_A::_10)
    }
    #[doc = "Samples the input level of GTETRGn pin three times per PCLKB/128 clock"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(NFCS_A::_11)
    }
}
impl R {
    #[doc = "Bit 0 - Port Input Detection Flag"]
    #[inline(always)]
    pub fn pidf(&self) -> PIDF_R {
        PIDF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPT or ACMPHS Output Stop Request Detection Flag"]
    #[inline(always)]
    pub fn iocf(&self) -> IOCF_R {
        IOCF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Oscillation Stop Detection Flag"]
    #[inline(always)]
    pub fn ostpf(&self) -> OSTPF_R {
        OSTPF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Software Stop Flag"]
    #[inline(always)]
    pub fn ssf(&self) -> SSF_R {
        SSF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port Input Detection Enable"]
    #[inline(always)]
    pub fn pide(&self) -> PIDE_R {
        PIDE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPT Output Stop Request Enable"]
    #[inline(always)]
    pub fn ioce(&self) -> IOCE_R {
        IOCE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Stopping Output on Stopping of Oscillation"]
    #[inline(always)]
    pub fn ostpe(&self) -> OSTPE_R {
        OSTPE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - ACMPHS0 Enable"]
    #[inline(always)]
    pub fn cdre0(&self) -> CDRE0_R {
        CDRE0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ACMPHS1 Enable"]
    #[inline(always)]
    pub fn cdre1(&self) -> CDRE1_R {
        CDRE1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ACMPHS2 Enable"]
    #[inline(always)]
    pub fn cdre2(&self) -> CDRE2_R {
        CDRE2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ACMPHS3 Enable"]
    #[inline(always)]
    pub fn cdre3(&self) -> CDRE3_R {
        CDRE3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - GTETRGn Input Status Flag"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 28 - GTETRGn Input Inverting"]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Noise filter Enable"]
    #[inline(always)]
    pub fn nfen(&self) -> NFEN_R {
        NFEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Noise filter Clock Select"]
    #[inline(always)]
    pub fn nfcs(&self) -> NFCS_R {
        NFCS_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Port Input Detection Flag"]
    #[inline(always)]
    #[must_use]
    pub fn pidf(&mut self) -> PIDF_W<0> {
        PIDF_W::new(self)
    }
    #[doc = "Bit 1 - GPT or ACMPHS Output Stop Request Detection Flag"]
    #[inline(always)]
    #[must_use]
    pub fn iocf(&mut self) -> IOCF_W<1> {
        IOCF_W::new(self)
    }
    #[doc = "Bit 2 - Oscillation Stop Detection Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ostpf(&mut self) -> OSTPF_W<2> {
        OSTPF_W::new(self)
    }
    #[doc = "Bit 3 - Software Stop Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ssf(&mut self) -> SSF_W<3> {
        SSF_W::new(self)
    }
    #[doc = "Bit 4 - Port Input Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pide(&mut self) -> PIDE_W<4> {
        PIDE_W::new(self)
    }
    #[doc = "Bit 5 - GPT Output Stop Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ioce(&mut self) -> IOCE_W<5> {
        IOCE_W::new(self)
    }
    #[doc = "Bit 6 - Enable Stopping Output on Stopping of Oscillation"]
    #[inline(always)]
    #[must_use]
    pub fn ostpe(&mut self) -> OSTPE_W<6> {
        OSTPE_W::new(self)
    }
    #[doc = "Bit 8 - ACMPHS0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cdre0(&mut self) -> CDRE0_W<8> {
        CDRE0_W::new(self)
    }
    #[doc = "Bit 9 - ACMPHS1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cdre1(&mut self) -> CDRE1_W<9> {
        CDRE1_W::new(self)
    }
    #[doc = "Bit 10 - ACMPHS2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cdre2(&mut self) -> CDRE2_W<10> {
        CDRE2_W::new(self)
    }
    #[doc = "Bit 11 - ACMPHS3 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cdre3(&mut self) -> CDRE3_W<11> {
        CDRE3_W::new(self)
    }
    #[doc = "Bit 28 - GTETRGn Input Inverting"]
    #[inline(always)]
    #[must_use]
    pub fn inv(&mut self) -> INV_W<28> {
        INV_W::new(self)
    }
    #[doc = "Bit 29 - Noise filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nfen(&mut self) -> NFEN_W<29> {
        NFEN_W::new(self)
    }
    #[doc = "Bits 30:31 - Noise filter Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn nfcs(&mut self) -> NFCS_W<30> {
        NFCS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "POEG Group D Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [poeggd](index.html) module"]
pub struct POEGGD_SPEC;
impl crate::RegisterSpec for POEGGD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [poeggd::R](R) reader structure"]
impl crate::Readable for POEGGD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [poeggd::W](W) writer structure"]
impl crate::Writable for POEGGD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets POEGGD to value 0"]
impl crate::Resettable for POEGGD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
