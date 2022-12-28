#[doc = "Register `FRSR` reader"]
pub struct R(crate::R<FRSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DR` reader - Receive Data Ready flag"]
pub type DR_R = crate::BitReader<DR_A>;
#[doc = "Receive Data Ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DR_A {
    #[doc = "0: Receiving is in progress, or no received data has remained in receive-FIFO after normally completed receiving.(receive-FIFO is empty)"]
    _0 = 0,
    #[doc = "1: The following receive data does not come for a fixed period after storing data under the threshold in the receive-FIFO"]
    _1 = 1,
}
impl From<DR_A> for bool {
    #[inline(always)]
    fn from(variant: DR_A) -> Self {
        variant as u8 != 0
    }
}
impl DR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DR_A {
        match self.bits {
            false => DR_A::_0,
            true => DR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DR_A::_1
    }
}
#[doc = "Field `R` reader - Receive-FIFO Data Count"]
pub type R_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PNUM` reader - Parity Error Count"]
pub type PNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FNUM` reader - Framing Error Count"]
pub type FNUM_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Receive Data Ready flag"]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:13 - Receive-FIFO Data Count"]
    #[inline(always)]
    pub fn r(&self) -> R_R {
        R_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Parity Error Count"]
    #[inline(always)]
    pub fn pnum(&self) -> PNUM_R {
        PNUM_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Framing Error Count"]
    #[inline(always)]
    pub fn fnum(&self) -> FNUM_R {
        FNUM_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
#[doc = "FIFO Receive Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frsr](index.html) module"]
pub struct FRSR_SPEC;
impl crate::RegisterSpec for FRSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frsr::R](R) reader structure"]
impl crate::Readable for FRSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FRSR to value 0"]
impl crate::Resettable for FRSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
