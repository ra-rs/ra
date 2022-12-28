#[doc = "Register `EESR` reader"]
pub struct R(crate::R<EESR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EESR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EESR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EESR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EESR` writer"]
pub struct W(crate::W<EESR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EESR_SPEC>;
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
impl From<crate::W<EESR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EESR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CERF` reader - CRC Error Flag"]
pub type CERF_R = crate::BitReader<CERF_A>;
#[doc = "CRC Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CERF_A {
    #[doc = "0: CRC error not detected"]
    _0 = 0,
    #[doc = "1: CRC error detected."]
    _1 = 1,
}
impl From<CERF_A> for bool {
    #[inline(always)]
    fn from(variant: CERF_A) -> Self {
        variant as u8 != 0
    }
}
impl CERF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CERF_A {
        match self.bits {
            false => CERF_A::_0,
            true => CERF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CERF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CERF_A::_1
    }
}
#[doc = "Field `CERF` writer - CRC Error Flag"]
pub type CERF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESR_SPEC, CERF_A, O>;
impl<'a, const O: u8> CERF_W<'a, O> {
    #[doc = "CRC error not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CERF_A::_0)
    }
    #[doc = "CRC error detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CERF_A::_1)
    }
}
#[doc = "Field `PRE` reader - PHY-LSI Receive Error Flag"]
pub type PRE_R = crate::BitReader<PRE_A>;
#[doc = "PHY-LSI Receive Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRE_A {
    #[doc = "0: PHY-LSI receive error not detected"]
    _0 = 0,
    #[doc = "1: PHY-LSI receive error detected."]
    _1 = 1,
}
impl From<PRE_A> for bool {
    #[inline(always)]
    fn from(variant: PRE_A) -> Self {
        variant as u8 != 0
    }
}
impl PRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRE_A {
        match self.bits {
            false => PRE_A::_0,
            true => PRE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PRE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PRE_A::_1
    }
}
#[doc = "Field `PRE` writer - PHY-LSI Receive Error Flag"]
pub type PRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESR_SPEC, PRE_A, O>;
impl<'a, const O: u8> PRE_W<'a, O> {
    #[doc = "PHY-LSI receive error not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PRE_A::_0)
    }
    #[doc = "PHY-LSI receive error detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PRE_A::_1)
    }
}
#[doc = "Field `RTSF` reader - Frame-Too-Short Error Flag"]
pub type RTSF_R = crate::BitReader<RTSF_A>;
#[doc = "Frame-Too-Short Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTSF_A {
    #[doc = "0: Frame-too-short error not detected"]
    _0 = 0,
    #[doc = "1: Frame-too-short error detected."]
    _1 = 1,
}
impl From<RTSF_A> for bool {
    #[inline(always)]
    fn from(variant: RTSF_A) -> Self {
        variant as u8 != 0
    }
}
impl RTSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTSF_A {
        match self.bits {
            false => RTSF_A::_0,
            true => RTSF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RTSF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTSF_A::_1
    }
}
#[doc = "Field `RTSF` writer - Frame-Too-Short Error Flag"]
pub type RTSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESR_SPEC, RTSF_A, O>;
impl<'a, const O: u8> RTSF_W<'a, O> {
    #[doc = "Frame-too-short error not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTSF_A::_0)
    }
    #[doc = "Frame-too-short error detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTSF_A::_1)
    }
}
#[doc = "Field `RTLF` reader - Frame-Too-Long Error Flag"]
pub type RTLF_R = crate::BitReader<RTLF_A>;
#[doc = "Frame-Too-Long Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTLF_A {
    #[doc = "0: Frame-too-long error not detected"]
    _0 = 0,
    #[doc = "1: Frame-too-long error detected."]
    _1 = 1,
}
impl From<RTLF_A> for bool {
    #[inline(always)]
    fn from(variant: RTLF_A) -> Self {
        variant as u8 != 0
    }
}
impl RTLF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTLF_A {
        match self.bits {
            false => RTLF_A::_0,
            true => RTLF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RTLF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTLF_A::_1
    }
}
#[doc = "Field `RTLF` writer - Frame-Too-Long Error Flag"]
pub type RTLF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESR_SPEC, RTLF_A, O>;
impl<'a, const O: u8> RTLF_W<'a, O> {
    #[doc = "Frame-too-long error not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTLF_A::_0)
    }
    #[doc = "Frame-too-long error detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTLF_A::_1)
    }
}
#[doc = "Field `RRF` reader - Alignment Error Flag"]
pub type RRF_R = crate::BitReader<RRF_A>;
#[doc = "Alignment Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RRF_A {
    #[doc = "0: Alignment error not detected"]
    _0 = 0,
    #[doc = "1: Alignment error detected."]
    _1 = 1,
}
impl From<RRF_A> for bool {
    #[inline(always)]
    fn from(variant: RRF_A) -> Self {
        variant as u8 != 0
    }
}
impl RRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RRF_A {
        match self.bits {
            false => RRF_A::_0,
            true => RRF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RRF_A::_1
    }
}
#[doc = "Field `RRF` writer - Alignment Error Flag"]
pub type RRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESR_SPEC, RRF_A, O>;
impl<'a, const O: u8> RRF_W<'a, O> {
    #[doc = "Alignment error not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RRF_A::_0)
    }
    #[doc = "Alignment error detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RRF_A::_1)
    }
}
#[doc = "Field `RMAF` reader - Multicast Address Frame Receive Flag"]
pub type RMAF_R = crate::BitReader<RMAF_A>;
#[doc = "Multicast Address Frame Receive Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMAF_A {
    #[doc = "0: Multicast address frame not received"]
    _0 = 0,
    #[doc = "1: Multicast address frame received."]
    _1 = 1,
}
impl From<RMAF_A> for bool {
    #[inline(always)]
    fn from(variant: RMAF_A) -> Self {
        variant as u8 != 0
    }
}
impl RMAF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMAF_A {
        match self.bits {
            false => RMAF_A::_0,
            true => RMAF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RMAF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RMAF_A::_1
    }
}
#[doc = "Field `RMAF` writer - Multicast Address Frame Receive Flag"]
pub type RMAF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESR_SPEC, RMAF_A, O>;
impl<'a, const O: u8> RMAF_W<'a, O> {
    #[doc = "Multicast address frame not received"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RMAF_A::_0)
    }
    #[doc = "Multicast address frame received."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RMAF_A::_1)
    }
}
#[doc = "Field `TRO` reader - Transmit Retry Over Flag"]
pub type TRO_R = crate::BitReader<TRO_A>;
#[doc = "Transmit Retry Over Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRO_A {
    #[doc = "0: Transmit retry-over condition not detected"]
    _0 = 0,
    #[doc = "1: Transmit retry-over condition detected."]
    _1 = 1,
}
impl From<TRO_A> for bool {
    #[inline(always)]
    fn from(variant: TRO_A) -> Self {
        variant as u8 != 0
    }
}
impl TRO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRO_A {
        match self.bits {
            false => TRO_A::_0,
            true => TRO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRO_A::_1
    }
}
#[doc = "Field `TRO` writer - Transmit Retry Over Flag"]
pub type TRO_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESR_SPEC, TRO_A, O>;
impl<'a, const O: u8> TRO_W<'a, O> {
    #[doc = "Transmit retry-over condition not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRO_A::_0)
    }
    #[doc = "Transmit retry-over condition detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRO_A::_1)
    }
}
#[doc = "Field `CD` reader - Late Collision Detect Flag"]
pub type CD_R = crate::BitReader<CD_A>;
#[doc = "Late Collision Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CD_A {
    #[doc = "0: Late collision not detected"]
    _0 = 0,
    #[doc = "1: Late collision detected during frame transmission."]
    _1 = 1,
}
impl From<CD_A> for bool {
    #[inline(always)]
    fn from(variant: CD_A) -> Self {
        variant as u8 != 0
    }
}
impl CD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CD_A {
        match self.bits {
            false => CD_A::_0,
            true => CD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CD_A::_1
    }
}
#[doc = "Field `CD` writer - Late Collision Detect Flag"]
pub type CD_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESR_SPEC, CD_A, O>;
impl<'a, const O: u8> CD_W<'a, O> {
    #[doc = "Late collision not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CD_A::_0)
    }
    #[doc = "Late collision detected during frame transmission."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CD_A::_1)
    }
}
#[doc = "Field `DLC` reader - Loss of Carrier Detect Flag"]
pub type DLC_R = crate::BitReader<DLC_A>;
#[doc = "Loss of Carrier Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLC_A {
    #[doc = "0: Loss of carrier not detected"]
    _0 = 0,
    #[doc = "1: Loss of carrier detected during frame transmission."]
    _1 = 1,
}
impl From<DLC_A> for bool {
    #[inline(always)]
    fn from(variant: DLC_A) -> Self {
        variant as u8 != 0
    }
}
impl DLC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLC_A {
        match self.bits {
            false => DLC_A::_0,
            true => DLC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLC_A::_1
    }
}
#[doc = "Field `DLC` writer - Loss of Carrier Detect Flag"]
pub type DLC_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESR_SPEC, DLC_A, O>;
impl<'a, const O: u8> DLC_W<'a, O> {
    #[doc = "Loss of carrier not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DLC_A::_0)
    }
    #[doc = "Loss of carrier detected during frame transmission."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DLC_A::_1)
    }
}
#[doc = "Field `CND` reader - Carrier Not Detect Flag"]
pub type CND_R = crate::BitReader<CND_A>;
#[doc = "Carrier Not Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CND_A {
    #[doc = "0: Carrier detected when transmission started"]
    _0 = 0,
    #[doc = "1: Carrier not detected during preamble transmission."]
    _1 = 1,
}
impl From<CND_A> for bool {
    #[inline(always)]
    fn from(variant: CND_A) -> Self {
        variant as u8 != 0
    }
}
impl CND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CND_A {
        match self.bits {
            false => CND_A::_0,
            true => CND_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CND_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CND_A::_1
    }
}
#[doc = "Field `CND` writer - Carrier Not Detect Flag"]
pub type CND_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESR_SPEC, CND_A, O>;
impl<'a, const O: u8> CND_W<'a, O> {
    #[doc = "Carrier detected when transmission started"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CND_A::_0)
    }
    #[doc = "Carrier not detected during preamble transmission."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CND_A::_1)
    }
}
#[doc = "Field `RFOF` reader - Receive FIFO Overflow Flag"]
pub type RFOF_R = crate::BitReader<RFOF_A>;
#[doc = "Receive FIFO Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFOF_A {
    #[doc = "0: No overflow occurred"]
    _0 = 0,
    #[doc = "1: Overflow occurred."]
    _1 = 1,
}
impl From<RFOF_A> for bool {
    #[inline(always)]
    fn from(variant: RFOF_A) -> Self {
        variant as u8 != 0
    }
}
impl RFOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFOF_A {
        match self.bits {
            false => RFOF_A::_0,
            true => RFOF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFOF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFOF_A::_1
    }
}
#[doc = "Field `RFOF` writer - Receive FIFO Overflow Flag"]
pub type RFOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESR_SPEC, RFOF_A, O>;
impl<'a, const O: u8> RFOF_W<'a, O> {
    #[doc = "No overflow occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFOF_A::_0)
    }
    #[doc = "Overflow occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFOF_A::_1)
    }
}
#[doc = "Field `RDE` reader - Receive Descriptor Empty Flag"]
pub type RDE_R = crate::BitReader<RDE_A>;
#[doc = "Receive Descriptor Empty Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDE_A {
    #[doc = "0: EDMAC detected that the receive descriptor valid bit (RD0.RACT) is 1"]
    _0 = 0,
    #[doc = "1: EDMAC detected that the receive descriptor valid bit (RD0.RACT) is 0."]
    _1 = 1,
}
impl From<RDE_A> for bool {
    #[inline(always)]
    fn from(variant: RDE_A) -> Self {
        variant as u8 != 0
    }
}
impl RDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDE_A {
        match self.bits {
            false => RDE_A::_0,
            true => RDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDE_A::_1
    }
}
#[doc = "Field `RDE` writer - Receive Descriptor Empty Flag"]
pub type RDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESR_SPEC, RDE_A, O>;
impl<'a, const O: u8> RDE_W<'a, O> {
    #[doc = "EDMAC detected that the receive descriptor valid bit (RD0.RACT) is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RDE_A::_0)
    }
    #[doc = "EDMAC detected that the receive descriptor valid bit (RD0.RACT) is 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RDE_A::_1)
    }
}
#[doc = "Field `FR` reader - Frame Receive Flag"]
pub type FR_R = crate::BitReader<FR_A>;
#[doc = "Frame Receive Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FR_A {
    #[doc = "0: Frame not received"]
    _0 = 0,
    #[doc = "1: Frame received and update of the receive descriptor is complete."]
    _1 = 1,
}
impl From<FR_A> for bool {
    #[inline(always)]
    fn from(variant: FR_A) -> Self {
        variant as u8 != 0
    }
}
impl FR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FR_A {
        match self.bits {
            false => FR_A::_0,
            true => FR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FR_A::_1
    }
}
#[doc = "Field `FR` writer - Frame Receive Flag"]
pub type FR_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESR_SPEC, FR_A, O>;
impl<'a, const O: u8> FR_W<'a, O> {
    #[doc = "Frame not received"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FR_A::_0)
    }
    #[doc = "Frame received and update of the receive descriptor is complete."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FR_A::_1)
    }
}
#[doc = "Field `TFUF` reader - Transmit FIFO Underflow Flag"]
pub type TFUF_R = crate::BitReader<TFUF_A>;
#[doc = "Transmit FIFO Underflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFUF_A {
    #[doc = "0: No underflow occurred"]
    _0 = 0,
    #[doc = "1: Underflow occurred."]
    _1 = 1,
}
impl From<TFUF_A> for bool {
    #[inline(always)]
    fn from(variant: TFUF_A) -> Self {
        variant as u8 != 0
    }
}
impl TFUF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFUF_A {
        match self.bits {
            false => TFUF_A::_0,
            true => TFUF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TFUF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TFUF_A::_1
    }
}
#[doc = "Field `TFUF` writer - Transmit FIFO Underflow Flag"]
pub type TFUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESR_SPEC, TFUF_A, O>;
impl<'a, const O: u8> TFUF_W<'a, O> {
    #[doc = "No underflow occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFUF_A::_0)
    }
    #[doc = "Underflow occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFUF_A::_1)
    }
}
#[doc = "Field `TDE` reader - Transmit Descriptor Empty Flag"]
pub type TDE_R = crate::BitReader<TDE_A>;
#[doc = "Transmit Descriptor Empty Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDE_A {
    #[doc = "0: EDMAC detected that the transmit descriptor valid bit (TD0.TACT) is 1"]
    _0 = 0,
    #[doc = "1: EDMAC detected that the transmit descriptor valid bit (TD0.TACT) is 0."]
    _1 = 1,
}
impl From<TDE_A> for bool {
    #[inline(always)]
    fn from(variant: TDE_A) -> Self {
        variant as u8 != 0
    }
}
impl TDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDE_A {
        match self.bits {
            false => TDE_A::_0,
            true => TDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDE_A::_1
    }
}
#[doc = "Field `TDE` writer - Transmit Descriptor Empty Flag"]
pub type TDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESR_SPEC, TDE_A, O>;
impl<'a, const O: u8> TDE_W<'a, O> {
    #[doc = "EDMAC detected that the transmit descriptor valid bit (TD0.TACT) is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDE_A::_0)
    }
    #[doc = "EDMAC detected that the transmit descriptor valid bit (TD0.TACT) is 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDE_A::_1)
    }
}
#[doc = "Field `TC` reader - Frame Transfer Complete Flag"]
pub type TC_R = crate::BitReader<TC_A>;
#[doc = "Frame Transfer Complete Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TC_A {
    #[doc = "0: Transfer not complete or no transfer requested"]
    _0 = 0,
    #[doc = "1: All frames indicated in the transmit descriptor were completely transferred to the transmit FIFO."]
    _1 = 1,
}
impl From<TC_A> for bool {
    #[inline(always)]
    fn from(variant: TC_A) -> Self {
        variant as u8 != 0
    }
}
impl TC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TC_A {
        match self.bits {
            false => TC_A::_0,
            true => TC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TC_A::_1
    }
}
#[doc = "Field `TC` writer - Frame Transfer Complete Flag"]
pub type TC_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESR_SPEC, TC_A, O>;
impl<'a, const O: u8> TC_W<'a, O> {
    #[doc = "Transfer not complete or no transfer requested"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TC_A::_0)
    }
    #[doc = "All frames indicated in the transmit descriptor were completely transferred to the transmit FIFO."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TC_A::_1)
    }
}
#[doc = "Field `ECI` reader - ETHERC Status Register Source Flag"]
pub type ECI_R = crate::BitReader<ECI_A>;
#[doc = "ETHERC Status Register Source Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECI_A {
    #[doc = "0: ETHERC status interrupt source not detected"]
    _0 = 0,
    #[doc = "1: ETHERC status interrupt source detected."]
    _1 = 1,
}
impl From<ECI_A> for bool {
    #[inline(always)]
    fn from(variant: ECI_A) -> Self {
        variant as u8 != 0
    }
}
impl ECI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECI_A {
        match self.bits {
            false => ECI_A::_0,
            true => ECI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ECI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ECI_A::_1
    }
}
#[doc = "Field `ADE` reader - Address Error Flag"]
pub type ADE_R = crate::BitReader<ADE_A>;
#[doc = "Address Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADE_A {
    #[doc = "0: Invalid memory address not detected (normal operation)"]
    _0 = 0,
    #[doc = "1: Invalid memory address detected."]
    _1 = 1,
}
impl From<ADE_A> for bool {
    #[inline(always)]
    fn from(variant: ADE_A) -> Self {
        variant as u8 != 0
    }
}
impl ADE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADE_A {
        match self.bits {
            false => ADE_A::_0,
            true => ADE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADE_A::_1
    }
}
#[doc = "Field `ADE` writer - Address Error Flag"]
pub type ADE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESR_SPEC, ADE_A, O>;
impl<'a, const O: u8> ADE_W<'a, O> {
    #[doc = "Invalid memory address not detected (normal operation)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADE_A::_0)
    }
    #[doc = "Invalid memory address detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADE_A::_1)
    }
}
#[doc = "Field `RFCOF` reader - Receive Frame Counter Overflow Flag"]
pub type RFCOF_R = crate::BitReader<RFCOF_A>;
#[doc = "Receive Frame Counter Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFCOF_A {
    #[doc = "0: Receive frame counter did not overflow"]
    _0 = 0,
    #[doc = "1: Receive frame counter overflowed."]
    _1 = 1,
}
impl From<RFCOF_A> for bool {
    #[inline(always)]
    fn from(variant: RFCOF_A) -> Self {
        variant as u8 != 0
    }
}
impl RFCOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFCOF_A {
        match self.bits {
            false => RFCOF_A::_0,
            true => RFCOF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFCOF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFCOF_A::_1
    }
}
#[doc = "Field `RFCOF` writer - Receive Frame Counter Overflow Flag"]
pub type RFCOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESR_SPEC, RFCOF_A, O>;
impl<'a, const O: u8> RFCOF_W<'a, O> {
    #[doc = "Receive frame counter did not overflow"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFCOF_A::_0)
    }
    #[doc = "Receive frame counter overflowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFCOF_A::_1)
    }
}
#[doc = "Field `RABT` reader - Receive Abort Detect Flag"]
pub type RABT_R = crate::BitReader<RABT_A>;
#[doc = "Receive Abort Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RABT_A {
    #[doc = "0: Frame reception not aborted or no reception requested"]
    _0 = 0,
    #[doc = "1: Frame reception aborted."]
    _1 = 1,
}
impl From<RABT_A> for bool {
    #[inline(always)]
    fn from(variant: RABT_A) -> Self {
        variant as u8 != 0
    }
}
impl RABT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RABT_A {
        match self.bits {
            false => RABT_A::_0,
            true => RABT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RABT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RABT_A::_1
    }
}
#[doc = "Field `RABT` writer - Receive Abort Detect Flag"]
pub type RABT_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESR_SPEC, RABT_A, O>;
impl<'a, const O: u8> RABT_W<'a, O> {
    #[doc = "Frame reception not aborted or no reception requested"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RABT_A::_0)
    }
    #[doc = "Frame reception aborted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RABT_A::_1)
    }
}
#[doc = "Field `TABT` reader - Transmit Abort Detect Flag"]
pub type TABT_R = crate::BitReader<TABT_A>;
#[doc = "Transmit Abort Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TABT_A {
    #[doc = "0: Frame transmission not aborted or no transmission requested."]
    _0 = 0,
    #[doc = "1: Frame transmission aborted."]
    _1 = 1,
}
impl From<TABT_A> for bool {
    #[inline(always)]
    fn from(variant: TABT_A) -> Self {
        variant as u8 != 0
    }
}
impl TABT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TABT_A {
        match self.bits {
            false => TABT_A::_0,
            true => TABT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TABT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TABT_A::_1
    }
}
#[doc = "Field `TABT` writer - Transmit Abort Detect Flag"]
pub type TABT_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESR_SPEC, TABT_A, O>;
impl<'a, const O: u8> TABT_W<'a, O> {
    #[doc = "Frame transmission not aborted or no transmission requested."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TABT_A::_0)
    }
    #[doc = "Frame transmission aborted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TABT_A::_1)
    }
}
#[doc = "Field `TWB` reader - Write-Back Complete Flag"]
pub type TWB_R = crate::BitReader<TWB_A>;
#[doc = "Write-Back Complete Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWB_A {
    #[doc = "0: Write-back not complete or no transmission requested"]
    _0 = 0,
    #[doc = "1: Write-back to the transmit descriptor completed."]
    _1 = 1,
}
impl From<TWB_A> for bool {
    #[inline(always)]
    fn from(variant: TWB_A) -> Self {
        variant as u8 != 0
    }
}
impl TWB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWB_A {
        match self.bits {
            false => TWB_A::_0,
            true => TWB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TWB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TWB_A::_1
    }
}
#[doc = "Field `TWB` writer - Write-Back Complete Flag"]
pub type TWB_W<'a, const O: u8> = crate::BitWriter<'a, u32, EESR_SPEC, TWB_A, O>;
impl<'a, const O: u8> TWB_W<'a, O> {
    #[doc = "Write-back not complete or no transmission requested"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWB_A::_0)
    }
    #[doc = "Write-back to the transmit descriptor completed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWB_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - CRC Error Flag"]
    #[inline(always)]
    pub fn cerf(&self) -> CERF_R {
        CERF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PHY-LSI Receive Error Flag"]
    #[inline(always)]
    pub fn pre(&self) -> PRE_R {
        PRE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Frame-Too-Short Error Flag"]
    #[inline(always)]
    pub fn rtsf(&self) -> RTSF_R {
        RTSF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Frame-Too-Long Error Flag"]
    #[inline(always)]
    pub fn rtlf(&self) -> RTLF_R {
        RTLF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Alignment Error Flag"]
    #[inline(always)]
    pub fn rrf(&self) -> RRF_R {
        RRF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Multicast Address Frame Receive Flag"]
    #[inline(always)]
    pub fn rmaf(&self) -> RMAF_R {
        RMAF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit Retry Over Flag"]
    #[inline(always)]
    pub fn tro(&self) -> TRO_R {
        TRO_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Late Collision Detect Flag"]
    #[inline(always)]
    pub fn cd(&self) -> CD_R {
        CD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Loss of Carrier Detect Flag"]
    #[inline(always)]
    pub fn dlc(&self) -> DLC_R {
        DLC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Carrier Not Detect Flag"]
    #[inline(always)]
    pub fn cnd(&self) -> CND_R {
        CND_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Receive FIFO Overflow Flag"]
    #[inline(always)]
    pub fn rfof(&self) -> RFOF_R {
        RFOF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Receive Descriptor Empty Flag"]
    #[inline(always)]
    pub fn rde(&self) -> RDE_R {
        RDE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Frame Receive Flag"]
    #[inline(always)]
    pub fn fr(&self) -> FR_R {
        FR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Transmit FIFO Underflow Flag"]
    #[inline(always)]
    pub fn tfuf(&self) -> TFUF_R {
        TFUF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Transmit Descriptor Empty Flag"]
    #[inline(always)]
    pub fn tde(&self) -> TDE_R {
        TDE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Frame Transfer Complete Flag"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ETHERC Status Register Source Flag"]
    #[inline(always)]
    pub fn eci(&self) -> ECI_R {
        ECI_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Address Error Flag"]
    #[inline(always)]
    pub fn ade(&self) -> ADE_R {
        ADE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Receive Frame Counter Overflow Flag"]
    #[inline(always)]
    pub fn rfcof(&self) -> RFCOF_R {
        RFCOF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Receive Abort Detect Flag"]
    #[inline(always)]
    pub fn rabt(&self) -> RABT_R {
        RABT_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Transmit Abort Detect Flag"]
    #[inline(always)]
    pub fn tabt(&self) -> TABT_R {
        TABT_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 30 - Write-Back Complete Flag"]
    #[inline(always)]
    pub fn twb(&self) -> TWB_R {
        TWB_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CRC Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cerf(&mut self) -> CERF_W<0> {
        CERF_W::new(self)
    }
    #[doc = "Bit 1 - PHY-LSI Receive Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn pre(&mut self) -> PRE_W<1> {
        PRE_W::new(self)
    }
    #[doc = "Bit 2 - Frame-Too-Short Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rtsf(&mut self) -> RTSF_W<2> {
        RTSF_W::new(self)
    }
    #[doc = "Bit 3 - Frame-Too-Long Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rtlf(&mut self) -> RTLF_W<3> {
        RTLF_W::new(self)
    }
    #[doc = "Bit 4 - Alignment Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rrf(&mut self) -> RRF_W<4> {
        RRF_W::new(self)
    }
    #[doc = "Bit 7 - Multicast Address Frame Receive Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rmaf(&mut self) -> RMAF_W<7> {
        RMAF_W::new(self)
    }
    #[doc = "Bit 8 - Transmit Retry Over Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tro(&mut self) -> TRO_W<8> {
        TRO_W::new(self)
    }
    #[doc = "Bit 9 - Late Collision Detect Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cd(&mut self) -> CD_W<9> {
        CD_W::new(self)
    }
    #[doc = "Bit 10 - Loss of Carrier Detect Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dlc(&mut self) -> DLC_W<10> {
        DLC_W::new(self)
    }
    #[doc = "Bit 11 - Carrier Not Detect Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cnd(&mut self) -> CND_W<11> {
        CND_W::new(self)
    }
    #[doc = "Bit 16 - Receive FIFO Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rfof(&mut self) -> RFOF_W<16> {
        RFOF_W::new(self)
    }
    #[doc = "Bit 17 - Receive Descriptor Empty Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rde(&mut self) -> RDE_W<17> {
        RDE_W::new(self)
    }
    #[doc = "Bit 18 - Frame Receive Flag"]
    #[inline(always)]
    #[must_use]
    pub fn fr(&mut self) -> FR_W<18> {
        FR_W::new(self)
    }
    #[doc = "Bit 19 - Transmit FIFO Underflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tfuf(&mut self) -> TFUF_W<19> {
        TFUF_W::new(self)
    }
    #[doc = "Bit 20 - Transmit Descriptor Empty Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tde(&mut self) -> TDE_W<20> {
        TDE_W::new(self)
    }
    #[doc = "Bit 21 - Frame Transfer Complete Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tc(&mut self) -> TC_W<21> {
        TC_W::new(self)
    }
    #[doc = "Bit 23 - Address Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ade(&mut self) -> ADE_W<23> {
        ADE_W::new(self)
    }
    #[doc = "Bit 24 - Receive Frame Counter Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rfcof(&mut self) -> RFCOF_W<24> {
        RFCOF_W::new(self)
    }
    #[doc = "Bit 25 - Receive Abort Detect Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rabt(&mut self) -> RABT_W<25> {
        RABT_W::new(self)
    }
    #[doc = "Bit 26 - Transmit Abort Detect Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tabt(&mut self) -> TABT_W<26> {
        TABT_W::new(self)
    }
    #[doc = "Bit 30 - Write-Back Complete Flag"]
    #[inline(always)]
    #[must_use]
    pub fn twb(&mut self) -> TWB_W<30> {
        TWB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ETHERC/EDMAC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eesr](index.html) module"]
pub struct EESR_SPEC;
impl crate::RegisterSpec for EESR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eesr::R](R) reader structure"]
impl crate::Readable for EESR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eesr::W](W) writer structure"]
impl crate::Writable for EESR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EESR to value 0"]
impl crate::Resettable for EESR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
