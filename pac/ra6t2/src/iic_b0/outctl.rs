#[doc = "Register `OUTCTL` reader"]
pub struct R(crate::R<OUTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTCTL` writer"]
pub struct W(crate::W<OUTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTCTL_SPEC>;
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
impl From<crate::W<OUTCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDOC` reader - SDA Output Control"]
pub type SDOC_R = crate::BitReader<SDOC_A>;
#[doc = "SDA Output Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDOC_A {
    #[doc = "0: IIC drives the SDAn pin low."]
    _0 = 0,
    #[doc = "1: IIC releases the SDAn pin."]
    _1 = 1,
}
impl From<SDOC_A> for bool {
    #[inline(always)]
    fn from(variant: SDOC_A) -> Self {
        variant as u8 != 0
    }
}
impl SDOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDOC_A {
        match self.bits {
            false => SDOC_A::_0,
            true => SDOC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDOC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDOC_A::_1
    }
}
#[doc = "Field `SDOC` writer - SDA Output Control"]
pub type SDOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTCTL_SPEC, SDOC_A, O>;
impl<'a, const O: u8> SDOC_W<'a, O> {
    #[doc = "IIC drives the SDAn pin low."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SDOC_A::_0)
    }
    #[doc = "IIC releases the SDAn pin."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SDOC_A::_1)
    }
}
#[doc = "Field `SCOC` reader - SCL Output Control"]
pub type SCOC_R = crate::BitReader<SCOC_A>;
#[doc = "SCL Output Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCOC_A {
    #[doc = "0: IIC drives the SCLn pin low."]
    _0 = 0,
    #[doc = "1: IIC releases the SCLn pin."]
    _1 = 1,
}
impl From<SCOC_A> for bool {
    #[inline(always)]
    fn from(variant: SCOC_A) -> Self {
        variant as u8 != 0
    }
}
impl SCOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCOC_A {
        match self.bits {
            false => SCOC_A::_0,
            true => SCOC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SCOC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SCOC_A::_1
    }
}
#[doc = "Field `SCOC` writer - SCL Output Control"]
pub type SCOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTCTL_SPEC, SCOC_A, O>;
impl<'a, const O: u8> SCOC_W<'a, O> {
    #[doc = "IIC drives the SCLn pin low."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SCOC_A::_0)
    }
    #[doc = "IIC releases the SCLn pin."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SCOC_A::_1)
    }
}
#[doc = "SCL/SDA Output Control Write Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOCWP_AW {
    #[doc = "0: Bits SCOC and SDOC are protected."]
    _0 = 0,
    #[doc = "1: Bits SCOC and SDOC can be written (When writing simultaneously with the value of the target bit). This bit is read as 0."]
    _1 = 1,
}
impl From<SOCWP_AW> for bool {
    #[inline(always)]
    fn from(variant: SOCWP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOCWP` writer - SCL/SDA Output Control Write Protect"]
pub type SOCWP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTCTL_SPEC, SOCWP_AW, O>;
impl<'a, const O: u8> SOCWP_W<'a, O> {
    #[doc = "Bits SCOC and SDOC are protected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SOCWP_AW::_0)
    }
    #[doc = "Bits SCOC and SDOC can be written (When writing simultaneously with the value of the target bit). This bit is read as 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SOCWP_AW::_1)
    }
}
#[doc = "Field `EXCYC` reader - Extra SCL Clock Cycle Output"]
pub type EXCYC_R = crate::BitReader<EXCYC_A>;
#[doc = "Extra SCL Clock Cycle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXCYC_A {
    #[doc = "0: Does not output an extra SCL clock cycle (default)."]
    _0 = 0,
    #[doc = "1: Outputs an extra SCL clock cycle."]
    _1 = 1,
}
impl From<EXCYC_A> for bool {
    #[inline(always)]
    fn from(variant: EXCYC_A) -> Self {
        variant as u8 != 0
    }
}
impl EXCYC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXCYC_A {
        match self.bits {
            false => EXCYC_A::_0,
            true => EXCYC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EXCYC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EXCYC_A::_1
    }
}
#[doc = "Field `EXCYC` writer - Extra SCL Clock Cycle Output"]
pub type EXCYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTCTL_SPEC, EXCYC_A, O>;
impl<'a, const O: u8> EXCYC_W<'a, O> {
    #[doc = "Does not output an extra SCL clock cycle (default)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EXCYC_A::_0)
    }
    #[doc = "Outputs an extra SCL clock cycle."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EXCYC_A::_1)
    }
}
#[doc = "Field `SDOD` reader - SDA Output Delay"]
pub type SDOD_R = crate::FieldReader<u8, SDOD_A>;
#[doc = "SDA Output Delay\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SDOD_A {
    #[doc = "0: No output delay"]
    _000 = 0,
    #[doc = "1: 1 IICÏ\u{86} cycle (When OUTCTL.SDODCS = 0 (IICÏ\u{86})) 1 or 2 IICÏ\u{86} cycles (When OUTCTL.SDODCS = 1 (IICÏ\u{86}/2))"]
    _001 = 1,
    #[doc = "2: 2 IICÏ\u{86} cycles (When OUTCTL.SDODCS = 0 (IICÏ\u{86})) 3 or 4 IICÏ\u{86} cycles (When OUTCTL.SDODCS = 1 (IICÏ\u{86}/2))"]
    _010 = 2,
    #[doc = "3: 3 IICÏ\u{86} cycles (When OUTCTL.SDODCS = 0 (IICÏ\u{86})) 5 or 6 IICÏ\u{86} cycles (When OUTCTL.SDODCS = 1 (IICÏ\u{86}/2))"]
    _011 = 3,
    #[doc = "4: 4 IICÏ\u{86} cycles (When OUTCTL.SDODCS = 0 (IICÏ\u{86})) 7 or 8 IICÏ\u{86} cycles (When OUTCTL.SDODCS = 1 (IICÏ\u{86}/2))"]
    _100 = 4,
    #[doc = "5: 5 IICÏ\u{86} cycles (When OUTCTL.SDODCS = 0 (IICÏ\u{86})) 9 or 10 IICÏ\u{86} cycles (When OUTCTL.SDODCS = 1 (IICÏ\u{86}/2))"]
    _101 = 5,
    #[doc = "6: 6 IICÏ\u{86} cycles (When OUTCTL.SDODCS = 0 (IICÏ\u{86})) 11 or 12 IICÏ\u{86} cycles (When OUTCTL.SDODCS = 1 (IICÏ\u{86}/2))"]
    _110 = 6,
    #[doc = "7: 7 IICÏ\u{86} cycles (When OUTCTL.SDODCS = 0 (IICÏ\u{86})) 13 or 14 IICÏ\u{86} cycles (When OUTCTL.SDODCS = 1 (IICÏ\u{86}/2))"]
    _111 = 7,
}
impl From<SDOD_A> for u8 {
    #[inline(always)]
    fn from(variant: SDOD_A) -> Self {
        variant as _
    }
}
impl SDOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDOD_A {
        match self.bits {
            0 => SDOD_A::_000,
            1 => SDOD_A::_001,
            2 => SDOD_A::_010,
            3 => SDOD_A::_011,
            4 => SDOD_A::_100,
            5 => SDOD_A::_101,
            6 => SDOD_A::_110,
            7 => SDOD_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SDOD_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SDOD_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SDOD_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == SDOD_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == SDOD_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == SDOD_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == SDOD_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == SDOD_A::_111
    }
}
#[doc = "Field `SDOD` writer - SDA Output Delay"]
pub type SDOD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, OUTCTL_SPEC, u8, SDOD_A, 3, O>;
impl<'a, const O: u8> SDOD_W<'a, O> {
    #[doc = "No output delay"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(SDOD_A::_000)
    }
    #[doc = "1 IICÏ\u{86} cycle (When OUTCTL.SDODCS = 0 (IICÏ\u{86})) 1 or 2 IICÏ\u{86} cycles (When OUTCTL.SDODCS = 1 (IICÏ\u{86}/2))"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(SDOD_A::_001)
    }
    #[doc = "2 IICÏ\u{86} cycles (When OUTCTL.SDODCS = 0 (IICÏ\u{86})) 3 or 4 IICÏ\u{86} cycles (When OUTCTL.SDODCS = 1 (IICÏ\u{86}/2))"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(SDOD_A::_010)
    }
    #[doc = "3 IICÏ\u{86} cycles (When OUTCTL.SDODCS = 0 (IICÏ\u{86})) 5 or 6 IICÏ\u{86} cycles (When OUTCTL.SDODCS = 1 (IICÏ\u{86}/2))"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(SDOD_A::_011)
    }
    #[doc = "4 IICÏ\u{86} cycles (When OUTCTL.SDODCS = 0 (IICÏ\u{86})) 7 or 8 IICÏ\u{86} cycles (When OUTCTL.SDODCS = 1 (IICÏ\u{86}/2))"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(SDOD_A::_100)
    }
    #[doc = "5 IICÏ\u{86} cycles (When OUTCTL.SDODCS = 0 (IICÏ\u{86})) 9 or 10 IICÏ\u{86} cycles (When OUTCTL.SDODCS = 1 (IICÏ\u{86}/2))"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(SDOD_A::_101)
    }
    #[doc = "6 IICÏ\u{86} cycles (When OUTCTL.SDODCS = 0 (IICÏ\u{86})) 11 or 12 IICÏ\u{86} cycles (When OUTCTL.SDODCS = 1 (IICÏ\u{86}/2))"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(SDOD_A::_110)
    }
    #[doc = "7 IICÏ\u{86} cycles (When OUTCTL.SDODCS = 0 (IICÏ\u{86})) 13 or 14 IICÏ\u{86} cycles (When OUTCTL.SDODCS = 1 (IICÏ\u{86}/2))"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(SDOD_A::_111)
    }
}
#[doc = "Field `SDODCS` reader - SDA Output Delay Clock Source Selection"]
pub type SDODCS_R = crate::BitReader<SDODCS_A>;
#[doc = "SDA Output Delay Clock Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDODCS_A {
    #[doc = "0: The internal reference clock (IICÏ\u{86}) is selected as the clock source of the SDA output delay counter."]
    _0 = 0,
    #[doc = "1: The internal reference clock divided by 2 (IICÏ\u{86}/2) is selected as the clock source of the SDA output delay counter."]
    _1 = 1,
}
impl From<SDODCS_A> for bool {
    #[inline(always)]
    fn from(variant: SDODCS_A) -> Self {
        variant as u8 != 0
    }
}
impl SDODCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDODCS_A {
        match self.bits {
            false => SDODCS_A::_0,
            true => SDODCS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDODCS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDODCS_A::_1
    }
}
#[doc = "Field `SDODCS` writer - SDA Output Delay Clock Source Selection"]
pub type SDODCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTCTL_SPEC, SDODCS_A, O>;
impl<'a, const O: u8> SDODCS_W<'a, O> {
    #[doc = "The internal reference clock (IICÏ\u{86}) is selected as the clock source of the SDA output delay counter."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SDODCS_A::_0)
    }
    #[doc = "The internal reference clock divided by 2 (IICÏ\u{86}/2) is selected as the clock source of the SDA output delay counter."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SDODCS_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - SDA Output Control"]
    #[inline(always)]
    pub fn sdoc(&self) -> SDOC_R {
        SDOC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCL Output Control"]
    #[inline(always)]
    pub fn scoc(&self) -> SCOC_R {
        SCOC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Extra SCL Clock Cycle Output"]
    #[inline(always)]
    pub fn excyc(&self) -> EXCYC_R {
        EXCYC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:10 - SDA Output Delay"]
    #[inline(always)]
    pub fn sdod(&self) -> SDOD_R {
        SDOD_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - SDA Output Delay Clock Source Selection"]
    #[inline(always)]
    pub fn sdodcs(&self) -> SDODCS_R {
        SDODCS_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SDA Output Control"]
    #[inline(always)]
    #[must_use]
    pub fn sdoc(&mut self) -> SDOC_W<0> {
        SDOC_W::new(self)
    }
    #[doc = "Bit 1 - SCL Output Control"]
    #[inline(always)]
    #[must_use]
    pub fn scoc(&mut self) -> SCOC_W<1> {
        SCOC_W::new(self)
    }
    #[doc = "Bit 2 - SCL/SDA Output Control Write Protect"]
    #[inline(always)]
    #[must_use]
    pub fn socwp(&mut self) -> SOCWP_W<2> {
        SOCWP_W::new(self)
    }
    #[doc = "Bit 4 - Extra SCL Clock Cycle Output"]
    #[inline(always)]
    #[must_use]
    pub fn excyc(&mut self) -> EXCYC_W<4> {
        EXCYC_W::new(self)
    }
    #[doc = "Bits 8:10 - SDA Output Delay"]
    #[inline(always)]
    #[must_use]
    pub fn sdod(&mut self) -> SDOD_W<8> {
        SDOD_W::new(self)
    }
    #[doc = "Bit 15 - SDA Output Delay Clock Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn sdodcs(&mut self) -> SDODCS_W<15> {
        SDODCS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outctl](index.html) module"]
pub struct OUTCTL_SPEC;
impl crate::RegisterSpec for OUTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outctl::R](R) reader structure"]
impl crate::Readable for OUTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [outctl::W](W) writer structure"]
impl crate::Writable for OUTCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUTCTL to value 0x03"]
impl crate::Resettable for OUTCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
