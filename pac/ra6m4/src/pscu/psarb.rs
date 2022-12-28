#[doc = "Register `PSARB` reader"]
pub struct R(crate::R<PSARB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSARB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSARB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSARB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSARB` writer"]
pub struct W(crate::W<PSARB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSARB_SPEC>;
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
impl From<crate::W<PSARB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSARB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSARB1` reader - CAN1 and the MSTPCRB.MSTPB1 bit security attribution"]
pub type PSARB1_R = crate::BitReader<PSARB1_A>;
#[doc = "CAN1 and the MSTPCRB.MSTPB1 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARB1_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARB1_A> for bool {
    #[inline(always)]
    fn from(variant: PSARB1_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARB1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARB1_A {
        match self.bits {
            false => PSARB1_A::_0,
            true => PSARB1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARB1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARB1_A::_1
    }
}
#[doc = "Field `PSARB1` writer - CAN1 and the MSTPCRB.MSTPB1 bit security attribution"]
pub type PSARB1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARB_SPEC, PSARB1_A, O>;
impl<'a, const O: u8> PSARB1_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARB1_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARB1_A::_1)
    }
}
#[doc = "Field `PSARB2` reader - CAN0 and the MSTPCRB.MSTPB2 bit security attribution"]
pub type PSARB2_R = crate::BitReader<PSARB2_A>;
#[doc = "CAN0 and the MSTPCRB.MSTPB2 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARB2_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARB2_A> for bool {
    #[inline(always)]
    fn from(variant: PSARB2_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARB2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARB2_A {
        match self.bits {
            false => PSARB2_A::_0,
            true => PSARB2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARB2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARB2_A::_1
    }
}
#[doc = "Field `PSARB2` writer - CAN0 and the MSTPCRB.MSTPB2 bit security attribution"]
pub type PSARB2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARB_SPEC, PSARB2_A, O>;
impl<'a, const O: u8> PSARB2_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARB2_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARB2_A::_1)
    }
}
#[doc = "Field `PSARB6` reader - QSPI and the MSTPCRB.MSTPB6 bit security attribution"]
pub type PSARB6_R = crate::BitReader<bool>;
#[doc = "Field `PSARB8` reader - IIC1 and the MSTPCRB.MSTPB8 bit security attribution"]
pub type PSARB8_R = crate::BitReader<PSARB8_A>;
#[doc = "IIC1 and the MSTPCRB.MSTPB8 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARB8_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARB8_A> for bool {
    #[inline(always)]
    fn from(variant: PSARB8_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARB8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARB8_A {
        match self.bits {
            false => PSARB8_A::_0,
            true => PSARB8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARB8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARB8_A::_1
    }
}
#[doc = "Field `PSARB8` writer - IIC1 and the MSTPCRB.MSTPB8 bit security attribution"]
pub type PSARB8_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARB_SPEC, PSARB8_A, O>;
impl<'a, const O: u8> PSARB8_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARB8_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARB8_A::_1)
    }
}
#[doc = "Field `PSARB9` reader - IIC0 and the MSTPCRB.MSTPB9 bit security attribution"]
pub type PSARB9_R = crate::BitReader<PSARB9_A>;
#[doc = "IIC0 and the MSTPCRB.MSTPB9 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARB9_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARB9_A> for bool {
    #[inline(always)]
    fn from(variant: PSARB9_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARB9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARB9_A {
        match self.bits {
            false => PSARB9_A::_0,
            true => PSARB9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARB9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARB9_A::_1
    }
}
#[doc = "Field `PSARB9` writer - IIC0 and the MSTPCRB.MSTPB9 bit security attribution"]
pub type PSARB9_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARB_SPEC, PSARB9_A, O>;
impl<'a, const O: u8> PSARB9_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARB9_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARB9_A::_1)
    }
}
#[doc = "Field `PSARB11` reader - USBFS and the MSTPCRB.MSTPB11 bit security attribution"]
pub type PSARB11_R = crate::BitReader<PSARB11_A>;
#[doc = "USBFS and the MSTPCRB.MSTPB11 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARB11_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARB11_A> for bool {
    #[inline(always)]
    fn from(variant: PSARB11_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARB11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARB11_A {
        match self.bits {
            false => PSARB11_A::_0,
            true => PSARB11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARB11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARB11_A::_1
    }
}
#[doc = "Field `PSARB11` writer - USBFS and the MSTPCRB.MSTPB11 bit security attribution"]
pub type PSARB11_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARB_SPEC, PSARB11_A, O>;
impl<'a, const O: u8> PSARB11_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARB11_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARB11_A::_1)
    }
}
#[doc = "Field `PSARB15` reader - ETHER0/EDMAC0, the MSTPCRB.MSTPB15 bit and the PFENET.PHYMODE0 bit security attribution"]
pub type PSARB15_R = crate::BitReader<bool>;
#[doc = "Field `PSARB16` reader - OSPI and the MSTPCRB.MSTPB16 bit security attribution"]
pub type PSARB16_R = crate::BitReader<bool>;
#[doc = "Field `PSARB18` reader - SPI1 and the MSTPCRB.MSTPB18 bit security attribution"]
pub type PSARB18_R = crate::BitReader<PSARB18_A>;
#[doc = "SPI1 and the MSTPCRB.MSTPB18 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARB18_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARB18_A> for bool {
    #[inline(always)]
    fn from(variant: PSARB18_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARB18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARB18_A {
        match self.bits {
            false => PSARB18_A::_0,
            true => PSARB18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARB18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARB18_A::_1
    }
}
#[doc = "Field `PSARB18` writer - SPI1 and the MSTPCRB.MSTPB18 bit security attribution"]
pub type PSARB18_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARB_SPEC, PSARB18_A, O>;
impl<'a, const O: u8> PSARB18_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARB18_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARB18_A::_1)
    }
}
#[doc = "Field `PSARB19` reader - SPI0 and the MSTPCRB.MSTPB19 bit security attribution"]
pub type PSARB19_R = crate::BitReader<PSARB19_A>;
#[doc = "SPI0 and the MSTPCRB.MSTPB19 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARB19_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARB19_A> for bool {
    #[inline(always)]
    fn from(variant: PSARB19_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARB19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARB19_A {
        match self.bits {
            false => PSARB19_A::_0,
            true => PSARB19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARB19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARB19_A::_1
    }
}
#[doc = "Field `PSARB19` writer - SPI0 and the MSTPCRB.MSTPB19 bit security attribution"]
pub type PSARB19_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARB_SPEC, PSARB19_A, O>;
impl<'a, const O: u8> PSARB19_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARB19_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARB19_A::_1)
    }
}
#[doc = "Field `PSARB22` reader - SCI9 and the MSTPCRB.MSTPB22 bit security attribution"]
pub type PSARB22_R = crate::BitReader<PSARB22_A>;
#[doc = "SCI9 and the MSTPCRB.MSTPB22 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARB22_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARB22_A> for bool {
    #[inline(always)]
    fn from(variant: PSARB22_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARB22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARB22_A {
        match self.bits {
            false => PSARB22_A::_0,
            true => PSARB22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARB22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARB22_A::_1
    }
}
#[doc = "Field `PSARB22` writer - SCI9 and the MSTPCRB.MSTPB22 bit security attribution"]
pub type PSARB22_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARB_SPEC, PSARB22_A, O>;
impl<'a, const O: u8> PSARB22_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARB22_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARB22_A::_1)
    }
}
#[doc = "Field `PSARB23` reader - SCI8 and the MSTPCRB.MSTPB23 bit security attribution"]
pub type PSARB23_R = crate::BitReader<PSARB23_A>;
#[doc = "SCI8 and the MSTPCRB.MSTPB23 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARB23_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARB23_A> for bool {
    #[inline(always)]
    fn from(variant: PSARB23_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARB23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARB23_A {
        match self.bits {
            false => PSARB23_A::_0,
            true => PSARB23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARB23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARB23_A::_1
    }
}
#[doc = "Field `PSARB23` writer - SCI8 and the MSTPCRB.MSTPB23 bit security attribution"]
pub type PSARB23_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARB_SPEC, PSARB23_A, O>;
impl<'a, const O: u8> PSARB23_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARB23_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARB23_A::_1)
    }
}
#[doc = "Field `PSARB24` reader - SCI7 and the MSTPCRB.MSTPB24 bit security attribution"]
pub type PSARB24_R = crate::BitReader<PSARB24_A>;
#[doc = "SCI7 and the MSTPCRB.MSTPB24 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARB24_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARB24_A> for bool {
    #[inline(always)]
    fn from(variant: PSARB24_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARB24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARB24_A {
        match self.bits {
            false => PSARB24_A::_0,
            true => PSARB24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARB24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARB24_A::_1
    }
}
#[doc = "Field `PSARB24` writer - SCI7 and the MSTPCRB.MSTPB24 bit security attribution"]
pub type PSARB24_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARB_SPEC, PSARB24_A, O>;
impl<'a, const O: u8> PSARB24_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARB24_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARB24_A::_1)
    }
}
#[doc = "Field `PSARB25` reader - SCI6 and the MSTPCRB.MSTPB25 bit security attribution"]
pub type PSARB25_R = crate::BitReader<PSARB25_A>;
#[doc = "SCI6 and the MSTPCRB.MSTPB25 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARB25_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARB25_A> for bool {
    #[inline(always)]
    fn from(variant: PSARB25_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARB25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARB25_A {
        match self.bits {
            false => PSARB25_A::_0,
            true => PSARB25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARB25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARB25_A::_1
    }
}
#[doc = "Field `PSARB25` writer - SCI6 and the MSTPCRB.MSTPB25 bit security attribution"]
pub type PSARB25_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARB_SPEC, PSARB25_A, O>;
impl<'a, const O: u8> PSARB25_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARB25_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARB25_A::_1)
    }
}
#[doc = "Field `PSARB26` reader - SCI5 and the MSTPCRB.MSTPB26 bit security attribution"]
pub type PSARB26_R = crate::BitReader<PSARB26_A>;
#[doc = "SCI5 and the MSTPCRB.MSTPB26 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARB26_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARB26_A> for bool {
    #[inline(always)]
    fn from(variant: PSARB26_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARB26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARB26_A {
        match self.bits {
            false => PSARB26_A::_0,
            true => PSARB26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARB26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARB26_A::_1
    }
}
#[doc = "Field `PSARB26` writer - SCI5 and the MSTPCRB.MSTPB26 bit security attribution"]
pub type PSARB26_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARB_SPEC, PSARB26_A, O>;
impl<'a, const O: u8> PSARB26_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARB26_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARB26_A::_1)
    }
}
#[doc = "Field `PSARB27` reader - SCI4 and the MSTPCRB.MSTPB27 bit security attribution"]
pub type PSARB27_R = crate::BitReader<PSARB27_A>;
#[doc = "SCI4 and the MSTPCRB.MSTPB27 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARB27_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARB27_A> for bool {
    #[inline(always)]
    fn from(variant: PSARB27_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARB27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARB27_A {
        match self.bits {
            false => PSARB27_A::_0,
            true => PSARB27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARB27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARB27_A::_1
    }
}
#[doc = "Field `PSARB27` writer - SCI4 and the MSTPCRB.MSTPB27 bit security attribution"]
pub type PSARB27_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARB_SPEC, PSARB27_A, O>;
impl<'a, const O: u8> PSARB27_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARB27_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARB27_A::_1)
    }
}
#[doc = "Field `PSARB28` reader - SCI3 and the MSTPCRB.MSTPB28 bit security attribution"]
pub type PSARB28_R = crate::BitReader<PSARB28_A>;
#[doc = "SCI3 and the MSTPCRB.MSTPB28 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARB28_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARB28_A> for bool {
    #[inline(always)]
    fn from(variant: PSARB28_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARB28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARB28_A {
        match self.bits {
            false => PSARB28_A::_0,
            true => PSARB28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARB28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARB28_A::_1
    }
}
#[doc = "Field `PSARB28` writer - SCI3 and the MSTPCRB.MSTPB28 bit security attribution"]
pub type PSARB28_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARB_SPEC, PSARB28_A, O>;
impl<'a, const O: u8> PSARB28_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARB28_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARB28_A::_1)
    }
}
#[doc = "Field `PSARB29` reader - SCI2 and the MSTPCRB.MSTPB29 bit security attribution"]
pub type PSARB29_R = crate::BitReader<PSARB29_A>;
#[doc = "SCI2 and the MSTPCRB.MSTPB29 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARB29_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARB29_A> for bool {
    #[inline(always)]
    fn from(variant: PSARB29_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARB29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARB29_A {
        match self.bits {
            false => PSARB29_A::_0,
            true => PSARB29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARB29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARB29_A::_1
    }
}
#[doc = "Field `PSARB29` writer - SCI2 and the MSTPCRB.MSTPB29 bit security attribution"]
pub type PSARB29_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARB_SPEC, PSARB29_A, O>;
impl<'a, const O: u8> PSARB29_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARB29_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARB29_A::_1)
    }
}
#[doc = "Field `PSARB30` reader - SCI1 and the MSTPCRB.MSTPB30 bit security attribution"]
pub type PSARB30_R = crate::BitReader<PSARB30_A>;
#[doc = "SCI1 and the MSTPCRB.MSTPB30 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARB30_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARB30_A> for bool {
    #[inline(always)]
    fn from(variant: PSARB30_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARB30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARB30_A {
        match self.bits {
            false => PSARB30_A::_0,
            true => PSARB30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARB30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARB30_A::_1
    }
}
#[doc = "Field `PSARB30` writer - SCI1 and the MSTPCRB.MSTPB30 bit security attribution"]
pub type PSARB30_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARB_SPEC, PSARB30_A, O>;
impl<'a, const O: u8> PSARB30_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARB30_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARB30_A::_1)
    }
}
#[doc = "Field `PSARB31` reader - SCI0 and the MSTPCRB.MSTPB31 bit security attribution"]
pub type PSARB31_R = crate::BitReader<PSARB31_A>;
#[doc = "SCI0 and the MSTPCRB.MSTPB31 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARB31_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARB31_A> for bool {
    #[inline(always)]
    fn from(variant: PSARB31_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARB31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARB31_A {
        match self.bits {
            false => PSARB31_A::_0,
            true => PSARB31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARB31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARB31_A::_1
    }
}
#[doc = "Field `PSARB31` writer - SCI0 and the MSTPCRB.MSTPB31 bit security attribution"]
pub type PSARB31_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARB_SPEC, PSARB31_A, O>;
impl<'a, const O: u8> PSARB31_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARB31_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARB31_A::_1)
    }
}
impl R {
    #[doc = "Bit 1 - CAN1 and the MSTPCRB.MSTPB1 bit security attribution"]
    #[inline(always)]
    pub fn psarb1(&self) -> PSARB1_R {
        PSARB1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CAN0 and the MSTPCRB.MSTPB2 bit security attribution"]
    #[inline(always)]
    pub fn psarb2(&self) -> PSARB2_R {
        PSARB2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - QSPI and the MSTPCRB.MSTPB6 bit security attribution"]
    #[inline(always)]
    pub fn psarb6(&self) -> PSARB6_R {
        PSARB6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - IIC1 and the MSTPCRB.MSTPB8 bit security attribution"]
    #[inline(always)]
    pub fn psarb8(&self) -> PSARB8_R {
        PSARB8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - IIC0 and the MSTPCRB.MSTPB9 bit security attribution"]
    #[inline(always)]
    pub fn psarb9(&self) -> PSARB9_R {
        PSARB9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - USBFS and the MSTPCRB.MSTPB11 bit security attribution"]
    #[inline(always)]
    pub fn psarb11(&self) -> PSARB11_R {
        PSARB11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - ETHER0/EDMAC0, the MSTPCRB.MSTPB15 bit and the PFENET.PHYMODE0 bit security attribution"]
    #[inline(always)]
    pub fn psarb15(&self) -> PSARB15_R {
        PSARB15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - OSPI and the MSTPCRB.MSTPB16 bit security attribution"]
    #[inline(always)]
    pub fn psarb16(&self) -> PSARB16_R {
        PSARB16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - SPI1 and the MSTPCRB.MSTPB18 bit security attribution"]
    #[inline(always)]
    pub fn psarb18(&self) -> PSARB18_R {
        PSARB18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SPI0 and the MSTPCRB.MSTPB19 bit security attribution"]
    #[inline(always)]
    pub fn psarb19(&self) -> PSARB19_R {
        PSARB19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 22 - SCI9 and the MSTPCRB.MSTPB22 bit security attribution"]
    #[inline(always)]
    pub fn psarb22(&self) -> PSARB22_R {
        PSARB22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SCI8 and the MSTPCRB.MSTPB23 bit security attribution"]
    #[inline(always)]
    pub fn psarb23(&self) -> PSARB23_R {
        PSARB23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - SCI7 and the MSTPCRB.MSTPB24 bit security attribution"]
    #[inline(always)]
    pub fn psarb24(&self) -> PSARB24_R {
        PSARB24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCI6 and the MSTPCRB.MSTPB25 bit security attribution"]
    #[inline(always)]
    pub fn psarb25(&self) -> PSARB25_R {
        PSARB25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - SCI5 and the MSTPCRB.MSTPB26 bit security attribution"]
    #[inline(always)]
    pub fn psarb26(&self) -> PSARB26_R {
        PSARB26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - SCI4 and the MSTPCRB.MSTPB27 bit security attribution"]
    #[inline(always)]
    pub fn psarb27(&self) -> PSARB27_R {
        PSARB27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - SCI3 and the MSTPCRB.MSTPB28 bit security attribution"]
    #[inline(always)]
    pub fn psarb28(&self) -> PSARB28_R {
        PSARB28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SCI2 and the MSTPCRB.MSTPB29 bit security attribution"]
    #[inline(always)]
    pub fn psarb29(&self) -> PSARB29_R {
        PSARB29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - SCI1 and the MSTPCRB.MSTPB30 bit security attribution"]
    #[inline(always)]
    pub fn psarb30(&self) -> PSARB30_R {
        PSARB30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SCI0 and the MSTPCRB.MSTPB31 bit security attribution"]
    #[inline(always)]
    pub fn psarb31(&self) -> PSARB31_R {
        PSARB31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - CAN1 and the MSTPCRB.MSTPB1 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psarb1(&mut self) -> PSARB1_W<1> {
        PSARB1_W::new(self)
    }
    #[doc = "Bit 2 - CAN0 and the MSTPCRB.MSTPB2 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psarb2(&mut self) -> PSARB2_W<2> {
        PSARB2_W::new(self)
    }
    #[doc = "Bit 8 - IIC1 and the MSTPCRB.MSTPB8 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psarb8(&mut self) -> PSARB8_W<8> {
        PSARB8_W::new(self)
    }
    #[doc = "Bit 9 - IIC0 and the MSTPCRB.MSTPB9 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psarb9(&mut self) -> PSARB9_W<9> {
        PSARB9_W::new(self)
    }
    #[doc = "Bit 11 - USBFS and the MSTPCRB.MSTPB11 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psarb11(&mut self) -> PSARB11_W<11> {
        PSARB11_W::new(self)
    }
    #[doc = "Bit 18 - SPI1 and the MSTPCRB.MSTPB18 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psarb18(&mut self) -> PSARB18_W<18> {
        PSARB18_W::new(self)
    }
    #[doc = "Bit 19 - SPI0 and the MSTPCRB.MSTPB19 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psarb19(&mut self) -> PSARB19_W<19> {
        PSARB19_W::new(self)
    }
    #[doc = "Bit 22 - SCI9 and the MSTPCRB.MSTPB22 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psarb22(&mut self) -> PSARB22_W<22> {
        PSARB22_W::new(self)
    }
    #[doc = "Bit 23 - SCI8 and the MSTPCRB.MSTPB23 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psarb23(&mut self) -> PSARB23_W<23> {
        PSARB23_W::new(self)
    }
    #[doc = "Bit 24 - SCI7 and the MSTPCRB.MSTPB24 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psarb24(&mut self) -> PSARB24_W<24> {
        PSARB24_W::new(self)
    }
    #[doc = "Bit 25 - SCI6 and the MSTPCRB.MSTPB25 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psarb25(&mut self) -> PSARB25_W<25> {
        PSARB25_W::new(self)
    }
    #[doc = "Bit 26 - SCI5 and the MSTPCRB.MSTPB26 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psarb26(&mut self) -> PSARB26_W<26> {
        PSARB26_W::new(self)
    }
    #[doc = "Bit 27 - SCI4 and the MSTPCRB.MSTPB27 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psarb27(&mut self) -> PSARB27_W<27> {
        PSARB27_W::new(self)
    }
    #[doc = "Bit 28 - SCI3 and the MSTPCRB.MSTPB28 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psarb28(&mut self) -> PSARB28_W<28> {
        PSARB28_W::new(self)
    }
    #[doc = "Bit 29 - SCI2 and the MSTPCRB.MSTPB29 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psarb29(&mut self) -> PSARB29_W<29> {
        PSARB29_W::new(self)
    }
    #[doc = "Bit 30 - SCI1 and the MSTPCRB.MSTPB30 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psarb30(&mut self) -> PSARB30_W<30> {
        PSARB30_W::new(self)
    }
    #[doc = "Bit 31 - SCI0 and the MSTPCRB.MSTPB31 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psarb31(&mut self) -> PSARB31_W<31> {
        PSARB31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Security Attribution Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psarb](index.html) module"]
pub struct PSARB_SPEC;
impl crate::RegisterSpec for PSARB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psarb::R](R) reader structure"]
impl crate::Readable for PSARB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psarb::W](W) writer structure"]
impl crate::Writable for PSARB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSARB to value 0xffff_ffff"]
impl crate::Resettable for PSARB_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
