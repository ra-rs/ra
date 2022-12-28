#[doc = "Register `MSR` reader"]
pub struct R(crate::R<MSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PFER` reader - Preface Error flag"]
pub type PFER_R = crate::BitReader<PFER_A>;
#[doc = "Preface Error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PFER_A {
    #[doc = "0: No preface error detected"]
    _0 = 0,
    #[doc = "1: Preface error detected"]
    _1 = 1,
}
impl From<PFER_A> for bool {
    #[inline(always)]
    fn from(variant: PFER_A) -> Self {
        variant as u8 != 0
    }
}
impl PFER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFER_A {
        match self.bits {
            false => PFER_A::_0,
            true => PFER_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PFER_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PFER_A::_1
    }
}
#[doc = "Field `SYER` reader - SYNC Error flag"]
pub type SYER_R = crate::BitReader<SYER_A>;
#[doc = "SYNC Error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYER_A {
    #[doc = "0: No receive SYNC error detected"]
    _0 = 0,
    #[doc = "1: Receive SYNC error detected"]
    _1 = 1,
}
impl From<SYER_A> for bool {
    #[inline(always)]
    fn from(variant: SYER_A) -> Self {
        variant as u8 != 0
    }
}
impl SYER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYER_A {
        match self.bits {
            false => SYER_A::_0,
            true => SYER_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SYER_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SYER_A::_1
    }
}
#[doc = "Field `SBER` reader - Start Bit Error flag"]
pub type SBER_R = crate::BitReader<SBER_A>;
#[doc = "Start Bit Error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBER_A {
    #[doc = "0: No start bit error detected"]
    _0 = 0,
    #[doc = "1: Start bit error detected"]
    _1 = 1,
}
impl From<SBER_A> for bool {
    #[inline(always)]
    fn from(variant: SBER_A) -> Self {
        variant as u8 != 0
    }
}
impl SBER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBER_A {
        match self.bits {
            false => SBER_A::_0,
            true => SBER_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SBER_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SBER_A::_1
    }
}
#[doc = "Field `MER` reader - Manchester Error Flag"]
pub type MER_R = crate::BitReader<MER_A>;
#[doc = "Manchester Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MER_A {
    #[doc = "0: No Manchester error occurred"]
    _0 = 0,
    #[doc = "1: Manchester error has occurred"]
    _1 = 1,
}
impl From<MER_A> for bool {
    #[inline(always)]
    fn from(variant: MER_A) -> Self {
        variant as u8 != 0
    }
}
impl MER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MER_A {
        match self.bits {
            false => MER_A::_0,
            true => MER_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MER_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MER_A::_1
    }
}
#[doc = "Field `RSYNC` reader - Receive SYNC data bit"]
pub type RSYNC_R = crate::BitReader<RSYNC_A>;
#[doc = "Receive SYNC data bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSYNC_A {
    #[doc = "0: The received the Start Bit is DATA SYNC"]
    _0 = 0,
    #[doc = "1: The received the Start Bit is COMMAND SYNC"]
    _1 = 1,
}
impl From<RSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: RSYNC_A) -> Self {
        variant as u8 != 0
    }
}
impl RSYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSYNC_A {
        match self.bits {
            false => RSYNC_A::_0,
            true => RSYNC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSYNC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSYNC_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Preface Error flag"]
    #[inline(always)]
    pub fn pfer(&self) -> PFER_R {
        PFER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SYNC Error flag"]
    #[inline(always)]
    pub fn syer(&self) -> SYER_R {
        SYER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Start Bit Error flag"]
    #[inline(always)]
    pub fn sber(&self) -> SBER_R {
        SBER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Manchester Error Flag"]
    #[inline(always)]
    pub fn mer(&self) -> MER_R {
        MER_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive SYNC data bit"]
    #[inline(always)]
    pub fn rsync(&self) -> RSYNC_R {
        RSYNC_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Manchester Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msr](index.html) module"]
pub struct MSR_SPEC;
impl crate::RegisterSpec for MSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msr::R](R) reader structure"]
impl crate::Readable for MSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MSR to value 0"]
impl crate::Resettable for MSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
