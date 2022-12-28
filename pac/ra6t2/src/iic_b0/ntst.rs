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
    #[doc = "0: Normal Transmit Data Buffer 0 contains transmit data."]
    _0 = 0,
    #[doc = "1: Normal Transmit Data Buffer 0 contains no transmit data."]
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
    #[doc = "Normal Transmit Data Buffer 0 contains transmit data."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDBEF0_A::_0)
    }
    #[doc = "Normal Transmit Data Buffer 0 contains no transmit data."]
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
    #[doc = "0: Normal Receive Data Buffer0 contains no receive data."]
    _0 = 0,
    #[doc = "1: Normal Receive Data Buffer0 contains receive data."]
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
    #[doc = "Normal Receive Data Buffer0 contains no receive data."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RDBFF0_A::_0)
    }
    #[doc = "Normal Receive Data Buffer0 contains receive data."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RDBFF0_A::_1)
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
