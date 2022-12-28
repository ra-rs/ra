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
#[doc = "Field `CMDIDX` reader - Command Index Field Value Select"]
pub type CMDIDX_R = crate::FieldReader<u8, CMDIDX_A>;
#[doc = "Command Index Field Value Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMDIDX_A {
    #[doc = "6: CMD6"]
    CMD6 = 6,
    #[doc = "13: ACMD13"]
    ACMD13 = 13,
    #[doc = "18: CMD18"]
    CMD18 = 18,
}
impl From<CMDIDX_A> for u8 {
    #[inline(always)]
    fn from(variant: CMDIDX_A) -> Self {
        variant as _
    }
}
impl CMDIDX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMDIDX_A> {
        match self.bits {
            6 => Some(CMDIDX_A::CMD6),
            13 => Some(CMDIDX_A::ACMD13),
            18 => Some(CMDIDX_A::CMD18),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CMD6`"]
    #[inline(always)]
    pub fn is_cmd6(&self) -> bool {
        *self == CMDIDX_A::CMD6
    }
    #[doc = "Checks if the value of the field is `ACMD13`"]
    #[inline(always)]
    pub fn is_acmd13(&self) -> bool {
        *self == CMDIDX_A::ACMD13
    }
    #[doc = "Checks if the value of the field is `CMD18`"]
    #[inline(always)]
    pub fn is_cmd18(&self) -> bool {
        *self == CMDIDX_A::CMD18
    }
}
#[doc = "Field `CMDIDX` writer - Command Index Field Value Select"]
pub type CMDIDX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SD_CMD_SPEC, u8, CMDIDX_A, 6, O>;
impl<'a, const O: u8> CMDIDX_W<'a, O> {
    #[doc = "CMD6"]
    #[inline(always)]
    pub fn cmd6(self) -> &'a mut W {
        self.variant(CMDIDX_A::CMD6)
    }
    #[doc = "ACMD13"]
    #[inline(always)]
    pub fn acmd13(self) -> &'a mut W {
        self.variant(CMDIDX_A::ACMD13)
    }
    #[doc = "CMD18"]
    #[inline(always)]
    pub fn cmd18(self) -> &'a mut W {
        self.variant(CMDIDX_A::CMD18)
    }
}
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
#[doc = "Field `RSPTP` reader - Response Type Select"]
pub type RSPTP_R = crate::FieldReader<u8, RSPTP_A>;
#[doc = "Response Type Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RSPTP_A {
    #[doc = "0: Normal mode Depending on the command, the response type and transfer method are selected in the ACMD\\[1:0\\]
and CMDIDX\\[5:0\\]
bits. At this time, the values for bits 15 to 11 in this register are invalid."]
    _000 = 0,
    #[doc = "3: Extended mode and no response"]
    _011 = 3,
    #[doc = "4: Extended mode and R1, R5, R6, or R7 response"]
    _100 = 4,
    #[doc = "5: Extended mode and R1b response"]
    _101 = 5,
    #[doc = "6: Extended mode and R2 response"]
    _110 = 6,
    #[doc = "7: Extended mode and R3 or R4 response"]
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
#[doc = "Field `RSPTP` writer - Response Type Select"]
pub type RSPTP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SD_CMD_SPEC, u8, RSPTP_A, 3, O>;
impl<'a, const O: u8> RSPTP_W<'a, O> {
    #[doc = "Normal mode Depending on the command, the response type and transfer method are selected in the ACMD\\[1:0\\]
and CMDIDX\\[5:0\\]
bits. At this time, the values for bits 15 to 11 in this register are invalid."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(RSPTP_A::_000)
    }
    #[doc = "Extended mode and no response"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(RSPTP_A::_011)
    }
    #[doc = "Extended mode and R1, R5, R6, or R7 response"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(RSPTP_A::_100)
    }
    #[doc = "Extended mode and R1b response"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(RSPTP_A::_101)
    }
    #[doc = "Extended mode and R2 response"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(RSPTP_A::_110)
    }
    #[doc = "Extended mode and R3 or R4 response"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(RSPTP_A::_111)
    }
}
#[doc = "Field `CMDTP` reader - Data Transfer Select"]
pub type CMDTP_R = crate::BitReader<CMDTP_A>;
#[doc = "Data Transfer Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDTP_A {
    #[doc = "0: Do not include data transfer (bc, bcr, or ac) in command"]
    _0 = 0,
    #[doc = "1: Include data transfer (adtc) in command"]
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
#[doc = "Field `CMDTP` writer - Data Transfer Select"]
pub type CMDTP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_CMD_SPEC, CMDTP_A, O>;
impl<'a, const O: u8> CMDTP_W<'a, O> {
    #[doc = "Do not include data transfer (bc, bcr, or ac) in command"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMDTP_A::_0)
    }
    #[doc = "Include data transfer (adtc) in command"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMDTP_A::_1)
    }
}
#[doc = "Field `CMDRW` reader - Data Transfer Direction Select"]
pub type CMDRW_R = crate::BitReader<CMDRW_A>;
#[doc = "Data Transfer Direction Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDRW_A {
    #[doc = "0: Write (SD/MMC Host Interface → SD card/MMC)"]
    _0 = 0,
    #[doc = "1: Read (SD/MMC Host Interface ← SD card/MMC)"]
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
#[doc = "Field `CMDRW` writer - Data Transfer Direction Select"]
pub type CMDRW_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_CMD_SPEC, CMDRW_A, O>;
impl<'a, const O: u8> CMDRW_W<'a, O> {
    #[doc = "Write (SD/MMC Host Interface → SD card/MMC)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMDRW_A::_0)
    }
    #[doc = "Read (SD/MMC Host Interface ← SD card/MMC)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMDRW_A::_1)
    }
}
#[doc = "Field `TRSTP` reader - Block Transfer Select"]
pub type TRSTP_R = crate::BitReader<TRSTP_A>;
#[doc = "Block Transfer Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRSTP_A {
    #[doc = "0: Single block transfer"]
    _0 = 0,
    #[doc = "1: Multiple blocks transfer"]
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
#[doc = "Field `TRSTP` writer - Block Transfer Select"]
pub type TRSTP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SD_CMD_SPEC, TRSTP_A, O>;
impl<'a, const O: u8> TRSTP_W<'a, O> {
    #[doc = "Single block transfer"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRSTP_A::_0)
    }
    #[doc = "Multiple blocks transfer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRSTP_A::_1)
    }
}
#[doc = "Field `CMD12AT` reader - CMD12 Automatic Issue Select"]
pub type CMD12AT_R = crate::FieldReader<u8, CMD12AT_A>;
#[doc = "CMD12 Automatic Issue Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMD12AT_A {
    #[doc = "0: Automatically issue CMD12 during multiblock transfer"]
    _00 = 0,
    #[doc = "1: Do not automatically issue CMD12 during multiblock transfer"]
    _01 = 1,
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
    pub fn variant(&self) -> Option<CMD12AT_A> {
        match self.bits {
            0 => Some(CMD12AT_A::_00),
            1 => Some(CMD12AT_A::_01),
            _ => None,
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
}
#[doc = "Field `CMD12AT` writer - CMD12 Automatic Issue Select"]
pub type CMD12AT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SD_CMD_SPEC, u8, CMD12AT_A, 2, O>;
impl<'a, const O: u8> CMD12AT_W<'a, O> {
    #[doc = "Automatically issue CMD12 during multiblock transfer"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CMD12AT_A::_00)
    }
    #[doc = "Do not automatically issue CMD12 during multiblock transfer"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CMD12AT_A::_01)
    }
}
impl R {
    #[doc = "Bits 0:5 - Command Index Field Value Select"]
    #[inline(always)]
    pub fn cmdidx(&self) -> CMDIDX_R {
        CMDIDX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Command Type Select"]
    #[inline(always)]
    pub fn acmd(&self) -> ACMD_R {
        ACMD_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - Response Type Select"]
    #[inline(always)]
    pub fn rsptp(&self) -> RSPTP_R {
        RSPTP_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Data Transfer Select"]
    #[inline(always)]
    pub fn cmdtp(&self) -> CMDTP_R {
        CMDTP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Data Transfer Direction Select"]
    #[inline(always)]
    pub fn cmdrw(&self) -> CMDRW_R {
        CMDRW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Block Transfer Select"]
    #[inline(always)]
    pub fn trstp(&self) -> TRSTP_R {
        TRSTP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - CMD12 Automatic Issue Select"]
    #[inline(always)]
    pub fn cmd12at(&self) -> CMD12AT_R {
        CMD12AT_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Command Index Field Value Select"]
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
    #[doc = "Bits 8:10 - Response Type Select"]
    #[inline(always)]
    #[must_use]
    pub fn rsptp(&mut self) -> RSPTP_W<8> {
        RSPTP_W::new(self)
    }
    #[doc = "Bit 11 - Data Transfer Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmdtp(&mut self) -> CMDTP_W<11> {
        CMDTP_W::new(self)
    }
    #[doc = "Bit 12 - Data Transfer Direction Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmdrw(&mut self) -> CMDRW_W<12> {
        CMDRW_W::new(self)
    }
    #[doc = "Bit 13 - Block Transfer Select"]
    #[inline(always)]
    #[must_use]
    pub fn trstp(&mut self) -> TRSTP_W<13> {
        TRSTP_W::new(self)
    }
    #[doc = "Bits 14:15 - CMD12 Automatic Issue Select"]
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
