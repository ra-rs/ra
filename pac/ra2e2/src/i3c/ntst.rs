#[doc = "Register `NTST` reader"]
pub struct R(crate::R<NTST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NTST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NTST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NTST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NTST` writer"]
pub struct W(crate::W<NTST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NTST_SPEC>;
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
impl From<crate::W<NTST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NTST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDBEF0` reader - Normal Transmit Data Buffer Empty Flag 0"]
pub type TDBEF0_R = crate::BitReader<TDBEF0_A>;
#[doc = "Normal Transmit Data Buffer Empty Flag 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDBEF0_A {
    #[doc = "0: For I2C protocol mode: PRTS.PRTMD bit = 1. Normal Transmit Data Buffer 0 contains transmit data. For I3C protocol mode: PRTS.PRTMD bit = 0. The number of empties in the Normal Transmit Data Buffer 0 is less than the NTBTHCTL0.TXDBTH\\[2:0\\]
threshold."]
    _0 = 0,
    #[doc = "1: For I2C protocol mode: PRTS.PRTMD bit = 1. Normal Transmit Data Buffer 0 contains no transmit data. For I3C protocol mode: PRTS.PRTMD bit = 0. The number of empties in the Normal Transmit Data Buffer 0 is the NTBTHCTL0.TXDBTH\\[2:0\\]
threshold or more."]
    _1 = 1,
}
impl From<TDBEF0_A> for bool {
    #[inline(always)]
    fn from(variant: TDBEF0_A) -> Self {
        variant as u8 != 0
    }
}
impl TDBEF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDBEF0_A {
        match self.bits {
            false => TDBEF0_A::_0,
            true => TDBEF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDBEF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDBEF0_A::_1
    }
}
#[doc = "Field `TDBEF0` writer - Normal Transmit Data Buffer Empty Flag 0"]
pub type TDBEF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, NTST_SPEC, TDBEF0_A, O>;
impl<'a, const O: u8> TDBEF0_W<'a, O> {
    #[doc = "For I2C protocol mode: PRTS.PRTMD bit = 1. Normal Transmit Data Buffer 0 contains transmit data. For I3C protocol mode: PRTS.PRTMD bit = 0. The number of empties in the Normal Transmit Data Buffer 0 is less than the NTBTHCTL0.TXDBTH\\[2:0\\]
threshold."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDBEF0_A::_0)
    }
    #[doc = "For I2C protocol mode: PRTS.PRTMD bit = 1. Normal Transmit Data Buffer 0 contains no transmit data. For I3C protocol mode: PRTS.PRTMD bit = 0. The number of empties in the Normal Transmit Data Buffer 0 is the NTBTHCTL0.TXDBTH\\[2:0\\]
threshold or more."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDBEF0_A::_1)
    }
}
#[doc = "Field `RDBFF0` reader - Normal Receive Data Buffer Full Flag 0"]
pub type RDBFF0_R = crate::BitReader<RDBFF0_A>;
#[doc = "Normal Receive Data Buffer Full Flag 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDBFF0_A {
    #[doc = "0: For I2C protocol mode: PRTS.PRTMD bit = 1. Normal Receive Data Buffer0 contains no receive data. For I3C Protocol mode: PRTS.PRTMD bit = 0. The number of entries in the Normal Receive Data Buffer 0 is less than the NTBTHCTL0.RXDBTH\\[2:0\\]
threshold."]
    _0 = 0,
    #[doc = "1: For I2C protocol mode: PRTS.PRTMD bit = 1. Normal Receive Data Buffer0 contains receive data. For I3C Protocol mode: PRTS.PRTMD bit = 0. The number of entries in the Normal Receive Data Buffer 0 is the NTBTHCTL0.RXDBTH\\[2:0\\]
threshold or more."]
    _1 = 1,
}
impl From<RDBFF0_A> for bool {
    #[inline(always)]
    fn from(variant: RDBFF0_A) -> Self {
        variant as u8 != 0
    }
}
impl RDBFF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDBFF0_A {
        match self.bits {
            false => RDBFF0_A::_0,
            true => RDBFF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDBFF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDBFF0_A::_1
    }
}
#[doc = "Field `RDBFF0` writer - Normal Receive Data Buffer Full Flag 0"]
pub type RDBFF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, NTST_SPEC, RDBFF0_A, O>;
impl<'a, const O: u8> RDBFF0_W<'a, O> {
    #[doc = "For I2C protocol mode: PRTS.PRTMD bit = 1. Normal Receive Data Buffer0 contains no receive data. For I3C Protocol mode: PRTS.PRTMD bit = 0. The number of entries in the Normal Receive Data Buffer 0 is less than the NTBTHCTL0.RXDBTH\\[2:0\\]
threshold."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RDBFF0_A::_0)
    }
    #[doc = "For I2C protocol mode: PRTS.PRTMD bit = 1. Normal Receive Data Buffer0 contains receive data. For I3C Protocol mode: PRTS.PRTMD bit = 0. The number of entries in the Normal Receive Data Buffer 0 is the NTBTHCTL0.RXDBTH\\[2:0\\]
threshold or more."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RDBFF0_A::_1)
    }
}
#[doc = "Field `IBIQEFF` reader - Normal IBI Queue Empty/Full Flag"]
pub type IBIQEFF_R = crate::BitReader<IBIQEFF_A>;
#[doc = "Normal IBI Queue Empty/Full Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IBIQEFF_A {
    #[doc = "0: For I3C protocol mode (Master): PRTS.PRTMD bit = 0, PRSST.CRMS bit = 1. The number of IBI Status Queue entries is the NQTHCTL.IBIQTH threshold or less. For I3C protocol mode (Slave) : PRTS.PRTMD bit = 0, PRSST.CRMS bit = 0. If the NQTHCTL.IBIQTH = 0: The number of IBI Data Buffer empties is less than the IBI Data Buffer size. If the NQTHCTL.IBIQTH is other than 0: The number of IBI Data Buffer empties is less than the NQTHCTL.IBIQTH threshold."]
    _0 = 0,
    #[doc = "1: For I3C protocol mode (Master): PRTS.PRTMD bit = 0, PRSST.CRMS bit = 1. The number of IBI Status Queue entries is more than the NQTHCTL.IBIQTH threshold. For I3C protocol mode (Slave) : PRTS.PRTMD bit = 0, PRSST.CRMS bit = 0. If the NQTHCTL.IBIQTH = 0: The number of IBI Data Buffer empties is the IBI Data Buffer size. If the NQTHCTL.IBIQTH is other than 0: The number of IBI Data Buffer empties is the NQTHCTL.IBIQTH threshold or more."]
    _1 = 1,
}
impl From<IBIQEFF_A> for bool {
    #[inline(always)]
    fn from(variant: IBIQEFF_A) -> Self {
        variant as u8 != 0
    }
}
impl IBIQEFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IBIQEFF_A {
        match self.bits {
            false => IBIQEFF_A::_0,
            true => IBIQEFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IBIQEFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IBIQEFF_A::_1
    }
}
#[doc = "Field `IBIQEFF` writer - Normal IBI Queue Empty/Full Flag"]
pub type IBIQEFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, NTST_SPEC, IBIQEFF_A, O>;
impl<'a, const O: u8> IBIQEFF_W<'a, O> {
    #[doc = "For I3C protocol mode (Master): PRTS.PRTMD bit = 0, PRSST.CRMS bit = 1. The number of IBI Status Queue entries is the NQTHCTL.IBIQTH threshold or less. For I3C protocol mode (Slave) : PRTS.PRTMD bit = 0, PRSST.CRMS bit = 0. If the NQTHCTL.IBIQTH = 0: The number of IBI Data Buffer empties is less than the IBI Data Buffer size. If the NQTHCTL.IBIQTH is other than 0: The number of IBI Data Buffer empties is less than the NQTHCTL.IBIQTH threshold."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IBIQEFF_A::_0)
    }
    #[doc = "For I3C protocol mode (Master): PRTS.PRTMD bit = 0, PRSST.CRMS bit = 1. The number of IBI Status Queue entries is more than the NQTHCTL.IBIQTH threshold. For I3C protocol mode (Slave) : PRTS.PRTMD bit = 0, PRSST.CRMS bit = 0. If the NQTHCTL.IBIQTH = 0: The number of IBI Data Buffer empties is the IBI Data Buffer size. If the NQTHCTL.IBIQTH is other than 0: The number of IBI Data Buffer empties is the NQTHCTL.IBIQTH threshold or more."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IBIQEFF_A::_1)
    }
}
#[doc = "Field `CMDQEF` reader - Normal Command Queue Empty Flag"]
pub type CMDQEF_R = crate::BitReader<CMDQEF_A>;
#[doc = "Normal Command Queue Empty Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDQEF_A {
    #[doc = "0: If the NQTHCTL.CMDQTH = 0: The number of Command Queue empties is less than the Command Queue size. If the NQTHCTL.CMDQTH is other than 0: The number of Command Queue empties is less than the NQTHCTL.CMDQTH threshold."]
    _0 = 0,
    #[doc = "1: If the NQTHCTL.CMDQTH = 0: The number of Command Queue empties is the Command Queue size. If the NQTHCTL.CMDQTH is other than 0: 1: The number of Command Queue empties is the NQTHCTL.CMDQTH threshold or more."]
    _1 = 1,
}
impl From<CMDQEF_A> for bool {
    #[inline(always)]
    fn from(variant: CMDQEF_A) -> Self {
        variant as u8 != 0
    }
}
impl CMDQEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDQEF_A {
        match self.bits {
            false => CMDQEF_A::_0,
            true => CMDQEF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMDQEF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMDQEF_A::_1
    }
}
#[doc = "Field `CMDQEF` writer - Normal Command Queue Empty Flag"]
pub type CMDQEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, NTST_SPEC, CMDQEF_A, O>;
impl<'a, const O: u8> CMDQEF_W<'a, O> {
    #[doc = "If the NQTHCTL.CMDQTH = 0: The number of Command Queue empties is less than the Command Queue size. If the NQTHCTL.CMDQTH is other than 0: The number of Command Queue empties is less than the NQTHCTL.CMDQTH threshold."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMDQEF_A::_0)
    }
    #[doc = "If the NQTHCTL.CMDQTH = 0: The number of Command Queue empties is the Command Queue size. If the NQTHCTL.CMDQTH is other than 0: 1: The number of Command Queue empties is the NQTHCTL.CMDQTH threshold or more."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMDQEF_A::_1)
    }
}
#[doc = "Field `RSPQFF` reader - Normal Response Queue Full Flag"]
pub type RSPQFF_R = crate::BitReader<RSPQFF_A>;
#[doc = "Normal Response Queue Full Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSPQFF_A {
    #[doc = "0: The number of Response Queue entries is the NQTHCTL.RSPQTH threshold or less."]
    _0 = 0,
    #[doc = "1: The number of Response Queue entries is more than the NQTHCTL.RSPQTH threshold."]
    _1 = 1,
}
impl From<RSPQFF_A> for bool {
    #[inline(always)]
    fn from(variant: RSPQFF_A) -> Self {
        variant as u8 != 0
    }
}
impl RSPQFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSPQFF_A {
        match self.bits {
            false => RSPQFF_A::_0,
            true => RSPQFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSPQFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSPQFF_A::_1
    }
}
#[doc = "Field `RSPQFF` writer - Normal Response Queue Full Flag"]
pub type RSPQFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, NTST_SPEC, RSPQFF_A, O>;
impl<'a, const O: u8> RSPQFF_W<'a, O> {
    #[doc = "The number of Response Queue entries is the NQTHCTL.RSPQTH threshold or less."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSPQFF_A::_0)
    }
    #[doc = "The number of Response Queue entries is more than the NQTHCTL.RSPQTH threshold."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSPQFF_A::_1)
    }
}
#[doc = "Field `TABTF` reader - Normal Transfer Abort Flag"]
pub type TABTF_R = crate::BitReader<TABTF_A>;
#[doc = "Normal Transfer Abort Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TABTF_A {
    #[doc = "0: Transfer Abort does not occur."]
    _0 = 0,
    #[doc = "1: Transfer Abort occur. To clear, write 0 to this bit after 1 state is read."]
    _1 = 1,
}
impl From<TABTF_A> for bool {
    #[inline(always)]
    fn from(variant: TABTF_A) -> Self {
        variant as u8 != 0
    }
}
impl TABTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TABTF_A {
        match self.bits {
            false => TABTF_A::_0,
            true => TABTF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TABTF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TABTF_A::_1
    }
}
#[doc = "Field `TABTF` writer - Normal Transfer Abort Flag"]
pub type TABTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, NTST_SPEC, TABTF_A, O>;
impl<'a, const O: u8> TABTF_W<'a, O> {
    #[doc = "Transfer Abort does not occur."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TABTF_A::_0)
    }
    #[doc = "Transfer Abort occur. To clear, write 0 to this bit after 1 state is read."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TABTF_A::_1)
    }
}
#[doc = "Field `TEF` reader - Normal Transfer Error Flag"]
pub type TEF_R = crate::BitReader<TEF_A>;
#[doc = "Normal Transfer Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEF_A {
    #[doc = "0: Transfer Error does not occur."]
    _0 = 0,
    #[doc = "1: Transfer Error occurs. To clear, write 0 to this bit after 1 state is read."]
    _1 = 1,
}
impl From<TEF_A> for bool {
    #[inline(always)]
    fn from(variant: TEF_A) -> Self {
        variant as u8 != 0
    }
}
impl TEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEF_A {
        match self.bits {
            false => TEF_A::_0,
            true => TEF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TEF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TEF_A::_1
    }
}
#[doc = "Field `TEF` writer - Normal Transfer Error Flag"]
pub type TEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, NTST_SPEC, TEF_A, O>;
impl<'a, const O: u8> TEF_W<'a, O> {
    #[doc = "Transfer Error does not occur."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TEF_A::_0)
    }
    #[doc = "Transfer Error occurs. To clear, write 0 to this bit after 1 state is read."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TEF_A::_1)
    }
}
#[doc = "Field `RSQFF` reader - Normal Receive Status Queue Full Flag"]
pub type RSQFF_R = crate::BitReader<RSQFF_A>;
#[doc = "Normal Receive Status Queue Full Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSQFF_A {
    #[doc = "0: The number of Receive Status Queue entries is the NRQTHCTL.RSQTH threshold or less."]
    _0 = 0,
    #[doc = "1: The number of Receive Status Queue entries is more than the NRQTHCTL.RSQTH threshold."]
    _1 = 1,
}
impl From<RSQFF_A> for bool {
    #[inline(always)]
    fn from(variant: RSQFF_A) -> Self {
        variant as u8 != 0
    }
}
impl RSQFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSQFF_A {
        match self.bits {
            false => RSQFF_A::_0,
            true => RSQFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSQFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSQFF_A::_1
    }
}
#[doc = "Field `RSQFF` writer - Normal Receive Status Queue Full Flag"]
pub type RSQFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, NTST_SPEC, RSQFF_A, O>;
impl<'a, const O: u8> RSQFF_W<'a, O> {
    #[doc = "The number of Receive Status Queue entries is the NRQTHCTL.RSQTH threshold or less."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSQFF_A::_0)
    }
    #[doc = "The number of Receive Status Queue entries is more than the NRQTHCTL.RSQTH threshold."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSQFF_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Normal Transmit Data Buffer Empty Flag 0"]
    #[inline(always)]
    pub fn tdbef0(&self) -> TDBEF0_R {
        TDBEF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Normal Receive Data Buffer Full Flag 0"]
    #[inline(always)]
    pub fn rdbff0(&self) -> RDBFF0_R {
        RDBFF0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Normal IBI Queue Empty/Full Flag"]
    #[inline(always)]
    pub fn ibiqeff(&self) -> IBIQEFF_R {
        IBIQEFF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Normal Command Queue Empty Flag"]
    #[inline(always)]
    pub fn cmdqef(&self) -> CMDQEF_R {
        CMDQEF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Normal Response Queue Full Flag"]
    #[inline(always)]
    pub fn rspqff(&self) -> RSPQFF_R {
        RSPQFF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Normal Transfer Abort Flag"]
    #[inline(always)]
    pub fn tabtf(&self) -> TABTF_R {
        TABTF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - Normal Transfer Error Flag"]
    #[inline(always)]
    pub fn tef(&self) -> TEF_R {
        TEF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 20 - Normal Receive Status Queue Full Flag"]
    #[inline(always)]
    pub fn rsqff(&self) -> RSQFF_R {
        RSQFF_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Normal Transmit Data Buffer Empty Flag 0"]
    #[inline(always)]
    #[must_use]
    pub fn tdbef0(&mut self) -> TDBEF0_W<0> {
        TDBEF0_W::new(self)
    }
    #[doc = "Bit 1 - Normal Receive Data Buffer Full Flag 0"]
    #[inline(always)]
    #[must_use]
    pub fn rdbff0(&mut self) -> RDBFF0_W<1> {
        RDBFF0_W::new(self)
    }
    #[doc = "Bit 2 - Normal IBI Queue Empty/Full Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ibiqeff(&mut self) -> IBIQEFF_W<2> {
        IBIQEFF_W::new(self)
    }
    #[doc = "Bit 3 - Normal Command Queue Empty Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmdqef(&mut self) -> CMDQEF_W<3> {
        CMDQEF_W::new(self)
    }
    #[doc = "Bit 4 - Normal Response Queue Full Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rspqff(&mut self) -> RSPQFF_W<4> {
        RSPQFF_W::new(self)
    }
    #[doc = "Bit 5 - Normal Transfer Abort Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tabtf(&mut self) -> TABTF_W<5> {
        TABTF_W::new(self)
    }
    #[doc = "Bit 9 - Normal Transfer Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tef(&mut self) -> TEF_W<9> {
        TEF_W::new(self)
    }
    #[doc = "Bit 20 - Normal Receive Status Queue Full Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rsqff(&mut self) -> RSQFF_W<20> {
        RSQFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Normal Transfer Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ntst](index.html) module"]
pub struct NTST_SPEC;
impl crate::RegisterSpec for NTST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ntst::R](R) reader structure"]
impl crate::Readable for NTST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ntst::W](W) writer structure"]
impl crate::Writable for NTST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NTST to value 0"]
impl crate::Resettable for NTST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
