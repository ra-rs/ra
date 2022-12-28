#[doc = "Register `OPSCR` reader"]
pub struct R(crate::R<OPSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPSCR` writer"]
pub struct W(crate::W<OPSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPSCR_SPEC>;
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
impl From<crate::W<OPSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UF` reader - Input Phase Soft Setting WFThis bit sets the input phase by the software settings.This bit setting is valid when the OPSCR.FB bit = 1."]
pub type UF_R = crate::BitReader<bool>;
#[doc = "Field `UF` writer - Input Phase Soft Setting WFThis bit sets the input phase by the software settings.This bit setting is valid when the OPSCR.FB bit = 1."]
pub type UF_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPSCR_SPEC, bool, O>;
#[doc = "Field `VF` reader - Input Phase Soft Setting VFThis bit sets the input phase by the software settings.This bit setting is valid when the OPSCR.FB bit = 1."]
pub type VF_R = crate::BitReader<bool>;
#[doc = "Field `VF` writer - Input Phase Soft Setting VFThis bit sets the input phase by the software settings.This bit setting is valid when the OPSCR.FB bit = 1."]
pub type VF_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPSCR_SPEC, bool, O>;
#[doc = "Field `WF` reader - Input Phase Soft Setting UFThis bit sets the input phase by the software settings.This bit setting is valid when the OPSCR.FB bit = 1."]
pub type WF_R = crate::BitReader<bool>;
#[doc = "Field `WF` writer - Input Phase Soft Setting UFThis bit sets the input phase by the software settings.This bit setting is valid when the OPSCR.FB bit = 1."]
pub type WF_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPSCR_SPEC, bool, O>;
#[doc = "Field `U` reader - Input U-Phase MonitorThis bit monitors the state of the input phase.OPSCR.FB=0:External input monitoring by PCLKOPSCR.FB=1:Software settings (UF/VF/WF)"]
pub type U_R = crate::BitReader<bool>;
#[doc = "Field `V` reader - Input V-Phase MonitorThis bit monitors the state of the input phase.OPSCR.FB=0:External input monitoring by PCLKOPSCR.FB=1:Software settings (UF/VF/WF)"]
pub type V_R = crate::BitReader<bool>;
#[doc = "Field `W` reader - Input W-Phase MonitorThis bit monitors the state of the input phase.OPSCR.FB=0:External input monitoring by PCLKOPSCR.FB=1:Software settings (UF/VF/WF)"]
pub type W_R = crate::BitReader<bool>;
#[doc = "Field `EN` reader - Enable-Phase Output Control"]
pub type EN_R = crate::BitReader<EN_A>;
#[doc = "Enable-Phase Output Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_A {
    #[doc = "0: Not Output(Hi-Z external terminals)."]
    _0 = 0,
    #[doc = "1: Output"]
    _1 = 1,
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
            false => EN_A::_0,
            true => EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EN_A::_1
    }
}
#[doc = "Field `EN` writer - Enable-Phase Output Control"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPSCR_SPEC, EN_A, O>;
impl<'a, const O: u8> EN_W<'a, O> {
    #[doc = "Not Output(Hi-Z external terminals)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN_A::_0)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN_A::_1)
    }
}
#[doc = "Field `FB` reader - External Feedback Signal EnableThis bit selects the input phase from the software settings and external input."]
pub type FB_R = crate::BitReader<FB_A>;
#[doc = "External Feedback Signal EnableThis bit selects the input phase from the software settings and external input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FB_A {
    #[doc = "0: Select the external input."]
    _0 = 0,
    #[doc = "1: Select the soft setting(OPSCR.UF, VF, WF)."]
    _1 = 1,
}
impl From<FB_A> for bool {
    #[inline(always)]
    fn from(variant: FB_A) -> Self {
        variant as u8 != 0
    }
}
impl FB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FB_A {
        match self.bits {
            false => FB_A::_0,
            true => FB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FB_A::_1
    }
}
#[doc = "Field `FB` writer - External Feedback Signal EnableThis bit selects the input phase from the software settings and external input."]
pub type FB_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPSCR_SPEC, FB_A, O>;
impl<'a, const O: u8> FB_W<'a, O> {
    #[doc = "Select the external input."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FB_A::_0)
    }
    #[doc = "Select the soft setting(OPSCR.UF, VF, WF)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FB_A::_1)
    }
}
#[doc = "Field `P` reader - Positive-Phase Output (P) Control"]
pub type P_R = crate::BitReader<P_A>;
#[doc = "Positive-Phase Output (P) Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P_A {
    #[doc = "0: Level signal output"]
    _0 = 0,
    #[doc = "1: PWM signal output (PWM of GPT0)"]
    _1 = 1,
}
impl From<P_A> for bool {
    #[inline(always)]
    fn from(variant: P_A) -> Self {
        variant as u8 != 0
    }
}
impl P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P_A {
        match self.bits {
            false => P_A::_0,
            true => P_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == P_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == P_A::_1
    }
}
#[doc = "Field `P` writer - Positive-Phase Output (P) Control"]
pub type P_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPSCR_SPEC, P_A, O>;
impl<'a, const O: u8> P_W<'a, O> {
    #[doc = "Level signal output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(P_A::_0)
    }
    #[doc = "PWM signal output (PWM of GPT0)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(P_A::_1)
    }
}
#[doc = "Field `N` reader - Negative-Phase Output (N) Control"]
pub type N_R = crate::BitReader<N_A>;
#[doc = "Negative-Phase Output (N) Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum N_A {
    #[doc = "0: Level signal output"]
    _0 = 0,
    #[doc = "1: PWM signal output (PWM of GPT0)"]
    _1 = 1,
}
impl From<N_A> for bool {
    #[inline(always)]
    fn from(variant: N_A) -> Self {
        variant as u8 != 0
    }
}
impl N_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> N_A {
        match self.bits {
            false => N_A::_0,
            true => N_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == N_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == N_A::_1
    }
}
#[doc = "Field `N` writer - Negative-Phase Output (N) Control"]
pub type N_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPSCR_SPEC, N_A, O>;
impl<'a, const O: u8> N_W<'a, O> {
    #[doc = "Level signal output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(N_A::_0)
    }
    #[doc = "PWM signal output (PWM of GPT0)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(N_A::_1)
    }
}
#[doc = "Field `INV` reader - Invert-Phase Output Control"]
pub type INV_R = crate::BitReader<INV_A>;
#[doc = "Invert-Phase Output Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INV_A {
    #[doc = "0: Positive Logic (Active High)output"]
    _0 = 0,
    #[doc = "1: Negative Logic (Active Low)output"]
    _1 = 1,
}
impl From<INV_A> for bool {
    #[inline(always)]
    fn from(variant: INV_A) -> Self {
        variant as u8 != 0
    }
}
impl INV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INV_A {
        match self.bits {
            false => INV_A::_0,
            true => INV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INV_A::_1
    }
}
#[doc = "Field `INV` writer - Invert-Phase Output Control"]
pub type INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPSCR_SPEC, INV_A, O>;
impl<'a, const O: u8> INV_W<'a, O> {
    #[doc = "Positive Logic (Active High)output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INV_A::_0)
    }
    #[doc = "Negative Logic (Active Low)output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INV_A::_1)
    }
}
#[doc = "Field `ALIGN` reader - Input phase alignment"]
pub type ALIGN_R = crate::BitReader<ALIGN_A>;
#[doc = "Input phase alignment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALIGN_A {
    #[doc = "0: Input phase is aligned to PCLK."]
    _0 = 0,
    #[doc = "1: Input phase is aligned PWM."]
    _1 = 1,
}
impl From<ALIGN_A> for bool {
    #[inline(always)]
    fn from(variant: ALIGN_A) -> Self {
        variant as u8 != 0
    }
}
impl ALIGN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALIGN_A {
        match self.bits {
            false => ALIGN_A::_0,
            true => ALIGN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ALIGN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ALIGN_A::_1
    }
}
#[doc = "Field `ALIGN` writer - Input phase alignment"]
pub type ALIGN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPSCR_SPEC, ALIGN_A, O>;
impl<'a, const O: u8> ALIGN_W<'a, O> {
    #[doc = "Input phase is aligned to PCLK."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALIGN_A::_0)
    }
    #[doc = "Input phase is aligned PWM."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALIGN_A::_1)
    }
}
#[doc = "Field `GRP` reader - Output disabled source selection"]
pub type GRP_R = crate::FieldReader<u8, GRP_A>;
#[doc = "Output disabled source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GRP_A {
    #[doc = "0: Select Group A output disable source"]
    _00 = 0,
    #[doc = "1: Select Group B output disable source"]
    _01 = 1,
    #[doc = "2: Select Group C output disable source"]
    _10 = 2,
    #[doc = "3: Select Group D output disable source"]
    _11 = 3,
}
impl From<GRP_A> for u8 {
    #[inline(always)]
    fn from(variant: GRP_A) -> Self {
        variant as _
    }
}
impl GRP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GRP_A {
        match self.bits {
            0 => GRP_A::_00,
            1 => GRP_A::_01,
            2 => GRP_A::_10,
            3 => GRP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == GRP_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == GRP_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == GRP_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == GRP_A::_11
    }
}
#[doc = "Field `GRP` writer - Output disabled source selection"]
pub type GRP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, OPSCR_SPEC, u8, GRP_A, 2, O>;
impl<'a, const O: u8> GRP_W<'a, O> {
    #[doc = "Select Group A output disable source"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(GRP_A::_00)
    }
    #[doc = "Select Group B output disable source"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(GRP_A::_01)
    }
    #[doc = "Select Group C output disable source"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(GRP_A::_10)
    }
    #[doc = "Select Group D output disable source"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(GRP_A::_11)
    }
}
#[doc = "Field `GODF` reader - Group output disable function"]
pub type GODF_R = crate::BitReader<GODF_A>;
#[doc = "Group output disable function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GODF_A {
    #[doc = "0: This bit function is ignored."]
    _0 = 0,
    #[doc = "1: Group disable will clear OPSCR.EN Bit."]
    _1 = 1,
}
impl From<GODF_A> for bool {
    #[inline(always)]
    fn from(variant: GODF_A) -> Self {
        variant as u8 != 0
    }
}
impl GODF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GODF_A {
        match self.bits {
            false => GODF_A::_0,
            true => GODF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GODF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GODF_A::_1
    }
}
#[doc = "Field `GODF` writer - Group output disable function"]
pub type GODF_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPSCR_SPEC, GODF_A, O>;
impl<'a, const O: u8> GODF_W<'a, O> {
    #[doc = "This bit function is ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GODF_A::_0)
    }
    #[doc = "Group disable will clear OPSCR.EN Bit."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GODF_A::_1)
    }
}
#[doc = "Field `NFEN` reader - External Input Noise Filter Enable"]
pub type NFEN_R = crate::BitReader<NFEN_A>;
#[doc = "External Input Noise Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NFEN_A {
    #[doc = "0: Do not use a noise filter to the external input."]
    _0 = 0,
    #[doc = "1: Use a noise filter to the external input."]
    _1 = 1,
}
impl From<NFEN_A> for bool {
    #[inline(always)]
    fn from(variant: NFEN_A) -> Self {
        variant as u8 != 0
    }
}
impl NFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NFEN_A {
        match self.bits {
            false => NFEN_A::_0,
            true => NFEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NFEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NFEN_A::_1
    }
}
#[doc = "Field `NFEN` writer - External Input Noise Filter Enable"]
pub type NFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPSCR_SPEC, NFEN_A, O>;
impl<'a, const O: u8> NFEN_W<'a, O> {
    #[doc = "Do not use a noise filter to the external input."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NFEN_A::_0)
    }
    #[doc = "Use a noise filter to the external input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NFEN_A::_1)
    }
}
#[doc = "Field `NFCS` reader - External Input Noise Filter Clock selectionNoise filter sampling clock setting of the external input."]
pub type NFCS_R = crate::FieldReader<u8, NFCS_A>;
#[doc = "External Input Noise Filter Clock selectionNoise filter sampling clock setting of the external input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NFCS_A {
    #[doc = "0: PCLK/1"]
    _00 = 0,
    #[doc = "1: PCLK/4"]
    _01 = 1,
    #[doc = "2: PCLK/16"]
    _10 = 2,
    #[doc = "3: PCLK/64"]
    _11 = 3,
}
impl From<NFCS_A> for u8 {
    #[inline(always)]
    fn from(variant: NFCS_A) -> Self {
        variant as _
    }
}
impl NFCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NFCS_A {
        match self.bits {
            0 => NFCS_A::_00,
            1 => NFCS_A::_01,
            2 => NFCS_A::_10,
            3 => NFCS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == NFCS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == NFCS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == NFCS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == NFCS_A::_11
    }
}
#[doc = "Field `NFCS` writer - External Input Noise Filter Clock selectionNoise filter sampling clock setting of the external input."]
pub type NFCS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, OPSCR_SPEC, u8, NFCS_A, 2, O>;
impl<'a, const O: u8> NFCS_W<'a, O> {
    #[doc = "PCLK/1"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(NFCS_A::_00)
    }
    #[doc = "PCLK/4"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(NFCS_A::_01)
    }
    #[doc = "PCLK/16"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(NFCS_A::_10)
    }
    #[doc = "PCLK/64"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(NFCS_A::_11)
    }
}
impl R {
    #[doc = "Bit 0 - Input Phase Soft Setting WFThis bit sets the input phase by the software settings.This bit setting is valid when the OPSCR.FB bit = 1."]
    #[inline(always)]
    pub fn uf(&self) -> UF_R {
        UF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Input Phase Soft Setting VFThis bit sets the input phase by the software settings.This bit setting is valid when the OPSCR.FB bit = 1."]
    #[inline(always)]
    pub fn vf(&self) -> VF_R {
        VF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Input Phase Soft Setting UFThis bit sets the input phase by the software settings.This bit setting is valid when the OPSCR.FB bit = 1."]
    #[inline(always)]
    pub fn wf(&self) -> WF_R {
        WF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Input U-Phase MonitorThis bit monitors the state of the input phase.OPSCR.FB=0:External input monitoring by PCLKOPSCR.FB=1:Software settings (UF/VF/WF)"]
    #[inline(always)]
    pub fn u(&self) -> U_R {
        U_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Input V-Phase MonitorThis bit monitors the state of the input phase.OPSCR.FB=0:External input monitoring by PCLKOPSCR.FB=1:Software settings (UF/VF/WF)"]
    #[inline(always)]
    pub fn v(&self) -> V_R {
        V_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Input W-Phase MonitorThis bit monitors the state of the input phase.OPSCR.FB=0:External input monitoring by PCLKOPSCR.FB=1:Software settings (UF/VF/WF)"]
    #[inline(always)]
    pub fn w(&self) -> W_R {
        W_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable-Phase Output Control"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - External Feedback Signal EnableThis bit selects the input phase from the software settings and external input."]
    #[inline(always)]
    pub fn fb(&self) -> FB_R {
        FB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Positive-Phase Output (P) Control"]
    #[inline(always)]
    pub fn p(&self) -> P_R {
        P_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Negative-Phase Output (N) Control"]
    #[inline(always)]
    pub fn n(&self) -> N_R {
        N_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Invert-Phase Output Control"]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Input phase alignment"]
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Output disabled source selection"]
    #[inline(always)]
    pub fn grp(&self) -> GRP_R {
        GRP_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Group output disable function"]
    #[inline(always)]
    pub fn godf(&self) -> GODF_R {
        GODF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 29 - External Input Noise Filter Enable"]
    #[inline(always)]
    pub fn nfen(&self) -> NFEN_R {
        NFEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - External Input Noise Filter Clock selectionNoise filter sampling clock setting of the external input."]
    #[inline(always)]
    pub fn nfcs(&self) -> NFCS_R {
        NFCS_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Input Phase Soft Setting WFThis bit sets the input phase by the software settings.This bit setting is valid when the OPSCR.FB bit = 1."]
    #[inline(always)]
    #[must_use]
    pub fn uf(&mut self) -> UF_W<0> {
        UF_W::new(self)
    }
    #[doc = "Bit 1 - Input Phase Soft Setting VFThis bit sets the input phase by the software settings.This bit setting is valid when the OPSCR.FB bit = 1."]
    #[inline(always)]
    #[must_use]
    pub fn vf(&mut self) -> VF_W<1> {
        VF_W::new(self)
    }
    #[doc = "Bit 2 - Input Phase Soft Setting UFThis bit sets the input phase by the software settings.This bit setting is valid when the OPSCR.FB bit = 1."]
    #[inline(always)]
    #[must_use]
    pub fn wf(&mut self) -> WF_W<2> {
        WF_W::new(self)
    }
    #[doc = "Bit 8 - Enable-Phase Output Control"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<8> {
        EN_W::new(self)
    }
    #[doc = "Bit 16 - External Feedback Signal EnableThis bit selects the input phase from the software settings and external input."]
    #[inline(always)]
    #[must_use]
    pub fn fb(&mut self) -> FB_W<16> {
        FB_W::new(self)
    }
    #[doc = "Bit 17 - Positive-Phase Output (P) Control"]
    #[inline(always)]
    #[must_use]
    pub fn p(&mut self) -> P_W<17> {
        P_W::new(self)
    }
    #[doc = "Bit 18 - Negative-Phase Output (N) Control"]
    #[inline(always)]
    #[must_use]
    pub fn n(&mut self) -> N_W<18> {
        N_W::new(self)
    }
    #[doc = "Bit 19 - Invert-Phase Output Control"]
    #[inline(always)]
    #[must_use]
    pub fn inv(&mut self) -> INV_W<19> {
        INV_W::new(self)
    }
    #[doc = "Bit 21 - Input phase alignment"]
    #[inline(always)]
    #[must_use]
    pub fn align(&mut self) -> ALIGN_W<21> {
        ALIGN_W::new(self)
    }
    #[doc = "Bits 24:25 - Output disabled source selection"]
    #[inline(always)]
    #[must_use]
    pub fn grp(&mut self) -> GRP_W<24> {
        GRP_W::new(self)
    }
    #[doc = "Bit 26 - Group output disable function"]
    #[inline(always)]
    #[must_use]
    pub fn godf(&mut self) -> GODF_W<26> {
        GODF_W::new(self)
    }
    #[doc = "Bit 29 - External Input Noise Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nfen(&mut self) -> NFEN_W<29> {
        NFEN_W::new(self)
    }
    #[doc = "Bits 30:31 - External Input Noise Filter Clock selectionNoise filter sampling clock setting of the external input."]
    #[inline(always)]
    #[must_use]
    pub fn nfcs(&mut self) -> NFCS_W<30> {
        NFCS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output Phase Switching Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opscr](index.html) module"]
pub struct OPSCR_SPEC;
impl crate::RegisterSpec for OPSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opscr::R](R) reader structure"]
impl crate::Readable for OPSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opscr::W](W) writer structure"]
impl crate::Writable for OPSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPSCR to value 0"]
impl crate::Resettable for OPSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
