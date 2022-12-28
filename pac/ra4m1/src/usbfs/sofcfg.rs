#[doc = "Register `SOFCFG` reader"]
pub struct R(crate::R<SOFCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOFCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOFCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOFCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOFCFG` writer"]
pub struct W(crate::W<SOFCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOFCFG_SPEC>;
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
impl From<crate::W<SOFCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOFCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EDGESTS` reader - Edge Interrupt Output Status Monitor"]
pub type EDGESTS_R = crate::BitReader<EDGESTS_A>;
#[doc = "Edge Interrupt Output Status Monitor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGESTS_A {
    #[doc = "0: before stopping the clock supply to the USB module"]
    _0 = 0,
    #[doc = "1: the edge interrupt output signal is in the middle of the edge processing"]
    _1 = 1,
}
impl From<EDGESTS_A> for bool {
    #[inline(always)]
    fn from(variant: EDGESTS_A) -> Self {
        variant as u8 != 0
    }
}
impl EDGESTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGESTS_A {
        match self.bits {
            false => EDGESTS_A::_0,
            true => EDGESTS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EDGESTS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EDGESTS_A::_1
    }
}
#[doc = "Field `BRDYM` reader - BRDY Interrupt Status Clear Timing"]
pub type BRDYM_R = crate::BitReader<BRDYM_A>;
#[doc = "BRDY Interrupt Status Clear Timing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRDYM_A {
    #[doc = "0: BRDY flag cleared by software"]
    _0 = 0,
    #[doc = "1: BRDY flag cleared by the USBFS through a data read from the FIFO buffer or data write to the FIFO buffer."]
    _1 = 1,
}
impl From<BRDYM_A> for bool {
    #[inline(always)]
    fn from(variant: BRDYM_A) -> Self {
        variant as u8 != 0
    }
}
impl BRDYM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRDYM_A {
        match self.bits {
            false => BRDYM_A::_0,
            true => BRDYM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BRDYM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BRDYM_A::_1
    }
}
#[doc = "Field `BRDYM` writer - BRDY Interrupt Status Clear Timing"]
pub type BRDYM_W<'a, const O: u8> = crate::BitWriter<'a, u16, SOFCFG_SPEC, BRDYM_A, O>;
impl<'a, const O: u8> BRDYM_W<'a, O> {
    #[doc = "BRDY flag cleared by software"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRDYM_A::_0)
    }
    #[doc = "BRDY flag cleared by the USBFS through a data read from the FIFO buffer or data write to the FIFO buffer."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRDYM_A::_1)
    }
}
#[doc = "Field `TRNENSEL` reader - Transaction-Enabled Time Select"]
pub type TRNENSEL_R = crate::BitReader<TRNENSEL_A>;
#[doc = "Transaction-Enabled Time Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRNENSEL_A {
    #[doc = "0: Not low-speed communication"]
    _0 = 0,
    #[doc = "1: Low-speed communication."]
    _1 = 1,
}
impl From<TRNENSEL_A> for bool {
    #[inline(always)]
    fn from(variant: TRNENSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl TRNENSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRNENSEL_A {
        match self.bits {
            false => TRNENSEL_A::_0,
            true => TRNENSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRNENSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRNENSEL_A::_1
    }
}
#[doc = "Field `TRNENSEL` writer - Transaction-Enabled Time Select"]
pub type TRNENSEL_W<'a, const O: u8> = crate::BitWriter<'a, u16, SOFCFG_SPEC, TRNENSEL_A, O>;
impl<'a, const O: u8> TRNENSEL_W<'a, O> {
    #[doc = "Not low-speed communication"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRNENSEL_A::_0)
    }
    #[doc = "Low-speed communication."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRNENSEL_A::_1)
    }
}
impl R {
    #[doc = "Bit 4 - Edge Interrupt Output Status Monitor"]
    #[inline(always)]
    pub fn edgests(&self) -> EDGESTS_R {
        EDGESTS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - BRDY Interrupt Status Clear Timing"]
    #[inline(always)]
    pub fn brdym(&self) -> BRDYM_R {
        BRDYM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Transaction-Enabled Time Select"]
    #[inline(always)]
    pub fn trnensel(&self) -> TRNENSEL_R {
        TRNENSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - BRDY Interrupt Status Clear Timing"]
    #[inline(always)]
    #[must_use]
    pub fn brdym(&mut self) -> BRDYM_W<6> {
        BRDYM_W::new(self)
    }
    #[doc = "Bit 8 - Transaction-Enabled Time Select"]
    #[inline(always)]
    #[must_use]
    pub fn trnensel(&mut self) -> TRNENSEL_W<8> {
        TRNENSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SOF Output Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sofcfg](index.html) module"]
pub struct SOFCFG_SPEC;
impl crate::RegisterSpec for SOFCFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sofcfg::R](R) reader structure"]
impl crate::Readable for SOFCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sofcfg::W](W) writer structure"]
impl crate::Writable for SOFCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOFCFG to value 0"]
impl crate::Resettable for SOFCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
