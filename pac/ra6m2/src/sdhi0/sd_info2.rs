#[doc = "Register `SD_INFO2` reader"]
pub struct R(crate::R<SD_INFO2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_INFO2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_INFO2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_INFO2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SD_INFO2` writer"]
pub struct W(crate::W<SD_INFO2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_INFO2_SPEC>;
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
impl From<crate::W<SD_INFO2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_INFO2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMDE` reader - Command Error\n\nThe field is **modified** in some way after a read operation."]
pub type CMDE_R = crate::BitReader<CMDE_A>;
#[doc = "Command Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDE_A {
    #[doc = "0: Command error not detected"]
    _0 = 0,
    #[doc = "1: Command error detected"]
    _1 = 1,
}
impl From<CMDE_A> for bool {
    #[inline(always)]
    fn from(variant: CMDE_A) -> Self {
        variant as u8 != 0
    }
}
impl CMDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDE_A {
        match self.bits {
            false => CMDE_A::_0,
            true => CMDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMDE_A::_1
    }
}
#[doc = "Field `CMDE` writer - Command Error"]
pub type CMDE_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, SD_INFO2_SPEC, CMDE_A, O>;
impl<'a, const O: u8> CMDE_W<'a, O> {
    #[doc = "Command error not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMDE_A::_0)
    }
    #[doc = "Command error detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMDE_A::_1)
    }
}
#[doc = "Field `CRCE` reader - CRC Error\n\nThe field is **modified** in some way after a read operation."]
pub type CRCE_R = crate::BitReader<CRCE_A>;
#[doc = "CRC Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCE_A {
    #[doc = "0: CRC error not detected"]
    _0 = 0,
    #[doc = "1: CRC error detected"]
    _1 = 1,
}
impl From<CRCE_A> for bool {
    #[inline(always)]
    fn from(variant: CRCE_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCE_A {
        match self.bits {
            false => CRCE_A::_0,
            true => CRCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CRCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CRCE_A::_1
    }
}
#[doc = "Field `CRCE` writer - CRC Error"]
pub type CRCE_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, SD_INFO2_SPEC, CRCE_A, O>;
impl<'a, const O: u8> CRCE_W<'a, O> {
    #[doc = "CRC error not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRCE_A::_0)
    }
    #[doc = "CRC error detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRCE_A::_1)
    }
}
#[doc = "Field `ENDE` reader - END Error\n\nThe field is **modified** in some way after a read operation."]
pub type ENDE_R = crate::BitReader<ENDE_A>;
#[doc = "END Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENDE_A {
    #[doc = "0: End bit error not detected"]
    _0 = 0,
    #[doc = "1: End bit error detected"]
    _1 = 1,
}
impl From<ENDE_A> for bool {
    #[inline(always)]
    fn from(variant: ENDE_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDE_A {
        match self.bits {
            false => ENDE_A::_0,
            true => ENDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENDE_A::_1
    }
}
#[doc = "Field `ENDE` writer - END Error"]
pub type ENDE_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, SD_INFO2_SPEC, ENDE_A, O>;
impl<'a, const O: u8> ENDE_W<'a, O> {
    #[doc = "End bit error not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENDE_A::_0)
    }
    #[doc = "End bit error detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENDE_A::_1)
    }
}
#[doc = "Field `DTO` reader - Data Timeout\n\nThe field is **modified** in some way after a read operation."]
pub type DTO_R = crate::BitReader<DTO_A>;
#[doc = "Data Timeout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTO_A {
    #[doc = "0: Data timeout not detected"]
    _0 = 0,
    #[doc = "1: Data timeout detected"]
    _1 = 1,
}
impl From<DTO_A> for bool {
    #[inline(always)]
    fn from(variant: DTO_A) -> Self {
        variant as u8 != 0
    }
}
impl DTO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTO_A {
        match self.bits {
            false => DTO_A::_0,
            true => DTO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTO_A::_1
    }
}
#[doc = "Field `DTO` writer - Data Timeout"]
pub type DTO_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, SD_INFO2_SPEC, DTO_A, O>;
impl<'a, const O: u8> DTO_W<'a, O> {
    #[doc = "Data timeout not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTO_A::_0)
    }
    #[doc = "Data timeout detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTO_A::_1)
    }
}
#[doc = "Field `ILW` reader - SD_BUF Illegal Write Access\n\nThe field is **modified** in some way after a read operation."]
pub type ILW_R = crate::BitReader<ILW_A>;
#[doc = "SD_BUF Illegal Write Access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ILW_A {
    #[doc = "0: Illegal write access to the SD_BUF register not detected"]
    _0 = 0,
    #[doc = "1: Illegal write access to the SD_BUF register detected"]
    _1 = 1,
}
impl From<ILW_A> for bool {
    #[inline(always)]
    fn from(variant: ILW_A) -> Self {
        variant as u8 != 0
    }
}
impl ILW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ILW_A {
        match self.bits {
            false => ILW_A::_0,
            true => ILW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ILW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ILW_A::_1
    }
}
#[doc = "Field `ILW` writer - SD_BUF Illegal Write Access"]
pub type ILW_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, SD_INFO2_SPEC, ILW_A, O>;
impl<'a, const O: u8> ILW_W<'a, O> {
    #[doc = "Illegal write access to the SD_BUF register not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ILW_A::_0)
    }
    #[doc = "Illegal write access to the SD_BUF register detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ILW_A::_1)
    }
}
#[doc = "Field `ILR` reader - SD_BUF Illegal Read Access\n\nThe field is **modified** in some way after a read operation."]
pub type ILR_R = crate::BitReader<ILR_A>;
#[doc = "SD_BUF Illegal Read Access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ILR_A {
    #[doc = "0: Illegal read access to the SD_BUF register not detected"]
    _0 = 0,
    #[doc = "1: Illegal read access to the SD_BUF register detected"]
    _1 = 1,
}
impl From<ILR_A> for bool {
    #[inline(always)]
    fn from(variant: ILR_A) -> Self {
        variant as u8 != 0
    }
}
impl ILR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ILR_A {
        match self.bits {
            false => ILR_A::_0,
            true => ILR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ILR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ILR_A::_1
    }
}
#[doc = "Field `ILR` writer - SD_BUF Illegal Read Access"]
pub type ILR_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, SD_INFO2_SPEC, ILR_A, O>;
impl<'a, const O: u8> ILR_W<'a, O> {
    #[doc = "Illegal read access to the SD_BUF register not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ILR_A::_0)
    }
    #[doc = "Illegal read access to the SD_BUF register detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ILR_A::_1)
    }
}
#[doc = "Field `RSPTO` reader - Response Timeout\n\nThe field is **modified** in some way after a read operation."]
pub type RSPTO_R = crate::BitReader<RSPTO_A>;
#[doc = "Response Timeout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSPTO_A {
    #[doc = "0: Response timeout not detected"]
    _0 = 0,
    #[doc = "1: Response timeout detected"]
    _1 = 1,
}
impl From<RSPTO_A> for bool {
    #[inline(always)]
    fn from(variant: RSPTO_A) -> Self {
        variant as u8 != 0
    }
}
impl RSPTO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSPTO_A {
        match self.bits {
            false => RSPTO_A::_0,
            true => RSPTO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSPTO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSPTO_A::_1
    }
}
#[doc = "Field `RSPTO` writer - Response Timeout"]
pub type RSPTO_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, SD_INFO2_SPEC, RSPTO_A, O>;
impl<'a, const O: u8> RSPTO_W<'a, O> {
    #[doc = "Response timeout not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSPTO_A::_0)
    }
    #[doc = "Response timeout detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSPTO_A::_1)
    }
}
#[doc = "Field `SDD0MON` reader - SDDAT0Indicates the SDDAT0 state of the port specified by SD_PORTSEL."]
pub type SDD0MON_R = crate::BitReader<SDD0MON_A>;
#[doc = "SDDAT0Indicates the SDDAT0 state of the port specified by SD_PORTSEL.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDD0MON_A {
    #[doc = "1: SDDAT0 is set to 1."]
    _1 = 1,
    #[doc = "0: SDDAT0 is set to 0."]
    _0 = 0,
}
impl From<SDD0MON_A> for bool {
    #[inline(always)]
    fn from(variant: SDD0MON_A) -> Self {
        variant as u8 != 0
    }
}
impl SDD0MON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDD0MON_A {
        match self.bits {
            true => SDD0MON_A::_1,
            false => SDD0MON_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDD0MON_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDD0MON_A::_0
    }
}
#[doc = "Field `BRE` reader - SD_BUF Read Enable\n\nThe field is **modified** in some way after a read operation."]
pub type BRE_R = crate::BitReader<BRE_A>;
#[doc = "SD_BUF Read Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRE_A {
    #[doc = "1: Data can be read from SD_BUF0."]
    _1 = 1,
    #[doc = "0: Data cannot be read from SD_BUF0."]
    _0 = 0,
}
impl From<BRE_A> for bool {
    #[inline(always)]
    fn from(variant: BRE_A) -> Self {
        variant as u8 != 0
    }
}
impl BRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRE_A {
        match self.bits {
            true => BRE_A::_1,
            false => BRE_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BRE_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BRE_A::_0
    }
}
#[doc = "Field `BRE` writer - SD_BUF Read Enable"]
pub type BRE_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, SD_INFO2_SPEC, BRE_A, O>;
impl<'a, const O: u8> BRE_W<'a, O> {
    #[doc = "Data can be read from SD_BUF0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRE_A::_1)
    }
    #[doc = "Data cannot be read from SD_BUF0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRE_A::_0)
    }
}
#[doc = "Field `BWE` reader - SD_BUF Write Enable\n\nThe field is **modified** in some way after a read operation."]
pub type BWE_R = crate::BitReader<BWE_A>;
#[doc = "SD_BUF Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BWE_A {
    #[doc = "1: Data can be written in SD_BUF0."]
    _1 = 1,
    #[doc = "0: Data cannot be written in SD_BUF0."]
    _0 = 0,
}
impl From<BWE_A> for bool {
    #[inline(always)]
    fn from(variant: BWE_A) -> Self {
        variant as u8 != 0
    }
}
impl BWE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWE_A {
        match self.bits {
            true => BWE_A::_1,
            false => BWE_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BWE_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BWE_A::_0
    }
}
#[doc = "Field `BWE` writer - SD_BUF Write Enable"]
pub type BWE_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, SD_INFO2_SPEC, BWE_A, O>;
impl<'a, const O: u8> BWE_W<'a, O> {
    #[doc = "Data can be written in SD_BUF0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BWE_A::_1)
    }
    #[doc = "Data cannot be written in SD_BUF0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BWE_A::_0)
    }
}
#[doc = "Field `SD_CLK_CTRLEN` reader - When a command sequence is started by writing to SD_CMD, the CBSY bit is set to 1 and, at the same time, the SCLKDIVEN bit is set to 0. The SCLKDIVEN bit is set to 1 after 8 cycles of SDCLK have elapsed after setting of the CBSY bit to 0 due to completion of the command sequence."]
pub type SD_CLK_CTRLEN_R = crate::BitReader<SD_CLK_CTRLEN_A>;
#[doc = "When a command sequence is started by writing to SD_CMD, the CBSY bit is set to 1 and, at the same time, the SCLKDIVEN bit is set to 0. The SCLKDIVEN bit is set to 1 after 8 cycles of SDCLK have elapsed after setting of the CBSY bit to 0 due to completion of the command sequence.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SD_CLK_CTRLEN_A {
    #[doc = "0: The SD/MMC bus (CMD, DAT) is busy. Writing to the SCLKEN and DIV bits in SD_CLK_CTRL is not possible."]
    _0 = 0,
    #[doc = "1: The SD/MMC bus (CMD, DAT) is not busy."]
    _1 = 1,
}
impl From<SD_CLK_CTRLEN_A> for bool {
    #[inline(always)]
    fn from(variant: SD_CLK_CTRLEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SD_CLK_CTRLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SD_CLK_CTRLEN_A {
        match self.bits {
            false => SD_CLK_CTRLEN_A::_0,
            true => SD_CLK_CTRLEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SD_CLK_CTRLEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SD_CLK_CTRLEN_A::_1
    }
}
#[doc = "Field `CBSY` reader - Command Type Register Busy"]
pub type CBSY_R = crate::BitReader<CBSY_A>;
#[doc = "Command Type Register Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CBSY_A {
    #[doc = "0: A command sequence is being executed."]
    _0 = 0,
    #[doc = "1: A command sequence has been completed."]
    _1 = 1,
}
impl From<CBSY_A> for bool {
    #[inline(always)]
    fn from(variant: CBSY_A) -> Self {
        variant as u8 != 0
    }
}
impl CBSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CBSY_A {
        match self.bits {
            false => CBSY_A::_0,
            true => CBSY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CBSY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CBSY_A::_1
    }
}
#[doc = "Field `ILA` reader - Illegal Access Error\n\nThe field is **modified** in some way after a read operation."]
pub type ILA_R = crate::BitReader<ILA_A>;
#[doc = "Illegal Access Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ILA_A {
    #[doc = "0: Illegal access error not detected"]
    _0 = 0,
    #[doc = "1: Illegal access error detected"]
    _1 = 1,
}
impl From<ILA_A> for bool {
    #[inline(always)]
    fn from(variant: ILA_A) -> Self {
        variant as u8 != 0
    }
}
impl ILA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ILA_A {
        match self.bits {
            false => ILA_A::_0,
            true => ILA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ILA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ILA_A::_1
    }
}
#[doc = "Field `ILA` writer - Illegal Access Error"]
pub type ILA_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, SD_INFO2_SPEC, ILA_A, O>;
impl<'a, const O: u8> ILA_W<'a, O> {
    #[doc = "Illegal access error not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ILA_A::_0)
    }
    #[doc = "Illegal access error detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ILA_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Command Error"]
    #[inline(always)]
    pub fn cmde(&self) -> CMDE_R {
        CMDE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CRC Error"]
    #[inline(always)]
    pub fn crce(&self) -> CRCE_R {
        CRCE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - END Error"]
    #[inline(always)]
    pub fn ende(&self) -> ENDE_R {
        ENDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Timeout"]
    #[inline(always)]
    pub fn dto(&self) -> DTO_R {
        DTO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SD_BUF Illegal Write Access"]
    #[inline(always)]
    pub fn ilw(&self) -> ILW_R {
        ILW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SD_BUF Illegal Read Access"]
    #[inline(always)]
    pub fn ilr(&self) -> ILR_R {
        ILR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Response Timeout"]
    #[inline(always)]
    pub fn rspto(&self) -> RSPTO_R {
        RSPTO_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SDDAT0Indicates the SDDAT0 state of the port specified by SD_PORTSEL."]
    #[inline(always)]
    pub fn sdd0mon(&self) -> SDD0MON_R {
        SDD0MON_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SD_BUF Read Enable"]
    #[inline(always)]
    pub fn bre(&self) -> BRE_R {
        BRE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SD_BUF Write Enable"]
    #[inline(always)]
    pub fn bwe(&self) -> BWE_R {
        BWE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - When a command sequence is started by writing to SD_CMD, the CBSY bit is set to 1 and, at the same time, the SCLKDIVEN bit is set to 0. The SCLKDIVEN bit is set to 1 after 8 cycles of SDCLK have elapsed after setting of the CBSY bit to 0 due to completion of the command sequence."]
    #[inline(always)]
    pub fn sd_clk_ctrlen(&self) -> SD_CLK_CTRLEN_R {
        SD_CLK_CTRLEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Command Type Register Busy"]
    #[inline(always)]
    pub fn cbsy(&self) -> CBSY_R {
        CBSY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Illegal Access Error"]
    #[inline(always)]
    pub fn ila(&self) -> ILA_R {
        ILA_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Error"]
    #[inline(always)]
    #[must_use]
    pub fn cmde(&mut self) -> CMDE_W<0> {
        CMDE_W::new(self)
    }
    #[doc = "Bit 1 - CRC Error"]
    #[inline(always)]
    #[must_use]
    pub fn crce(&mut self) -> CRCE_W<1> {
        CRCE_W::new(self)
    }
    #[doc = "Bit 2 - END Error"]
    #[inline(always)]
    #[must_use]
    pub fn ende(&mut self) -> ENDE_W<2> {
        ENDE_W::new(self)
    }
    #[doc = "Bit 3 - Data Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn dto(&mut self) -> DTO_W<3> {
        DTO_W::new(self)
    }
    #[doc = "Bit 4 - SD_BUF Illegal Write Access"]
    #[inline(always)]
    #[must_use]
    pub fn ilw(&mut self) -> ILW_W<4> {
        ILW_W::new(self)
    }
    #[doc = "Bit 5 - SD_BUF Illegal Read Access"]
    #[inline(always)]
    #[must_use]
    pub fn ilr(&mut self) -> ILR_W<5> {
        ILR_W::new(self)
    }
    #[doc = "Bit 6 - Response Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn rspto(&mut self) -> RSPTO_W<6> {
        RSPTO_W::new(self)
    }
    #[doc = "Bit 8 - SD_BUF Read Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bre(&mut self) -> BRE_W<8> {
        BRE_W::new(self)
    }
    #[doc = "Bit 9 - SD_BUF Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bwe(&mut self) -> BWE_W<9> {
        BWE_W::new(self)
    }
    #[doc = "Bit 15 - Illegal Access Error"]
    #[inline(always)]
    #[must_use]
    pub fn ila(&mut self) -> ILA_W<15> {
        ILA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SD Card Interrupt Flag Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_info2](index.html) module"]
pub struct SD_INFO2_SPEC;
impl crate::RegisterSpec for SD_INFO2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sd_info2::R](R) reader structure"]
impl crate::Readable for SD_INFO2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_info2::W](W) writer structure"]
impl crate::Writable for SD_INFO2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x837f;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SD_INFO2 to value 0x2000"]
impl crate::Resettable for SD_INFO2_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000;
}
