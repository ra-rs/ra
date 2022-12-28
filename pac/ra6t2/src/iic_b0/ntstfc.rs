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
