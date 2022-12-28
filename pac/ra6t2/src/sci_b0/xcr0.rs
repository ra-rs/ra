#[doc = "Register `XCR0` reader"]
pub struct R(crate::R<XCR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XCR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XCR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XCR0` writer"]
pub struct W(crate::W<XCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XCR0_SPEC>;
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
impl From<crate::W<XCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCSS` reader - Timer count clock source selection"]
pub type TCSS_R = crate::FieldReader<u8, TCSS_A>;
#[doc = "Timer count clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TCSS_A {
    #[doc = "1: TCLK/4"]
    _01 = 1,
    #[doc = "2: TCLK/16"]
    _10 = 2,
    #[doc = "3: TCLK/64"]
    _11 = 3,
}
impl From<TCSS_A> for u8 {
    #[inline(always)]
    fn from(variant: TCSS_A) -> Self {
        variant as _
    }
}
impl TCSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TCSS_A> {
        match self.bits {
            1 => Some(TCSS_A::_01),
            2 => Some(TCSS_A::_10),
            3 => Some(TCSS_A::_11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TCSS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TCSS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TCSS_A::_11
    }
}
#[doc = "Field `TCSS` writer - Timer count clock source selection"]
pub type TCSS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, XCR0_SPEC, u8, TCSS_A, 2, O>;
impl<'a, const O: u8> TCSS_W<'a, O> {
    #[doc = "TCLK/4"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TCSS_A::_01)
    }
    #[doc = "TCLK/16"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TCSS_A::_10)
    }
    #[doc = "TCLK/64"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TCSS_A::_11)
    }
}
#[doc = "Field `BFE` reader - Break Field enable"]
pub type BFE_R = crate::BitReader<BFE_A>;
#[doc = "Break Field enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFE_A {
    #[doc = "0: No Break Field"]
    _0 = 0,
    #[doc = "1: With Break Field"]
    _1 = 1,
}
impl From<BFE_A> for bool {
    #[inline(always)]
    fn from(variant: BFE_A) -> Self {
        variant as u8 != 0
    }
}
impl BFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFE_A {
        match self.bits {
            false => BFE_A::_0,
            true => BFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BFE_A::_1
    }
}
#[doc = "Field `BFE` writer - Break Field enable"]
pub type BFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, XCR0_SPEC, BFE_A, O>;
impl<'a, const O: u8> BFE_W<'a, O> {
    #[doc = "No Break Field"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BFE_A::_0)
    }
    #[doc = "With Break Field"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BFE_A::_1)
    }
}
#[doc = "Field `CF0RE` reader - Control Field 0 enable"]
pub type CF0RE_R = crate::BitReader<CF0RE_A>;
#[doc = "Control Field 0 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CF0RE_A {
    #[doc = "0: No Control Field 0"]
    _0 = 0,
    #[doc = "1: With Control Field 0"]
    _1 = 1,
}
impl From<CF0RE_A> for bool {
    #[inline(always)]
    fn from(variant: CF0RE_A) -> Self {
        variant as u8 != 0
    }
}
impl CF0RE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CF0RE_A {
        match self.bits {
            false => CF0RE_A::_0,
            true => CF0RE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CF0RE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CF0RE_A::_1
    }
}
#[doc = "Field `CF0RE` writer - Control Field 0 enable"]
pub type CF0RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, XCR0_SPEC, CF0RE_A, O>;
impl<'a, const O: u8> CF0RE_W<'a, O> {
    #[doc = "No Control Field 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CF0RE_A::_0)
    }
    #[doc = "With Control Field 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CF0RE_A::_1)
    }
}
#[doc = "Field `CF1DS` reader - Control Field1 compare data select"]
pub type CF1DS_R = crate::FieldReader<u8, CF1DS_A>;
#[doc = "Control Field1 compare data select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CF1DS_A {
    #[doc = "0: Select XCR1.PCF1D\\[7:0\\]
as the compare data"]
    _00 = 0,
    #[doc = "1: Select XCR1.SCF1D\\[7:0\\]
as the compare data"]
    _01 = 1,
    #[doc = "2: Select both XCR1.PCF1D\\[7:0\\]
and XCR1.SCF1D\\[7:0\\]
as the compare data"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<CF1DS_A> for u8 {
    #[inline(always)]
    fn from(variant: CF1DS_A) -> Self {
        variant as _
    }
}
impl CF1DS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CF1DS_A {
        match self.bits {
            0 => CF1DS_A::_00,
            1 => CF1DS_A::_01,
            2 => CF1DS_A::_10,
            3 => CF1DS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CF1DS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CF1DS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CF1DS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CF1DS_A::_11
    }
}
#[doc = "Field `CF1DS` writer - Control Field1 compare data select"]
pub type CF1DS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, XCR0_SPEC, u8, CF1DS_A, 2, O>;
impl<'a, const O: u8> CF1DS_W<'a, O> {
    #[doc = "Select XCR1.PCF1D\\[7:0\\]
as the compare data"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CF1DS_A::_00)
    }
    #[doc = "Select XCR1.SCF1D\\[7:0\\]
as the compare data"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CF1DS_A::_01)
    }
    #[doc = "Select both XCR1.PCF1D\\[7:0\\]
and XCR1.SCF1D\\[7:0\\]
as the compare data"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CF1DS_A::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CF1DS_A::_11)
    }
}
#[doc = "Field `PIBE` reader - Priority interrupt bit enable"]
pub type PIBE_R = crate::BitReader<PIBE_A>;
#[doc = "Priority interrupt bit enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIBE_A {
    #[doc = "0: Priority interrupt bit disable"]
    _0 = 0,
    #[doc = "1: Priority interrupt bit enable"]
    _1 = 1,
}
impl From<PIBE_A> for bool {
    #[inline(always)]
    fn from(variant: PIBE_A) -> Self {
        variant as u8 != 0
    }
}
impl PIBE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIBE_A {
        match self.bits {
            false => PIBE_A::_0,
            true => PIBE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIBE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIBE_A::_1
    }
}
#[doc = "Field `PIBE` writer - Priority interrupt bit enable"]
pub type PIBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, XCR0_SPEC, PIBE_A, O>;
impl<'a, const O: u8> PIBE_W<'a, O> {
    #[doc = "Priority interrupt bit disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIBE_A::_0)
    }
    #[doc = "Priority interrupt bit enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIBE_A::_1)
    }
}
#[doc = "Field `PIBS` reader - Priority interrupt bit select"]
pub type PIBS_R = crate::FieldReader<u8, PIBS_A>;
#[doc = "Priority interrupt bit select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PIBS_A {
    #[doc = "0: bit 0 of Control Field 1"]
    _000 = 0,
    #[doc = "1: bit 1 of Control Field 1"]
    _001 = 1,
    #[doc = "2: bit 2 of Control Field 1"]
    _010 = 2,
    #[doc = "3: bit 3 of Control Field 1"]
    _011 = 3,
    #[doc = "4: bit 4 of Control Field 1"]
    _100 = 4,
    #[doc = "5: bit 5 of Control Field 1"]
    _101 = 5,
    #[doc = "6: bit 6 of Control Field 1"]
    _110 = 6,
    #[doc = "7: bit 7 of Control Field 1"]
    _111 = 7,
}
impl From<PIBS_A> for u8 {
    #[inline(always)]
    fn from(variant: PIBS_A) -> Self {
        variant as _
    }
}
impl PIBS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIBS_A {
        match self.bits {
            0 => PIBS_A::_000,
            1 => PIBS_A::_001,
            2 => PIBS_A::_010,
            3 => PIBS_A::_011,
            4 => PIBS_A::_100,
            5 => PIBS_A::_101,
            6 => PIBS_A::_110,
            7 => PIBS_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == PIBS_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == PIBS_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == PIBS_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == PIBS_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == PIBS_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == PIBS_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == PIBS_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == PIBS_A::_111
    }
}
#[doc = "Field `PIBS` writer - Priority interrupt bit select"]
pub type PIBS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, XCR0_SPEC, u8, PIBS_A, 3, O>;
impl<'a, const O: u8> PIBS_W<'a, O> {
    #[doc = "bit 0 of Control Field 1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(PIBS_A::_000)
    }
    #[doc = "bit 1 of Control Field 1"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(PIBS_A::_001)
    }
    #[doc = "bit 2 of Control Field 1"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(PIBS_A::_010)
    }
    #[doc = "bit 3 of Control Field 1"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(PIBS_A::_011)
    }
    #[doc = "bit 4 of Control Field 1"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(PIBS_A::_100)
    }
    #[doc = "bit 5 of Control Field 1"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(PIBS_A::_101)
    }
    #[doc = "bit 6 of Control Field 1"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(PIBS_A::_110)
    }
    #[doc = "bit 7 of Control Field 1"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(PIBS_A::_111)
    }
}
#[doc = "Field `BFOIE` reader - Break Field output completion interrupt enable"]
pub type BFOIE_R = crate::BitReader<BFOIE_A>;
#[doc = "Break Field output completion interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFOIE_A {
    #[doc = "0: Break Field output completion is not included in SCIn_TXI interrupt factor"]
    _0 = 0,
    #[doc = "1: Break Field output completion is included in SCIn_TXI interrupt factor"]
    _1 = 1,
}
impl From<BFOIE_A> for bool {
    #[inline(always)]
    fn from(variant: BFOIE_A) -> Self {
        variant as u8 != 0
    }
}
impl BFOIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFOIE_A {
        match self.bits {
            false => BFOIE_A::_0,
            true => BFOIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BFOIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BFOIE_A::_1
    }
}
#[doc = "Field `BFOIE` writer - Break Field output completion interrupt enable"]
pub type BFOIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, XCR0_SPEC, BFOIE_A, O>;
impl<'a, const O: u8> BFOIE_W<'a, O> {
    #[doc = "Break Field output completion is not included in SCIn_TXI interrupt factor"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BFOIE_A::_0)
    }
    #[doc = "Break Field output completion is included in SCIn_TXI interrupt factor"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BFOIE_A::_1)
    }
}
#[doc = "Field `BCDIE` reader - Bus conflict detection interrupt enable"]
pub type BCDIE_R = crate::BitReader<BCDIE_A>;
#[doc = "Bus conflict detection interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BCDIE_A {
    #[doc = "0: Bus conflict detection is not included in SCIn_ERI interrupt factor"]
    _0 = 0,
    #[doc = "1: Bus conflict detection is included in SCIn_ERI interrupt factor"]
    _1 = 1,
}
impl From<BCDIE_A> for bool {
    #[inline(always)]
    fn from(variant: BCDIE_A) -> Self {
        variant as u8 != 0
    }
}
impl BCDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCDIE_A {
        match self.bits {
            false => BCDIE_A::_0,
            true => BCDIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BCDIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BCDIE_A::_1
    }
}
#[doc = "Field `BCDIE` writer - Bus conflict detection interrupt enable"]
pub type BCDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, XCR0_SPEC, BCDIE_A, O>;
impl<'a, const O: u8> BCDIE_W<'a, O> {
    #[doc = "Bus conflict detection is not included in SCIn_ERI interrupt factor"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BCDIE_A::_0)
    }
    #[doc = "Bus conflict detection is included in SCIn_ERI interrupt factor"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BCDIE_A::_1)
    }
}
#[doc = "Field `BFDIE` reader - Break Field detection interrupt enable"]
pub type BFDIE_R = crate::BitReader<BFDIE_A>;
#[doc = "Break Field detection interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFDIE_A {
    #[doc = "0: Break Field detection interrupt disable"]
    _0 = 0,
    #[doc = "1: Break Field detection interrupt enable"]
    _1 = 1,
}
impl From<BFDIE_A> for bool {
    #[inline(always)]
    fn from(variant: BFDIE_A) -> Self {
        variant as u8 != 0
    }
}
impl BFDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFDIE_A {
        match self.bits {
            false => BFDIE_A::_0,
            true => BFDIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BFDIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BFDIE_A::_1
    }
}
#[doc = "Field `BFDIE` writer - Break Field detection interrupt enable"]
pub type BFDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, XCR0_SPEC, BFDIE_A, O>;
impl<'a, const O: u8> BFDIE_W<'a, O> {
    #[doc = "Break Field detection interrupt disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BFDIE_A::_0)
    }
    #[doc = "Break Field detection interrupt enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BFDIE_A::_1)
    }
}
#[doc = "Field `COFIE` reader - Counter overflow interrupt enable"]
pub type COFIE_R = crate::BitReader<COFIE_A>;
#[doc = "Counter overflow interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COFIE_A {
    #[doc = "0: Counter overflow is not included in SCIn_ERI interrupt factor"]
    _0 = 0,
    #[doc = "1: Counter overflow is included in SCIn_ERI interrupt factor"]
    _1 = 1,
}
impl From<COFIE_A> for bool {
    #[inline(always)]
    fn from(variant: COFIE_A) -> Self {
        variant as u8 != 0
    }
}
impl COFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COFIE_A {
        match self.bits {
            false => COFIE_A::_0,
            true => COFIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == COFIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == COFIE_A::_1
    }
}
#[doc = "Field `COFIE` writer - Counter overflow interrupt enable"]
pub type COFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, XCR0_SPEC, COFIE_A, O>;
impl<'a, const O: u8> COFIE_W<'a, O> {
    #[doc = "Counter overflow is not included in SCIn_ERI interrupt factor"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(COFIE_A::_0)
    }
    #[doc = "Counter overflow is included in SCIn_ERI interrupt factor"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(COFIE_A::_1)
    }
}
#[doc = "Field `AEDIE` reader - Active edge detection interrupt enable"]
pub type AEDIE_R = crate::BitReader<AEDIE_A>;
#[doc = "Active edge detection interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AEDIE_A {
    #[doc = "0: Active edge detection interrupt disable"]
    _0 = 0,
    #[doc = "1: Active edge detection interrupt enable"]
    _1 = 1,
}
impl From<AEDIE_A> for bool {
    #[inline(always)]
    fn from(variant: AEDIE_A) -> Self {
        variant as u8 != 0
    }
}
impl AEDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AEDIE_A {
        match self.bits {
            false => AEDIE_A::_0,
            true => AEDIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AEDIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AEDIE_A::_1
    }
}
#[doc = "Field `AEDIE` writer - Active edge detection interrupt enable"]
pub type AEDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, XCR0_SPEC, AEDIE_A, O>;
impl<'a, const O: u8> AEDIE_W<'a, O> {
    #[doc = "Active edge detection interrupt disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AEDIE_A::_0)
    }
    #[doc = "Active edge detection interrupt enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AEDIE_A::_1)
    }
}
#[doc = "Field `BCCS` reader - Bus conflict detection clock selection"]
pub type BCCS_R = crate::FieldReader<u8, BCCS_A>;
#[doc = "Bus conflict detection clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BCCS_A {
    #[doc = "0: Base clock"]
    _00 = 0,
    #[doc = "1: Base clock/2"]
    _01 = 1,
    #[doc = "2: Base clock/4"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<BCCS_A> for u8 {
    #[inline(always)]
    fn from(variant: BCCS_A) -> Self {
        variant as _
    }
}
impl BCCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCCS_A {
        match self.bits {
            0 => BCCS_A::_00,
            1 => BCCS_A::_01,
            2 => BCCS_A::_10,
            3 => BCCS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == BCCS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == BCCS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == BCCS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == BCCS_A::_11
    }
}
#[doc = "Field `BCCS` writer - Bus conflict detection clock selection"]
pub type BCCS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, XCR0_SPEC, u8, BCCS_A, 2, O>;
impl<'a, const O: u8> BCCS_W<'a, O> {
    #[doc = "Base clock"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(BCCS_A::_00)
    }
    #[doc = "Base clock/2"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(BCCS_A::_01)
    }
    #[doc = "Base clock/4"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(BCCS_A::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(BCCS_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - Timer count clock source selection"]
    #[inline(always)]
    pub fn tcss(&self) -> TCSS_R {
        TCSS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - Break Field enable"]
    #[inline(always)]
    pub fn bfe(&self) -> BFE_R {
        BFE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Control Field 0 enable"]
    #[inline(always)]
    pub fn cf0re(&self) -> CF0RE_R {
        CF0RE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Control Field1 compare data select"]
    #[inline(always)]
    pub fn cf1ds(&self) -> CF1DS_R {
        CF1DS_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Priority interrupt bit enable"]
    #[inline(always)]
    pub fn pibe(&self) -> PIBE_R {
        PIBE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Priority interrupt bit select"]
    #[inline(always)]
    pub fn pibs(&self) -> PIBS_R {
        PIBS_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16 - Break Field output completion interrupt enable"]
    #[inline(always)]
    pub fn bfoie(&self) -> BFOIE_R {
        BFOIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Bus conflict detection interrupt enable"]
    #[inline(always)]
    pub fn bcdie(&self) -> BCDIE_R {
        BCDIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - Break Field detection interrupt enable"]
    #[inline(always)]
    pub fn bfdie(&self) -> BFDIE_R {
        BFDIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Counter overflow interrupt enable"]
    #[inline(always)]
    pub fn cofie(&self) -> COFIE_R {
        COFIE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Active edge detection interrupt enable"]
    #[inline(always)]
    pub fn aedie(&self) -> AEDIE_R {
        AEDIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Bus conflict detection clock selection"]
    #[inline(always)]
    pub fn bccs(&self) -> BCCS_R {
        BCCS_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timer count clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn tcss(&mut self) -> TCSS_W<0> {
        TCSS_W::new(self)
    }
    #[doc = "Bit 8 - Break Field enable"]
    #[inline(always)]
    #[must_use]
    pub fn bfe(&mut self) -> BFE_W<8> {
        BFE_W::new(self)
    }
    #[doc = "Bit 9 - Control Field 0 enable"]
    #[inline(always)]
    #[must_use]
    pub fn cf0re(&mut self) -> CF0RE_W<9> {
        CF0RE_W::new(self)
    }
    #[doc = "Bits 10:11 - Control Field1 compare data select"]
    #[inline(always)]
    #[must_use]
    pub fn cf1ds(&mut self) -> CF1DS_W<10> {
        CF1DS_W::new(self)
    }
    #[doc = "Bit 12 - Priority interrupt bit enable"]
    #[inline(always)]
    #[must_use]
    pub fn pibe(&mut self) -> PIBE_W<12> {
        PIBE_W::new(self)
    }
    #[doc = "Bits 13:15 - Priority interrupt bit select"]
    #[inline(always)]
    #[must_use]
    pub fn pibs(&mut self) -> PIBS_W<13> {
        PIBS_W::new(self)
    }
    #[doc = "Bit 16 - Break Field output completion interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn bfoie(&mut self) -> BFOIE_W<16> {
        BFOIE_W::new(self)
    }
    #[doc = "Bit 17 - Bus conflict detection interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn bcdie(&mut self) -> BCDIE_W<17> {
        BCDIE_W::new(self)
    }
    #[doc = "Bit 20 - Break Field detection interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn bfdie(&mut self) -> BFDIE_W<20> {
        BFDIE_W::new(self)
    }
    #[doc = "Bit 21 - Counter overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cofie(&mut self) -> COFIE_W<21> {
        COFIE_W::new(self)
    }
    #[doc = "Bit 22 - Active edge detection interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn aedie(&mut self) -> AEDIE_W<22> {
        AEDIE_W::new(self)
    }
    #[doc = "Bits 24:25 - Bus conflict detection clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn bccs(&mut self) -> BCCS_W<24> {
        BCCS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Simple LIN Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xcr0](index.html) module"]
pub struct XCR0_SPEC;
impl crate::RegisterSpec for XCR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xcr0::R](R) reader structure"]
impl crate::Readable for XCR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xcr0::W](W) writer structure"]
impl crate::Writable for XCR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XCR0 to value 0"]
impl crate::Resettable for XCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
