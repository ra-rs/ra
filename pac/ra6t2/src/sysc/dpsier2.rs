#[doc = "Register `DPSIER2` reader"]
pub struct R(crate::R<DPSIER2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPSIER2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPSIER2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPSIER2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPSIER2` writer"]
pub struct W(crate::W<DPSIER2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPSIER2_SPEC>;
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
impl From<crate::W<DPSIER2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPSIER2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLVD1IE` reader - LVD1 Deep Software Standby Cancel Signal Enable"]
pub type DLVD1IE_R = crate::BitReader<DLVD1IE_A>;
#[doc = "LVD1 Deep Software Standby Cancel Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLVD1IE_A {
    #[doc = "0: Cancelling Deep Software Standby mode is disabled"]
    _0 = 0,
    #[doc = "1: Cancelling Deep Software Standby mode is enabled"]
    _1 = 1,
}
impl From<DLVD1IE_A> for bool {
    #[inline(always)]
    fn from(variant: DLVD1IE_A) -> Self {
        variant as u8 != 0
    }
}
impl DLVD1IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLVD1IE_A {
        match self.bits {
            false => DLVD1IE_A::_0,
            true => DLVD1IE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLVD1IE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLVD1IE_A::_1
    }
}
#[doc = "Field `DLVD1IE` writer - LVD1 Deep Software Standby Cancel Signal Enable"]
pub type DLVD1IE_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIER2_SPEC, DLVD1IE_A, O>;
impl<'a, const O: u8> DLVD1IE_W<'a, O> {
    #[doc = "Cancelling Deep Software Standby mode is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DLVD1IE_A::_0)
    }
    #[doc = "Cancelling Deep Software Standby mode is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DLVD1IE_A::_1)
    }
}
#[doc = "Field `DLVD2IE` reader - LVD2 Deep Software Standby Cancel Signal Enable"]
pub type DLVD2IE_R = crate::BitReader<DLVD2IE_A>;
#[doc = "LVD2 Deep Software Standby Cancel Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLVD2IE_A {
    #[doc = "0: Cancelling Deep Software Standby mode is disabled"]
    _0 = 0,
    #[doc = "1: Cancelling Deep Software Standby mode is enabled"]
    _1 = 1,
}
impl From<DLVD2IE_A> for bool {
    #[inline(always)]
    fn from(variant: DLVD2IE_A) -> Self {
        variant as u8 != 0
    }
}
impl DLVD2IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLVD2IE_A {
        match self.bits {
            false => DLVD2IE_A::_0,
            true => DLVD2IE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLVD2IE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLVD2IE_A::_1
    }
}
#[doc = "Field `DLVD2IE` writer - LVD2 Deep Software Standby Cancel Signal Enable"]
pub type DLVD2IE_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIER2_SPEC, DLVD2IE_A, O>;
impl<'a, const O: u8> DLVD2IE_W<'a, O> {
    #[doc = "Cancelling Deep Software Standby mode is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DLVD2IE_A::_0)
    }
    #[doc = "Cancelling Deep Software Standby mode is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DLVD2IE_A::_1)
    }
}
#[doc = "Field `DNMIE` reader - NMI Pin Enable"]
pub type DNMIE_R = crate::BitReader<DNMIE_A>;
#[doc = "NMI Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DNMIE_A {
    #[doc = "0: Cancelling Deep Software Standby mode is disabled"]
    _0 = 0,
    #[doc = "1: Cancelling Deep Software Standby mode is enabled"]
    _1 = 1,
}
impl From<DNMIE_A> for bool {
    #[inline(always)]
    fn from(variant: DNMIE_A) -> Self {
        variant as u8 != 0
    }
}
impl DNMIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DNMIE_A {
        match self.bits {
            false => DNMIE_A::_0,
            true => DNMIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DNMIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DNMIE_A::_1
    }
}
#[doc = "Field `DNMIE` writer - NMI Pin Enable"]
pub type DNMIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIER2_SPEC, DNMIE_A, O>;
impl<'a, const O: u8> DNMIE_W<'a, O> {
    #[doc = "Cancelling Deep Software Standby mode is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DNMIE_A::_0)
    }
    #[doc = "Cancelling Deep Software Standby mode is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DNMIE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - LVD1 Deep Software Standby Cancel Signal Enable"]
    #[inline(always)]
    pub fn dlvd1ie(&self) -> DLVD1IE_R {
        DLVD1IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LVD2 Deep Software Standby Cancel Signal Enable"]
    #[inline(always)]
    pub fn dlvd2ie(&self) -> DLVD2IE_R {
        DLVD2IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - NMI Pin Enable"]
    #[inline(always)]
    pub fn dnmie(&self) -> DNMIE_R {
        DNMIE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LVD1 Deep Software Standby Cancel Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dlvd1ie(&mut self) -> DLVD1IE_W<0> {
        DLVD1IE_W::new(self)
    }
    #[doc = "Bit 1 - LVD2 Deep Software Standby Cancel Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dlvd2ie(&mut self) -> DLVD2IE_W<1> {
        DLVD2IE_W::new(self)
    }
    #[doc = "Bit 4 - NMI Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dnmie(&mut self) -> DNMIE_W<4> {
        DNMIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Deep Software Standby Interrupt Enable Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpsier2](index.html) module"]
pub struct DPSIER2_SPEC;
impl crate::RegisterSpec for DPSIER2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dpsier2::R](R) reader structure"]
impl crate::Readable for DPSIER2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dpsier2::W](W) writer structure"]
impl crate::Writable for DPSIER2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPSIER2 to value 0"]
impl crate::Resettable for DPSIER2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
