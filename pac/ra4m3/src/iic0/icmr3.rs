#[doc = "Register `ICMR3` reader"]
pub struct R(crate::R<ICMR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICMR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICMR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICMR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICMR3` writer"]
pub struct W(crate::W<ICMR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICMR3_SPEC>;
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
impl From<crate::W<ICMR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICMR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NF` reader - Noise Filter Stage Select"]
pub type NF_R = crate::FieldReader<u8, NF_A>;
#[doc = "Noise Filter Stage Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NF_A {
    #[doc = "0: Filter out noise of up to 1 IIC-phi cycle (single-stage filter)"]
    _00 = 0,
    #[doc = "1: Filter out noise of up to 2 IIC-phi cycles (2-stage filter)"]
    _01 = 1,
    #[doc = "2: Filter out noise of up to 3 IIC-phi cycles (3-stage filter)"]
    _10 = 2,
    #[doc = "3: Filter out noise of up to 4 IIC-phi cycles (4-stage filter)"]
    _11 = 3,
}
impl From<NF_A> for u8 {
    #[inline(always)]
    fn from(variant: NF_A) -> Self {
        variant as _
    }
}
impl NF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NF_A {
        match self.bits {
            0 => NF_A::_00,
            1 => NF_A::_01,
            2 => NF_A::_10,
            3 => NF_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == NF_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == NF_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == NF_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == NF_A::_11
    }
}
#[doc = "Field `NF` writer - Noise Filter Stage Select"]
pub type NF_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, ICMR3_SPEC, u8, NF_A, 2, O>;
impl<'a, const O: u8> NF_W<'a, O> {
    #[doc = "Filter out noise of up to 1 IIC-phi cycle (single-stage filter)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(NF_A::_00)
    }
    #[doc = "Filter out noise of up to 2 IIC-phi cycles (2-stage filter)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(NF_A::_01)
    }
    #[doc = "Filter out noise of up to 3 IIC-phi cycles (3-stage filter)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(NF_A::_10)
    }
    #[doc = "Filter out noise of up to 4 IIC-phi cycles (4-stage filter)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(NF_A::_11)
    }
}
#[doc = "Field `ACKBR` reader - Receive Acknowledge"]
pub type ACKBR_R = crate::BitReader<ACKBR_A>;
#[doc = "Receive Acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACKBR_A {
    #[doc = "0: 0 received as the acknowledge bit (ACK reception)"]
    _0 = 0,
    #[doc = "1: 1 received as the acknowledge bit (NACK reception)"]
    _1 = 1,
}
impl From<ACKBR_A> for bool {
    #[inline(always)]
    fn from(variant: ACKBR_A) -> Self {
        variant as u8 != 0
    }
}
impl ACKBR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACKBR_A {
        match self.bits {
            false => ACKBR_A::_0,
            true => ACKBR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACKBR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACKBR_A::_1
    }
}
#[doc = "Field `ACKBT` reader - Transmit Acknowledge"]
pub type ACKBT_R = crate::BitReader<ACKBT_A>;
#[doc = "Transmit Acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACKBT_A {
    #[doc = "0: Send 0 as the acknowledge bit (ACK transmission)"]
    _0 = 0,
    #[doc = "1: Send 1 as the acknowledge bit (NACK transmission)"]
    _1 = 1,
}
impl From<ACKBT_A> for bool {
    #[inline(always)]
    fn from(variant: ACKBT_A) -> Self {
        variant as u8 != 0
    }
}
impl ACKBT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACKBT_A {
        match self.bits {
            false => ACKBT_A::_0,
            true => ACKBT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACKBT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACKBT_A::_1
    }
}
#[doc = "Field `ACKBT` writer - Transmit Acknowledge"]
pub type ACKBT_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICMR3_SPEC, ACKBT_A, O>;
impl<'a, const O: u8> ACKBT_W<'a, O> {
    #[doc = "Send 0 as the acknowledge bit (ACK transmission)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACKBT_A::_0)
    }
    #[doc = "Send 1 as the acknowledge bit (NACK transmission)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACKBT_A::_1)
    }
}
#[doc = "Field `ACKWP` reader - ACKBT Write Protect"]
pub type ACKWP_R = crate::BitReader<ACKWP_A>;
#[doc = "ACKBT Write Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACKWP_A {
    #[doc = "0: Write protect ACKBT bit"]
    _0 = 0,
    #[doc = "1: Write enable ACKBT bit"]
    _1 = 1,
}
impl From<ACKWP_A> for bool {
    #[inline(always)]
    fn from(variant: ACKWP_A) -> Self {
        variant as u8 != 0
    }
}
impl ACKWP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACKWP_A {
        match self.bits {
            false => ACKWP_A::_0,
            true => ACKWP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACKWP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACKWP_A::_1
    }
}
#[doc = "Field `ACKWP` writer - ACKBT Write Protect"]
pub type ACKWP_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICMR3_SPEC, ACKWP_A, O>;
impl<'a, const O: u8> ACKWP_W<'a, O> {
    #[doc = "Write protect ACKBT bit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACKWP_A::_0)
    }
    #[doc = "Write enable ACKBT bit"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACKWP_A::_1)
    }
}
#[doc = "Field `RDRFS` reader - RDRF Flag Set Timing Select"]
pub type RDRFS_R = crate::BitReader<RDRFS_A>;
#[doc = "RDRF Flag Set Timing Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDRFS_A {
    #[doc = "0: Set the RDRF flag on the rising edge of the 9th SCL clock cycle. The SCLn line is not held low on the falling edge of the 8th clock cycle."]
    _0 = 0,
    #[doc = "1: Set the RDRF flag on the rising edge of the 8th SCL clock cycle. The SCLn line is held low on the falling edge of the 8th clock cycle."]
    _1 = 1,
}
impl From<RDRFS_A> for bool {
    #[inline(always)]
    fn from(variant: RDRFS_A) -> Self {
        variant as u8 != 0
    }
}
impl RDRFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDRFS_A {
        match self.bits {
            false => RDRFS_A::_0,
            true => RDRFS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDRFS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDRFS_A::_1
    }
}
#[doc = "Field `RDRFS` writer - RDRF Flag Set Timing Select"]
pub type RDRFS_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICMR3_SPEC, RDRFS_A, O>;
impl<'a, const O: u8> RDRFS_W<'a, O> {
    #[doc = "Set the RDRF flag on the rising edge of the 9th SCL clock cycle. The SCLn line is not held low on the falling edge of the 8th clock cycle."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RDRFS_A::_0)
    }
    #[doc = "Set the RDRF flag on the rising edge of the 8th SCL clock cycle. The SCLn line is held low on the falling edge of the 8th clock cycle."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RDRFS_A::_1)
    }
}
#[doc = "Field `WAIT` reader - Low-hold is released by reading ICDRR."]
pub type WAIT_R = crate::BitReader<WAIT_A>;
#[doc = "Low-hold is released by reading ICDRR.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAIT_A {
    #[doc = "0: No wait (The SCLn line is not held low during the period between the 9th clock cycle and the 1st clock cycle.)"]
    _0 = 0,
    #[doc = "1: Wait (The SCLn line is held low during the period between the 9th clock cycle and the 1st clock cycle.)"]
    _1 = 1,
}
impl From<WAIT_A> for bool {
    #[inline(always)]
    fn from(variant: WAIT_A) -> Self {
        variant as u8 != 0
    }
}
impl WAIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAIT_A {
        match self.bits {
            false => WAIT_A::_0,
            true => WAIT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WAIT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WAIT_A::_1
    }
}
#[doc = "Field `WAIT` writer - Low-hold is released by reading ICDRR."]
pub type WAIT_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICMR3_SPEC, WAIT_A, O>;
impl<'a, const O: u8> WAIT_W<'a, O> {
    #[doc = "No wait (The SCLn line is not held low during the period between the 9th clock cycle and the 1st clock cycle.)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WAIT_A::_0)
    }
    #[doc = "Wait (The SCLn line is held low during the period between the 9th clock cycle and the 1st clock cycle.)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WAIT_A::_1)
    }
}
#[doc = "Field `SMBS` reader - SMBus/I2C Bus Select"]
pub type SMBS_R = crate::BitReader<SMBS_A>;
#[doc = "SMBus/I2C Bus Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMBS_A {
    #[doc = "0: Select I2C Bus"]
    _0 = 0,
    #[doc = "1: Select SMBus"]
    _1 = 1,
}
impl From<SMBS_A> for bool {
    #[inline(always)]
    fn from(variant: SMBS_A) -> Self {
        variant as u8 != 0
    }
}
impl SMBS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMBS_A {
        match self.bits {
            false => SMBS_A::_0,
            true => SMBS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SMBS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SMBS_A::_1
    }
}
#[doc = "Field `SMBS` writer - SMBus/I2C Bus Select"]
pub type SMBS_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICMR3_SPEC, SMBS_A, O>;
impl<'a, const O: u8> SMBS_W<'a, O> {
    #[doc = "Select I2C Bus"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SMBS_A::_0)
    }
    #[doc = "Select SMBus"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SMBS_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Noise Filter Stage Select"]
    #[inline(always)]
    pub fn nf(&self) -> NF_R {
        NF_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Receive Acknowledge"]
    #[inline(always)]
    pub fn ackbr(&self) -> ACKBR_R {
        ACKBR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Acknowledge"]
    #[inline(always)]
    pub fn ackbt(&self) -> ACKBT_R {
        ACKBT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ACKBT Write Protect"]
    #[inline(always)]
    pub fn ackwp(&self) -> ACKWP_R {
        ACKWP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RDRF Flag Set Timing Select"]
    #[inline(always)]
    pub fn rdrfs(&self) -> RDRFS_R {
        RDRFS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Low-hold is released by reading ICDRR."]
    #[inline(always)]
    pub fn wait(&self) -> WAIT_R {
        WAIT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SMBus/I2C Bus Select"]
    #[inline(always)]
    pub fn smbs(&self) -> SMBS_R {
        SMBS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Noise Filter Stage Select"]
    #[inline(always)]
    #[must_use]
    pub fn nf(&mut self) -> NF_W<0> {
        NF_W::new(self)
    }
    #[doc = "Bit 3 - Transmit Acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn ackbt(&mut self) -> ACKBT_W<3> {
        ACKBT_W::new(self)
    }
    #[doc = "Bit 4 - ACKBT Write Protect"]
    #[inline(always)]
    #[must_use]
    pub fn ackwp(&mut self) -> ACKWP_W<4> {
        ACKWP_W::new(self)
    }
    #[doc = "Bit 5 - RDRF Flag Set Timing Select"]
    #[inline(always)]
    #[must_use]
    pub fn rdrfs(&mut self) -> RDRFS_W<5> {
        RDRFS_W::new(self)
    }
    #[doc = "Bit 6 - Low-hold is released by reading ICDRR."]
    #[inline(always)]
    #[must_use]
    pub fn wait(&mut self) -> WAIT_W<6> {
        WAIT_W::new(self)
    }
    #[doc = "Bit 7 - SMBus/I2C Bus Select"]
    #[inline(always)]
    #[must_use]
    pub fn smbs(&mut self) -> SMBS_W<7> {
        SMBS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Bus Mode Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icmr3](index.html) module"]
pub struct ICMR3_SPEC;
impl crate::RegisterSpec for ICMR3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [icmr3::R](R) reader structure"]
impl crate::Readable for ICMR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icmr3::W](W) writer structure"]
impl crate::Writable for ICMR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICMR3 to value 0"]
impl crate::Resettable for ICMR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
