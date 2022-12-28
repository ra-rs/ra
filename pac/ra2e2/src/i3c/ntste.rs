#[doc = "Register `NTSTE` reader"]
pub struct R(crate::R<NTSTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NTSTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NTSTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NTSTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NTSTE` writer"]
pub struct W(crate::W<NTSTE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NTSTE_SPEC>;
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
impl From<crate::W<NTSTE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NTSTE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDBEE0` reader - Normal Transmit Data Buffer Empty Enable 0"]
pub type TDBEE0_R = crate::BitReader<TDBEE0_A>;
#[doc = "Normal Transmit Data Buffer Empty Enable 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDBEE0_A {
    #[doc = "0: Disables Tx0 Data Buffer Empty Interrupt Status logging."]
    _0 = 0,
    #[doc = "1: Enables Tx0 Data Buffer Empty Interrupt Status logging."]
    _1 = 1,
}
impl From<TDBEE0_A> for bool {
    #[inline(always)]
    fn from(variant: TDBEE0_A) -> Self {
        variant as u8 != 0
    }
}
impl TDBEE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDBEE0_A {
        match self.bits {
            false => TDBEE0_A::_0,
            true => TDBEE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDBEE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDBEE0_A::_1
    }
}
#[doc = "Field `TDBEE0` writer - Normal Transmit Data Buffer Empty Enable 0"]
pub type TDBEE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, NTSTE_SPEC, TDBEE0_A, O>;
impl<'a, const O: u8> TDBEE0_W<'a, O> {
    #[doc = "Disables Tx0 Data Buffer Empty Interrupt Status logging."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDBEE0_A::_0)
    }
    #[doc = "Enables Tx0 Data Buffer Empty Interrupt Status logging."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDBEE0_A::_1)
    }
}
#[doc = "Field `RDBFE0` reader - Normal Receive Data Buffer Full Enable 0"]
pub type RDBFE0_R = crate::BitReader<RDBFE0_A>;
#[doc = "Normal Receive Data Buffer Full Enable 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDBFE0_A {
    #[doc = "0: Disables Rx0 Data Buffer Full Interrupt Status logging."]
    _0 = 0,
    #[doc = "1: Enables Rx0 Data Buffer Full Interrupt Status logging."]
    _1 = 1,
}
impl From<RDBFE0_A> for bool {
    #[inline(always)]
    fn from(variant: RDBFE0_A) -> Self {
        variant as u8 != 0
    }
}
impl RDBFE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDBFE0_A {
        match self.bits {
            false => RDBFE0_A::_0,
            true => RDBFE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDBFE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDBFE0_A::_1
    }
}
#[doc = "Field `RDBFE0` writer - Normal Receive Data Buffer Full Enable 0"]
pub type RDBFE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, NTSTE_SPEC, RDBFE0_A, O>;
impl<'a, const O: u8> RDBFE0_W<'a, O> {
    #[doc = "Disables Rx0 Data Buffer Full Interrupt Status logging."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RDBFE0_A::_0)
    }
    #[doc = "Enables Rx0 Data Buffer Full Interrupt Status logging."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RDBFE0_A::_1)
    }
}
#[doc = "Field `IBIQEFE` reader - Normal IBI Queue Empty/Full Enable"]
pub type IBIQEFE_R = crate::BitReader<IBIQEFE_A>;
#[doc = "Normal IBI Queue Empty/Full Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IBIQEFE_A {
    #[doc = "0: Disables IBI Status Buffer Empty/Full Interrupt Status logging."]
    _0 = 0,
    #[doc = "1: Enables IBI Status Buffer Empty/Full Interrupt Status logging."]
    _1 = 1,
}
impl From<IBIQEFE_A> for bool {
    #[inline(always)]
    fn from(variant: IBIQEFE_A) -> Self {
        variant as u8 != 0
    }
}
impl IBIQEFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IBIQEFE_A {
        match self.bits {
            false => IBIQEFE_A::_0,
            true => IBIQEFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IBIQEFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IBIQEFE_A::_1
    }
}
#[doc = "Field `IBIQEFE` writer - Normal IBI Queue Empty/Full Enable"]
pub type IBIQEFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, NTSTE_SPEC, IBIQEFE_A, O>;
impl<'a, const O: u8> IBIQEFE_W<'a, O> {
    #[doc = "Disables IBI Status Buffer Empty/Full Interrupt Status logging."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IBIQEFE_A::_0)
    }
    #[doc = "Enables IBI Status Buffer Empty/Full Interrupt Status logging."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IBIQEFE_A::_1)
    }
}
#[doc = "Field `CMDQEE` reader - Normal Command Queue Empty Enable"]
pub type CMDQEE_R = crate::BitReader<CMDQEE_A>;
#[doc = "Normal Command Queue Empty Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDQEE_A {
    #[doc = "0: Disables Command Buffer Empty Interrupt Status logging."]
    _0 = 0,
    #[doc = "1: Enables Command Buffer Empty Interrupt Status logging."]
    _1 = 1,
}
impl From<CMDQEE_A> for bool {
    #[inline(always)]
    fn from(variant: CMDQEE_A) -> Self {
        variant as u8 != 0
    }
}
impl CMDQEE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDQEE_A {
        match self.bits {
            false => CMDQEE_A::_0,
            true => CMDQEE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMDQEE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMDQEE_A::_1
    }
}
#[doc = "Field `CMDQEE` writer - Normal Command Queue Empty Enable"]
pub type CMDQEE_W<'a, const O: u8> = crate::BitWriter<'a, u32, NTSTE_SPEC, CMDQEE_A, O>;
impl<'a, const O: u8> CMDQEE_W<'a, O> {
    #[doc = "Disables Command Buffer Empty Interrupt Status logging."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMDQEE_A::_0)
    }
    #[doc = "Enables Command Buffer Empty Interrupt Status logging."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMDQEE_A::_1)
    }
}
#[doc = "Field `RSPQFE` reader - Normal Response Queue Full Enable"]
pub type RSPQFE_R = crate::BitReader<RSPQFE_A>;
#[doc = "Normal Response Queue Full Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSPQFE_A {
    #[doc = "0: Disables Response Buffer Full Interrupt Status logging."]
    _0 = 0,
    #[doc = "1: Enables Response Buffer Full Interrupt Status logging."]
    _1 = 1,
}
impl From<RSPQFE_A> for bool {
    #[inline(always)]
    fn from(variant: RSPQFE_A) -> Self {
        variant as u8 != 0
    }
}
impl RSPQFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSPQFE_A {
        match self.bits {
            false => RSPQFE_A::_0,
            true => RSPQFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSPQFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSPQFE_A::_1
    }
}
#[doc = "Field `RSPQFE` writer - Normal Response Queue Full Enable"]
pub type RSPQFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, NTSTE_SPEC, RSPQFE_A, O>;
impl<'a, const O: u8> RSPQFE_W<'a, O> {
    #[doc = "Disables Response Buffer Full Interrupt Status logging."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSPQFE_A::_0)
    }
    #[doc = "Enables Response Buffer Full Interrupt Status logging."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSPQFE_A::_1)
    }
}
#[doc = "Field `TABTE` reader - Normal Transfer Abort Enable"]
pub type TABTE_R = crate::BitReader<TABTE_A>;
#[doc = "Normal Transfer Abort Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TABTE_A {
    #[doc = "0: Disables Transfer Abort Interrupt Status logging."]
    _0 = 0,
    #[doc = "1: Enables Transfer Abort Interrupt Status logging."]
    _1 = 1,
}
impl From<TABTE_A> for bool {
    #[inline(always)]
    fn from(variant: TABTE_A) -> Self {
        variant as u8 != 0
    }
}
impl TABTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TABTE_A {
        match self.bits {
            false => TABTE_A::_0,
            true => TABTE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TABTE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TABTE_A::_1
    }
}
#[doc = "Field `TABTE` writer - Normal Transfer Abort Enable"]
pub type TABTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, NTSTE_SPEC, TABTE_A, O>;
impl<'a, const O: u8> TABTE_W<'a, O> {
    #[doc = "Disables Transfer Abort Interrupt Status logging."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TABTE_A::_0)
    }
    #[doc = "Enables Transfer Abort Interrupt Status logging."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TABTE_A::_1)
    }
}
#[doc = "Field `TEE` reader - Normal Transfer Error Enable"]
pub type TEE_R = crate::BitReader<TEE_A>;
#[doc = "Normal Transfer Error Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEE_A {
    #[doc = "0: Disables Transfer Error Interrupt Status logging."]
    _0 = 0,
    #[doc = "1: Enables Transfer Error Interrupt Status logging."]
    _1 = 1,
}
impl From<TEE_A> for bool {
    #[inline(always)]
    fn from(variant: TEE_A) -> Self {
        variant as u8 != 0
    }
}
impl TEE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEE_A {
        match self.bits {
            false => TEE_A::_0,
            true => TEE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TEE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TEE_A::_1
    }
}
#[doc = "Field `TEE` writer - Normal Transfer Error Enable"]
pub type TEE_W<'a, const O: u8> = crate::BitWriter<'a, u32, NTSTE_SPEC, TEE_A, O>;
impl<'a, const O: u8> TEE_W<'a, O> {
    #[doc = "Disables Transfer Error Interrupt Status logging."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TEE_A::_0)
    }
    #[doc = "Enables Transfer Error Interrupt Status logging."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TEE_A::_1)
    }
}
#[doc = "Field `RSQFE` reader - Normal Receive Status Queue Full Enable"]
pub type RSQFE_R = crate::BitReader<RSQFE_A>;
#[doc = "Normal Receive Status Queue Full Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSQFE_A {
    #[doc = "0: Disables Receive Status Buffer Full Interrupt Status logging."]
    _0 = 0,
    #[doc = "1: Enables Receive Status Buffer Full Interrupt Status logging."]
    _1 = 1,
}
impl From<RSQFE_A> for bool {
    #[inline(always)]
    fn from(variant: RSQFE_A) -> Self {
        variant as u8 != 0
    }
}
impl RSQFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSQFE_A {
        match self.bits {
            false => RSQFE_A::_0,
            true => RSQFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSQFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSQFE_A::_1
    }
}
#[doc = "Field `RSQFE` writer - Normal Receive Status Queue Full Enable"]
pub type RSQFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, NTSTE_SPEC, RSQFE_A, O>;
impl<'a, const O: u8> RSQFE_W<'a, O> {
    #[doc = "Disables Receive Status Buffer Full Interrupt Status logging."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSQFE_A::_0)
    }
    #[doc = "Enables Receive Status Buffer Full Interrupt Status logging."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSQFE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Normal Transmit Data Buffer Empty Enable 0"]
    #[inline(always)]
    pub fn tdbee0(&self) -> TDBEE0_R {
        TDBEE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Normal Receive Data Buffer Full Enable 0"]
    #[inline(always)]
    pub fn rdbfe0(&self) -> RDBFE0_R {
        RDBFE0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Normal IBI Queue Empty/Full Enable"]
    #[inline(always)]
    pub fn ibiqefe(&self) -> IBIQEFE_R {
        IBIQEFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Normal Command Queue Empty Enable"]
    #[inline(always)]
    pub fn cmdqee(&self) -> CMDQEE_R {
        CMDQEE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Normal Response Queue Full Enable"]
    #[inline(always)]
    pub fn rspqfe(&self) -> RSPQFE_R {
        RSPQFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Normal Transfer Abort Enable"]
    #[inline(always)]
    pub fn tabte(&self) -> TABTE_R {
        TABTE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - Normal Transfer Error Enable"]
    #[inline(always)]
    pub fn tee(&self) -> TEE_R {
        TEE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 20 - Normal Receive Status Queue Full Enable"]
    #[inline(always)]
    pub fn rsqfe(&self) -> RSQFE_R {
        RSQFE_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Normal Transmit Data Buffer Empty Enable 0"]
    #[inline(always)]
    #[must_use]
    pub fn tdbee0(&mut self) -> TDBEE0_W<0> {
        TDBEE0_W::new(self)
    }
    #[doc = "Bit 1 - Normal Receive Data Buffer Full Enable 0"]
    #[inline(always)]
    #[must_use]
    pub fn rdbfe0(&mut self) -> RDBFE0_W<1> {
        RDBFE0_W::new(self)
    }
    #[doc = "Bit 2 - Normal IBI Queue Empty/Full Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ibiqefe(&mut self) -> IBIQEFE_W<2> {
        IBIQEFE_W::new(self)
    }
    #[doc = "Bit 3 - Normal Command Queue Empty Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdqee(&mut self) -> CMDQEE_W<3> {
        CMDQEE_W::new(self)
    }
    #[doc = "Bit 4 - Normal Response Queue Full Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rspqfe(&mut self) -> RSPQFE_W<4> {
        RSPQFE_W::new(self)
    }
    #[doc = "Bit 5 - Normal Transfer Abort Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tabte(&mut self) -> TABTE_W<5> {
        TABTE_W::new(self)
    }
    #[doc = "Bit 9 - Normal Transfer Error Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tee(&mut self) -> TEE_W<9> {
        TEE_W::new(self)
    }
    #[doc = "Bit 20 - Normal Receive Status Queue Full Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rsqfe(&mut self) -> RSQFE_W<20> {
        RSQFE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Normal Transfer Status Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ntste](index.html) module"]
pub struct NTSTE_SPEC;
impl crate::RegisterSpec for NTSTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ntste::R](R) reader structure"]
impl crate::Readable for NTSTE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ntste::W](W) writer structure"]
impl crate::Writable for NTSTE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NTSTE to value 0"]
impl crate::Resettable for NTSTE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
