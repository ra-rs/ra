#[doc = "Register `ECMR` reader"]
pub struct R(crate::R<ECMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECMR` writer"]
pub struct W(crate::W<ECMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECMR_SPEC>;
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
impl From<crate::W<ECMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRM` reader - Promiscuous Mode"]
pub type PRM_R = crate::BitReader<PRM_A>;
#[doc = "Promiscuous Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRM_A {
    #[doc = "0: Disable promiscuous mode"]
    _0 = 0,
    #[doc = "1: Enable promiscuous mode."]
    _1 = 1,
}
impl From<PRM_A> for bool {
    #[inline(always)]
    fn from(variant: PRM_A) -> Self {
        variant as u8 != 0
    }
}
impl PRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRM_A {
        match self.bits {
            false => PRM_A::_0,
            true => PRM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PRM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PRM_A::_1
    }
}
#[doc = "Field `PRM` writer - Promiscuous Mode"]
pub type PRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECMR_SPEC, PRM_A, O>;
impl<'a, const O: u8> PRM_W<'a, O> {
    #[doc = "Disable promiscuous mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PRM_A::_0)
    }
    #[doc = "Enable promiscuous mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PRM_A::_1)
    }
}
#[doc = "Field `DM` reader - Duplex Mode"]
pub type DM_R = crate::BitReader<DM_A>;
#[doc = "Duplex Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DM_A {
    #[doc = "0: Half-duplex mode"]
    _0 = 0,
    #[doc = "1: Full-duplex mode."]
    _1 = 1,
}
impl From<DM_A> for bool {
    #[inline(always)]
    fn from(variant: DM_A) -> Self {
        variant as u8 != 0
    }
}
impl DM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DM_A {
        match self.bits {
            false => DM_A::_0,
            true => DM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DM_A::_1
    }
}
#[doc = "Field `DM` writer - Duplex Mode"]
pub type DM_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECMR_SPEC, DM_A, O>;
impl<'a, const O: u8> DM_W<'a, O> {
    #[doc = "Half-duplex mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DM_A::_0)
    }
    #[doc = "Full-duplex mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DM_A::_1)
    }
}
#[doc = "Field `RTM` reader - Bit Rate"]
pub type RTM_R = crate::BitReader<RTM_A>;
#[doc = "Bit Rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTM_A {
    #[doc = "0: 10 Mbps"]
    _0 = 0,
    #[doc = "1: 100 Mbps."]
    _1 = 1,
}
impl From<RTM_A> for bool {
    #[inline(always)]
    fn from(variant: RTM_A) -> Self {
        variant as u8 != 0
    }
}
impl RTM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTM_A {
        match self.bits {
            false => RTM_A::_0,
            true => RTM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RTM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTM_A::_1
    }
}
#[doc = "Field `RTM` writer - Bit Rate"]
pub type RTM_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECMR_SPEC, RTM_A, O>;
impl<'a, const O: u8> RTM_W<'a, O> {
    #[doc = "10 Mbps"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTM_A::_0)
    }
    #[doc = "100 Mbps."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTM_A::_1)
    }
}
#[doc = "Field `ILB` reader - Internal Loopback Mode"]
pub type ILB_R = crate::BitReader<ILB_A>;
#[doc = "Internal Loopback Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ILB_A {
    #[doc = "0: Perform normal data transmission or reception"]
    _0 = 0,
    #[doc = "1: Loop data back in the ETHERC when full-duplex mode is selected."]
    _1 = 1,
}
impl From<ILB_A> for bool {
    #[inline(always)]
    fn from(variant: ILB_A) -> Self {
        variant as u8 != 0
    }
}
impl ILB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ILB_A {
        match self.bits {
            false => ILB_A::_0,
            true => ILB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ILB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ILB_A::_1
    }
}
#[doc = "Field `ILB` writer - Internal Loopback Mode"]
pub type ILB_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECMR_SPEC, ILB_A, O>;
impl<'a, const O: u8> ILB_W<'a, O> {
    #[doc = "Perform normal data transmission or reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ILB_A::_0)
    }
    #[doc = "Loop data back in the ETHERC when full-duplex mode is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ILB_A::_1)
    }
}
#[doc = "Field `TE` reader - Transmission Enable"]
pub type TE_R = crate::BitReader<TE_A>;
#[doc = "Transmission Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TE_A {
    #[doc = "0: Disable transmit function"]
    _0 = 0,
    #[doc = "1: Enable transmit function."]
    _1 = 1,
}
impl From<TE_A> for bool {
    #[inline(always)]
    fn from(variant: TE_A) -> Self {
        variant as u8 != 0
    }
}
impl TE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TE_A {
        match self.bits {
            false => TE_A::_0,
            true => TE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TE_A::_1
    }
}
#[doc = "Field `TE` writer - Transmission Enable"]
pub type TE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECMR_SPEC, TE_A, O>;
impl<'a, const O: u8> TE_W<'a, O> {
    #[doc = "Disable transmit function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TE_A::_0)
    }
    #[doc = "Enable transmit function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TE_A::_1)
    }
}
#[doc = "Field `RE` reader - Reception Enable"]
pub type RE_R = crate::BitReader<RE_A>;
#[doc = "Reception Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RE_A {
    #[doc = "0: Disable receive function"]
    _0 = 0,
    #[doc = "1: Enable receive function."]
    _1 = 1,
}
impl From<RE_A> for bool {
    #[inline(always)]
    fn from(variant: RE_A) -> Self {
        variant as u8 != 0
    }
}
impl RE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RE_A {
        match self.bits {
            false => RE_A::_0,
            true => RE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RE_A::_1
    }
}
#[doc = "Field `RE` writer - Reception Enable"]
pub type RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECMR_SPEC, RE_A, O>;
impl<'a, const O: u8> RE_W<'a, O> {
    #[doc = "Disable receive function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RE_A::_0)
    }
    #[doc = "Enable receive function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RE_A::_1)
    }
}
#[doc = "Field `MPDE` reader - Magic Packet Detection Enable"]
pub type MPDE_R = crate::BitReader<MPDE_A>;
#[doc = "Magic Packet Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPDE_A {
    #[doc = "0: Disable Magic Packet detection"]
    _0 = 0,
    #[doc = "1: Enable Magic Packet detection."]
    _1 = 1,
}
impl From<MPDE_A> for bool {
    #[inline(always)]
    fn from(variant: MPDE_A) -> Self {
        variant as u8 != 0
    }
}
impl MPDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPDE_A {
        match self.bits {
            false => MPDE_A::_0,
            true => MPDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MPDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MPDE_A::_1
    }
}
#[doc = "Field `MPDE` writer - Magic Packet Detection Enable"]
pub type MPDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECMR_SPEC, MPDE_A, O>;
impl<'a, const O: u8> MPDE_W<'a, O> {
    #[doc = "Disable Magic Packet detection"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MPDE_A::_0)
    }
    #[doc = "Enable Magic Packet detection."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MPDE_A::_1)
    }
}
#[doc = "Field `PRCEF` reader - CRC Error Frame Receive Mode"]
pub type PRCEF_R = crate::BitReader<PRCEF_A>;
#[doc = "CRC Error Frame Receive Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRCEF_A {
    #[doc = "0: Notify EDMAC of a CRC error"]
    _0 = 0,
    #[doc = "1: Do not notify EDMAC of a CRC error."]
    _1 = 1,
}
impl From<PRCEF_A> for bool {
    #[inline(always)]
    fn from(variant: PRCEF_A) -> Self {
        variant as u8 != 0
    }
}
impl PRCEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRCEF_A {
        match self.bits {
            false => PRCEF_A::_0,
            true => PRCEF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PRCEF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PRCEF_A::_1
    }
}
#[doc = "Field `PRCEF` writer - CRC Error Frame Receive Mode"]
pub type PRCEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECMR_SPEC, PRCEF_A, O>;
impl<'a, const O: u8> PRCEF_W<'a, O> {
    #[doc = "Notify EDMAC of a CRC error"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PRCEF_A::_0)
    }
    #[doc = "Do not notify EDMAC of a CRC error."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PRCEF_A::_1)
    }
}
#[doc = "Field `TXF` reader - Transmit Flow Control Operating Mode"]
pub type TXF_R = crate::BitReader<TXF_A>;
#[doc = "Transmit Flow Control Operating Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXF_A {
    #[doc = "0: Disable automatic PAUSE frame transmission (PAUSE frame is not automatically transmitted)"]
    _0 = 0,
    #[doc = "1: Enable automatic PAUSE frame transmission (PAUSE frame is automatically transmitted as required)."]
    _1 = 1,
}
impl From<TXF_A> for bool {
    #[inline(always)]
    fn from(variant: TXF_A) -> Self {
        variant as u8 != 0
    }
}
impl TXF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXF_A {
        match self.bits {
            false => TXF_A::_0,
            true => TXF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXF_A::_1
    }
}
#[doc = "Field `TXF` writer - Transmit Flow Control Operating Mode"]
pub type TXF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECMR_SPEC, TXF_A, O>;
impl<'a, const O: u8> TXF_W<'a, O> {
    #[doc = "Disable automatic PAUSE frame transmission (PAUSE frame is not automatically transmitted)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXF_A::_0)
    }
    #[doc = "Enable automatic PAUSE frame transmission (PAUSE frame is automatically transmitted as required)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXF_A::_1)
    }
}
#[doc = "Field `RXF` reader - Receive Flow Control Operating Mode"]
pub type RXF_R = crate::BitReader<RXF_A>;
#[doc = "Receive Flow Control Operating Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXF_A {
    #[doc = "0: Disable PAUSE frame detection"]
    _0 = 0,
    #[doc = "1: Enable PAUSE frame detection."]
    _1 = 1,
}
impl From<RXF_A> for bool {
    #[inline(always)]
    fn from(variant: RXF_A) -> Self {
        variant as u8 != 0
    }
}
impl RXF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXF_A {
        match self.bits {
            false => RXF_A::_0,
            true => RXF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXF_A::_1
    }
}
#[doc = "Field `RXF` writer - Receive Flow Control Operating Mode"]
pub type RXF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECMR_SPEC, RXF_A, O>;
impl<'a, const O: u8> RXF_W<'a, O> {
    #[doc = "Disable PAUSE frame detection"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXF_A::_0)
    }
    #[doc = "Enable PAUSE frame detection."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXF_A::_1)
    }
}
#[doc = "Field `PFR` reader - PAUSE Frame Receive Mode"]
pub type PFR_R = crate::BitReader<PFR_A>;
#[doc = "PAUSE Frame Receive Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PFR_A {
    #[doc = "0: Do not transfer PAUSE frame to the EDMAC"]
    _0 = 0,
    #[doc = "1: Transfer PAUSE frame to the EDMAC."]
    _1 = 1,
}
impl From<PFR_A> for bool {
    #[inline(always)]
    fn from(variant: PFR_A) -> Self {
        variant as u8 != 0
    }
}
impl PFR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFR_A {
        match self.bits {
            false => PFR_A::_0,
            true => PFR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PFR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PFR_A::_1
    }
}
#[doc = "Field `PFR` writer - PAUSE Frame Receive Mode"]
pub type PFR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECMR_SPEC, PFR_A, O>;
impl<'a, const O: u8> PFR_W<'a, O> {
    #[doc = "Do not transfer PAUSE frame to the EDMAC"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PFR_A::_0)
    }
    #[doc = "Transfer PAUSE frame to the EDMAC."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PFR_A::_1)
    }
}
#[doc = "Field `ZPF` reader - 0 Time PAUSE Frame Enable"]
pub type ZPF_R = crate::BitReader<ZPF_A>;
#[doc = "0 Time PAUSE Frame Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ZPF_A {
    #[doc = "0: Do not use PAUSE frames that containing a pause_time parameter of 0"]
    _0 = 0,
    #[doc = "1: Use PAUSE frames that containing a pause_time parameter of 0."]
    _1 = 1,
}
impl From<ZPF_A> for bool {
    #[inline(always)]
    fn from(variant: ZPF_A) -> Self {
        variant as u8 != 0
    }
}
impl ZPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ZPF_A {
        match self.bits {
            false => ZPF_A::_0,
            true => ZPF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ZPF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ZPF_A::_1
    }
}
#[doc = "Field `ZPF` writer - 0 Time PAUSE Frame Enable"]
pub type ZPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECMR_SPEC, ZPF_A, O>;
impl<'a, const O: u8> ZPF_W<'a, O> {
    #[doc = "Do not use PAUSE frames that containing a pause_time parameter of 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ZPF_A::_0)
    }
    #[doc = "Use PAUSE frames that containing a pause_time parameter of 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ZPF_A::_1)
    }
}
#[doc = "Field `TPC` reader - PAUSE Frame Transmit"]
pub type TPC_R = crate::BitReader<TPC_A>;
#[doc = "PAUSE Frame Transmit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TPC_A {
    #[doc = "0: Transmit PAUSE frame even during a PAUSE period"]
    _0 = 0,
    #[doc = "1: Do not transmit PAUSE frame during a PAUSE period."]
    _1 = 1,
}
impl From<TPC_A> for bool {
    #[inline(always)]
    fn from(variant: TPC_A) -> Self {
        variant as u8 != 0
    }
}
impl TPC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPC_A {
        match self.bits {
            false => TPC_A::_0,
            true => TPC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TPC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TPC_A::_1
    }
}
#[doc = "Field `TPC` writer - PAUSE Frame Transmit"]
pub type TPC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECMR_SPEC, TPC_A, O>;
impl<'a, const O: u8> TPC_W<'a, O> {
    #[doc = "Transmit PAUSE frame even during a PAUSE period"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TPC_A::_0)
    }
    #[doc = "Do not transmit PAUSE frame during a PAUSE period."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TPC_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Promiscuous Mode"]
    #[inline(always)]
    pub fn prm(&self) -> PRM_R {
        PRM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Duplex Mode"]
    #[inline(always)]
    pub fn dm(&self) -> DM_R {
        DM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bit Rate"]
    #[inline(always)]
    pub fn rtm(&self) -> RTM_R {
        RTM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Internal Loopback Mode"]
    #[inline(always)]
    pub fn ilb(&self) -> ILB_R {
        ILB_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmission Enable"]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reception Enable"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Magic Packet Detection Enable"]
    #[inline(always)]
    pub fn mpde(&self) -> MPDE_R {
        MPDE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC Error Frame Receive Mode"]
    #[inline(always)]
    pub fn prcef(&self) -> PRCEF_R {
        PRCEF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Transmit Flow Control Operating Mode"]
    #[inline(always)]
    pub fn txf(&self) -> TXF_R {
        TXF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Receive Flow Control Operating Mode"]
    #[inline(always)]
    pub fn rxf(&self) -> RXF_R {
        RXF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PAUSE Frame Receive Mode"]
    #[inline(always)]
    pub fn pfr(&self) -> PFR_R {
        PFR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 0 Time PAUSE Frame Enable"]
    #[inline(always)]
    pub fn zpf(&self) -> ZPF_R {
        ZPF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PAUSE Frame Transmit"]
    #[inline(always)]
    pub fn tpc(&self) -> TPC_R {
        TPC_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Promiscuous Mode"]
    #[inline(always)]
    #[must_use]
    pub fn prm(&mut self) -> PRM_W<0> {
        PRM_W::new(self)
    }
    #[doc = "Bit 1 - Duplex Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dm(&mut self) -> DM_W<1> {
        DM_W::new(self)
    }
    #[doc = "Bit 2 - Bit Rate"]
    #[inline(always)]
    #[must_use]
    pub fn rtm(&mut self) -> RTM_W<2> {
        RTM_W::new(self)
    }
    #[doc = "Bit 3 - Internal Loopback Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ilb(&mut self) -> ILB_W<3> {
        ILB_W::new(self)
    }
    #[doc = "Bit 5 - Transmission Enable"]
    #[inline(always)]
    #[must_use]
    pub fn te(&mut self) -> TE_W<5> {
        TE_W::new(self)
    }
    #[doc = "Bit 6 - Reception Enable"]
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> RE_W<6> {
        RE_W::new(self)
    }
    #[doc = "Bit 9 - Magic Packet Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mpde(&mut self) -> MPDE_W<9> {
        MPDE_W::new(self)
    }
    #[doc = "Bit 12 - CRC Error Frame Receive Mode"]
    #[inline(always)]
    #[must_use]
    pub fn prcef(&mut self) -> PRCEF_W<12> {
        PRCEF_W::new(self)
    }
    #[doc = "Bit 16 - Transmit Flow Control Operating Mode"]
    #[inline(always)]
    #[must_use]
    pub fn txf(&mut self) -> TXF_W<16> {
        TXF_W::new(self)
    }
    #[doc = "Bit 17 - Receive Flow Control Operating Mode"]
    #[inline(always)]
    #[must_use]
    pub fn rxf(&mut self) -> RXF_W<17> {
        RXF_W::new(self)
    }
    #[doc = "Bit 18 - PAUSE Frame Receive Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pfr(&mut self) -> PFR_W<18> {
        PFR_W::new(self)
    }
    #[doc = "Bit 19 - 0 Time PAUSE Frame Enable"]
    #[inline(always)]
    #[must_use]
    pub fn zpf(&mut self) -> ZPF_W<19> {
        ZPF_W::new(self)
    }
    #[doc = "Bit 20 - PAUSE Frame Transmit"]
    #[inline(always)]
    #[must_use]
    pub fn tpc(&mut self) -> TPC_W<20> {
        TPC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ETHERC Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecmr](index.html) module"]
pub struct ECMR_SPEC;
impl crate::RegisterSpec for ECMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecmr::R](R) reader structure"]
impl crate::Readable for ECMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ecmr::W](W) writer structure"]
impl crate::Writable for ECMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECMR to value 0"]
impl crate::Resettable for ECMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
