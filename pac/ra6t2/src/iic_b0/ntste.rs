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
