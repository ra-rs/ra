#[doc = "Register `NTSTFC` writer"]
pub struct W(crate::W<NTSTFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NTSTFC_SPEC>;
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
impl From<crate::W<NTSTFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NTSTFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Normal Transmit Data Buffer Empty Force 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDBEFC0_AW {
    #[doc = "0: Not Force Tx0 Data Buffer Empty Interrupt for software testing."]
    _0 = 0,
    #[doc = "1: Force Tx0 Data Buffer Empty Interrupt for software testing."]
    _1 = 1,
}
impl From<TDBEFC0_AW> for bool {
    #[inline(always)]
    fn from(variant: TDBEFC0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDBEFC0` writer - Normal Transmit Data Buffer Empty Force 0"]
pub type TDBEFC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, NTSTFC_SPEC, TDBEFC0_AW, O>;
impl<'a, const O: u8> TDBEFC0_W<'a, O> {
    #[doc = "Not Force Tx0 Data Buffer Empty Interrupt for software testing."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDBEFC0_AW::_0)
    }
    #[doc = "Force Tx0 Data Buffer Empty Interrupt for software testing."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDBEFC0_AW::_1)
    }
}
#[doc = "Normal Receive Data Buffer Full Force 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDBFFC0_AW {
    #[doc = "0: Not Force Rx0 Data Buffer Full Interrupt for software testing."]
    _0 = 0,
    #[doc = "1: Force Rx0 Data Buffer Full Interrupt for software testing."]
    _1 = 1,
}
impl From<RDBFFC0_AW> for bool {
    #[inline(always)]
    fn from(variant: RDBFFC0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDBFFC0` writer - Normal Receive Data Buffer Full Force 0"]
pub type RDBFFC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, NTSTFC_SPEC, RDBFFC0_AW, O>;
impl<'a, const O: u8> RDBFFC0_W<'a, O> {
    #[doc = "Not Force Rx0 Data Buffer Full Interrupt for software testing."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RDBFFC0_AW::_0)
    }
    #[doc = "Force Rx0 Data Buffer Full Interrupt for software testing."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RDBFFC0_AW::_1)
    }
}
#[doc = "Normal IBI Queue Empty/Full Force\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IBIQEFFC_AW {
    #[doc = "0: Not Force IBI Status Buffer Full Interrupt for software testing."]
    _0 = 0,
    #[doc = "1: Force IBI Status Buffer Full Interrupt for software testing."]
    _1 = 1,
}
impl From<IBIQEFFC_AW> for bool {
    #[inline(always)]
    fn from(variant: IBIQEFFC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IBIQEFFC` writer - Normal IBI Queue Empty/Full Force"]
pub type IBIQEFFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, NTSTFC_SPEC, IBIQEFFC_AW, O>;
impl<'a, const O: u8> IBIQEFFC_W<'a, O> {
    #[doc = "Not Force IBI Status Buffer Full Interrupt for software testing."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IBIQEFFC_AW::_0)
    }
    #[doc = "Force IBI Status Buffer Full Interrupt for software testing."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IBIQEFFC_AW::_1)
    }
}
#[doc = "Normal Command Queue Empty Force\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDQEFC_AW {
    #[doc = "0: Not Force Command Buffer Empty Interrupt for software testing."]
    _0 = 0,
    #[doc = "1: Force Command Buffer Empty Interrupt for software testing."]
    _1 = 1,
}
impl From<CMDQEFC_AW> for bool {
    #[inline(always)]
    fn from(variant: CMDQEFC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDQEFC` writer - Normal Command Queue Empty Force"]
pub type CMDQEFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, NTSTFC_SPEC, CMDQEFC_AW, O>;
impl<'a, const O: u8> CMDQEFC_W<'a, O> {
    #[doc = "Not Force Command Buffer Empty Interrupt for software testing."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMDQEFC_AW::_0)
    }
    #[doc = "Force Command Buffer Empty Interrupt for software testing."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMDQEFC_AW::_1)
    }
}
#[doc = "Normal Response Queue Full Force\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSPQFFC_AW {
    #[doc = "0: Not Force Response Buffer Full Interrupt for software testing."]
    _0 = 0,
    #[doc = "1: Force Response Buffer Full Interrupt for software testing."]
    _1 = 1,
}
impl From<RSPQFFC_AW> for bool {
    #[inline(always)]
    fn from(variant: RSPQFFC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSPQFFC` writer - Normal Response Queue Full Force"]
pub type RSPQFFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, NTSTFC_SPEC, RSPQFFC_AW, O>;
impl<'a, const O: u8> RSPQFFC_W<'a, O> {
    #[doc = "Not Force Response Buffer Full Interrupt for software testing."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSPQFFC_AW::_0)
    }
    #[doc = "Force Response Buffer Full Interrupt for software testing."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSPQFFC_AW::_1)
    }
}
#[doc = "Normal Transfer Abort Force\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TABTFC_AW {
    #[doc = "0: Not Force Transfer Abort Interrupt for software testing."]
    _0 = 0,
    #[doc = "1: Force Transfer Abort Interrupt for software testing."]
    _1 = 1,
}
impl From<TABTFC_AW> for bool {
    #[inline(always)]
    fn from(variant: TABTFC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TABTFC` writer - Normal Transfer Abort Force"]
pub type TABTFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, NTSTFC_SPEC, TABTFC_AW, O>;
impl<'a, const O: u8> TABTFC_W<'a, O> {
    #[doc = "Not Force Transfer Abort Interrupt for software testing."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TABTFC_AW::_0)
    }
    #[doc = "Force Transfer Abort Interrupt for software testing."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TABTFC_AW::_1)
    }
}
#[doc = "Normal Transfer Error Force\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEFC_AW {
    #[doc = "0: Not Force Transfer Error Interrupt for software testing."]
    _0 = 0,
    #[doc = "1: Force Transfer Error Interrupt for software testing."]
    _1 = 1,
}
impl From<TEFC_AW> for bool {
    #[inline(always)]
    fn from(variant: TEFC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEFC` writer - Normal Transfer Error Force"]
pub type TEFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, NTSTFC_SPEC, TEFC_AW, O>;
impl<'a, const O: u8> TEFC_W<'a, O> {
    #[doc = "Not Force Transfer Error Interrupt for software testing."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TEFC_AW::_0)
    }
    #[doc = "Force Transfer Error Interrupt for software testing."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TEFC_AW::_1)
    }
}
#[doc = "Normal Receive Status Queue Full Force\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSQFFC_AW {
    #[doc = "0: Not Force Receive Status Buffer Full Interrupt for software testing."]
    _0 = 0,
    #[doc = "1: Force Receive Status Buffer Full Interrupt for software testing."]
    _1 = 1,
}
impl From<RSQFFC_AW> for bool {
    #[inline(always)]
    fn from(variant: RSQFFC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSQFFC` writer - Normal Receive Status Queue Full Force"]
pub type RSQFFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, NTSTFC_SPEC, RSQFFC_AW, O>;
impl<'a, const O: u8> RSQFFC_W<'a, O> {
    #[doc = "Not Force Receive Status Buffer Full Interrupt for software testing."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSQFFC_AW::_0)
    }
    #[doc = "Force Receive Status Buffer Full Interrupt for software testing."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSQFFC_AW::_1)
    }
}
impl W {
    #[doc = "Bit 0 - Normal Transmit Data Buffer Empty Force 0"]
    #[inline(always)]
    #[must_use]
    pub fn tdbefc0(&mut self) -> TDBEFC0_W<0> {
        TDBEFC0_W::new(self)
    }
    #[doc = "Bit 1 - Normal Receive Data Buffer Full Force 0"]
    #[inline(always)]
    #[must_use]
    pub fn rdbffc0(&mut self) -> RDBFFC0_W<1> {
        RDBFFC0_W::new(self)
    }
    #[doc = "Bit 2 - Normal IBI Queue Empty/Full Force"]
    #[inline(always)]
    #[must_use]
    pub fn ibiqeffc(&mut self) -> IBIQEFFC_W<2> {
        IBIQEFFC_W::new(self)
    }
    #[doc = "Bit 3 - Normal Command Queue Empty Force"]
    #[inline(always)]
    #[must_use]
    pub fn cmdqefc(&mut self) -> CMDQEFC_W<3> {
        CMDQEFC_W::new(self)
    }
    #[doc = "Bit 4 - Normal Response Queue Full Force"]
    #[inline(always)]
    #[must_use]
    pub fn rspqffc(&mut self) -> RSPQFFC_W<4> {
        RSPQFFC_W::new(self)
    }
    #[doc = "Bit 5 - Normal Transfer Abort Force"]
    #[inline(always)]
    #[must_use]
    pub fn tabtfc(&mut self) -> TABTFC_W<5> {
        TABTFC_W::new(self)
    }
    #[doc = "Bit 9 - Normal Transfer Error Force"]
    #[inline(always)]
    #[must_use]
    pub fn tefc(&mut self) -> TEFC_W<9> {
        TEFC_W::new(self)
    }
    #[doc = "Bit 20 - Normal Receive Status Queue Full Force"]
    #[inline(always)]
    #[must_use]
    pub fn rsqffc(&mut self) -> RSQFFC_W<20> {
        RSQFFC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Normal Transfer Status Force Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ntstfc](index.html) module"]
pub struct NTSTFC_SPEC;
impl crate::RegisterSpec for NTSTFC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ntstfc::W](W) writer structure"]
impl crate::Writable for NTSTFC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NTSTFC to value 0"]
impl crate::Resettable for NTSTFC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
