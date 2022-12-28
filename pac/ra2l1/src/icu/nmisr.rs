#[doc = "Register `NMISR` reader"]
pub struct R(crate::R<NMISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NMISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NMISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NMISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IWDTST` reader - IWDT Underflow/Refresh Error Interrupt Status Flag"]
pub type IWDTST_R = crate::BitReader<IWDTST_A>;
#[doc = "IWDT Underflow/Refresh Error Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IWDTST_A {
    #[doc = "0: Interrupt not requested"]
    _0 = 0,
    #[doc = "1: Interrupt requested"]
    _1 = 1,
}
impl From<IWDTST_A> for bool {
    #[inline(always)]
    fn from(variant: IWDTST_A) -> Self {
        variant as u8 != 0
    }
}
impl IWDTST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IWDTST_A {
        match self.bits {
            false => IWDTST_A::_0,
            true => IWDTST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IWDTST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IWDTST_A::_1
    }
}
#[doc = "Field `WDTST` reader - WDT Underflow/Refresh Error Interrupt Status Flag"]
pub type WDTST_R = crate::BitReader<WDTST_A>;
#[doc = "WDT Underflow/Refresh Error Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDTST_A {
    #[doc = "0: Interrupt not requested"]
    _0 = 0,
    #[doc = "1: Interrupt requested"]
    _1 = 1,
}
impl From<WDTST_A> for bool {
    #[inline(always)]
    fn from(variant: WDTST_A) -> Self {
        variant as u8 != 0
    }
}
impl WDTST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTST_A {
        match self.bits {
            false => WDTST_A::_0,
            true => WDTST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WDTST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WDTST_A::_1
    }
}
#[doc = "Field `LVD1ST` reader - Voltage Monitor 1 Interrupt Status Flag"]
pub type LVD1ST_R = crate::BitReader<LVD1ST_A>;
#[doc = "Voltage Monitor 1 Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVD1ST_A {
    #[doc = "0: Interrupt not requested"]
    _0 = 0,
    #[doc = "1: Interrupt requested"]
    _1 = 1,
}
impl From<LVD1ST_A> for bool {
    #[inline(always)]
    fn from(variant: LVD1ST_A) -> Self {
        variant as u8 != 0
    }
}
impl LVD1ST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVD1ST_A {
        match self.bits {
            false => LVD1ST_A::_0,
            true => LVD1ST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVD1ST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVD1ST_A::_1
    }
}
#[doc = "Field `LVD2ST` reader - Voltage Monitor 2 Interrupt Status Flag"]
pub type LVD2ST_R = crate::BitReader<LVD2ST_A>;
#[doc = "Voltage Monitor 2 Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVD2ST_A {
    #[doc = "0: Interrupt not requested"]
    _0 = 0,
    #[doc = "1: Interrupt requested"]
    _1 = 1,
}
impl From<LVD2ST_A> for bool {
    #[inline(always)]
    fn from(variant: LVD2ST_A) -> Self {
        variant as u8 != 0
    }
}
impl LVD2ST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVD2ST_A {
        match self.bits {
            false => LVD2ST_A::_0,
            true => LVD2ST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVD2ST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVD2ST_A::_1
    }
}
#[doc = "Field `OSTST` reader - Oscillation Stop Detection Interrupt Status Flag"]
pub type OSTST_R = crate::BitReader<OSTST_A>;
#[doc = "Oscillation Stop Detection Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSTST_A {
    #[doc = "0: Interrupt not requested for main oscillation stop"]
    _0 = 0,
    #[doc = "1: Interrupt requested for main oscillation stop"]
    _1 = 1,
}
impl From<OSTST_A> for bool {
    #[inline(always)]
    fn from(variant: OSTST_A) -> Self {
        variant as u8 != 0
    }
}
impl OSTST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSTST_A {
        match self.bits {
            false => OSTST_A::_0,
            true => OSTST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OSTST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OSTST_A::_1
    }
}
#[doc = "Field `NMIST` reader - NMI Pin Interrupt Status Flag"]
pub type NMIST_R = crate::BitReader<NMIST_A>;
#[doc = "NMI Pin Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NMIST_A {
    #[doc = "0: Interrupt not requested"]
    _0 = 0,
    #[doc = "1: Interrupt requested"]
    _1 = 1,
}
impl From<NMIST_A> for bool {
    #[inline(always)]
    fn from(variant: NMIST_A) -> Self {
        variant as u8 != 0
    }
}
impl NMIST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NMIST_A {
        match self.bits {
            false => NMIST_A::_0,
            true => NMIST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NMIST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NMIST_A::_1
    }
}
#[doc = "Field `RPEST` reader - SRAM Parity Error Interrupt Status Flag"]
pub type RPEST_R = crate::BitReader<RPEST_A>;
#[doc = "SRAM Parity Error Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPEST_A {
    #[doc = "0: Interrupt not requested"]
    _0 = 0,
    #[doc = "1: Interrupt requested"]
    _1 = 1,
}
impl From<RPEST_A> for bool {
    #[inline(always)]
    fn from(variant: RPEST_A) -> Self {
        variant as u8 != 0
    }
}
impl RPEST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RPEST_A {
        match self.bits {
            false => RPEST_A::_0,
            true => RPEST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RPEST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RPEST_A::_1
    }
}
#[doc = "Field `RECCST` reader - SRAM ECC Error Interrupt Status Flag"]
pub type RECCST_R = crate::BitReader<RECCST_A>;
#[doc = "SRAM ECC Error Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RECCST_A {
    #[doc = "0: Interrupt not requested"]
    _0 = 0,
    #[doc = "1: Interrupt requested"]
    _1 = 1,
}
impl From<RECCST_A> for bool {
    #[inline(always)]
    fn from(variant: RECCST_A) -> Self {
        variant as u8 != 0
    }
}
impl RECCST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECCST_A {
        match self.bits {
            false => RECCST_A::_0,
            true => RECCST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RECCST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RECCST_A::_1
    }
}
#[doc = "Field `BUSSST` reader - Bus Slave MPU Error Interrupt Status Flag"]
pub type BUSSST_R = crate::BitReader<BUSSST_A>;
#[doc = "Bus Slave MPU Error Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSSST_A {
    #[doc = "0: Interrupt not requested"]
    _0 = 0,
    #[doc = "1: Interrupt requested."]
    _1 = 1,
}
impl From<BUSSST_A> for bool {
    #[inline(always)]
    fn from(variant: BUSSST_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSSST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSSST_A {
        match self.bits {
            false => BUSSST_A::_0,
            true => BUSSST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUSSST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUSSST_A::_1
    }
}
#[doc = "Field `BUSMST` reader - Bus Master MPU Error Interrupt Status Flag"]
pub type BUSMST_R = crate::BitReader<BUSMST_A>;
#[doc = "Bus Master MPU Error Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSMST_A {
    #[doc = "0: Interrupt not requested"]
    _0 = 0,
    #[doc = "1: Interrupt requested"]
    _1 = 1,
}
impl From<BUSMST_A> for bool {
    #[inline(always)]
    fn from(variant: BUSMST_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSMST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSMST_A {
        match self.bits {
            false => BUSMST_A::_0,
            true => BUSMST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUSMST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUSMST_A::_1
    }
}
#[doc = "Field `SPEST` reader - CPU Stack Pointer Monitor Interrupt Status Flag"]
pub type SPEST_R = crate::BitReader<SPEST_A>;
#[doc = "CPU Stack Pointer Monitor Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPEST_A {
    #[doc = "0: Interrupt not requested"]
    _0 = 0,
    #[doc = "1: Interrupt requested"]
    _1 = 1,
}
impl From<SPEST_A> for bool {
    #[inline(always)]
    fn from(variant: SPEST_A) -> Self {
        variant as u8 != 0
    }
}
impl SPEST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPEST_A {
        match self.bits {
            false => SPEST_A::_0,
            true => SPEST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPEST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPEST_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - IWDT Underflow/Refresh Error Interrupt Status Flag"]
    #[inline(always)]
    pub fn iwdtst(&self) -> IWDTST_R {
        IWDTST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WDT Underflow/Refresh Error Interrupt Status Flag"]
    #[inline(always)]
    pub fn wdtst(&self) -> WDTST_R {
        WDTST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Voltage Monitor 1 Interrupt Status Flag"]
    #[inline(always)]
    pub fn lvd1st(&self) -> LVD1ST_R {
        LVD1ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Voltage Monitor 2 Interrupt Status Flag"]
    #[inline(always)]
    pub fn lvd2st(&self) -> LVD2ST_R {
        LVD2ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Oscillation Stop Detection Interrupt Status Flag"]
    #[inline(always)]
    pub fn ostst(&self) -> OSTST_R {
        OSTST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NMI Pin Interrupt Status Flag"]
    #[inline(always)]
    pub fn nmist(&self) -> NMIST_R {
        NMIST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SRAM Parity Error Interrupt Status Flag"]
    #[inline(always)]
    pub fn rpest(&self) -> RPEST_R {
        RPEST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SRAM ECC Error Interrupt Status Flag"]
    #[inline(always)]
    pub fn reccst(&self) -> RECCST_R {
        RECCST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Bus Slave MPU Error Interrupt Status Flag"]
    #[inline(always)]
    pub fn bussst(&self) -> BUSSST_R {
        BUSSST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Bus Master MPU Error Interrupt Status Flag"]
    #[inline(always)]
    pub fn busmst(&self) -> BUSMST_R {
        BUSMST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CPU Stack Pointer Monitor Interrupt Status Flag"]
    #[inline(always)]
    pub fn spest(&self) -> SPEST_R {
        SPEST_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[doc = "Non-Maskable Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nmisr](index.html) module"]
pub struct NMISR_SPEC;
impl crate::RegisterSpec for NMISR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [nmisr::R](R) reader structure"]
impl crate::Readable for NMISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets NMISR to value 0"]
impl crate::Resettable for NMISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
