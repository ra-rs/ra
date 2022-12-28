#[doc = "Register `WUPEN0` reader"]
pub struct R(crate::R<WUPEN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WUPEN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WUPEN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WUPEN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WUPEN0` writer"]
pub struct W(crate::W<WUPEN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WUPEN0_SPEC>;
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
impl From<crate::W<WUPEN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WUPEN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRQWUPEN` reader - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 15)"]
pub type IRQWUPEN_R = crate::FieldReader<u16, IRQWUPEN_A>;
#[doc = "IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum IRQWUPEN_A {
    #[doc = "0: Software Standby/Snooze Mode returns by IRQn interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Software Standby/Snooze Mode returns by IRQn interrupt is enabled"]
    _1 = 1,
}
impl From<IRQWUPEN_A> for u16 {
    #[inline(always)]
    fn from(variant: IRQWUPEN_A) -> Self {
        variant as _
    }
}
impl IRQWUPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IRQWUPEN_A> {
        match self.bits {
            0 => Some(IRQWUPEN_A::_0),
            1 => Some(IRQWUPEN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRQWUPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRQWUPEN_A::_1
    }
}
#[doc = "Field `IRQWUPEN` writer - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 15)"]
pub type IRQWUPEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WUPEN0_SPEC, u16, IRQWUPEN_A, 16, O>;
impl<'a, const O: u8> IRQWUPEN_W<'a, O> {
    #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRQWUPEN_A::_0)
    }
    #[doc = "Software Standby/Snooze Mode returns by IRQn interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRQWUPEN_A::_1)
    }
}
#[doc = "Field `IWDTWUPEN` reader - IWDT Interrupt Software Standby/Snooze Mode Returns Enable bit"]
pub type IWDTWUPEN_R = crate::BitReader<IWDTWUPEN_A>;
#[doc = "IWDT Interrupt Software Standby/Snooze Mode Returns Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IWDTWUPEN_A {
    #[doc = "0: Software Standby/Snooze Mode returns by IWDT interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Software Standby/Snooze Mode returns by IWDT interrupt is enabled"]
    _1 = 1,
}
impl From<IWDTWUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: IWDTWUPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl IWDTWUPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IWDTWUPEN_A {
        match self.bits {
            false => IWDTWUPEN_A::_0,
            true => IWDTWUPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IWDTWUPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IWDTWUPEN_A::_1
    }
}
#[doc = "Field `IWDTWUPEN` writer - IWDT Interrupt Software Standby/Snooze Mode Returns Enable bit"]
pub type IWDTWUPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN0_SPEC, IWDTWUPEN_A, O>;
impl<'a, const O: u8> IWDTWUPEN_W<'a, O> {
    #[doc = "Software Standby/Snooze Mode returns by IWDT interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IWDTWUPEN_A::_0)
    }
    #[doc = "Software Standby/Snooze Mode returns by IWDT interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IWDTWUPEN_A::_1)
    }
}
#[doc = "Field `KEYWUPEN` reader - Key interrupt S/W standby returns enable bit"]
pub type KEYWUPEN_R = crate::BitReader<KEYWUPEN_A>;
#[doc = "Key interrupt S/W standby returns enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KEYWUPEN_A {
    #[doc = "0: S/W standby returns by KEY interrupt is disabled"]
    _0 = 0,
    #[doc = "1: S/W standby returns by KEY interrupt is enabled"]
    _1 = 1,
}
impl From<KEYWUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: KEYWUPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl KEYWUPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KEYWUPEN_A {
        match self.bits {
            false => KEYWUPEN_A::_0,
            true => KEYWUPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KEYWUPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KEYWUPEN_A::_1
    }
}
#[doc = "Field `KEYWUPEN` writer - Key interrupt S/W standby returns enable bit"]
pub type KEYWUPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN0_SPEC, KEYWUPEN_A, O>;
impl<'a, const O: u8> KEYWUPEN_W<'a, O> {
    #[doc = "S/W standby returns by KEY interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KEYWUPEN_A::_0)
    }
    #[doc = "S/W standby returns by KEY interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KEYWUPEN_A::_1)
    }
}
#[doc = "Field `LVD1WUPEN` reader - LVD1 Interrupt Software Standby/Snooze Mode Returns Enable bit"]
pub type LVD1WUPEN_R = crate::BitReader<LVD1WUPEN_A>;
#[doc = "LVD1 Interrupt Software Standby/Snooze Mode Returns Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVD1WUPEN_A {
    #[doc = "0: Software Standby/Snooze Mode returns by LVD1 interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Software Standby/Snooze Mode returns by LVD1 interrupt is enabled"]
    _1 = 1,
}
impl From<LVD1WUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: LVD1WUPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl LVD1WUPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVD1WUPEN_A {
        match self.bits {
            false => LVD1WUPEN_A::_0,
            true => LVD1WUPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVD1WUPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVD1WUPEN_A::_1
    }
}
#[doc = "Field `LVD1WUPEN` writer - LVD1 Interrupt Software Standby/Snooze Mode Returns Enable bit"]
pub type LVD1WUPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN0_SPEC, LVD1WUPEN_A, O>;
impl<'a, const O: u8> LVD1WUPEN_W<'a, O> {
    #[doc = "Software Standby/Snooze Mode returns by LVD1 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LVD1WUPEN_A::_0)
    }
    #[doc = "Software Standby/Snooze Mode returns by LVD1 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LVD1WUPEN_A::_1)
    }
}
#[doc = "Field `LVD2WUPEN` reader - LVD2 Interrupt Software Standby/Snooze Mode Returns Enable bit"]
pub type LVD2WUPEN_R = crate::BitReader<LVD2WUPEN_A>;
#[doc = "LVD2 Interrupt Software Standby/Snooze Mode Returns Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVD2WUPEN_A {
    #[doc = "0: Software Standby/Snooze Mode returns by LVD2 interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Software Standby/Snooze Mode returns by LVD2 interrupt is enabled"]
    _1 = 1,
}
impl From<LVD2WUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: LVD2WUPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl LVD2WUPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVD2WUPEN_A {
        match self.bits {
            false => LVD2WUPEN_A::_0,
            true => LVD2WUPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVD2WUPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVD2WUPEN_A::_1
    }
}
#[doc = "Field `LVD2WUPEN` writer - LVD2 Interrupt Software Standby/Snooze Mode Returns Enable bit"]
pub type LVD2WUPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN0_SPEC, LVD2WUPEN_A, O>;
impl<'a, const O: u8> LVD2WUPEN_W<'a, O> {
    #[doc = "Software Standby/Snooze Mode returns by LVD2 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LVD2WUPEN_A::_0)
    }
    #[doc = "Software Standby/Snooze Mode returns by LVD2 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LVD2WUPEN_A::_1)
    }
}
#[doc = "Field `AGT1UDWUPEN` reader - AGT1 Underflow Interrupt Software Standby/Snooze Mode Returns Enable bit"]
pub type AGT1UDWUPEN_R = crate::BitReader<AGT1UDWUPEN_A>;
#[doc = "AGT1 Underflow Interrupt Software Standby/Snooze Mode Returns Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AGT1UDWUPEN_A {
    #[doc = "0: Software Standby/Snooze Mode returns by AGT1 underflow interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Software Standby/Snooze Mode returns by AGT1 underflow interrupt is enabled"]
    _1 = 1,
}
impl From<AGT1UDWUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: AGT1UDWUPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl AGT1UDWUPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AGT1UDWUPEN_A {
        match self.bits {
            false => AGT1UDWUPEN_A::_0,
            true => AGT1UDWUPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AGT1UDWUPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AGT1UDWUPEN_A::_1
    }
}
#[doc = "Field `AGT1UDWUPEN` writer - AGT1 Underflow Interrupt Software Standby/Snooze Mode Returns Enable bit"]
pub type AGT1UDWUPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN0_SPEC, AGT1UDWUPEN_A, O>;
impl<'a, const O: u8> AGT1UDWUPEN_W<'a, O> {
    #[doc = "Software Standby/Snooze Mode returns by AGT1 underflow interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AGT1UDWUPEN_A::_0)
    }
    #[doc = "Software Standby/Snooze Mode returns by AGT1 underflow interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AGT1UDWUPEN_A::_1)
    }
}
#[doc = "Field `AGT1CAWUPEN` reader - AGT1 Compare Match A Interrupt Software Standby/Snooze Mode Returns Enable bit"]
pub type AGT1CAWUPEN_R = crate::BitReader<AGT1CAWUPEN_A>;
#[doc = "AGT1 Compare Match A Interrupt Software Standby/Snooze Mode Returns Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AGT1CAWUPEN_A {
    #[doc = "0: Software Standby/Snooze Mode returns by AGT1 compare match A interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Software Standby/Snooze Mode returns by AGT1 compare match A interrupt is enabled"]
    _1 = 1,
}
impl From<AGT1CAWUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: AGT1CAWUPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl AGT1CAWUPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AGT1CAWUPEN_A {
        match self.bits {
            false => AGT1CAWUPEN_A::_0,
            true => AGT1CAWUPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AGT1CAWUPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AGT1CAWUPEN_A::_1
    }
}
#[doc = "Field `AGT1CAWUPEN` writer - AGT1 Compare Match A Interrupt Software Standby/Snooze Mode Returns Enable bit"]
pub type AGT1CAWUPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN0_SPEC, AGT1CAWUPEN_A, O>;
impl<'a, const O: u8> AGT1CAWUPEN_W<'a, O> {
    #[doc = "Software Standby/Snooze Mode returns by AGT1 compare match A interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AGT1CAWUPEN_A::_0)
    }
    #[doc = "Software Standby/Snooze Mode returns by AGT1 compare match A interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AGT1CAWUPEN_A::_1)
    }
}
#[doc = "Field `AGT1CBWUPEN` reader - AGT1 Compare Match B Interrupt Software Standby/Snooze Mode Returns Enable bit"]
pub type AGT1CBWUPEN_R = crate::BitReader<AGT1CBWUPEN_A>;
#[doc = "AGT1 Compare Match B Interrupt Software Standby/Snooze Mode Returns Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AGT1CBWUPEN_A {
    #[doc = "0: Software Standby/Snooze Mode returns by AGT1 compare match B interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Software Standby/Snooze Mode returns by AGT1 compare match B interrupt is enabled"]
    _1 = 1,
}
impl From<AGT1CBWUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: AGT1CBWUPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl AGT1CBWUPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AGT1CBWUPEN_A {
        match self.bits {
            false => AGT1CBWUPEN_A::_0,
            true => AGT1CBWUPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AGT1CBWUPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AGT1CBWUPEN_A::_1
    }
}
#[doc = "Field `AGT1CBWUPEN` writer - AGT1 Compare Match B Interrupt Software Standby/Snooze Mode Returns Enable bit"]
pub type AGT1CBWUPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN0_SPEC, AGT1CBWUPEN_A, O>;
impl<'a, const O: u8> AGT1CBWUPEN_W<'a, O> {
    #[doc = "Software Standby/Snooze Mode returns by AGT1 compare match B interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AGT1CBWUPEN_A::_0)
    }
    #[doc = "Software Standby/Snooze Mode returns by AGT1 compare match B interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AGT1CBWUPEN_A::_1)
    }
}
#[doc = "Field `IIC0WUPEN` reader - IIC0 Address Match Interrupt Software Standby/Snooze Mode Returns Enable bit"]
pub type IIC0WUPEN_R = crate::BitReader<IIC0WUPEN_A>;
#[doc = "IIC0 Address Match Interrupt Software Standby/Snooze Mode Returns Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IIC0WUPEN_A {
    #[doc = "0: Software Standby/Snooze Mode returns by IIC0 address match interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Software Standby/Snooze Mode returns by IIC0 address match interrupt is enabled"]
    _1 = 1,
}
impl From<IIC0WUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: IIC0WUPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl IIC0WUPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IIC0WUPEN_A {
        match self.bits {
            false => IIC0WUPEN_A::_0,
            true => IIC0WUPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IIC0WUPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IIC0WUPEN_A::_1
    }
}
#[doc = "Field `IIC0WUPEN` writer - IIC0 Address Match Interrupt Software Standby/Snooze Mode Returns Enable bit"]
pub type IIC0WUPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUPEN0_SPEC, IIC0WUPEN_A, O>;
impl<'a, const O: u8> IIC0WUPEN_W<'a, O> {
    #[doc = "Software Standby/Snooze Mode returns by IIC0 address match interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IIC0WUPEN_A::_0)
    }
    #[doc = "Software Standby/Snooze Mode returns by IIC0 address match interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IIC0WUPEN_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:15 - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 15)"]
    #[inline(always)]
    pub fn irqwupen(&self) -> IRQWUPEN_R {
        IRQWUPEN_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - IWDT Interrupt Software Standby/Snooze Mode Returns Enable bit"]
    #[inline(always)]
    pub fn iwdtwupen(&self) -> IWDTWUPEN_R {
        IWDTWUPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Key interrupt S/W standby returns enable bit"]
    #[inline(always)]
    pub fn keywupen(&self) -> KEYWUPEN_R {
        KEYWUPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - LVD1 Interrupt Software Standby/Snooze Mode Returns Enable bit"]
    #[inline(always)]
    pub fn lvd1wupen(&self) -> LVD1WUPEN_R {
        LVD1WUPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - LVD2 Interrupt Software Standby/Snooze Mode Returns Enable bit"]
    #[inline(always)]
    pub fn lvd2wupen(&self) -> LVD2WUPEN_R {
        LVD2WUPEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 28 - AGT1 Underflow Interrupt Software Standby/Snooze Mode Returns Enable bit"]
    #[inline(always)]
    pub fn agt1udwupen(&self) -> AGT1UDWUPEN_R {
        AGT1UDWUPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - AGT1 Compare Match A Interrupt Software Standby/Snooze Mode Returns Enable bit"]
    #[inline(always)]
    pub fn agt1cawupen(&self) -> AGT1CAWUPEN_R {
        AGT1CAWUPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - AGT1 Compare Match B Interrupt Software Standby/Snooze Mode Returns Enable bit"]
    #[inline(always)]
    pub fn agt1cbwupen(&self) -> AGT1CBWUPEN_R {
        AGT1CBWUPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - IIC0 Address Match Interrupt Software Standby/Snooze Mode Returns Enable bit"]
    #[inline(always)]
    pub fn iic0wupen(&self) -> IIC0WUPEN_R {
        IIC0WUPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - IRQn Interrupt Software Standby/Snooze Mode Returns Enable bit (n = 0 to 15)"]
    #[inline(always)]
    #[must_use]
    pub fn irqwupen(&mut self) -> IRQWUPEN_W<0> {
        IRQWUPEN_W::new(self)
    }
    #[doc = "Bit 16 - IWDT Interrupt Software Standby/Snooze Mode Returns Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn iwdtwupen(&mut self) -> IWDTWUPEN_W<16> {
        IWDTWUPEN_W::new(self)
    }
    #[doc = "Bit 17 - Key interrupt S/W standby returns enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn keywupen(&mut self) -> KEYWUPEN_W<17> {
        KEYWUPEN_W::new(self)
    }
    #[doc = "Bit 18 - LVD1 Interrupt Software Standby/Snooze Mode Returns Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn lvd1wupen(&mut self) -> LVD1WUPEN_W<18> {
        LVD1WUPEN_W::new(self)
    }
    #[doc = "Bit 19 - LVD2 Interrupt Software Standby/Snooze Mode Returns Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn lvd2wupen(&mut self) -> LVD2WUPEN_W<19> {
        LVD2WUPEN_W::new(self)
    }
    #[doc = "Bit 28 - AGT1 Underflow Interrupt Software Standby/Snooze Mode Returns Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn agt1udwupen(&mut self) -> AGT1UDWUPEN_W<28> {
        AGT1UDWUPEN_W::new(self)
    }
    #[doc = "Bit 29 - AGT1 Compare Match A Interrupt Software Standby/Snooze Mode Returns Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn agt1cawupen(&mut self) -> AGT1CAWUPEN_W<29> {
        AGT1CAWUPEN_W::new(self)
    }
    #[doc = "Bit 30 - AGT1 Compare Match B Interrupt Software Standby/Snooze Mode Returns Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn agt1cbwupen(&mut self) -> AGT1CBWUPEN_W<30> {
        AGT1CBWUPEN_W::new(self)
    }
    #[doc = "Bit 31 - IIC0 Address Match Interrupt Software Standby/Snooze Mode Returns Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn iic0wupen(&mut self) -> IIC0WUPEN_W<31> {
        IIC0WUPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wake Up Interrupt Enable Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wupen0](index.html) module"]
pub struct WUPEN0_SPEC;
impl crate::RegisterSpec for WUPEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wupen0::R](R) reader structure"]
impl crate::Readable for WUPEN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wupen0::W](W) writer structure"]
impl crate::Writable for WUPEN0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WUPEN0 to value 0"]
impl crate::Resettable for WUPEN0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
