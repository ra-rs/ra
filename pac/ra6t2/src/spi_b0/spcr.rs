#[doc = "Register `SPCR` reader"]
pub struct R(crate::R<SPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPCR` writer"]
pub struct W(crate::W<SPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPCR_SPEC>;
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
impl From<crate::W<SPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPE` reader - SPI Function Enable"]
pub type SPE_R = crate::BitReader<SPE_A>;
#[doc = "SPI Function Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPE_A {
    #[doc = "0: SPI function is disabled."]
    _0 = 0,
    #[doc = "1: SPI function is enabled."]
    _1 = 1,
}
impl From<SPE_A> for bool {
    #[inline(always)]
    fn from(variant: SPE_A) -> Self {
        variant as u8 != 0
    }
}
impl SPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPE_A {
        match self.bits {
            false => SPE_A::_0,
            true => SPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPE_A::_1
    }
}
#[doc = "Field `SPE` writer - SPI Function Enable"]
pub type SPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPCR_SPEC, SPE_A, O>;
impl<'a, const O: u8> SPE_W<'a, O> {
    #[doc = "SPI function is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPE_A::_0)
    }
    #[doc = "SPI function is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPE_A::_1)
    }
}
#[doc = "Field `SPPE` reader - Parity Enable"]
pub type SPPE_R = crate::BitReader<SPPE_A>;
#[doc = "Parity Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPPE_A {
    #[doc = "0: A parity bit is not added to transmit data. Received-data parity check is not performed."]
    _0 = 0,
    #[doc = "1: A parity bit is added to transmit data. Received-data parity check is performed."]
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
pub type SPPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPCR_SPEC, SPPE_A, O>;
impl<'a, const O: u8> SPPE_W<'a, O> {
    #[doc = "A parity bit is not added to transmit data. Received-data parity check is not performed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPPE_A::_0)
    }
    #[doc = "A parity bit is added to transmit data. Received-data parity check is performed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPPE_A::_1)
    }
}
#[doc = "Field `SPOE` reader - Parity Mode"]
pub type SPOE_R = crate::BitReader<SPOE_A>;
#[doc = "Parity Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPOE_A {
    #[doc = "0: Even parity is used for transmission and reception."]
    _0 = 0,
    #[doc = "1: Odd parity is used for transmission and reception."]
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
pub type SPOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPCR_SPEC, SPOE_A, O>;
impl<'a, const O: u8> SPOE_W<'a, O> {
    #[doc = "Even parity is used for transmission and reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPOE_A::_0)
    }
    #[doc = "Odd parity is used for transmission and reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPOE_A::_1)
    }
}
#[doc = "Field `PTE` reader - Parity Self-Diagnosis Enable"]
pub type PTE_R = crate::BitReader<PTE_A>;
#[doc = "Parity Self-Diagnosis Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTE_A {
    #[doc = "0: Parity circuit self-diagnosis function is disabled."]
    _0 = 0,
    #[doc = "1: Parity circuit self-diagnosis function is enabled."]
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
#[doc = "Field `PTE` writer - Parity Self-Diagnosis Enable"]
pub type PTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPCR_SPEC, PTE_A, O>;
impl<'a, const O: u8> PTE_W<'a, O> {
    #[doc = "Parity circuit self-diagnosis function is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTE_A::_0)
    }
    #[doc = "Parity circuit self-diagnosis function is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTE_A::_1)
    }
}
#[doc = "Field `SCKASE` reader - RSPCK Auto-Stop Function Enable"]
pub type SCKASE_R = crate::BitReader<SCKASE_A>;
#[doc = "RSPCK Auto-Stop Function Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCKASE_A {
    #[doc = "0: RSPCK auto-stop function is disabled."]
    _0 = 0,
    #[doc = "1: RSPCK auto-stop function is enabled."]
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
pub type SCKASE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPCR_SPEC, SCKASE_A, O>;
impl<'a, const O: u8> SCKASE_W<'a, O> {
    #[doc = "RSPCK auto-stop function is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SCKASE_A::_0)
    }
    #[doc = "RSPCK auto-stop function is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SCKASE_A::_1)
    }
}
#[doc = "Field `BFDS` reader - Between Burst Transfer Frames Delay Select"]
pub type BFDS_R = crate::BitReader<BFDS_A>;
#[doc = "Between Burst Transfer Frames Delay Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFDS_A {
    #[doc = "0: Delay (RSPCK delay, SSL negation delay and next-access delay) between frames is inserted in burst transfer"]
    _0 = 0,
    #[doc = "1: Delay between frames is not inserted in burst transfer."]
    _1 = 1,
}
impl From<BFDS_A> for bool {
    #[inline(always)]
    fn from(variant: BFDS_A) -> Self {
        variant as u8 != 0
    }
}
impl BFDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFDS_A {
        match self.bits {
            false => BFDS_A::_0,
            true => BFDS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BFDS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BFDS_A::_1
    }
}
#[doc = "Field `BFDS` writer - Between Burst Transfer Frames Delay Select"]
pub type BFDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPCR_SPEC, BFDS_A, O>;
impl<'a, const O: u8> BFDS_W<'a, O> {
    #[doc = "Delay (RSPCK delay, SSL negation delay and next-access delay) between frames is inserted in burst transfer"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BFDS_A::_0)
    }
    #[doc = "Delay between frames is not inserted in burst transfer."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BFDS_A::_1)
    }
}
#[doc = "Field `MODFEN` reader - Mode Fault Error Detection Enable"]
pub type MODFEN_R = crate::BitReader<MODFEN_A>;
#[doc = "Mode Fault Error Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODFEN_A {
    #[doc = "0: Mode fault error detection is disabled."]
    _0 = 0,
    #[doc = "1: Mode fault error detection is enabled."]
    _1 = 1,
}
impl From<MODFEN_A> for bool {
    #[inline(always)]
    fn from(variant: MODFEN_A) -> Self {
        variant as u8 != 0
    }
}
impl MODFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODFEN_A {
        match self.bits {
            false => MODFEN_A::_0,
            true => MODFEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MODFEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MODFEN_A::_1
    }
}
#[doc = "Field `MODFEN` writer - Mode Fault Error Detection Enable"]
pub type MODFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPCR_SPEC, MODFEN_A, O>;
impl<'a, const O: u8> MODFEN_W<'a, O> {
    #[doc = "Mode fault error detection is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MODFEN_A::_0)
    }
    #[doc = "Mode fault error detection is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MODFEN_A::_1)
    }
}
#[doc = "Field `SPEIE` reader - SPI Error Interrupt Enable"]
pub type SPEIE_R = crate::BitReader<SPEIE_A>;
#[doc = "SPI Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPEIE_A {
    #[doc = "0: SPI error interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: SPI error interrupt request is enabled."]
    _1 = 1,
}
impl From<SPEIE_A> for bool {
    #[inline(always)]
    fn from(variant: SPEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl SPEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPEIE_A {
        match self.bits {
            false => SPEIE_A::_0,
            true => SPEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPEIE_A::_1
    }
}
#[doc = "Field `SPEIE` writer - SPI Error Interrupt Enable"]
pub type SPEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPCR_SPEC, SPEIE_A, O>;
impl<'a, const O: u8> SPEIE_W<'a, O> {
    #[doc = "SPI error interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPEIE_A::_0)
    }
    #[doc = "SPI error interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPEIE_A::_1)
    }
}
#[doc = "Field `SPRIE` reader - SPI Receive Buffer Full Interrupt Enable"]
pub type SPRIE_R = crate::BitReader<SPRIE_A>;
#[doc = "SPI Receive Buffer Full Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPRIE_A {
    #[doc = "0: SPI receive buffer full interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: SPI receive buffer full interrupt request is enabled."]
    _1 = 1,
}
impl From<SPRIE_A> for bool {
    #[inline(always)]
    fn from(variant: SPRIE_A) -> Self {
        variant as u8 != 0
    }
}
impl SPRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPRIE_A {
        match self.bits {
            false => SPRIE_A::_0,
            true => SPRIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPRIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPRIE_A::_1
    }
}
#[doc = "Field `SPRIE` writer - SPI Receive Buffer Full Interrupt Enable"]
pub type SPRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPCR_SPEC, SPRIE_A, O>;
impl<'a, const O: u8> SPRIE_W<'a, O> {
    #[doc = "SPI receive buffer full interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPRIE_A::_0)
    }
    #[doc = "SPI receive buffer full interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPRIE_A::_1)
    }
}
#[doc = "Field `SPIIE` reader - SPI Idle Interrupt Enable"]
pub type SPIIE_R = crate::BitReader<SPIIE_A>;
#[doc = "SPI Idle Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPIIE_A {
    #[doc = "0: Idle interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: Idle interrupt request is enabled."]
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
pub type SPIIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPCR_SPEC, SPIIE_A, O>;
impl<'a, const O: u8> SPIIE_W<'a, O> {
    #[doc = "Idle interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPIIE_A::_0)
    }
    #[doc = "Idle interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPIIE_A::_1)
    }
}
#[doc = "Field `SPDRES` reader - SPI receive data ready error select"]
pub type SPDRES_R = crate::BitReader<SPDRES_A>;
#[doc = "SPI receive data ready error select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPDRES_A {
    #[doc = "0: Receive data full interrupt"]
    _0 = 0,
    #[doc = "1: Error interrupt"]
    _1 = 1,
}
impl From<SPDRES_A> for bool {
    #[inline(always)]
    fn from(variant: SPDRES_A) -> Self {
        variant as u8 != 0
    }
}
impl SPDRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPDRES_A {
        match self.bits {
            false => SPDRES_A::_0,
            true => SPDRES_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPDRES_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPDRES_A::_1
    }
}
#[doc = "Field `SPDRES` writer - SPI receive data ready error select"]
pub type SPDRES_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPCR_SPEC, SPDRES_A, O>;
impl<'a, const O: u8> SPDRES_W<'a, O> {
    #[doc = "Receive data full interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPDRES_A::_0)
    }
    #[doc = "Error interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPDRES_A::_1)
    }
}
#[doc = "Field `SPTIE` reader - SPI Transmit Buffer Empty Interrupt Enable"]
pub type SPTIE_R = crate::BitReader<SPTIE_A>;
#[doc = "SPI Transmit Buffer Empty Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPTIE_A {
    #[doc = "0: SPI transmit buffer empty interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: SPI transmit buffer empty interrupt request is enabled."]
    _1 = 1,
}
impl From<SPTIE_A> for bool {
    #[inline(always)]
    fn from(variant: SPTIE_A) -> Self {
        variant as u8 != 0
    }
}
impl SPTIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPTIE_A {
        match self.bits {
            false => SPTIE_A::_0,
            true => SPTIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPTIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPTIE_A::_1
    }
}
#[doc = "Field `SPTIE` writer - SPI Transmit Buffer Empty Interrupt Enable"]
pub type SPTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPCR_SPEC, SPTIE_A, O>;
impl<'a, const O: u8> SPTIE_W<'a, O> {
    #[doc = "SPI transmit buffer empty interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPTIE_A::_0)
    }
    #[doc = "SPI transmit buffer empty interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPTIE_A::_1)
    }
}
#[doc = "Field `CENDIE` reader - SPI Communication End Interrupt Enable"]
pub type CENDIE_R = crate::BitReader<CENDIE_A>;
#[doc = "SPI Communication End Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CENDIE_A {
    #[doc = "0: Communication end interrupt request is disabled."]
    _0 = 0,
    #[doc = "1: Communication end interrupt request is enabled."]
    _1 = 1,
}
impl From<CENDIE_A> for bool {
    #[inline(always)]
    fn from(variant: CENDIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CENDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CENDIE_A {
        match self.bits {
            false => CENDIE_A::_0,
            true => CENDIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CENDIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CENDIE_A::_1
    }
}
#[doc = "Field `CENDIE` writer - SPI Communication End Interrupt Enable"]
pub type CENDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPCR_SPEC, CENDIE_A, O>;
impl<'a, const O: u8> CENDIE_W<'a, O> {
    #[doc = "Communication end interrupt request is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CENDIE_A::_0)
    }
    #[doc = "Communication end interrupt request is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CENDIE_A::_1)
    }
}
#[doc = "Field `SPMS` reader - SPI Mode Select"]
pub type SPMS_R = crate::BitReader<SPMS_A>;
#[doc = "SPI Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPMS_A {
    #[doc = "0: SPI operation (4-wire)"]
    _0 = 0,
    #[doc = "1: Clock synchronous operation (3-wire)"]
    _1 = 1,
}
impl From<SPMS_A> for bool {
    #[inline(always)]
    fn from(variant: SPMS_A) -> Self {
        variant as u8 != 0
    }
}
impl SPMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPMS_A {
        match self.bits {
            false => SPMS_A::_0,
            true => SPMS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPMS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPMS_A::_1
    }
}
#[doc = "Field `SPMS` writer - SPI Mode Select"]
pub type SPMS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPCR_SPEC, SPMS_A, O>;
impl<'a, const O: u8> SPMS_W<'a, O> {
    #[doc = "SPI operation (4-wire)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPMS_A::_0)
    }
    #[doc = "Clock synchronous operation (3-wire)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPMS_A::_1)
    }
}
#[doc = "Field `SPFRF` reader - SPI Frame Format Select"]
pub type SPFRF_R = crate::BitReader<SPFRF_A>;
#[doc = "SPI Frame Format Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPFRF_A {
    #[doc = "0: Motorola-SPI"]
    _0 = 0,
    #[doc = "1: TI-SSP"]
    _1 = 1,
}
impl From<SPFRF_A> for bool {
    #[inline(always)]
    fn from(variant: SPFRF_A) -> Self {
        variant as u8 != 0
    }
}
impl SPFRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPFRF_A {
        match self.bits {
            false => SPFRF_A::_0,
            true => SPFRF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPFRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPFRF_A::_1
    }
}
#[doc = "Field `SPFRF` writer - SPI Frame Format Select"]
pub type SPFRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPCR_SPEC, SPFRF_A, O>;
impl<'a, const O: u8> SPFRF_W<'a, O> {
    #[doc = "Motorola-SPI"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPFRF_A::_0)
    }
    #[doc = "TI-SSP"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPFRF_A::_1)
    }
}
#[doc = "Field `TXMD` reader - Communication Mode Select"]
pub type TXMD_R = crate::FieldReader<u8, TXMD_A>;
#[doc = "Communication Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TXMD_A {
    #[doc = "0: Transmit-Receive"]
    _00 = 0,
    #[doc = "1: Transmit only"]
    _01 = 1,
}
impl From<TXMD_A> for u8 {
    #[inline(always)]
    fn from(variant: TXMD_A) -> Self {
        variant as _
    }
}
impl TXMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TXMD_A> {
        match self.bits {
            0 => Some(TXMD_A::_00),
            1 => Some(TXMD_A::_01),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TXMD_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TXMD_A::_01
    }
}
#[doc = "Field `TXMD` writer - Communication Mode Select"]
pub type TXMD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPCR_SPEC, u8, TXMD_A, 2, O>;
impl<'a, const O: u8> TXMD_W<'a, O> {
    #[doc = "Transmit-Receive"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TXMD_A::_00)
    }
    #[doc = "Transmit only"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TXMD_A::_01)
    }
}
#[doc = "Field `MSTR` reader - SPI Master/Slave Mode Select"]
pub type MSTR_R = crate::BitReader<MSTR_A>;
#[doc = "SPI Master/Slave Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTR_A {
    #[doc = "0: Slave mode"]
    _0 = 0,
    #[doc = "1: Master mode"]
    _1 = 1,
}
impl From<MSTR_A> for bool {
    #[inline(always)]
    fn from(variant: MSTR_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTR_A {
        match self.bits {
            false => MSTR_A::_0,
            true => MSTR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTR_A::_1
    }
}
#[doc = "Field `MSTR` writer - SPI Master/Slave Mode Select"]
pub type MSTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPCR_SPEC, MSTR_A, O>;
impl<'a, const O: u8> MSTR_W<'a, O> {
    #[doc = "Slave mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTR_A::_0)
    }
    #[doc = "Master mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTR_A::_1)
    }
}
#[doc = "Field `BPEN` reader - Synchronization Circuit Bypass Enable"]
pub type BPEN_R = crate::BitReader<BPEN_A>;
#[doc = "Synchronization Circuit Bypass Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BPEN_A {
    #[doc = "0: Non-Bypass"]
    _0 = 0,
    #[doc = "1: Bypass"]
    _1 = 1,
}
impl From<BPEN_A> for bool {
    #[inline(always)]
    fn from(variant: BPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl BPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPEN_A {
        match self.bits {
            false => BPEN_A::_0,
            true => BPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPEN_A::_1
    }
}
#[doc = "Field `BPEN` writer - Synchronization Circuit Bypass Enable"]
pub type BPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPCR_SPEC, BPEN_A, O>;
impl<'a, const O: u8> BPEN_W<'a, O> {
    #[doc = "Non-Bypass"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPEN_A::_0)
    }
    #[doc = "Bypass"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - SPI Function Enable"]
    #[inline(always)]
    pub fn spe(&self) -> SPE_R {
        SPE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Parity Enable"]
    #[inline(always)]
    pub fn sppe(&self) -> SPPE_R {
        SPPE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Parity Mode"]
    #[inline(always)]
    pub fn spoe(&self) -> SPOE_R {
        SPOE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Parity Self-Diagnosis Enable"]
    #[inline(always)]
    pub fn pte(&self) -> PTE_R {
        PTE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RSPCK Auto-Stop Function Enable"]
    #[inline(always)]
    pub fn sckase(&self) -> SCKASE_R {
        SCKASE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Between Burst Transfer Frames Delay Select"]
    #[inline(always)]
    pub fn bfds(&self) -> BFDS_R {
        BFDS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Mode Fault Error Detection Enable"]
    #[inline(always)]
    pub fn modfen(&self) -> MODFEN_R {
        MODFEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - SPI Error Interrupt Enable"]
    #[inline(always)]
    pub fn speie(&self) -> SPEIE_R {
        SPEIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SPI Receive Buffer Full Interrupt Enable"]
    #[inline(always)]
    pub fn sprie(&self) -> SPRIE_R {
        SPRIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SPI Idle Interrupt Enable"]
    #[inline(always)]
    pub fn spiie(&self) -> SPIIE_R {
        SPIIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SPI receive data ready error select"]
    #[inline(always)]
    pub fn spdres(&self) -> SPDRES_R {
        SPDRES_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SPI Transmit Buffer Empty Interrupt Enable"]
    #[inline(always)]
    pub fn sptie(&self) -> SPTIE_R {
        SPTIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SPI Communication End Interrupt Enable"]
    #[inline(always)]
    pub fn cendie(&self) -> CENDIE_R {
        CENDIE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - SPI Mode Select"]
    #[inline(always)]
    pub fn spms(&self) -> SPMS_R {
        SPMS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SPI Frame Format Select"]
    #[inline(always)]
    pub fn spfrf(&self) -> SPFRF_R {
        SPFRF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Communication Mode Select"]
    #[inline(always)]
    pub fn txmd(&self) -> TXMD_R {
        TXMD_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - SPI Master/Slave Mode Select"]
    #[inline(always)]
    pub fn mstr(&self) -> MSTR_R {
        MSTR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Synchronization Circuit Bypass Enable"]
    #[inline(always)]
    pub fn bpen(&self) -> BPEN_R {
        BPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI Function Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spe(&mut self) -> SPE_W<0> {
        SPE_W::new(self)
    }
    #[doc = "Bit 8 - Parity Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sppe(&mut self) -> SPPE_W<8> {
        SPPE_W::new(self)
    }
    #[doc = "Bit 9 - Parity Mode"]
    #[inline(always)]
    #[must_use]
    pub fn spoe(&mut self) -> SPOE_W<9> {
        SPOE_W::new(self)
    }
    #[doc = "Bit 11 - Parity Self-Diagnosis Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pte(&mut self) -> PTE_W<11> {
        PTE_W::new(self)
    }
    #[doc = "Bit 12 - RSPCK Auto-Stop Function Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sckase(&mut self) -> SCKASE_W<12> {
        SCKASE_W::new(self)
    }
    #[doc = "Bit 13 - Between Burst Transfer Frames Delay Select"]
    #[inline(always)]
    #[must_use]
    pub fn bfds(&mut self) -> BFDS_W<13> {
        BFDS_W::new(self)
    }
    #[doc = "Bit 14 - Mode Fault Error Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn modfen(&mut self) -> MODFEN_W<14> {
        MODFEN_W::new(self)
    }
    #[doc = "Bit 16 - SPI Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn speie(&mut self) -> SPEIE_W<16> {
        SPEIE_W::new(self)
    }
    #[doc = "Bit 17 - SPI Receive Buffer Full Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sprie(&mut self) -> SPRIE_W<17> {
        SPRIE_W::new(self)
    }
    #[doc = "Bit 18 - SPI Idle Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spiie(&mut self) -> SPIIE_W<18> {
        SPIIE_W::new(self)
    }
    #[doc = "Bit 19 - SPI receive data ready error select"]
    #[inline(always)]
    #[must_use]
    pub fn spdres(&mut self) -> SPDRES_W<19> {
        SPDRES_W::new(self)
    }
    #[doc = "Bit 20 - SPI Transmit Buffer Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sptie(&mut self) -> SPTIE_W<20> {
        SPTIE_W::new(self)
    }
    #[doc = "Bit 21 - SPI Communication End Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cendie(&mut self) -> CENDIE_W<21> {
        CENDIE_W::new(self)
    }
    #[doc = "Bit 24 - SPI Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn spms(&mut self) -> SPMS_W<24> {
        SPMS_W::new(self)
    }
    #[doc = "Bit 25 - SPI Frame Format Select"]
    #[inline(always)]
    #[must_use]
    pub fn spfrf(&mut self) -> SPFRF_W<25> {
        SPFRF_W::new(self)
    }
    #[doc = "Bits 28:29 - Communication Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn txmd(&mut self) -> TXMD_W<28> {
        TXMD_W::new(self)
    }
    #[doc = "Bit 30 - SPI Master/Slave Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn mstr(&mut self) -> MSTR_W<30> {
        MSTR_W::new(self)
    }
    #[doc = "Bit 31 - Synchronization Circuit Bypass Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bpen(&mut self) -> BPEN_W<31> {
        BPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spcr](index.html) module"]
pub struct SPCR_SPEC;
impl crate::RegisterSpec for SPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spcr::R](R) reader structure"]
impl crate::Readable for SPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spcr::W](W) writer structure"]
impl crate::Writable for SPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPCR to value 0"]
impl crate::Resettable for SPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
