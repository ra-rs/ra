#[doc = "Register `SPCR2` reader"]
pub struct R(crate::R<SPCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPCR2` writer"]
pub struct W(crate::W<SPCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPCR2_SPEC>;
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
impl From<crate::W<SPCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPPE` reader - Parity Enable"]
pub type SPPE_R = crate::BitReader<SPPE_A>;
#[doc = "Parity Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPPE_A {
    #[doc = "0: Do not add parity bit to transmit data and do not check parity bit of receive data"]
    _0 = 0,
    #[doc = "1: When SPCR.TXMD = 0: Add parity bit to transmit data and check parity bit of receive data When SPCR.TXMD = 1: Add parity bit to transmit data but do not check parity bit of receive data"]
    _1 = 1,
}
impl From<SPPE_A> for bool {
    #[inline(always)]
    fn from(variant: SPPE_A) -> Self {
        variant as u8 != 0
    }
}
impl SPPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPPE_A {
        match self.bits {
            false => SPPE_A::_0,
            true => SPPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPPE_A::_1
    }
}
#[doc = "Field `SPPE` writer - Parity Enable"]
pub type SPPE_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPCR2_SPEC, SPPE_A, O>;
impl<'a, const O: u8> SPPE_W<'a, O> {
    #[doc = "Do not add parity bit to transmit data and do not check parity bit of receive data"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPPE_A::_0)
    }
    #[doc = "When SPCR.TXMD = 0: Add parity bit to transmit data and check parity bit of receive data When SPCR.TXMD = 1: Add parity bit to transmit data but do not check parity bit of receive data"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPPE_A::_1)
    }
}
#[doc = "Field `SPOE` reader - Parity Mode"]
pub type SPOE_R = crate::BitReader<SPOE_A>;
#[doc = "Parity Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPOE_A {
    #[doc = "0: Select even parity for transmission and reception"]
    _0 = 0,
    #[doc = "1: Select odd parity for transmission and reception"]
    _1 = 1,
}
impl From<SPOE_A> for bool {
    #[inline(always)]
    fn from(variant: SPOE_A) -> Self {
        variant as u8 != 0
    }
}
impl SPOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPOE_A {
        match self.bits {
            false => SPOE_A::_0,
            true => SPOE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPOE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPOE_A::_1
    }
}
#[doc = "Field `SPOE` writer - Parity Mode"]
pub type SPOE_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPCR2_SPEC, SPOE_A, O>;
impl<'a, const O: u8> SPOE_W<'a, O> {
    #[doc = "Select even parity for transmission and reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPOE_A::_0)
    }
    #[doc = "Select odd parity for transmission and reception"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPOE_A::_1)
    }
}
#[doc = "Field `SPIIE` reader - SPI Idle Interrupt Enable"]
pub type SPIIE_R = crate::BitReader<SPIIE_A>;
#[doc = "SPI Idle Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIIE_A {
    #[doc = "0: Disable idle interrupt requests"]
    _0 = 0,
    #[doc = "1: Enable idle interrupt requests"]
    _1 = 1,
}
impl From<SPIIE_A> for bool {
    #[inline(always)]
    fn from(variant: SPIIE_A) -> Self {
        variant as u8 != 0
    }
}
impl SPIIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPIIE_A {
        match self.bits {
            false => SPIIE_A::_0,
            true => SPIIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPIIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPIIE_A::_1
    }
}
#[doc = "Field `SPIIE` writer - SPI Idle Interrupt Enable"]
pub type SPIIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPCR2_SPEC, SPIIE_A, O>;
impl<'a, const O: u8> SPIIE_W<'a, O> {
    #[doc = "Disable idle interrupt requests"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPIIE_A::_0)
    }
    #[doc = "Enable idle interrupt requests"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPIIE_A::_1)
    }
}
#[doc = "Field `PTE` reader - Parity Self-Testing"]
pub type PTE_R = crate::BitReader<PTE_A>;
#[doc = "Parity Self-Testing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTE_A {
    #[doc = "0: Disable self-diagnosis function of the parity circuit"]
    _0 = 0,
    #[doc = "1: Enable self-diagnosis function of the parity circuit"]
    _1 = 1,
}
impl From<PTE_A> for bool {
    #[inline(always)]
    fn from(variant: PTE_A) -> Self {
        variant as u8 != 0
    }
}
impl PTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTE_A {
        match self.bits {
            false => PTE_A::_0,
            true => PTE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTE_A::_1
    }
}
#[doc = "Field `PTE` writer - Parity Self-Testing"]
pub type PTE_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPCR2_SPEC, PTE_A, O>;
impl<'a, const O: u8> PTE_W<'a, O> {
    #[doc = "Disable self-diagnosis function of the parity circuit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTE_A::_0)
    }
    #[doc = "Enable self-diagnosis function of the parity circuit"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTE_A::_1)
    }
}
#[doc = "Field `SCKASE` reader - RSPCK Auto-Stop Function Enable"]
pub type SCKASE_R = crate::BitReader<SCKASE_A>;
#[doc = "RSPCK Auto-Stop Function Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCKASE_A {
    #[doc = "0: Disable RSPCK auto-stop function"]
    _0 = 0,
    #[doc = "1: Enable RSPCK auto-stop function"]
    _1 = 1,
}
impl From<SCKASE_A> for bool {
    #[inline(always)]
    fn from(variant: SCKASE_A) -> Self {
        variant as u8 != 0
    }
}
impl SCKASE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCKASE_A {
        match self.bits {
            false => SCKASE_A::_0,
            true => SCKASE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SCKASE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SCKASE_A::_1
    }
}
#[doc = "Field `SCKASE` writer - RSPCK Auto-Stop Function Enable"]
pub type SCKASE_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPCR2_SPEC, SCKASE_A, O>;
impl<'a, const O: u8> SCKASE_W<'a, O> {
    #[doc = "Disable RSPCK auto-stop function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SCKASE_A::_0)
    }
    #[doc = "Enable RSPCK auto-stop function"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SCKASE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Parity Enable"]
    #[inline(always)]
    pub fn sppe(&self) -> SPPE_R {
        SPPE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parity Mode"]
    #[inline(always)]
    pub fn spoe(&self) -> SPOE_R {
        SPOE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SPI Idle Interrupt Enable"]
    #[inline(always)]
    pub fn spiie(&self) -> SPIIE_R {
        SPIIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Parity Self-Testing"]
    #[inline(always)]
    pub fn pte(&self) -> PTE_R {
        PTE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RSPCK Auto-Stop Function Enable"]
    #[inline(always)]
    pub fn sckase(&self) -> SCKASE_R {
        SCKASE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity Enable"]
    #[inline(always)]
    pub fn sppe(&mut self) -> SPPE_W<0> {
        SPPE_W::new(self)
    }
    #[doc = "Bit 1 - Parity Mode"]
    #[inline(always)]
    pub fn spoe(&mut self) -> SPOE_W<1> {
        SPOE_W::new(self)
    }
    #[doc = "Bit 2 - SPI Idle Interrupt Enable"]
    #[inline(always)]
    pub fn spiie(&mut self) -> SPIIE_W<2> {
        SPIIE_W::new(self)
    }
    #[doc = "Bit 3 - Parity Self-Testing"]
    #[inline(always)]
    pub fn pte(&mut self) -> PTE_W<3> {
        PTE_W::new(self)
    }
    #[doc = "Bit 4 - RSPCK Auto-Stop Function Enable"]
    #[inline(always)]
    pub fn sckase(&mut self) -> SCKASE_W<4> {
        SCKASE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spcr2](index.html) module"]
pub struct SPCR2_SPEC;
impl crate::RegisterSpec for SPCR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [spcr2::R](R) reader structure"]
impl crate::Readable for SPCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spcr2::W](W) writer structure"]
impl crate::Writable for SPCR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPCR2 to value 0"]
impl crate::Resettable for SPCR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
