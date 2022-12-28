#[doc = "Register `GTBER2` reader"]
pub struct R(crate::R<GTBER2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTBER2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTBER2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTBER2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTBER2` writer"]
pub struct W(crate::W<GTBER2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTBER2_SPEC>;
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
impl From<crate::W<GTBER2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTBER2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCTCA` reader - Counter Clear Source GTCCRA Register Buffer Transfer Disable"]
pub type CCTCA_R = crate::BitReader<CCTCA_A>;
#[doc = "Counter Clear Source GTCCRA Register Buffer Transfer Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCTCA_A {
    #[doc = "0: Enable GTCCRA register buffer transfer by counter clear"]
    _0 = 0,
    #[doc = "1: Disable GTCCRA register buffer transfer by counter clear"]
    _1 = 1,
}
impl From<CCTCA_A> for bool {
    #[inline(always)]
    fn from(variant: CCTCA_A) -> Self {
        variant as u8 != 0
    }
}
impl CCTCA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCTCA_A {
        match self.bits {
            false => CCTCA_A::_0,
            true => CCTCA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CCTCA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CCTCA_A::_1
    }
}
#[doc = "Field `CCTCA` writer - Counter Clear Source GTCCRA Register Buffer Transfer Disable"]
pub type CCTCA_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTBER2_SPEC, CCTCA_A, O>;
impl<'a, const O: u8> CCTCA_W<'a, O> {
    #[doc = "Enable GTCCRA register buffer transfer by counter clear"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCTCA_A::_0)
    }
    #[doc = "Disable GTCCRA register buffer transfer by counter clear"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCTCA_A::_1)
    }
}
#[doc = "Field `CCTCB` reader - Counter Clear Source GTCCRB Register Buffer Transfer Disable"]
pub type CCTCB_R = crate::BitReader<CCTCB_A>;
#[doc = "Counter Clear Source GTCCRB Register Buffer Transfer Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCTCB_A {
    #[doc = "0: Enable GTCCRB register buffer transfer by counter clear"]
    _0 = 0,
    #[doc = "1: Disable GTCCRB register buffer transfer by counter clear"]
    _1 = 1,
}
impl From<CCTCB_A> for bool {
    #[inline(always)]
    fn from(variant: CCTCB_A) -> Self {
        variant as u8 != 0
    }
}
impl CCTCB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCTCB_A {
        match self.bits {
            false => CCTCB_A::_0,
            true => CCTCB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CCTCB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CCTCB_A::_1
    }
}
#[doc = "Field `CCTCB` writer - Counter Clear Source GTCCRB Register Buffer Transfer Disable"]
pub type CCTCB_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTBER2_SPEC, CCTCB_A, O>;
impl<'a, const O: u8> CCTCB_W<'a, O> {
    #[doc = "Enable GTCCRB register buffer transfer by counter clear"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCTCB_A::_0)
    }
    #[doc = "Disable GTCCRB register buffer transfer by counter clear"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCTCB_A::_1)
    }
}
#[doc = "Field `CCTPR` reader - Counter Clear Source GTPR Register Buffer Transfer Disable"]
pub type CCTPR_R = crate::BitReader<CCTPR_A>;
#[doc = "Counter Clear Source GTPR Register Buffer Transfer Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCTPR_A {
    #[doc = "0: Enable GTPR register buffer transfer by counter clear"]
    _0 = 0,
    #[doc = "1: Disable GTPR register buffer transfer by counter clear"]
    _1 = 1,
}
impl From<CCTPR_A> for bool {
    #[inline(always)]
    fn from(variant: CCTPR_A) -> Self {
        variant as u8 != 0
    }
}
impl CCTPR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCTPR_A {
        match self.bits {
            false => CCTPR_A::_0,
            true => CCTPR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CCTPR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CCTPR_A::_1
    }
}
#[doc = "Field `CCTPR` writer - Counter Clear Source GTPR Register Buffer Transfer Disable"]
pub type CCTPR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTBER2_SPEC, CCTPR_A, O>;
impl<'a, const O: u8> CCTPR_W<'a, O> {
    #[doc = "Enable GTPR register buffer transfer by counter clear"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCTPR_A::_0)
    }
    #[doc = "Disable GTPR register buffer transfer by counter clear"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCTPR_A::_1)
    }
}
#[doc = "Field `CCTADA` reader - Counter Clear Source GTADTRA Register Buffer Transfer Disable"]
pub type CCTADA_R = crate::BitReader<CCTADA_A>;
#[doc = "Counter Clear Source GTADTRA Register Buffer Transfer Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCTADA_A {
    #[doc = "0: Enable GTADTRA register buffer transfer by counter clear"]
    _0 = 0,
    #[doc = "1: Disable GTADTRA register buffer transfer by counter clear"]
    _1 = 1,
}
impl From<CCTADA_A> for bool {
    #[inline(always)]
    fn from(variant: CCTADA_A) -> Self {
        variant as u8 != 0
    }
}
impl CCTADA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCTADA_A {
        match self.bits {
            false => CCTADA_A::_0,
            true => CCTADA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CCTADA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CCTADA_A::_1
    }
}
#[doc = "Field `CCTADA` writer - Counter Clear Source GTADTRA Register Buffer Transfer Disable"]
pub type CCTADA_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTBER2_SPEC, CCTADA_A, O>;
impl<'a, const O: u8> CCTADA_W<'a, O> {
    #[doc = "Enable GTADTRA register buffer transfer by counter clear"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCTADA_A::_0)
    }
    #[doc = "Disable GTADTRA register buffer transfer by counter clear"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCTADA_A::_1)
    }
}
#[doc = "Field `CCTADB` reader - Counter Clear Source GTADTRB Register Buffer Transfer Disable"]
pub type CCTADB_R = crate::BitReader<CCTADB_A>;
#[doc = "Counter Clear Source GTADTRB Register Buffer Transfer Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCTADB_A {
    #[doc = "0: Enable GTADTRB register buffer transfer by counter clear"]
    _0 = 0,
    #[doc = "1: Disable GTADTRB register buffer transfer by counter clear"]
    _1 = 1,
}
impl From<CCTADB_A> for bool {
    #[inline(always)]
    fn from(variant: CCTADB_A) -> Self {
        variant as u8 != 0
    }
}
impl CCTADB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCTADB_A {
        match self.bits {
            false => CCTADB_A::_0,
            true => CCTADB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CCTADB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CCTADB_A::_1
    }
}
#[doc = "Field `CCTADB` writer - Counter Clear Source GTADTRB Register Buffer Transfer Disable"]
pub type CCTADB_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTBER2_SPEC, CCTADB_A, O>;
impl<'a, const O: u8> CCTADB_W<'a, O> {
    #[doc = "Enable GTADTRB register buffer transfer by counter clear"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCTADB_A::_0)
    }
    #[doc = "Disable GTADTRB register buffer transfer by counter clear"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCTADB_A::_1)
    }
}
#[doc = "Field `CCTDV` reader - Counter Clear Source GTDVU/GTDVD Register Buffer Transfer Disable"]
pub type CCTDV_R = crate::BitReader<CCTDV_A>;
#[doc = "Counter Clear Source GTDVU/GTDVD Register Buffer Transfer Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCTDV_A {
    #[doc = "0: Enable GTDVU/GTDVD register buffer transfer by counter clear"]
    _0 = 0,
    #[doc = "1: Disable GTDVU/GTDVD register buffer transfer by counter clear"]
    _1 = 1,
}
impl From<CCTDV_A> for bool {
    #[inline(always)]
    fn from(variant: CCTDV_A) -> Self {
        variant as u8 != 0
    }
}
impl CCTDV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCTDV_A {
        match self.bits {
            false => CCTDV_A::_0,
            true => CCTDV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CCTDV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CCTDV_A::_1
    }
}
#[doc = "Field `CCTDV` writer - Counter Clear Source GTDVU/GTDVD Register Buffer Transfer Disable"]
pub type CCTDV_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTBER2_SPEC, CCTDV_A, O>;
impl<'a, const O: u8> CCTDV_W<'a, O> {
    #[doc = "Enable GTDVU/GTDVD register buffer transfer by counter clear"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCTDV_A::_0)
    }
    #[doc = "Disable GTDVU/GTDVD register buffer transfer by counter clear"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCTDV_A::_1)
    }
}
#[doc = "Field `CMTCA` reader - Compare Match Source GTCCRA Register Buffer Transfer Enable"]
pub type CMTCA_R = crate::FieldReader<u8, CMTCA_A>;
#[doc = "Compare Match Source GTCCRA Register Buffer Transfer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMTCA_A {
    #[doc = "0: Disable GTCCRA register Buffer Transfer by compare match of GTCCRA register and GTCCRB register"]
    _00 = 0,
    #[doc = "1: Enable GTCCRA register Buffer Transfer by compare match of GTCCRA register"]
    _01 = 1,
    #[doc = "2: Enable GTCCRA register Buffer Transfer by compare match of GTCCRB register"]
    _10 = 2,
    #[doc = "3: Enable GTCCRA register Buffer Transfer by compare match of GTCCRA register and GTCCRB register"]
    _11 = 3,
}
impl From<CMTCA_A> for u8 {
    #[inline(always)]
    fn from(variant: CMTCA_A) -> Self {
        variant as _
    }
}
impl CMTCA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMTCA_A {
        match self.bits {
            0 => CMTCA_A::_00,
            1 => CMTCA_A::_01,
            2 => CMTCA_A::_10,
            3 => CMTCA_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CMTCA_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CMTCA_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CMTCA_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CMTCA_A::_11
    }
}
#[doc = "Field `CMTCA` writer - Compare Match Source GTCCRA Register Buffer Transfer Enable"]
pub type CMTCA_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GTBER2_SPEC, u8, CMTCA_A, 2, O>;
impl<'a, const O: u8> CMTCA_W<'a, O> {
    #[doc = "Disable GTCCRA register Buffer Transfer by compare match of GTCCRA register and GTCCRB register"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CMTCA_A::_00)
    }
    #[doc = "Enable GTCCRA register Buffer Transfer by compare match of GTCCRA register"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CMTCA_A::_01)
    }
    #[doc = "Enable GTCCRA register Buffer Transfer by compare match of GTCCRB register"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CMTCA_A::_10)
    }
    #[doc = "Enable GTCCRA register Buffer Transfer by compare match of GTCCRA register and GTCCRB register"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CMTCA_A::_11)
    }
}
#[doc = "Field `CMTCB` reader - Compare Match Source GTCCRB Register Buffer Transfer Enable"]
pub type CMTCB_R = crate::FieldReader<u8, CMTCB_A>;
#[doc = "Compare Match Source GTCCRB Register Buffer Transfer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMTCB_A {
    #[doc = "0: Disable GTCCRB register Buffer Transfer by compare match of GTCCRA register and GTCCRB register"]
    _00 = 0,
    #[doc = "1: Enable GTCCRB register Buffer Transfer by compare match of GTCCRA register"]
    _01 = 1,
    #[doc = "2: Enable GTCCRB register Buffer Transfer by compare match of GTCCRB register"]
    _10 = 2,
    #[doc = "3: Enable GTCCRB register Buffer Transfer by compare match of GTCCRA register and GTCCRB register"]
    _11 = 3,
}
impl From<CMTCB_A> for u8 {
    #[inline(always)]
    fn from(variant: CMTCB_A) -> Self {
        variant as _
    }
}
impl CMTCB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMTCB_A {
        match self.bits {
            0 => CMTCB_A::_00,
            1 => CMTCB_A::_01,
            2 => CMTCB_A::_10,
            3 => CMTCB_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CMTCB_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CMTCB_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CMTCB_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CMTCB_A::_11
    }
}
#[doc = "Field `CMTCB` writer - Compare Match Source GTCCRB Register Buffer Transfer Enable"]
pub type CMTCB_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GTBER2_SPEC, u8, CMTCB_A, 2, O>;
impl<'a, const O: u8> CMTCB_W<'a, O> {
    #[doc = "Disable GTCCRB register Buffer Transfer by compare match of GTCCRA register and GTCCRB register"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CMTCB_A::_00)
    }
    #[doc = "Enable GTCCRB register Buffer Transfer by compare match of GTCCRA register"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CMTCB_A::_01)
    }
    #[doc = "Enable GTCCRB register Buffer Transfer by compare match of GTCCRB register"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CMTCB_A::_10)
    }
    #[doc = "Enable GTCCRB register Buffer Transfer by compare match of GTCCRA register and GTCCRB register"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CMTCB_A::_11)
    }
}
#[doc = "Field `CMTADA` reader - Compare Match Source GTADTRA Register Buffer Transfer Enable"]
pub type CMTADA_R = crate::BitReader<CMTADA_A>;
#[doc = "Compare Match Source GTADTRA Register Buffer Transfer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMTADA_A {
    #[doc = "0: Disable GTADTRA register buffer transfer by compare match of GTADTRA register"]
    _0 = 0,
    #[doc = "1: Enable GTADTRA register buffer transfer by compare match of GTADTRA register"]
    _1 = 1,
}
impl From<CMTADA_A> for bool {
    #[inline(always)]
    fn from(variant: CMTADA_A) -> Self {
        variant as u8 != 0
    }
}
impl CMTADA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMTADA_A {
        match self.bits {
            false => CMTADA_A::_0,
            true => CMTADA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMTADA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMTADA_A::_1
    }
}
#[doc = "Field `CMTADA` writer - Compare Match Source GTADTRA Register Buffer Transfer Enable"]
pub type CMTADA_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTBER2_SPEC, CMTADA_A, O>;
impl<'a, const O: u8> CMTADA_W<'a, O> {
    #[doc = "Disable GTADTRA register buffer transfer by compare match of GTADTRA register"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMTADA_A::_0)
    }
    #[doc = "Enable GTADTRA register buffer transfer by compare match of GTADTRA register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMTADA_A::_1)
    }
}
#[doc = "Field `CMTADB` reader - Compare Match Source GTADTRB Register Buffer Transfer Enable"]
pub type CMTADB_R = crate::BitReader<CMTADB_A>;
#[doc = "Compare Match Source GTADTRB Register Buffer Transfer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMTADB_A {
    #[doc = "0: Disable GTADTRB register buffer transfer by compare match of GTADTRB register"]
    _0 = 0,
    #[doc = "1: Enable GTADTRB register buffer transfer by compare match of GTADTRB register"]
    _1 = 1,
}
impl From<CMTADB_A> for bool {
    #[inline(always)]
    fn from(variant: CMTADB_A) -> Self {
        variant as u8 != 0
    }
}
impl CMTADB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMTADB_A {
        match self.bits {
            false => CMTADB_A::_0,
            true => CMTADB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMTADB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMTADB_A::_1
    }
}
#[doc = "Field `CMTADB` writer - Compare Match Source GTADTRB Register Buffer Transfer Enable"]
pub type CMTADB_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTBER2_SPEC, CMTADB_A, O>;
impl<'a, const O: u8> CMTADB_W<'a, O> {
    #[doc = "Disable GTADTRB register buffer transfer by compare match of GTADTRB register"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMTADB_A::_0)
    }
    #[doc = "Enable GTADTRB register buffer transfer by compare match of GTADTRB register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMTADB_A::_1)
    }
}
#[doc = "Field `CPTCA` reader - Overflow/Underflow Source GTCCRA Register Buffer Transfer Disable"]
pub type CPTCA_R = crate::BitReader<CPTCA_A>;
#[doc = "Overflow/Underflow Source GTCCRA Register Buffer Transfer Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPTCA_A {
    #[doc = "0: Enable GTCCRA register buffer transfer by overflow/underflow"]
    _0 = 0,
    #[doc = "1: Disable GTCCRA register buffer transfer by overflow/underflow"]
    _1 = 1,
}
impl From<CPTCA_A> for bool {
    #[inline(always)]
    fn from(variant: CPTCA_A) -> Self {
        variant as u8 != 0
    }
}
impl CPTCA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPTCA_A {
        match self.bits {
            false => CPTCA_A::_0,
            true => CPTCA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPTCA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPTCA_A::_1
    }
}
#[doc = "Field `CPTCA` writer - Overflow/Underflow Source GTCCRA Register Buffer Transfer Disable"]
pub type CPTCA_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTBER2_SPEC, CPTCA_A, O>;
impl<'a, const O: u8> CPTCA_W<'a, O> {
    #[doc = "Enable GTCCRA register buffer transfer by overflow/underflow"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPTCA_A::_0)
    }
    #[doc = "Disable GTCCRA register buffer transfer by overflow/underflow"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPTCA_A::_1)
    }
}
#[doc = "Field `CPTCB` reader - Overflow/Underflow Source GTCCRB Register Buffer Transfer Disable"]
pub type CPTCB_R = crate::BitReader<CPTCB_A>;
#[doc = "Overflow/Underflow Source GTCCRB Register Buffer Transfer Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPTCB_A {
    #[doc = "0: Enable GTCCRB register buffer transfer by overflow/underflow"]
    _0 = 0,
    #[doc = "1: Disable GTCCRB register buffer transfer by overflow/underflow"]
    _1 = 1,
}
impl From<CPTCB_A> for bool {
    #[inline(always)]
    fn from(variant: CPTCB_A) -> Self {
        variant as u8 != 0
    }
}
impl CPTCB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPTCB_A {
        match self.bits {
            false => CPTCB_A::_0,
            true => CPTCB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPTCB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPTCB_A::_1
    }
}
#[doc = "Field `CPTCB` writer - Overflow/Underflow Source GTCCRB Register Buffer Transfer Disable"]
pub type CPTCB_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTBER2_SPEC, CPTCB_A, O>;
impl<'a, const O: u8> CPTCB_W<'a, O> {
    #[doc = "Enable GTCCRB register buffer transfer by overflow/underflow"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPTCB_A::_0)
    }
    #[doc = "Disable GTCCRB register buffer transfer by overflow/underflow"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPTCB_A::_1)
    }
}
#[doc = "Field `CPTPR` reader - Overflow/Underflow Source GTPR Register Buffer Transfer Disable"]
pub type CPTPR_R = crate::BitReader<CPTPR_A>;
#[doc = "Overflow/Underflow Source GTPR Register Buffer Transfer Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPTPR_A {
    #[doc = "0: Enable GTPR register buffer transfer by overflow/underflow"]
    _0 = 0,
    #[doc = "1: Disable GTPR register buffer transfer by overflow/underflow"]
    _1 = 1,
}
impl From<CPTPR_A> for bool {
    #[inline(always)]
    fn from(variant: CPTPR_A) -> Self {
        variant as u8 != 0
    }
}
impl CPTPR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPTPR_A {
        match self.bits {
            false => CPTPR_A::_0,
            true => CPTPR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPTPR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPTPR_A::_1
    }
}
#[doc = "Field `CPTPR` writer - Overflow/Underflow Source GTPR Register Buffer Transfer Disable"]
pub type CPTPR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTBER2_SPEC, CPTPR_A, O>;
impl<'a, const O: u8> CPTPR_W<'a, O> {
    #[doc = "Enable GTPR register buffer transfer by overflow/underflow"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPTPR_A::_0)
    }
    #[doc = "Disable GTPR register buffer transfer by overflow/underflow"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPTPR_A::_1)
    }
}
#[doc = "Field `CPTADA` reader - Overflow/Underflow Source GTADTRA Register Buffer Transfer Disable"]
pub type CPTADA_R = crate::BitReader<CPTADA_A>;
#[doc = "Overflow/Underflow Source GTADTRA Register Buffer Transfer Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPTADA_A {
    #[doc = "0: Enable GTADTRA register buffer transfer by overflow/underflow"]
    _0 = 0,
    #[doc = "1: Disable GTADTRA register buffer transfer by overflow/underflow"]
    _1 = 1,
}
impl From<CPTADA_A> for bool {
    #[inline(always)]
    fn from(variant: CPTADA_A) -> Self {
        variant as u8 != 0
    }
}
impl CPTADA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPTADA_A {
        match self.bits {
            false => CPTADA_A::_0,
            true => CPTADA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPTADA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPTADA_A::_1
    }
}
#[doc = "Field `CPTADA` writer - Overflow/Underflow Source GTADTRA Register Buffer Transfer Disable"]
pub type CPTADA_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTBER2_SPEC, CPTADA_A, O>;
impl<'a, const O: u8> CPTADA_W<'a, O> {
    #[doc = "Enable GTADTRA register buffer transfer by overflow/underflow"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPTADA_A::_0)
    }
    #[doc = "Disable GTADTRA register buffer transfer by overflow/underflow"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPTADA_A::_1)
    }
}
#[doc = "Field `CPTADB` reader - Overflow/Underflow Source GTADTRB Register Buffer Transfer Disable"]
pub type CPTADB_R = crate::BitReader<CPTADB_A>;
#[doc = "Overflow/Underflow Source GTADTRB Register Buffer Transfer Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPTADB_A {
    #[doc = "0: Enable GTADTRB register buffer transfer by overflow/underflow"]
    _0 = 0,
    #[doc = "1: Disable GTADTRB register buffer transfer by overflow/underflow"]
    _1 = 1,
}
impl From<CPTADB_A> for bool {
    #[inline(always)]
    fn from(variant: CPTADB_A) -> Self {
        variant as u8 != 0
    }
}
impl CPTADB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPTADB_A {
        match self.bits {
            false => CPTADB_A::_0,
            true => CPTADB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPTADB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPTADB_A::_1
    }
}
#[doc = "Field `CPTADB` writer - Overflow/Underflow Source GTADTRB Register Buffer Transfer Disable"]
pub type CPTADB_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTBER2_SPEC, CPTADB_A, O>;
impl<'a, const O: u8> CPTADB_W<'a, O> {
    #[doc = "Enable GTADTRB register buffer transfer by overflow/underflow"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPTADB_A::_0)
    }
    #[doc = "Disable GTADTRB register buffer transfer by overflow/underflow"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPTADB_A::_1)
    }
}
#[doc = "Field `CPTDV` reader - Overflow/Underflow Source GTDVU/GTDVD Register Buffer Transfer Disable"]
pub type CPTDV_R = crate::BitReader<CPTDV_A>;
#[doc = "Overflow/Underflow Source GTDVU/GTDVD Register Buffer Transfer Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPTDV_A {
    #[doc = "0: Enable GTDVU/GTDVD register buffer transfer by overflow/underflow"]
    _0 = 0,
    #[doc = "1: Disable GTDVU/GTDVD register buffer transfer by overflow/underflow"]
    _1 = 1,
}
impl From<CPTDV_A> for bool {
    #[inline(always)]
    fn from(variant: CPTDV_A) -> Self {
        variant as u8 != 0
    }
}
impl CPTDV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPTDV_A {
        match self.bits {
            false => CPTDV_A::_0,
            true => CPTDV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPTDV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPTDV_A::_1
    }
}
#[doc = "Field `CPTDV` writer - Overflow/Underflow Source GTDVU/GTDVD Register Buffer Transfer Disable"]
pub type CPTDV_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTBER2_SPEC, CPTDV_A, O>;
impl<'a, const O: u8> CPTDV_W<'a, O> {
    #[doc = "Enable GTDVU/GTDVD register buffer transfer by overflow/underflow"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPTDV_A::_0)
    }
    #[doc = "Disable GTDVU/GTDVD register buffer transfer by overflow/underflow"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPTDV_A::_1)
    }
}
#[doc = "Field `CP3DB` reader - Complementary PWM mode 3,4 Double Buffer select"]
pub type CP3DB_R = crate::BitReader<CP3DB_A>;
#[doc = "Complementary PWM mode 3,4 Double Buffer select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CP3DB_A {
    #[doc = "0: Disable double buffer function in complementary PWM mode 3, 4"]
    _0 = 0,
    #[doc = "1: Enable double buffer function in complementary PWM mode 3, 4"]
    _1 = 1,
}
impl From<CP3DB_A> for bool {
    #[inline(always)]
    fn from(variant: CP3DB_A) -> Self {
        variant as u8 != 0
    }
}
impl CP3DB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CP3DB_A {
        match self.bits {
            false => CP3DB_A::_0,
            true => CP3DB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CP3DB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CP3DB_A::_1
    }
}
#[doc = "Field `CP3DB` writer - Complementary PWM mode 3,4 Double Buffer select"]
pub type CP3DB_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTBER2_SPEC, CP3DB_A, O>;
impl<'a, const O: u8> CP3DB_W<'a, O> {
    #[doc = "Disable double buffer function in complementary PWM mode 3, 4"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CP3DB_A::_0)
    }
    #[doc = "Enable double buffer function in complementary PWM mode 3, 4"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CP3DB_A::_1)
    }
}
#[doc = "Field `CPBTD` reader - Complementary PWM mode Buffer Transfer Disable"]
pub type CPBTD_R = crate::BitReader<CPBTD_A>;
#[doc = "Complementary PWM mode Buffer Transfer Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPBTD_A {
    #[doc = "0: Enable buffer transfer from temporary register to GTCCRC and GTPBR register"]
    _0 = 0,
    #[doc = "1: Disable buffer transfer from temporary register to GTCCRC and GTPBR register"]
    _1 = 1,
}
impl From<CPBTD_A> for bool {
    #[inline(always)]
    fn from(variant: CPBTD_A) -> Self {
        variant as u8 != 0
    }
}
impl CPBTD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPBTD_A {
        match self.bits {
            false => CPBTD_A::_0,
            true => CPBTD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPBTD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPBTD_A::_1
    }
}
#[doc = "Field `CPBTD` writer - Complementary PWM mode Buffer Transfer Disable"]
pub type CPBTD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTBER2_SPEC, CPBTD_A, O>;
impl<'a, const O: u8> CPBTD_W<'a, O> {
    #[doc = "Enable buffer transfer from temporary register to GTCCRC and GTPBR register"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPBTD_A::_0)
    }
    #[doc = "Disable buffer transfer from temporary register to GTCCRC and GTPBR register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPBTD_A::_1)
    }
}
#[doc = "Field `OLTTA` reader - GTIOCnA Output Level Buffer Transfer Timing Select"]
pub type OLTTA_R = crate::FieldReader<u8, OLTTA_A>;
#[doc = "GTIOCnA Output Level Buffer Transfer Timing Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OLTTA_A {
    #[doc = "0: No transfer"]
    _00 = 0,
    #[doc = "1: Triangle waves, complementary PWM mode: Transfer at crest Saw waves: Transfer at the end of period"]
    _01 = 1,
    #[doc = "2: Triangle waves, complementary PWM mode: Transfer at trough Saw waves: Transfer by compare match of GTCCRA register"]
    _10 = 2,
    #[doc = "3: Triangle waves, complementary PWM mode: Transfer at both crest and trough Saw waves: Setting prohibited"]
    _11 = 3,
}
impl From<OLTTA_A> for u8 {
    #[inline(always)]
    fn from(variant: OLTTA_A) -> Self {
        variant as _
    }
}
impl OLTTA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OLTTA_A {
        match self.bits {
            0 => OLTTA_A::_00,
            1 => OLTTA_A::_01,
            2 => OLTTA_A::_10,
            3 => OLTTA_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == OLTTA_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == OLTTA_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == OLTTA_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == OLTTA_A::_11
    }
}
#[doc = "Field `OLTTA` writer - GTIOCnA Output Level Buffer Transfer Timing Select"]
pub type OLTTA_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GTBER2_SPEC, u8, OLTTA_A, 2, O>;
impl<'a, const O: u8> OLTTA_W<'a, O> {
    #[doc = "No transfer"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(OLTTA_A::_00)
    }
    #[doc = "Triangle waves, complementary PWM mode: Transfer at crest Saw waves: Transfer at the end of period"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(OLTTA_A::_01)
    }
    #[doc = "Triangle waves, complementary PWM mode: Transfer at trough Saw waves: Transfer by compare match of GTCCRA register"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(OLTTA_A::_10)
    }
    #[doc = "Triangle waves, complementary PWM mode: Transfer at both crest and trough Saw waves: Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(OLTTA_A::_11)
    }
}
#[doc = "Field `OLTTB` reader - GTIOCnB Output Level Buffer Transfer Timing Select"]
pub type OLTTB_R = crate::FieldReader<u8, OLTTB_A>;
#[doc = "GTIOCnB Output Level Buffer Transfer Timing Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OLTTB_A {
    #[doc = "0: No transfer"]
    _00 = 0,
    #[doc = "1: Triangle waves, complementary PWM mode: Transfer at crest Saw waves: Transfer at the end of period"]
    _01 = 1,
    #[doc = "2: Triangle waves, complementary PWM mode: Transfer at trough Saw waves: Transfer by compare match of GTCCRB register"]
    _10 = 2,
    #[doc = "3: Triangle waves, complementary PWM mode: Transfer at both crest and trough Saw waves: Setting prohibited"]
    _11 = 3,
}
impl From<OLTTB_A> for u8 {
    #[inline(always)]
    fn from(variant: OLTTB_A) -> Self {
        variant as _
    }
}
impl OLTTB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OLTTB_A {
        match self.bits {
            0 => OLTTB_A::_00,
            1 => OLTTB_A::_01,
            2 => OLTTB_A::_10,
            3 => OLTTB_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == OLTTB_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == OLTTB_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == OLTTB_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == OLTTB_A::_11
    }
}
#[doc = "Field `OLTTB` writer - GTIOCnB Output Level Buffer Transfer Timing Select"]
pub type OLTTB_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GTBER2_SPEC, u8, OLTTB_A, 2, O>;
impl<'a, const O: u8> OLTTB_W<'a, O> {
    #[doc = "No transfer"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(OLTTB_A::_00)
    }
    #[doc = "Triangle waves, complementary PWM mode: Transfer at crest Saw waves: Transfer at the end of period"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(OLTTB_A::_01)
    }
    #[doc = "Triangle waves, complementary PWM mode: Transfer at trough Saw waves: Transfer by compare match of GTCCRB register"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(OLTTB_A::_10)
    }
    #[doc = "Triangle waves, complementary PWM mode: Transfer at both crest and trough Saw waves: Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(OLTTB_A::_11)
    }
}
impl R {
    #[doc = "Bit 0 - Counter Clear Source GTCCRA Register Buffer Transfer Disable"]
    #[inline(always)]
    pub fn cctca(&self) -> CCTCA_R {
        CCTCA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Counter Clear Source GTCCRB Register Buffer Transfer Disable"]
    #[inline(always)]
    pub fn cctcb(&self) -> CCTCB_R {
        CCTCB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Counter Clear Source GTPR Register Buffer Transfer Disable"]
    #[inline(always)]
    pub fn cctpr(&self) -> CCTPR_R {
        CCTPR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Counter Clear Source GTADTRA Register Buffer Transfer Disable"]
    #[inline(always)]
    pub fn cctada(&self) -> CCTADA_R {
        CCTADA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Counter Clear Source GTADTRB Register Buffer Transfer Disable"]
    #[inline(always)]
    pub fn cctadb(&self) -> CCTADB_R {
        CCTADB_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Counter Clear Source GTDVU/GTDVD Register Buffer Transfer Disable"]
    #[inline(always)]
    pub fn cctdv(&self) -> CCTDV_R {
        CCTDV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Compare Match Source GTCCRA Register Buffer Transfer Enable"]
    #[inline(always)]
    pub fn cmtca(&self) -> CMTCA_R {
        CMTCA_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Compare Match Source GTCCRB Register Buffer Transfer Enable"]
    #[inline(always)]
    pub fn cmtcb(&self) -> CMTCB_R {
        CMTCB_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 13 - Compare Match Source GTADTRA Register Buffer Transfer Enable"]
    #[inline(always)]
    pub fn cmtada(&self) -> CMTADA_R {
        CMTADA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Compare Match Source GTADTRB Register Buffer Transfer Enable"]
    #[inline(always)]
    pub fn cmtadb(&self) -> CMTADB_R {
        CMTADB_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Overflow/Underflow Source GTCCRA Register Buffer Transfer Disable"]
    #[inline(always)]
    pub fn cptca(&self) -> CPTCA_R {
        CPTCA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Overflow/Underflow Source GTCCRB Register Buffer Transfer Disable"]
    #[inline(always)]
    pub fn cptcb(&self) -> CPTCB_R {
        CPTCB_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Overflow/Underflow Source GTPR Register Buffer Transfer Disable"]
    #[inline(always)]
    pub fn cptpr(&self) -> CPTPR_R {
        CPTPR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Overflow/Underflow Source GTADTRA Register Buffer Transfer Disable"]
    #[inline(always)]
    pub fn cptada(&self) -> CPTADA_R {
        CPTADA_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Overflow/Underflow Source GTADTRB Register Buffer Transfer Disable"]
    #[inline(always)]
    pub fn cptadb(&self) -> CPTADB_R {
        CPTADB_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Overflow/Underflow Source GTDVU/GTDVD Register Buffer Transfer Disable"]
    #[inline(always)]
    pub fn cptdv(&self) -> CPTDV_R {
        CPTDV_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - Complementary PWM mode 3,4 Double Buffer select"]
    #[inline(always)]
    pub fn cp3db(&self) -> CP3DB_R {
        CP3DB_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Complementary PWM mode Buffer Transfer Disable"]
    #[inline(always)]
    pub fn cpbtd(&self) -> CPBTD_R {
        CPBTD_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - GTIOCnA Output Level Buffer Transfer Timing Select"]
    #[inline(always)]
    pub fn oltta(&self) -> OLTTA_R {
        OLTTA_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - GTIOCnB Output Level Buffer Transfer Timing Select"]
    #[inline(always)]
    pub fn olttb(&self) -> OLTTB_R {
        OLTTB_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Counter Clear Source GTCCRA Register Buffer Transfer Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cctca(&mut self) -> CCTCA_W<0> {
        CCTCA_W::new(self)
    }
    #[doc = "Bit 1 - Counter Clear Source GTCCRB Register Buffer Transfer Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cctcb(&mut self) -> CCTCB_W<1> {
        CCTCB_W::new(self)
    }
    #[doc = "Bit 2 - Counter Clear Source GTPR Register Buffer Transfer Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cctpr(&mut self) -> CCTPR_W<2> {
        CCTPR_W::new(self)
    }
    #[doc = "Bit 3 - Counter Clear Source GTADTRA Register Buffer Transfer Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cctada(&mut self) -> CCTADA_W<3> {
        CCTADA_W::new(self)
    }
    #[doc = "Bit 4 - Counter Clear Source GTADTRB Register Buffer Transfer Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cctadb(&mut self) -> CCTADB_W<4> {
        CCTADB_W::new(self)
    }
    #[doc = "Bit 5 - Counter Clear Source GTDVU/GTDVD Register Buffer Transfer Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cctdv(&mut self) -> CCTDV_W<5> {
        CCTDV_W::new(self)
    }
    #[doc = "Bits 8:9 - Compare Match Source GTCCRA Register Buffer Transfer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmtca(&mut self) -> CMTCA_W<8> {
        CMTCA_W::new(self)
    }
    #[doc = "Bits 10:11 - Compare Match Source GTCCRB Register Buffer Transfer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmtcb(&mut self) -> CMTCB_W<10> {
        CMTCB_W::new(self)
    }
    #[doc = "Bit 13 - Compare Match Source GTADTRA Register Buffer Transfer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmtada(&mut self) -> CMTADA_W<13> {
        CMTADA_W::new(self)
    }
    #[doc = "Bit 14 - Compare Match Source GTADTRB Register Buffer Transfer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmtadb(&mut self) -> CMTADB_W<14> {
        CMTADB_W::new(self)
    }
    #[doc = "Bit 16 - Overflow/Underflow Source GTCCRA Register Buffer Transfer Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cptca(&mut self) -> CPTCA_W<16> {
        CPTCA_W::new(self)
    }
    #[doc = "Bit 17 - Overflow/Underflow Source GTCCRB Register Buffer Transfer Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cptcb(&mut self) -> CPTCB_W<17> {
        CPTCB_W::new(self)
    }
    #[doc = "Bit 18 - Overflow/Underflow Source GTPR Register Buffer Transfer Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cptpr(&mut self) -> CPTPR_W<18> {
        CPTPR_W::new(self)
    }
    #[doc = "Bit 19 - Overflow/Underflow Source GTADTRA Register Buffer Transfer Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cptada(&mut self) -> CPTADA_W<19> {
        CPTADA_W::new(self)
    }
    #[doc = "Bit 20 - Overflow/Underflow Source GTADTRB Register Buffer Transfer Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cptadb(&mut self) -> CPTADB_W<20> {
        CPTADB_W::new(self)
    }
    #[doc = "Bit 21 - Overflow/Underflow Source GTDVU/GTDVD Register Buffer Transfer Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cptdv(&mut self) -> CPTDV_W<21> {
        CPTDV_W::new(self)
    }
    #[doc = "Bit 24 - Complementary PWM mode 3,4 Double Buffer select"]
    #[inline(always)]
    #[must_use]
    pub fn cp3db(&mut self) -> CP3DB_W<24> {
        CP3DB_W::new(self)
    }
    #[doc = "Bit 25 - Complementary PWM mode Buffer Transfer Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cpbtd(&mut self) -> CPBTD_W<25> {
        CPBTD_W::new(self)
    }
    #[doc = "Bits 26:27 - GTIOCnA Output Level Buffer Transfer Timing Select"]
    #[inline(always)]
    #[must_use]
    pub fn oltta(&mut self) -> OLTTA_W<26> {
        OLTTA_W::new(self)
    }
    #[doc = "Bits 28:29 - GTIOCnB Output Level Buffer Transfer Timing Select"]
    #[inline(always)]
    #[must_use]
    pub fn olttb(&mut self) -> OLTTB_W<28> {
        OLTTB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Buffer Enable Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtber2](index.html) module"]
pub struct GTBER2_SPEC;
impl crate::RegisterSpec for GTBER2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtber2::R](R) reader structure"]
impl crate::Readable for GTBER2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtber2::W](W) writer structure"]
impl crate::Writable for GTBER2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTBER2 to value 0"]
impl crate::Resettable for GTBER2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
