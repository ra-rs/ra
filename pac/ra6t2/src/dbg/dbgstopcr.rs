#[doc = "Register `DBGSTOPCR` reader"]
pub struct R(crate::R<DBGSTOPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBGSTOPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBGSTOPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBGSTOPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBGSTOPCR` writer"]
pub struct W(crate::W<DBGSTOPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBGSTOPCR_SPEC>;
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
impl From<crate::W<DBGSTOPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBGSTOPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBGSTOP_IWDT` reader - Mask bit for IWDT reset/interrupt in the OCD run mode"]
pub type DBGSTOP_IWDT_R = crate::BitReader<DBGSTOP_IWDT_A>;
#[doc = "Mask bit for IWDT reset/interrupt in the OCD run mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBGSTOP_IWDT_A {
    #[doc = "0: Enable IWDT reset/interrupt"]
    _0 = 0,
    #[doc = "1: Mask IWDT reset/interrupt and stop IWDT counter"]
    _1 = 1,
}
impl From<DBGSTOP_IWDT_A> for bool {
    #[inline(always)]
    fn from(variant: DBGSTOP_IWDT_A) -> Self {
        variant as u8 != 0
    }
}
impl DBGSTOP_IWDT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGSTOP_IWDT_A {
        match self.bits {
            false => DBGSTOP_IWDT_A::_0,
            true => DBGSTOP_IWDT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DBGSTOP_IWDT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DBGSTOP_IWDT_A::_1
    }
}
#[doc = "Field `DBGSTOP_IWDT` writer - Mask bit for IWDT reset/interrupt in the OCD run mode"]
pub type DBGSTOP_IWDT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DBGSTOPCR_SPEC, DBGSTOP_IWDT_A, O>;
impl<'a, const O: u8> DBGSTOP_IWDT_W<'a, O> {
    #[doc = "Enable IWDT reset/interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBGSTOP_IWDT_A::_0)
    }
    #[doc = "Mask IWDT reset/interrupt and stop IWDT counter"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBGSTOP_IWDT_A::_1)
    }
}
#[doc = "Field `DBGSTOP_WDT` reader - Mask bit for WDT reset/interrupt in the OCD run mode"]
pub type DBGSTOP_WDT_R = crate::BitReader<DBGSTOP_WDT_A>;
#[doc = "Mask bit for WDT reset/interrupt in the OCD run mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBGSTOP_WDT_A {
    #[doc = "0: Enable WDT reset/interrupt"]
    _0 = 0,
    #[doc = "1: Mask WDT reset/interrupt and stop WDT counter"]
    _1 = 1,
}
impl From<DBGSTOP_WDT_A> for bool {
    #[inline(always)]
    fn from(variant: DBGSTOP_WDT_A) -> Self {
        variant as u8 != 0
    }
}
impl DBGSTOP_WDT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGSTOP_WDT_A {
        match self.bits {
            false => DBGSTOP_WDT_A::_0,
            true => DBGSTOP_WDT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DBGSTOP_WDT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DBGSTOP_WDT_A::_1
    }
}
#[doc = "Field `DBGSTOP_WDT` writer - Mask bit for WDT reset/interrupt in the OCD run mode"]
pub type DBGSTOP_WDT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DBGSTOPCR_SPEC, DBGSTOP_WDT_A, O>;
impl<'a, const O: u8> DBGSTOP_WDT_W<'a, O> {
    #[doc = "Enable WDT reset/interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBGSTOP_WDT_A::_0)
    }
    #[doc = "Mask WDT reset/interrupt and stop WDT counter"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBGSTOP_WDT_A::_1)
    }
}
#[doc = "Field `DBGSTOP_LVD0` reader - Mask bit for LVD0 reset"]
pub type DBGSTOP_LVD0_R = crate::BitReader<DBGSTOP_LVD0_A>;
#[doc = "Mask bit for LVD0 reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBGSTOP_LVD0_A {
    #[doc = "0: Enable LVD0 reset"]
    _0 = 0,
    #[doc = "1: Mask LVD0 reset"]
    _1 = 1,
}
impl From<DBGSTOP_LVD0_A> for bool {
    #[inline(always)]
    fn from(variant: DBGSTOP_LVD0_A) -> Self {
        variant as u8 != 0
    }
}
impl DBGSTOP_LVD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGSTOP_LVD0_A {
        match self.bits {
            false => DBGSTOP_LVD0_A::_0,
            true => DBGSTOP_LVD0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DBGSTOP_LVD0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DBGSTOP_LVD0_A::_1
    }
}
#[doc = "Field `DBGSTOP_LVD0` writer - Mask bit for LVD0 reset"]
pub type DBGSTOP_LVD0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DBGSTOPCR_SPEC, DBGSTOP_LVD0_A, O>;
impl<'a, const O: u8> DBGSTOP_LVD0_W<'a, O> {
    #[doc = "Enable LVD0 reset"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBGSTOP_LVD0_A::_0)
    }
    #[doc = "Mask LVD0 reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBGSTOP_LVD0_A::_1)
    }
}
#[doc = "Field `DBGSTOP_LVD1` reader - Mask bit for LVD1 reset/interrupt"]
pub type DBGSTOP_LVD1_R = crate::BitReader<DBGSTOP_LVD1_A>;
#[doc = "Mask bit for LVD1 reset/interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBGSTOP_LVD1_A {
    #[doc = "0: Enable LVD1 reset/interrupt"]
    _0 = 0,
    #[doc = "1: Mask LVD1 reset/interrupt"]
    _1 = 1,
}
impl From<DBGSTOP_LVD1_A> for bool {
    #[inline(always)]
    fn from(variant: DBGSTOP_LVD1_A) -> Self {
        variant as u8 != 0
    }
}
impl DBGSTOP_LVD1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGSTOP_LVD1_A {
        match self.bits {
            false => DBGSTOP_LVD1_A::_0,
            true => DBGSTOP_LVD1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DBGSTOP_LVD1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DBGSTOP_LVD1_A::_1
    }
}
#[doc = "Field `DBGSTOP_LVD1` writer - Mask bit for LVD1 reset/interrupt"]
pub type DBGSTOP_LVD1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DBGSTOPCR_SPEC, DBGSTOP_LVD1_A, O>;
impl<'a, const O: u8> DBGSTOP_LVD1_W<'a, O> {
    #[doc = "Enable LVD1 reset/interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBGSTOP_LVD1_A::_0)
    }
    #[doc = "Mask LVD1 reset/interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBGSTOP_LVD1_A::_1)
    }
}
#[doc = "Field `DBGSTOP_LVD2` reader - Mask bit for LVD2 reset/interrupt"]
pub type DBGSTOP_LVD2_R = crate::BitReader<DBGSTOP_LVD2_A>;
#[doc = "Mask bit for LVD2 reset/interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBGSTOP_LVD2_A {
    #[doc = "0: Enable LVD2 reset/interrupt"]
    _0 = 0,
    #[doc = "1: Mask LVD2 reset/interrupt"]
    _1 = 1,
}
impl From<DBGSTOP_LVD2_A> for bool {
    #[inline(always)]
    fn from(variant: DBGSTOP_LVD2_A) -> Self {
        variant as u8 != 0
    }
}
impl DBGSTOP_LVD2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGSTOP_LVD2_A {
        match self.bits {
            false => DBGSTOP_LVD2_A::_0,
            true => DBGSTOP_LVD2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DBGSTOP_LVD2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DBGSTOP_LVD2_A::_1
    }
}
#[doc = "Field `DBGSTOP_LVD2` writer - Mask bit for LVD2 reset/interrupt"]
pub type DBGSTOP_LVD2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DBGSTOPCR_SPEC, DBGSTOP_LVD2_A, O>;
impl<'a, const O: u8> DBGSTOP_LVD2_W<'a, O> {
    #[doc = "Enable LVD2 reset/interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBGSTOP_LVD2_A::_0)
    }
    #[doc = "Mask LVD2 reset/interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBGSTOP_LVD2_A::_1)
    }
}
#[doc = "Field `DBGSTOP_RPER` reader - Mask bit for SRAM parity error reset/interrupt"]
pub type DBGSTOP_RPER_R = crate::BitReader<DBGSTOP_RPER_A>;
#[doc = "Mask bit for SRAM parity error reset/interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBGSTOP_RPER_A {
    #[doc = "0: Enable SRAM parity error reset/interrupt"]
    _0 = 0,
    #[doc = "1: Mask SRAM parity error reset/interrupt"]
    _1 = 1,
}
impl From<DBGSTOP_RPER_A> for bool {
    #[inline(always)]
    fn from(variant: DBGSTOP_RPER_A) -> Self {
        variant as u8 != 0
    }
}
impl DBGSTOP_RPER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGSTOP_RPER_A {
        match self.bits {
            false => DBGSTOP_RPER_A::_0,
            true => DBGSTOP_RPER_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DBGSTOP_RPER_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DBGSTOP_RPER_A::_1
    }
}
#[doc = "Field `DBGSTOP_RPER` writer - Mask bit for SRAM parity error reset/interrupt"]
pub type DBGSTOP_RPER_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DBGSTOPCR_SPEC, DBGSTOP_RPER_A, O>;
impl<'a, const O: u8> DBGSTOP_RPER_W<'a, O> {
    #[doc = "Enable SRAM parity error reset/interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBGSTOP_RPER_A::_0)
    }
    #[doc = "Mask SRAM parity error reset/interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBGSTOP_RPER_A::_1)
    }
}
#[doc = "Field `DBGSTOP_RECCR` reader - Mask bit for SRAM ECC error reset/interrupt"]
pub type DBGSTOP_RECCR_R = crate::BitReader<DBGSTOP_RECCR_A>;
#[doc = "Mask bit for SRAM ECC error reset/interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBGSTOP_RECCR_A {
    #[doc = "0: Enable SRAM ECC error reset/interrupt"]
    _0 = 0,
    #[doc = "1: Mask SRAM ECC error reset/interrupt"]
    _1 = 1,
}
impl From<DBGSTOP_RECCR_A> for bool {
    #[inline(always)]
    fn from(variant: DBGSTOP_RECCR_A) -> Self {
        variant as u8 != 0
    }
}
impl DBGSTOP_RECCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGSTOP_RECCR_A {
        match self.bits {
            false => DBGSTOP_RECCR_A::_0,
            true => DBGSTOP_RECCR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DBGSTOP_RECCR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DBGSTOP_RECCR_A::_1
    }
}
#[doc = "Field `DBGSTOP_RECCR` writer - Mask bit for SRAM ECC error reset/interrupt"]
pub type DBGSTOP_RECCR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DBGSTOPCR_SPEC, DBGSTOP_RECCR_A, O>;
impl<'a, const O: u8> DBGSTOP_RECCR_W<'a, O> {
    #[doc = "Enable SRAM ECC error reset/interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBGSTOP_RECCR_A::_0)
    }
    #[doc = "Mask SRAM ECC error reset/interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBGSTOP_RECCR_A::_1)
    }
}
#[doc = "Field `DBGSTOP_CPER` reader - Mask bit for Cache SRAM parity error reset/interrupt"]
pub type DBGSTOP_CPER_R = crate::BitReader<DBGSTOP_CPER_A>;
#[doc = "Mask bit for Cache SRAM parity error reset/interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBGSTOP_CPER_A {
    #[doc = "0: Enable Cache SRAM parity error reset/interrupt"]
    _0 = 0,
    #[doc = "1: Mask Cache SRAM parity error reset/interrupt"]
    _1 = 1,
}
impl From<DBGSTOP_CPER_A> for bool {
    #[inline(always)]
    fn from(variant: DBGSTOP_CPER_A) -> Self {
        variant as u8 != 0
    }
}
impl DBGSTOP_CPER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGSTOP_CPER_A {
        match self.bits {
            false => DBGSTOP_CPER_A::_0,
            true => DBGSTOP_CPER_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DBGSTOP_CPER_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DBGSTOP_CPER_A::_1
    }
}
#[doc = "Field `DBGSTOP_CPER` writer - Mask bit for Cache SRAM parity error reset/interrupt"]
pub type DBGSTOP_CPER_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DBGSTOPCR_SPEC, DBGSTOP_CPER_A, O>;
impl<'a, const O: u8> DBGSTOP_CPER_W<'a, O> {
    #[doc = "Enable Cache SRAM parity error reset/interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBGSTOP_CPER_A::_0)
    }
    #[doc = "Mask Cache SRAM parity error reset/interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBGSTOP_CPER_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Mask bit for IWDT reset/interrupt in the OCD run mode"]
    #[inline(always)]
    pub fn dbgstop_iwdt(&self) -> DBGSTOP_IWDT_R {
        DBGSTOP_IWDT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask bit for WDT reset/interrupt in the OCD run mode"]
    #[inline(always)]
    pub fn dbgstop_wdt(&self) -> DBGSTOP_WDT_R {
        DBGSTOP_WDT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - Mask bit for LVD0 reset"]
    #[inline(always)]
    pub fn dbgstop_lvd0(&self) -> DBGSTOP_LVD0_R {
        DBGSTOP_LVD0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Mask bit for LVD1 reset/interrupt"]
    #[inline(always)]
    pub fn dbgstop_lvd1(&self) -> DBGSTOP_LVD1_R {
        DBGSTOP_LVD1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Mask bit for LVD2 reset/interrupt"]
    #[inline(always)]
    pub fn dbgstop_lvd2(&self) -> DBGSTOP_LVD2_R {
        DBGSTOP_LVD2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - Mask bit for SRAM parity error reset/interrupt"]
    #[inline(always)]
    pub fn dbgstop_rper(&self) -> DBGSTOP_RPER_R {
        DBGSTOP_RPER_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Mask bit for SRAM ECC error reset/interrupt"]
    #[inline(always)]
    pub fn dbgstop_reccr(&self) -> DBGSTOP_RECCR_R {
        DBGSTOP_RECCR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 31 - Mask bit for Cache SRAM parity error reset/interrupt"]
    #[inline(always)]
    pub fn dbgstop_cper(&self) -> DBGSTOP_CPER_R {
        DBGSTOP_CPER_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask bit for IWDT reset/interrupt in the OCD run mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbgstop_iwdt(&mut self) -> DBGSTOP_IWDT_W<0> {
        DBGSTOP_IWDT_W::new(self)
    }
    #[doc = "Bit 1 - Mask bit for WDT reset/interrupt in the OCD run mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbgstop_wdt(&mut self) -> DBGSTOP_WDT_W<1> {
        DBGSTOP_WDT_W::new(self)
    }
    #[doc = "Bit 16 - Mask bit for LVD0 reset"]
    #[inline(always)]
    #[must_use]
    pub fn dbgstop_lvd0(&mut self) -> DBGSTOP_LVD0_W<16> {
        DBGSTOP_LVD0_W::new(self)
    }
    #[doc = "Bit 17 - Mask bit for LVD1 reset/interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn dbgstop_lvd1(&mut self) -> DBGSTOP_LVD1_W<17> {
        DBGSTOP_LVD1_W::new(self)
    }
    #[doc = "Bit 18 - Mask bit for LVD2 reset/interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn dbgstop_lvd2(&mut self) -> DBGSTOP_LVD2_W<18> {
        DBGSTOP_LVD2_W::new(self)
    }
    #[doc = "Bit 24 - Mask bit for SRAM parity error reset/interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn dbgstop_rper(&mut self) -> DBGSTOP_RPER_W<24> {
        DBGSTOP_RPER_W::new(self)
    }
    #[doc = "Bit 25 - Mask bit for SRAM ECC error reset/interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn dbgstop_reccr(&mut self) -> DBGSTOP_RECCR_W<25> {
        DBGSTOP_RECCR_W::new(self)
    }
    #[doc = "Bit 31 - Mask bit for Cache SRAM parity error reset/interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn dbgstop_cper(&mut self) -> DBGSTOP_CPER_W<31> {
        DBGSTOP_CPER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debug Stop Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbgstopcr](index.html) module"]
pub struct DBGSTOPCR_SPEC;
impl crate::RegisterSpec for DBGSTOPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbgstopcr::R](R) reader structure"]
impl crate::Readable for DBGSTOPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbgstopcr::W](W) writer structure"]
impl crate::Writable for DBGSTOPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DBGSTOPCR to value 0x03"]
impl crate::Resettable for DBGSTOPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
