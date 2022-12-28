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
