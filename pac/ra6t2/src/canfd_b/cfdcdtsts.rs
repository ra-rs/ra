#[doc = "Register `CFDCDTSTS` reader"]
pub struct R(crate::R<CFDCDTSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDCDTSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDCDTSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDCDTSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RFDMASTS0` reader - DMA Transfer Status for RX FIFO 0"]
pub type RFDMASTS0_R = crate::BitReader<RFDMASTS0_A>;
#[doc = "DMA Transfer Status for RX FIFO 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFDMASTS0_A {
    #[doc = "0: DMA transfer stopped"]
    _0 = 0,
    #[doc = "1: DMA transfer on going"]
    _1 = 1,
}
impl From<RFDMASTS0_A> for bool {
    #[inline(always)]
    fn from(variant: RFDMASTS0_A) -> Self {
        variant as u8 != 0
    }
}
impl RFDMASTS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFDMASTS0_A {
        match self.bits {
            false => RFDMASTS0_A::_0,
            true => RFDMASTS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFDMASTS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFDMASTS0_A::_1
    }
}
#[doc = "Field `RFDMASTS1` reader - DMA Transfer Status for RX FIFO 1"]
pub type RFDMASTS1_R = crate::BitReader<RFDMASTS1_A>;
#[doc = "DMA Transfer Status for RX FIFO 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFDMASTS1_A {
    #[doc = "0: DMA transfer stopped"]
    _0 = 0,
    #[doc = "1: DMA transfer on going"]
    _1 = 1,
}
impl From<RFDMASTS1_A> for bool {
    #[inline(always)]
    fn from(variant: RFDMASTS1_A) -> Self {
        variant as u8 != 0
    }
}
impl RFDMASTS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFDMASTS1_A {
        match self.bits {
            false => RFDMASTS1_A::_0,
            true => RFDMASTS1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFDMASTS1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFDMASTS1_A::_1
    }
}
#[doc = "Field `CFDMASTS` reader - DMA Transfer Status only for Common FIFO"]
pub type CFDMASTS_R = crate::BitReader<CFDMASTS_A>;
#[doc = "DMA Transfer Status only for Common FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFDMASTS_A {
    #[doc = "0: DMA transfer stopped"]
    _0 = 0,
    #[doc = "1: DMA transfer on going"]
    _1 = 1,
}
impl From<CFDMASTS_A> for bool {
    #[inline(always)]
    fn from(variant: CFDMASTS_A) -> Self {
        variant as u8 != 0
    }
}
impl CFDMASTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFDMASTS_A {
        match self.bits {
            false => CFDMASTS_A::_0,
            true => CFDMASTS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFDMASTS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFDMASTS_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - DMA Transfer Status for RX FIFO 0"]
    #[inline(always)]
    pub fn rfdmasts0(&self) -> RFDMASTS0_R {
        RFDMASTS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Transfer Status for RX FIFO 1"]
    #[inline(always)]
    pub fn rfdmasts1(&self) -> RFDMASTS1_R {
        RFDMASTS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA Transfer Status only for Common FIFO"]
    #[inline(always)]
    pub fn cfdmasts(&self) -> CFDMASTS_R {
        CFDMASTS_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "DMA Transfer Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdcdtsts](index.html) module"]
pub struct CFDCDTSTS_SPEC;
impl crate::RegisterSpec for CFDCDTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdcdtsts::R](R) reader structure"]
impl crate::Readable for CFDCDTSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CFDCDTSTS to value 0"]
impl crate::Resettable for CFDCDTSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
