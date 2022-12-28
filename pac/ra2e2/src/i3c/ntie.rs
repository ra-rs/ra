#[doc = "Register `NTIE` reader"]
pub struct R(crate::R<NTIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NTIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NTIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NTIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NTIE` writer"]
pub struct W(crate::W<NTIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NTIE_SPEC>;
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
impl From<crate::W<NTIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NTIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDBEIE0` reader - Normal Transmit Data Buffer Empty Interrupt Enable 0"]
pub type TDBEIE0_R = crate::BitReader<TDBEIE0_A>;
#[doc = "Normal Transmit Data Buffer Empty Interrupt Enable 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDBEIE0_A {
    #[doc = "0: Disables Tx0 Data Buffer Empty Interrupt Signal."]
    _0 = 0,
    #[doc = "1: Enables Tx0 Data Buffer Empty Interrupt Signal."]
    _1 = 1,
}
impl From<TDBEIE0_A> for bool {
    #[inline(always)]
    fn from(variant: TDBEIE0_A) -> Self {
        variant as u8 != 0
    }
}
impl TDBEIE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDBEIE0_A {
        match self.bits {
            false => TDBEIE0_A::_0,
            true => TDBEIE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDBEIE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDBEIE0_A::_1
    }
}
#[doc = "Field `TDBEIE0` writer - Normal Transmit Data Buffer Empty Interrupt Enable 0"]
pub type TDBEIE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, NTIE_SPEC, TDBEIE0_A, O>;
impl<'a, const O: u8> TDBEIE0_W<'a, O> {
    #[doc = "Disables Tx0 Data Buffer Empty Interrupt Signal."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDBEIE0_A::_0)
    }
    #[doc = "Enables Tx0 Data Buffer Empty Interrupt Signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDBEIE0_A::_1)
    }
}
#[doc = "Field `RDBFIE0` reader - Normal Receive Data Buffer Full Interrupt Enable 0"]
pub type RDBFIE0_R = crate::BitReader<RDBFIE0_A>;
#[doc = "Normal Receive Data Buffer Full Interrupt Enable 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDBFIE0_A {
    #[doc = "0: Disables Rx0 Data Buffer Full Interrupt Signal."]
    _0 = 0,
    #[doc = "1: Enables Rx0 Data Buffer Full Interrupt Signal."]
    _1 = 1,
}
impl From<RDBFIE0_A> for bool {
    #[inline(always)]
    fn from(variant: RDBFIE0_A) -> Self {
        variant as u8 != 0
    }
}
impl RDBFIE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDBFIE0_A {
        match self.bits {
            false => RDBFIE0_A::_0,
            true => RDBFIE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDBFIE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDBFIE0_A::_1
    }
}
#[doc = "Field `RDBFIE0` writer - Normal Receive Data Buffer Full Interrupt Enable 0"]
pub type RDBFIE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, NTIE_SPEC, RDBFIE0_A, O>;
impl<'a, const O: u8> RDBFIE0_W<'a, O> {
    #[doc = "Disables Rx0 Data Buffer Full Interrupt Signal."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RDBFIE0_A::_0)
    }
    #[doc = "Enables Rx0 Data Buffer Full Interrupt Signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RDBFIE0_A::_1)
    }
}
#[doc = "Field `IBIQEFIE` reader - Normal IBI Queue Empty/Full Interrupt Enable"]
pub type IBIQEFIE_R = crate::BitReader<IBIQEFIE_A>;
#[doc = "Normal IBI Queue Empty/Full Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IBIQEFIE_A {
    #[doc = "0: Disables IBI Status Buffer Empty/Full Interrupt Signal."]
    _0 = 0,
    #[doc = "1: Enables IBI Status Buffer Empty/Full Interrupt Signal."]
    _1 = 1,
}
impl From<IBIQEFIE_A> for bool {
    #[inline(always)]
    fn from(variant: IBIQEFIE_A) -> Self {
        variant as u8 != 0
    }
}
impl IBIQEFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IBIQEFIE_A {
        match self.bits {
            false => IBIQEFIE_A::_0,
            true => IBIQEFIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IBIQEFIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IBIQEFIE_A::_1
    }
}
#[doc = "Field `IBIQEFIE` writer - Normal IBI Queue Empty/Full Interrupt Enable"]
pub type IBIQEFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, NTIE_SPEC, IBIQEFIE_A, O>;
impl<'a, const O: u8> IBIQEFIE_W<'a, O> {
    #[doc = "Disables IBI Status Buffer Empty/Full Interrupt Signal."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IBIQEFIE_A::_0)
    }
    #[doc = "Enables IBI Status Buffer Empty/Full Interrupt Signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IBIQEFIE_A::_1)
    }
}
#[doc = "Field `CMDQEIE` reader - Normal Command Queue Empty Interrupt Enable"]
pub type CMDQEIE_R = crate::BitReader<CMDQEIE_A>;
#[doc = "Normal Command Queue Empty Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDQEIE_A {
    #[doc = "0: Disables Command Buffer Empty Interrupt Signal."]
    _0 = 0,
    #[doc = "1: Enables Command Buffer Empty Interrupt Signal."]
    _1 = 1,
}
impl From<CMDQEIE_A> for bool {
    #[inline(always)]
    fn from(variant: CMDQEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CMDQEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDQEIE_A {
        match self.bits {
            false => CMDQEIE_A::_0,
            true => CMDQEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMDQEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMDQEIE_A::_1
    }
}
#[doc = "Field `CMDQEIE` writer - Normal Command Queue Empty Interrupt Enable"]
pub type CMDQEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, NTIE_SPEC, CMDQEIE_A, O>;
impl<'a, const O: u8> CMDQEIE_W<'a, O> {
    #[doc = "Disables Command Buffer Empty Interrupt Signal."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMDQEIE_A::_0)
    }
    #[doc = "Enables Command Buffer Empty Interrupt Signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMDQEIE_A::_1)
    }
}
#[doc = "Field `RSPQFIE` reader - Normal Response Queue Full Interrupt Enable"]
pub type RSPQFIE_R = crate::BitReader<RSPQFIE_A>;
#[doc = "Normal Response Queue Full Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSPQFIE_A {
    #[doc = "0: Disables Response Buffer Full Interrupt Signal."]
    _0 = 0,
    #[doc = "1: Enables Response Buffer Full Interrupt Signal."]
    _1 = 1,
}
impl From<RSPQFIE_A> for bool {
    #[inline(always)]
    fn from(variant: RSPQFIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RSPQFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSPQFIE_A {
        match self.bits {
            false => RSPQFIE_A::_0,
            true => RSPQFIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSPQFIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSPQFIE_A::_1
    }
}
#[doc = "Field `RSPQFIE` writer - Normal Response Queue Full Interrupt Enable"]
pub type RSPQFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, NTIE_SPEC, RSPQFIE_A, O>;
impl<'a, const O: u8> RSPQFIE_W<'a, O> {
    #[doc = "Disables Response Buffer Full Interrupt Signal."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSPQFIE_A::_0)
    }
    #[doc = "Enables Response Buffer Full Interrupt Signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSPQFIE_A::_1)
    }
}
#[doc = "Field `TABTIE` reader - Normal Transfer Abort Interrupt Enable"]
pub type TABTIE_R = crate::BitReader<TABTIE_A>;
#[doc = "Normal Transfer Abort Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TABTIE_A {
    #[doc = "0: Disables Transfer Abort Interrupt Signal."]
    _0 = 0,
    #[doc = "1: Enables Transfer Abort Interrupt Signal."]
    _1 = 1,
}
impl From<TABTIE_A> for bool {
    #[inline(always)]
    fn from(variant: TABTIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TABTIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TABTIE_A {
        match self.bits {
            false => TABTIE_A::_0,
            true => TABTIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TABTIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TABTIE_A::_1
    }
}
#[doc = "Field `TABTIE` writer - Normal Transfer Abort Interrupt Enable"]
pub type TABTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, NTIE_SPEC, TABTIE_A, O>;
impl<'a, const O: u8> TABTIE_W<'a, O> {
    #[doc = "Disables Transfer Abort Interrupt Signal."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TABTIE_A::_0)
    }
    #[doc = "Enables Transfer Abort Interrupt Signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TABTIE_A::_1)
    }
}
#[doc = "Field `TEIE` reader - Normal Transfer Error Interrupt Enable"]
pub type TEIE_R = crate::BitReader<TEIE_A>;
#[doc = "Normal Transfer Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIE_A {
    #[doc = "0: Disables Transfer Error Interrupt Signal."]
    _0 = 0,
    #[doc = "1: Enables Transfer Error Interrupt Signal."]
    _1 = 1,
}
impl From<TEIE_A> for bool {
    #[inline(always)]
    fn from(variant: TEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEIE_A {
        match self.bits {
            false => TEIE_A::_0,
            true => TEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TEIE_A::_1
    }
}
#[doc = "Field `TEIE` writer - Normal Transfer Error Interrupt Enable"]
pub type TEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, NTIE_SPEC, TEIE_A, O>;
impl<'a, const O: u8> TEIE_W<'a, O> {
    #[doc = "Disables Transfer Error Interrupt Signal."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TEIE_A::_0)
    }
    #[doc = "Enables Transfer Error Interrupt Signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TEIE_A::_1)
    }
}
#[doc = "Field `RSQFIE` reader - Normal Receive Status Queue Full Interrupt Enable"]
pub type RSQFIE_R = crate::BitReader<RSQFIE_A>;
#[doc = "Normal Receive Status Queue Full Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSQFIE_A {
    #[doc = "0: Disables Receive Status Buffer Full Interrupt Signal."]
    _0 = 0,
    #[doc = "1: Enables Receive Status Buffer Full Interrupt Signal."]
    _1 = 1,
}
impl From<RSQFIE_A> for bool {
    #[inline(always)]
    fn from(variant: RSQFIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RSQFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSQFIE_A {
        match self.bits {
            false => RSQFIE_A::_0,
            true => RSQFIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSQFIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSQFIE_A::_1
    }
}
#[doc = "Field `RSQFIE` writer - Normal Receive Status Queue Full Interrupt Enable"]
pub type RSQFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, NTIE_SPEC, RSQFIE_A, O>;
impl<'a, const O: u8> RSQFIE_W<'a, O> {
    #[doc = "Disables Receive Status Buffer Full Interrupt Signal."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSQFIE_A::_0)
    }
    #[doc = "Enables Receive Status Buffer Full Interrupt Signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSQFIE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Normal Transmit Data Buffer Empty Interrupt Enable 0"]
    #[inline(always)]
    pub fn tdbeie0(&self) -> TDBEIE0_R {
        TDBEIE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Normal Receive Data Buffer Full Interrupt Enable 0"]
    #[inline(always)]
    pub fn rdbfie0(&self) -> RDBFIE0_R {
        RDBFIE0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Normal IBI Queue Empty/Full Interrupt Enable"]
    #[inline(always)]
    pub fn ibiqefie(&self) -> IBIQEFIE_R {
        IBIQEFIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Normal Command Queue Empty Interrupt Enable"]
    #[inline(always)]
    pub fn cmdqeie(&self) -> CMDQEIE_R {
        CMDQEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Normal Response Queue Full Interrupt Enable"]
    #[inline(always)]
    pub fn rspqfie(&self) -> RSPQFIE_R {
        RSPQFIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Normal Transfer Abort Interrupt Enable"]
    #[inline(always)]
    pub fn tabtie(&self) -> TABTIE_R {
        TABTIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - Normal Transfer Error Interrupt Enable"]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 20 - Normal Receive Status Queue Full Interrupt Enable"]
    #[inline(always)]
    pub fn rsqfie(&self) -> RSQFIE_R {
        RSQFIE_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Normal Transmit Data Buffer Empty Interrupt Enable 0"]
    #[inline(always)]
    #[must_use]
    pub fn tdbeie0(&mut self) -> TDBEIE0_W<0> {
        TDBEIE0_W::new(self)
    }
    #[doc = "Bit 1 - Normal Receive Data Buffer Full Interrupt Enable 0"]
    #[inline(always)]
    #[must_use]
    pub fn rdbfie0(&mut self) -> RDBFIE0_W<1> {
        RDBFIE0_W::new(self)
    }
    #[doc = "Bit 2 - Normal IBI Queue Empty/Full Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ibiqefie(&mut self) -> IBIQEFIE_W<2> {
        IBIQEFIE_W::new(self)
    }
    #[doc = "Bit 3 - Normal Command Queue Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdqeie(&mut self) -> CMDQEIE_W<3> {
        CMDQEIE_W::new(self)
    }
    #[doc = "Bit 4 - Normal Response Queue Full Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rspqfie(&mut self) -> RSPQFIE_W<4> {
        RSPQFIE_W::new(self)
    }
    #[doc = "Bit 5 - Normal Transfer Abort Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tabtie(&mut self) -> TABTIE_W<5> {
        TABTIE_W::new(self)
    }
    #[doc = "Bit 9 - Normal Transfer Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TEIE_W<9> {
        TEIE_W::new(self)
    }
    #[doc = "Bit 20 - Normal Receive Status Queue Full Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rsqfie(&mut self) -> RSQFIE_W<20> {
        RSQFIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Normal Transfer Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ntie](index.html) module"]
pub struct NTIE_SPEC;
impl crate::RegisterSpec for NTIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ntie::R](R) reader structure"]
impl crate::Readable for NTIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ntie::W](W) writer structure"]
impl crate::Writable for NTIE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NTIE to value 0"]
impl crate::Resettable for NTIE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
