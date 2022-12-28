#[doc = "Register `SD_CMD` reader"]
pub struct R(crate::R<SD_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SD_CMD` writer"]
pub struct W(crate::W<SD_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_CMD_SPEC>;
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
impl From<crate::W<SD_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMDIDX` reader - Command Index These bits specify Command Format\\[45:40\\]
(command index). \\[Examples\\]
CMD6: SD_CMD\\[7:0\\]
= 8'b00_000110 CMD18: SD_CMD\\[7:0\\]
= 8'b00_010010 ACMD13: SD_CMD\\[7:0\\]
= 8'b01_001101"]
pub type CMDIDX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMDIDX` writer - Command Index These bits specify Command Format\\[45:40\\]
(command index). \\[Examples\\]
CMD6: SD_CMD\\[7:0\\]
= 8'b00_000110 CMD18: SD_CMD\\[7:0\\]
= 8'b00_010010 ACMD13: SD_CMD\\[7:0\\]
= 8'b01_001101"]
pub type CMDIDX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SD_CMD_SPEC, u8, u8, 6, O>;
#[doc = "Field `ACMD` reader - Command Type Select"]
pub type ACMD_R = crate::FieldReader<u8, ACMD_A>;
#[doc = "Command Type Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACMD_A {
    #[doc = "0: CMD"]
    _00 = 0,
    #[doc = "1: ACMD"]
    _01 = 1,
}
impl From<ACMD_A> for u8 {
    #[inline(always)]
    fn from(variant: ACMD_A) -> Self {
        variant as _
    }
}
impl ACMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ACMD_A> {
        match self.bits {
            0 => Some(ACMD_A::_00),
            1 => Some(ACMD_A::_01),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == ACMD_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == ACMD_A::_01
    }
}
#[doc = "Field `ACMD` writer - Command Type Select"]
pub type ACMD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SD_CMD_SPEC, u8, ACMD_A, 2, O>;
impl<'a, const O: u8> ACMD_W<'a, O> {
    #[doc = "CMD"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(ACMD_A::_00)
    }
    #[doc = "ACMD"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(ACMD_A::_01)
    }
}
#[doc = "Field `RSPTP` reader - Mode/Response Type NOTE: As some commands cannot be used in normal mode, see section 1.4.10, Example of SD_CMD Register Setting to select mode/response type."]
pub type RSPTP_R = crate::FieldReader<u8, RSPTP_A>;
#[doc = "Mode/Response Type NOTE: As some commands cannot be used in normal mode, see section 1.4.10, Example of SD_CMD Register Setting to select mode/response type.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RSPTP_A {
    #[doc = "0: Normal mode The response type and the transfer mode are selected by SD_CMD\\[7:0\\], and the SD_CMD\\[15:11\\]
setting is disabled."]
    _000 = 0,
    #[doc = "3: Expansion mode and no response"]
    _011 = 3,
    #[doc = "4: Expansion mode and R1, R5, R6, or R7 response"]
    _100 = 4,
    #[doc = "5: Expansion mode and R1b response"]
    _101 = 5,
    #[doc = "6: Expansion mode and R2 response"]
    _110 = 6,
    #[doc = "7: Expansion mode and R3 or R4 response"]
    _111 = 7,
}
impl From<RSPTP_A> for u8 {
    #[inline(always)]
    fn from(variant: RSPTP_A) -> Self {
        variant as _
    }
}
impl RSPTP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RSPTP_A> {
        match self.bits {
            0 => Some(RSPTP_A::_000),
            3 => Some(RSPTP_A::_011),
            4 => Some(RSPTP_A::_100),
            5 => Some(RSPTP_A::_101),
            6 => Some(RSPTP_A::_110),
            7 => Some(RSPTP_A::_111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == RSPTP_A::_000
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == RSPTP_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == RSPTP_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == RSPTP_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == RSPTP_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == RSPTP_A::_111
    }
}
#[doc = "Field `RSPTP` writer - Mode/Response Type NOTE: As some commands cannot be used in normal mode, see section 1.4.10, Example of SD_CMD Register Setting to select mode/response type."]
pub type RSPTP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SD_CMD_SPEC, u8, RSPTP_A, 3, O>;
impl<'a, const O: u8> RSPTP_W<'a, O> {
    #[doc = "Normal mode The response type and the transfer mode are selected by SD_CMD\\[7:0\\], and the SD_CMD\\[15:11\\]
setting is disabled."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(RSPTP_A::_000)
    }
    #[doc = "Expansion mode and no response"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(RSPTP_A::_011)
    }
    #[doc = "Expansion mode and R1, R5, R6, or R7 response"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(RSPTP_A::_100)
    }
    #[doc = "Expansion mode and R1b response"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(RSPTP_A::_101)
    }
    #[doc = "Expansion mode and R2 response"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(RSPTP_A::_110)
    }
    #[doc = "Expansion mode and R3 or R4 response"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(RSPTP_A::_111)
    }
}
#[doc = "Field `CMDTP` reader - Data Mode (Command Type)"]
pub type CMDTP_R = crate::BitReader<CMDTP_A>;
#[doc = "Data Mode (Command Type)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDTP_A {
    #[doc = "0: Command does not include data transfer (bc, bcr, or ac)"]
    _0 = 0,
    #[doc = "1: Command includes data transfer (adtc)"]
    _1 = 1,
}
impl From<CMDTP_A> for bool {
    #[inline(always)]
    fn from(variant: CMDTP_A) -> Self {
        variant as u8 != 0
    }
}
impl CMDTP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDTP_A {
        match self.bits {
            false => CMDTP_A::_0,
            true => CMDTP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMDTP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMDTP_A::_1
    }
}
#[doc = "Field `CMDTP` writer - Data Mode (Command Type)"]
pub type CMDTP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_CMD_SPEC, CMDTP_A, O>;
impl<'a, const O: u8> CMDTP_W<'a, O> {
    #[doc = "Command does not include data transfer (bc, bcr, or ac)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMDTP_A::_0)
    }
    #[doc = "Command includes data transfer (adtc)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMDTP_A::_1)
    }
}
#[doc = "Field `CMDRW` reader - Write/Read Mode (enabled when the command with data is handled)"]
pub type CMDRW_R = crate::BitReader<CMDRW_A>;
#[doc = "Write/Read Mode (enabled when the command with data is handled)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDRW_A {
    #[doc = "0: Write (SD/MMC host interface -> SD card/MMC)"]
    _0 = 0,
    #[doc = "1: Read (SD/MMC host interface <- SD card/MMC)"]
    _1 = 1,
}
impl From<CMDRW_A> for bool {
    #[inline(always)]
    fn from(variant: CMDRW_A) -> Self {
        variant as u8 != 0
    }
}
impl CMDRW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDRW_A {
        match self.bits {
            false => CMDRW_A::_0,
            true => CMDRW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMDRW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMDRW_A::_1
    }
}
#[doc = "Field `CMDRW` writer - Write/Read Mode (enabled when the command with data is handled)"]
pub type CMDRW_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_CMD_SPEC, CMDRW_A, O>;
impl<'a, const O: u8> CMDRW_W<'a, O> {
    #[doc = "Write (SD/MMC host interface -> SD card/MMC)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMDRW_A::_0)
    }
    #[doc = "Read (SD/MMC host interface <- SD card/MMC)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMDRW_A::_1)
    }
}
#[doc = "Field `TRSTP` reader - Single/Multiple Block Transfer (enabled when the command with data is handled)"]
pub type TRSTP_R = crate::BitReader<TRSTP_A>;
#[doc = "Single/Multiple Block Transfer (enabled when the command with data is handled)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRSTP_A {
    #[doc = "0: Single block transfer"]
    _0 = 0,
    #[doc = "1: Multiple block transfer"]
    _1 = 1,
}
impl From<TRSTP_A> for bool {
    #[inline(always)]
    fn from(variant: TRSTP_A) -> Self {
        variant as u8 != 0
    }
}
impl TRSTP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRSTP_A {
        match self.bits {
            false => TRSTP_A::_0,
            true => TRSTP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRSTP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRSTP_A::_1
    }
}
#[doc = "Field `TRSTP` writer - Single/Multiple Block Transfer (enabled when the command with data is handled)"]
pub type TRSTP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_CMD_SPEC, TRSTP_A, O>;
impl<'a, const O: u8> TRSTP_W<'a, O> {
    #[doc = "Single block transfer"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRSTP_A::_0)
    }
    #[doc = "Multiple block transfer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRSTP_A::_1)
    }
}
#[doc = "Field `CMD12AT` reader - Multiple Block Transfer Mode (enabled at multiple block transfer)"]
pub type CMD12AT_R = crate::FieldReader<u8, CMD12AT_A>;
#[doc = "Multiple Block Transfer Mode (enabled at multiple block transfer)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMD12AT_A {
    #[doc = "0: CMD12 is automatically issued at multiple block transfer."]
    _00 = 0,
    #[doc = "1: CMD12 is not automatically issued at multiple block transfer."]
    _01 = 1,
    #[doc = "2: Setting prohibited"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<CMD12AT_A> for u8 {
    #[inline(always)]
    fn from(variant: CMD12AT_A) -> Self {
        variant as _
    }
}
impl CMD12AT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMD12AT_A {
        match self.bits {
            0 => CMD12AT_A::_00,
            1 => CMD12AT_A::_01,
            2 => CMD12AT_A::_10,
            3 => CMD12AT_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CMD12AT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CMD12AT_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CMD12AT_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CMD12AT_A::_11
    }
}
#[doc = "Field `CMD12AT` writer - Multiple Block Transfer Mode (enabled at multiple block transfer)"]
pub type CMD12AT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SD_CMD_SPEC, u8, CMD12AT_A, 2, O>;
impl<'a, const O: u8> CMD12AT_W<'a, O> {
    #[doc = "CMD12 is automatically issued at multiple block transfer."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CMD12AT_A::_00)
    }
    #[doc = "CMD12 is not automatically issued at multiple block transfer."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CMD12AT_A::_01)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CMD12AT_A::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CMD12AT_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:5 - Command Index These bits specify Command Format\\[45:40\\]
(command index). \\[Examples\\]
CMD6: SD_CMD\\[7:0\\]
= 8'b00_000110 CMD18: SD_CMD\\[7:0\\]
= 8'b00_010010 ACMD13: SD_CMD\\[7:0\\]
= 8'b01_001101"]
    #[inline(always)]
    pub fn cmdidx(&self) -> CMDIDX_R {
        CMDIDX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Command Type Select"]
    #[inline(always)]
    pub fn acmd(&self) -> ACMD_R {
        ACMD_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - Mode/Response Type NOTE: As some commands cannot be used in normal mode, see section 1.4.10, Example of SD_CMD Register Setting to select mode/response type."]
    #[inline(always)]
    pub fn rsptp(&self) -> RSPTP_R {
        RSPTP_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Data Mode (Command Type)"]
    #[inline(always)]
    pub fn cmdtp(&self) -> CMDTP_R {
        CMDTP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write/Read Mode (enabled when the command with data is handled)"]
    #[inline(always)]
    pub fn cmdrw(&self) -> CMDRW_R {
        CMDRW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Single/Multiple Block Transfer (enabled when the command with data is handled)"]
    #[inline(always)]
    pub fn trstp(&self) -> TRSTP_R {
        TRSTP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Multiple Block Transfer Mode (enabled at multiple block transfer)"]
    #[inline(always)]
    pub fn cmd12at(&self) -> CMD12AT_R {
        CMD12AT_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Command Index These bits specify Command Format\\[45:40\\]
(command index). \\[Examples\\]
CMD6: SD_CMD\\[7:0\\]
= 8'b00_000110 CMD18: SD_CMD\\[7:0\\]
= 8'b00_010010 ACMD13: SD_CMD\\[7:0\\]
= 8'b01_001101"]
    #[inline(always)]
    #[must_use]
    pub fn cmdidx(&mut self) -> CMDIDX_W<0> {
        CMDIDX_W::new(self)
    }
    #[doc = "Bits 6:7 - Command Type Select"]
    #[inline(always)]
    #[must_use]
    pub fn acmd(&mut self) -> ACMD_W<6> {
        ACMD_W::new(self)
    }
    #[doc = "Bits 8:10 - Mode/Response Type NOTE: As some commands cannot be used in normal mode, see section 1.4.10, Example of SD_CMD Register Setting to select mode/response type."]
    #[inline(always)]
    #[must_use]
    pub fn rsptp(&mut self) -> RSPTP_W<8> {
        RSPTP_W::new(self)
    }
    #[doc = "Bit 11 - Data Mode (Command Type)"]
    #[inline(always)]
    #[must_use]
    pub fn cmdtp(&mut self) -> CMDTP_W<11> {
        CMDTP_W::new(self)
    }
    #[doc = "Bit 12 - Write/Read Mode (enabled when the command with data is handled)"]
    #[inline(always)]
    #[must_use]
    pub fn cmdrw(&mut self) -> CMDRW_W<12> {
        CMDRW_W::new(self)
    }
    #[doc = "Bit 13 - Single/Multiple Block Transfer (enabled when the command with data is handled)"]
    #[inline(always)]
    #[must_use]
    pub fn trstp(&mut self) -> TRSTP_W<13> {
        TRSTP_W::new(self)
    }
    #[doc = "Bits 14:15 - Multiple Block Transfer Mode (enabled at multiple block transfer)"]
    #[inline(always)]
    #[must_use]
    pub fn cmd12at(&mut self) -> CMD12AT_W<14> {
        CMD12AT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command Type Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_cmd](index.html) module"]
pub struct SD_CMD_SPEC;
impl crate::RegisterSpec for SD_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sd_cmd::R](R) reader structure"]
impl crate::Readable for SD_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_cmd::W](W) writer structure"]
impl crate::Writable for SD_CMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SD_CMD to value 0"]
impl crate::Resettable for SD_CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
