#[doc = "Register `ADPGACR` reader"]
pub struct R(crate::R<ADPGACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPGACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPGACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPGACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADPGACR` writer"]
pub struct W(crate::W<ADPGACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPGACR_SPEC>;
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
impl From<crate::W<ADPGACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPGACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P000SEL0` reader - A through amplifier is enable for PGA P000"]
pub type P000SEL0_R = crate::BitReader<P000SEL0_A>;
#[doc = "A through amplifier is enable for PGA P000\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P000SEL0_A {
    #[doc = "0: Not through the PGA in amplifier"]
    _0 = 0,
    #[doc = "1: I will through in the PGA amplifier."]
    _1 = 1,
}
impl From<P000SEL0_A> for bool {
    #[inline(always)]
    fn from(variant: P000SEL0_A) -> Self {
        variant as u8 != 0
    }
}
impl P000SEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P000SEL0_A {
        match self.bits {
            false => P000SEL0_A::_0,
            true => P000SEL0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == P000SEL0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == P000SEL0_A::_1
    }
}
#[doc = "Field `P000SEL0` writer - A through amplifier is enable for PGA P000"]
pub type P000SEL0_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADPGACR_SPEC, P000SEL0_A, O>;
impl<'a, const O: u8> P000SEL0_W<'a, O> {
    #[doc = "Not through the PGA in amplifier"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(P000SEL0_A::_0)
    }
    #[doc = "I will through in the PGA amplifier."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(P000SEL0_A::_1)
    }
}
#[doc = "Field `P000SEL1` reader - The amplifier passing is enable for PGA P000"]
pub type P000SEL1_R = crate::BitReader<P000SEL1_A>;
#[doc = "The amplifier passing is enable for PGA P000\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P000SEL1_A {
    #[doc = "0: By way of the amplifier in PGA."]
    _0 = 0,
    #[doc = "1: Note 1 that by way of amplifier in PGA"]
    _1 = 1,
}
impl From<P000SEL1_A> for bool {
    #[inline(always)]
    fn from(variant: P000SEL1_A) -> Self {
        variant as u8 != 0
    }
}
impl P000SEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P000SEL1_A {
        match self.bits {
            false => P000SEL1_A::_0,
            true => P000SEL1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == P000SEL1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == P000SEL1_A::_1
    }
}
#[doc = "Field `P000SEL1` writer - The amplifier passing is enable for PGA P000"]
pub type P000SEL1_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADPGACR_SPEC, P000SEL1_A, O>;
impl<'a, const O: u8> P000SEL1_W<'a, O> {
    #[doc = "By way of the amplifier in PGA."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(P000SEL1_A::_0)
    }
    #[doc = "Note 1 that by way of amplifier in PGA"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(P000SEL1_A::_1)
    }
}
#[doc = "Field `P000ENAMP` reader - Amplifier enable bit for PGA P000"]
pub type P000ENAMP_R = crate::BitReader<P000ENAMP_A>;
#[doc = "Amplifier enable bit for PGA P000\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P000ENAMP_A {
    #[doc = "0: The amplifier in PGA is not used."]
    _0 = 0,
    #[doc = "1: The amplifier in PGA is used."]
    _1 = 1,
}
impl From<P000ENAMP_A> for bool {
    #[inline(always)]
    fn from(variant: P000ENAMP_A) -> Self {
        variant as u8 != 0
    }
}
impl P000ENAMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P000ENAMP_A {
        match self.bits {
            false => P000ENAMP_A::_0,
            true => P000ENAMP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == P000ENAMP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == P000ENAMP_A::_1
    }
}
#[doc = "Field `P000ENAMP` writer - Amplifier enable bit for PGA P000"]
pub type P000ENAMP_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADPGACR_SPEC, P000ENAMP_A, O>;
impl<'a, const O: u8> P000ENAMP_W<'a, O> {
    #[doc = "The amplifier in PGA is not used."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(P000ENAMP_A::_0)
    }
    #[doc = "The amplifier in PGA is used."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(P000ENAMP_A::_1)
    }
}
#[doc = "Field `P000GEN` reader - PGA P000 gain setting and enable bit"]
pub type P000GEN_R = crate::BitReader<P000GEN_A>;
#[doc = "PGA P000 gain setting and enable bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P000GEN_A {
    #[doc = "0: The gain setting is invalidated (AIN is not input in PGA)."]
    _0 = 0,
    #[doc = "1: The gain setting is effectively done (AIN is input in PGA)."]
    _1 = 1,
}
impl From<P000GEN_A> for bool {
    #[inline(always)]
    fn from(variant: P000GEN_A) -> Self {
        variant as u8 != 0
    }
}
impl P000GEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P000GEN_A {
        match self.bits {
            false => P000GEN_A::_0,
            true => P000GEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == P000GEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == P000GEN_A::_1
    }
}
#[doc = "Field `P000GEN` writer - PGA P000 gain setting and enable bit"]
pub type P000GEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADPGACR_SPEC, P000GEN_A, O>;
impl<'a, const O: u8> P000GEN_W<'a, O> {
    #[doc = "The gain setting is invalidated (AIN is not input in PGA)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(P000GEN_A::_0)
    }
    #[doc = "The gain setting is effectively done (AIN is input in PGA)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(P000GEN_A::_1)
    }
}
#[doc = "Field `P001SEL0` reader - A through amplifier is enable for PGA P001"]
pub type P001SEL0_R = crate::BitReader<P001SEL0_A>;
#[doc = "A through amplifier is enable for PGA P001\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P001SEL0_A {
    #[doc = "0: Not through the PGA in amplifier"]
    _0 = 0,
    #[doc = "1: I will through in the PGA amplifier."]
    _1 = 1,
}
impl From<P001SEL0_A> for bool {
    #[inline(always)]
    fn from(variant: P001SEL0_A) -> Self {
        variant as u8 != 0
    }
}
impl P001SEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P001SEL0_A {
        match self.bits {
            false => P001SEL0_A::_0,
            true => P001SEL0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == P001SEL0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == P001SEL0_A::_1
    }
}
#[doc = "Field `P001SEL0` writer - A through amplifier is enable for PGA P001"]
pub type P001SEL0_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADPGACR_SPEC, P001SEL0_A, O>;
impl<'a, const O: u8> P001SEL0_W<'a, O> {
    #[doc = "Not through the PGA in amplifier"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(P001SEL0_A::_0)
    }
    #[doc = "I will through in the PGA amplifier."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(P001SEL0_A::_1)
    }
}
#[doc = "Field `P001SEL1` reader - The amplifier passing is enable for PGA P001"]
pub type P001SEL1_R = crate::BitReader<P001SEL1_A>;
#[doc = "The amplifier passing is enable for PGA P001\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P001SEL1_A {
    #[doc = "0: By way of the amplifier in PGA."]
    _0 = 0,
    #[doc = "1: Note 1 that by way of amplifier in PGA"]
    _1 = 1,
}
impl From<P001SEL1_A> for bool {
    #[inline(always)]
    fn from(variant: P001SEL1_A) -> Self {
        variant as u8 != 0
    }
}
impl P001SEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P001SEL1_A {
        match self.bits {
            false => P001SEL1_A::_0,
            true => P001SEL1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == P001SEL1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == P001SEL1_A::_1
    }
}
#[doc = "Field `P001SEL1` writer - The amplifier passing is enable for PGA P001"]
pub type P001SEL1_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADPGACR_SPEC, P001SEL1_A, O>;
impl<'a, const O: u8> P001SEL1_W<'a, O> {
    #[doc = "By way of the amplifier in PGA."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(P001SEL1_A::_0)
    }
    #[doc = "Note 1 that by way of amplifier in PGA"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(P001SEL1_A::_1)
    }
}
#[doc = "Field `P001ENAMP` reader - Amplifier enable bit for PGA P001"]
pub type P001ENAMP_R = crate::BitReader<P001ENAMP_A>;
#[doc = "Amplifier enable bit for PGA P001\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P001ENAMP_A {
    #[doc = "0: The amplifier in PGA is not used."]
    _0 = 0,
    #[doc = "1: The amplifier in PGA is used."]
    _1 = 1,
}
impl From<P001ENAMP_A> for bool {
    #[inline(always)]
    fn from(variant: P001ENAMP_A) -> Self {
        variant as u8 != 0
    }
}
impl P001ENAMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P001ENAMP_A {
        match self.bits {
            false => P001ENAMP_A::_0,
            true => P001ENAMP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == P001ENAMP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == P001ENAMP_A::_1
    }
}
#[doc = "Field `P001ENAMP` writer - Amplifier enable bit for PGA P001"]
pub type P001ENAMP_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADPGACR_SPEC, P001ENAMP_A, O>;
impl<'a, const O: u8> P001ENAMP_W<'a, O> {
    #[doc = "The amplifier in PGA is not used."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(P001ENAMP_A::_0)
    }
    #[doc = "The amplifier in PGA is used."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(P001ENAMP_A::_1)
    }
}
#[doc = "Field `P001GEN` reader - PGA P001 gain setting and enable bit"]
pub type P001GEN_R = crate::BitReader<P001GEN_A>;
#[doc = "PGA P001 gain setting and enable bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P001GEN_A {
    #[doc = "0: The gain setting is invalidated (AIN is not input in PGA)."]
    _0 = 0,
    #[doc = "1: The gain setting is effectively done (AIN is input in PGA)."]
    _1 = 1,
}
impl From<P001GEN_A> for bool {
    #[inline(always)]
    fn from(variant: P001GEN_A) -> Self {
        variant as u8 != 0
    }
}
impl P001GEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P001GEN_A {
        match self.bits {
            false => P001GEN_A::_0,
            true => P001GEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == P001GEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == P001GEN_A::_1
    }
}
#[doc = "Field `P001GEN` writer - PGA P001 gain setting and enable bit"]
pub type P001GEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADPGACR_SPEC, P001GEN_A, O>;
impl<'a, const O: u8> P001GEN_W<'a, O> {
    #[doc = "The gain setting is invalidated (AIN is not input in PGA)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(P001GEN_A::_0)
    }
    #[doc = "The gain setting is effectively done (AIN is input in PGA)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(P001GEN_A::_1)
    }
}
#[doc = "Field `P002SEL0` reader - A through amplifier is enable for PGA P002"]
pub type P002SEL0_R = crate::BitReader<P002SEL0_A>;
#[doc = "A through amplifier is enable for PGA P002\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P002SEL0_A {
    #[doc = "0: Not through the PGA in amplifier"]
    _0 = 0,
    #[doc = "1: I will through in the PGA amplifier."]
    _1 = 1,
}
impl From<P002SEL0_A> for bool {
    #[inline(always)]
    fn from(variant: P002SEL0_A) -> Self {
        variant as u8 != 0
    }
}
impl P002SEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P002SEL0_A {
        match self.bits {
            false => P002SEL0_A::_0,
            true => P002SEL0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == P002SEL0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == P002SEL0_A::_1
    }
}
#[doc = "Field `P002SEL0` writer - A through amplifier is enable for PGA P002"]
pub type P002SEL0_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADPGACR_SPEC, P002SEL0_A, O>;
impl<'a, const O: u8> P002SEL0_W<'a, O> {
    #[doc = "Not through the PGA in amplifier"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(P002SEL0_A::_0)
    }
    #[doc = "I will through in the PGA amplifier."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(P002SEL0_A::_1)
    }
}
#[doc = "Field `P002SEL1` reader - The amplifier passing is enable for PGA P002"]
pub type P002SEL1_R = crate::BitReader<P002SEL1_A>;
#[doc = "The amplifier passing is enable for PGA P002\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P002SEL1_A {
    #[doc = "0: By way of the amplifier in PGA."]
    _0 = 0,
    #[doc = "1: Note 1 that by way of amplifier in PGA"]
    _1 = 1,
}
impl From<P002SEL1_A> for bool {
    #[inline(always)]
    fn from(variant: P002SEL1_A) -> Self {
        variant as u8 != 0
    }
}
impl P002SEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P002SEL1_A {
        match self.bits {
            false => P002SEL1_A::_0,
            true => P002SEL1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == P002SEL1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == P002SEL1_A::_1
    }
}
#[doc = "Field `P002SEL1` writer - The amplifier passing is enable for PGA P002"]
pub type P002SEL1_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADPGACR_SPEC, P002SEL1_A, O>;
impl<'a, const O: u8> P002SEL1_W<'a, O> {
    #[doc = "By way of the amplifier in PGA."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(P002SEL1_A::_0)
    }
    #[doc = "Note 1 that by way of amplifier in PGA"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(P002SEL1_A::_1)
    }
}
#[doc = "Field `P002ENAMP` reader - Amplifier enable bit for PGA P002"]
pub type P002ENAMP_R = crate::BitReader<P002ENAMP_A>;
#[doc = "Amplifier enable bit for PGA P002\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P002ENAMP_A {
    #[doc = "0: The amplifier in PGA is not used."]
    _0 = 0,
    #[doc = "1: The amplifier in PGA is used."]
    _1 = 1,
}
impl From<P002ENAMP_A> for bool {
    #[inline(always)]
    fn from(variant: P002ENAMP_A) -> Self {
        variant as u8 != 0
    }
}
impl P002ENAMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P002ENAMP_A {
        match self.bits {
            false => P002ENAMP_A::_0,
            true => P002ENAMP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == P002ENAMP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == P002ENAMP_A::_1
    }
}
#[doc = "Field `P002ENAMP` writer - Amplifier enable bit for PGA P002"]
pub type P002ENAMP_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADPGACR_SPEC, P002ENAMP_A, O>;
impl<'a, const O: u8> P002ENAMP_W<'a, O> {
    #[doc = "The amplifier in PGA is not used."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(P002ENAMP_A::_0)
    }
    #[doc = "The amplifier in PGA is used."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(P002ENAMP_A::_1)
    }
}
#[doc = "Field `P002GEN` reader - PGA P002 gain setting and enable bit"]
pub type P002GEN_R = crate::BitReader<P002GEN_A>;
#[doc = "PGA P002 gain setting and enable bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P002GEN_A {
    #[doc = "0: The gain setting is invalidated (AIN is not input in PGA)."]
    _0 = 0,
    #[doc = "1: The gain setting is effectively done (AIN is input in PGA)."]
    _1 = 1,
}
impl From<P002GEN_A> for bool {
    #[inline(always)]
    fn from(variant: P002GEN_A) -> Self {
        variant as u8 != 0
    }
}
impl P002GEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P002GEN_A {
        match self.bits {
            false => P002GEN_A::_0,
            true => P002GEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == P002GEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == P002GEN_A::_1
    }
}
#[doc = "Field `P002GEN` writer - PGA P002 gain setting and enable bit"]
pub type P002GEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADPGACR_SPEC, P002GEN_A, O>;
impl<'a, const O: u8> P002GEN_W<'a, O> {
    #[doc = "The gain setting is invalidated (AIN is not input in PGA)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(P002GEN_A::_0)
    }
    #[doc = "The gain setting is effectively done (AIN is input in PGA)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(P002GEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - A through amplifier is enable for PGA P000"]
    #[inline(always)]
    pub fn p000sel0(&self) -> P000SEL0_R {
        P000SEL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The amplifier passing is enable for PGA P000"]
    #[inline(always)]
    pub fn p000sel1(&self) -> P000SEL1_R {
        P000SEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Amplifier enable bit for PGA P000"]
    #[inline(always)]
    pub fn p000enamp(&self) -> P000ENAMP_R {
        P000ENAMP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PGA P000 gain setting and enable bit"]
    #[inline(always)]
    pub fn p000gen(&self) -> P000GEN_R {
        P000GEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - A through amplifier is enable for PGA P001"]
    #[inline(always)]
    pub fn p001sel0(&self) -> P001SEL0_R {
        P001SEL0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The amplifier passing is enable for PGA P001"]
    #[inline(always)]
    pub fn p001sel1(&self) -> P001SEL1_R {
        P001SEL1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Amplifier enable bit for PGA P001"]
    #[inline(always)]
    pub fn p001enamp(&self) -> P001ENAMP_R {
        P001ENAMP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PGA P001 gain setting and enable bit"]
    #[inline(always)]
    pub fn p001gen(&self) -> P001GEN_R {
        P001GEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - A through amplifier is enable for PGA P002"]
    #[inline(always)]
    pub fn p002sel0(&self) -> P002SEL0_R {
        P002SEL0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The amplifier passing is enable for PGA P002"]
    #[inline(always)]
    pub fn p002sel1(&self) -> P002SEL1_R {
        P002SEL1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Amplifier enable bit for PGA P002"]
    #[inline(always)]
    pub fn p002enamp(&self) -> P002ENAMP_R {
        P002ENAMP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PGA P002 gain setting and enable bit"]
    #[inline(always)]
    pub fn p002gen(&self) -> P002GEN_R {
        P002GEN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A through amplifier is enable for PGA P000"]
    #[inline(always)]
    #[must_use]
    pub fn p000sel0(&mut self) -> P000SEL0_W<0> {
        P000SEL0_W::new(self)
    }
    #[doc = "Bit 1 - The amplifier passing is enable for PGA P000"]
    #[inline(always)]
    #[must_use]
    pub fn p000sel1(&mut self) -> P000SEL1_W<1> {
        P000SEL1_W::new(self)
    }
    #[doc = "Bit 2 - Amplifier enable bit for PGA P000"]
    #[inline(always)]
    #[must_use]
    pub fn p000enamp(&mut self) -> P000ENAMP_W<2> {
        P000ENAMP_W::new(self)
    }
    #[doc = "Bit 3 - PGA P000 gain setting and enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn p000gen(&mut self) -> P000GEN_W<3> {
        P000GEN_W::new(self)
    }
    #[doc = "Bit 4 - A through amplifier is enable for PGA P001"]
    #[inline(always)]
    #[must_use]
    pub fn p001sel0(&mut self) -> P001SEL0_W<4> {
        P001SEL0_W::new(self)
    }
    #[doc = "Bit 5 - The amplifier passing is enable for PGA P001"]
    #[inline(always)]
    #[must_use]
    pub fn p001sel1(&mut self) -> P001SEL1_W<5> {
        P001SEL1_W::new(self)
    }
    #[doc = "Bit 6 - Amplifier enable bit for PGA P001"]
    #[inline(always)]
    #[must_use]
    pub fn p001enamp(&mut self) -> P001ENAMP_W<6> {
        P001ENAMP_W::new(self)
    }
    #[doc = "Bit 7 - PGA P001 gain setting and enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn p001gen(&mut self) -> P001GEN_W<7> {
        P001GEN_W::new(self)
    }
    #[doc = "Bit 8 - A through amplifier is enable for PGA P002"]
    #[inline(always)]
    #[must_use]
    pub fn p002sel0(&mut self) -> P002SEL0_W<8> {
        P002SEL0_W::new(self)
    }
    #[doc = "Bit 9 - The amplifier passing is enable for PGA P002"]
    #[inline(always)]
    #[must_use]
    pub fn p002sel1(&mut self) -> P002SEL1_W<9> {
        P002SEL1_W::new(self)
    }
    #[doc = "Bit 10 - Amplifier enable bit for PGA P002"]
    #[inline(always)]
    #[must_use]
    pub fn p002enamp(&mut self) -> P002ENAMP_W<10> {
        P002ENAMP_W::new(self)
    }
    #[doc = "Bit 11 - PGA P002 gain setting and enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn p002gen(&mut self) -> P002GEN_W<11> {
        P002GEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Programmable Gain Amplifier Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpgacr](index.html) module"]
pub struct ADPGACR_SPEC;
impl crate::RegisterSpec for ADPGACR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adpgacr::R](R) reader structure"]
impl crate::Readable for ADPGACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpgacr::W](W) writer structure"]
impl crate::Writable for ADPGACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADPGACR to value 0x9999"]
impl crate::Resettable for ADPGACR_SPEC {
    const RESET_VALUE: Self::Ux = 0x9999;
}
