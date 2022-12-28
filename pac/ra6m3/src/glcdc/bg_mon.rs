#[doc = "Register `BG_MON` reader"]
pub struct R(crate::R<BG_MON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BG_MON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BG_MON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BG_MON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EN` reader - Background plane generation module operation state monitor."]
pub type EN_R = crate::BitReader<EN_A>;
#[doc = "Background plane generation module operation state monitor.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_A {
    #[doc = "1: Operation is in progress."]
    _1 = 1,
    #[doc = "0: Operation is stopped."]
    _0 = 0,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            true => EN_A::_1,
            false => EN_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EN_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EN_A::_0
    }
}
#[doc = "Field `VEN` reader - Entire module internal operation reflection control signal monitor.The signal state for controlling reflection of the register values to the internal operations upon assertion of the vertical synchronization signal."]
pub type VEN_R = crate::BitReader<VEN_A>;
#[doc = "Entire module internal operation reflection control signal monitor.The signal state for controlling reflection of the register values to the internal operations upon assertion of the vertical synchronization signal.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VEN_A {
    #[doc = "1: The signal for controlling reflection of the register values to the internal operations upon assertion of the vertical synchronization signal is asserted."]
    _1 = 1,
    #[doc = "0: The signal for controlling reflection of the register values to the internal operations upon assertion of the vertical synchronization signal is negated."]
    _0 = 0,
}
impl From<VEN_A> for bool {
    #[inline(always)]
    fn from(variant: VEN_A) -> Self {
        variant as u8 != 0
    }
}
impl VEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VEN_A {
        match self.bits {
            true => VEN_A::_1,
            false => VEN_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VEN_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VEN_A::_0
    }
}
#[doc = "Field `SWRST` reader - Entire module SW reset state monitor."]
pub type SWRST_R = crate::BitReader<SWRST_A>;
#[doc = "Entire module SW reset state monitor.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWRST_A {
    #[doc = "1: The entire module is released from the SW reset state."]
    _1 = 1,
    #[doc = "0: The entire module is in the SW reset state."]
    _0 = 0,
}
impl From<SWRST_A> for bool {
    #[inline(always)]
    fn from(variant: SWRST_A) -> Self {
        variant as u8 != 0
    }
}
impl SWRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWRST_A {
        match self.bits {
            true => SWRST_A::_1,
            false => SWRST_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWRST_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWRST_A::_0
    }
}
impl R {
    #[doc = "Bit 0 - Background plane generation module operation state monitor."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Entire module internal operation reflection control signal monitor.The signal state for controlling reflection of the register values to the internal operations upon assertion of the vertical synchronization signal."]
    #[inline(always)]
    pub fn ven(&self) -> VEN_R {
        VEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Entire module SW reset state monitor."]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Background Plane Setting Status Monitor Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bg_mon](index.html) module"]
pub struct BG_MON_SPEC;
impl crate::RegisterSpec for BG_MON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bg_mon::R](R) reader structure"]
impl crate::Readable for BG_MON_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BG_MON to value 0"]
impl crate::Resettable for BG_MON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
