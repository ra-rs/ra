#[doc = "Register `DPSIER3` reader"]
pub struct R(crate::R<DPSIER3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPSIER3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPSIER3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPSIER3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPSIER3` writer"]
pub struct W(crate::W<DPSIER3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPSIER3_SPEC>;
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
impl From<crate::W<DPSIER3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPSIER3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DUSBFSIE` reader - USBFS Suspend/Resume Deep Standby Cancel Signal Enable"]
pub type DUSBFSIE_R = crate::BitReader<DUSBFSIE_A>;
#[doc = "USBFS Suspend/Resume Deep Standby Cancel Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DUSBFSIE_A {
    #[doc = "0: Canceling deep software standby mode is disabled"]
    _0 = 0,
    #[doc = "1: Canceling deep software standby mode is enabled"]
    _1 = 1,
}
impl From<DUSBFSIE_A> for bool {
    #[inline(always)]
    fn from(variant: DUSBFSIE_A) -> Self {
        variant as u8 != 0
    }
}
impl DUSBFSIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DUSBFSIE_A {
        match self.bits {
            false => DUSBFSIE_A::_0,
            true => DUSBFSIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DUSBFSIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DUSBFSIE_A::_1
    }
}
#[doc = "Field `DUSBFSIE` writer - USBFS Suspend/Resume Deep Standby Cancel Signal Enable"]
pub type DUSBFSIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIER3_SPEC, DUSBFSIE_A, O>;
impl<'a, const O: u8> DUSBFSIE_W<'a, O> {
    #[doc = "Canceling deep software standby mode is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DUSBFSIE_A::_0)
    }
    #[doc = "Canceling deep software standby mode is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DUSBFSIE_A::_1)
    }
}
#[doc = "Field `DUSBHSIE` reader - USBHS Suspend/Resume Deep Standby Cancel Signal Enable"]
pub type DUSBHSIE_R = crate::BitReader<DUSBHSIE_A>;
#[doc = "USBHS Suspend/Resume Deep Standby Cancel Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DUSBHSIE_A {
    #[doc = "0: Canceling deep software standby mode is disabled"]
    _0 = 0,
    #[doc = "1: Canceling deep software standby mode is enabled"]
    _1 = 1,
}
impl From<DUSBHSIE_A> for bool {
    #[inline(always)]
    fn from(variant: DUSBHSIE_A) -> Self {
        variant as u8 != 0
    }
}
impl DUSBHSIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DUSBHSIE_A {
        match self.bits {
            false => DUSBHSIE_A::_0,
            true => DUSBHSIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DUSBHSIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DUSBHSIE_A::_1
    }
}
#[doc = "Field `DUSBHSIE` writer - USBHS Suspend/Resume Deep Standby Cancel Signal Enable"]
pub type DUSBHSIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIER3_SPEC, DUSBHSIE_A, O>;
impl<'a, const O: u8> DUSBHSIE_W<'a, O> {
    #[doc = "Canceling deep software standby mode is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DUSBHSIE_A::_0)
    }
    #[doc = "Canceling deep software standby mode is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DUSBHSIE_A::_1)
    }
}
#[doc = "Field `DAGT1IE` reader - AGT1 Underflow Deep Standby Cancel Signal Enable"]
pub type DAGT1IE_R = crate::BitReader<DAGT1IE_A>;
#[doc = "AGT1 Underflow Deep Standby Cancel Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAGT1IE_A {
    #[doc = "0: Canceling deep software standby mode is disabled"]
    _0 = 0,
    #[doc = "1: Canceling deep software standby mode is enabled"]
    _1 = 1,
}
impl From<DAGT1IE_A> for bool {
    #[inline(always)]
    fn from(variant: DAGT1IE_A) -> Self {
        variant as u8 != 0
    }
}
impl DAGT1IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAGT1IE_A {
        match self.bits {
            false => DAGT1IE_A::_0,
            true => DAGT1IE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DAGT1IE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DAGT1IE_A::_1
    }
}
#[doc = "Field `DAGT1IE` writer - AGT1 Underflow Deep Standby Cancel Signal Enable"]
pub type DAGT1IE_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIER3_SPEC, DAGT1IE_A, O>;
impl<'a, const O: u8> DAGT1IE_W<'a, O> {
    #[doc = "Canceling deep software standby mode is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DAGT1IE_A::_0)
    }
    #[doc = "Canceling deep software standby mode is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DAGT1IE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - USBFS Suspend/Resume Deep Standby Cancel Signal Enable"]
    #[inline(always)]
    pub fn dusbfsie(&self) -> DUSBFSIE_R {
        DUSBFSIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USBHS Suspend/Resume Deep Standby Cancel Signal Enable"]
    #[inline(always)]
    pub fn dusbhsie(&self) -> DUSBHSIE_R {
        DUSBHSIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AGT1 Underflow Deep Standby Cancel Signal Enable"]
    #[inline(always)]
    pub fn dagt1ie(&self) -> DAGT1IE_R {
        DAGT1IE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USBFS Suspend/Resume Deep Standby Cancel Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dusbfsie(&mut self) -> DUSBFSIE_W<0> {
        DUSBFSIE_W::new(self)
    }
    #[doc = "Bit 1 - USBHS Suspend/Resume Deep Standby Cancel Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dusbhsie(&mut self) -> DUSBHSIE_W<1> {
        DUSBHSIE_W::new(self)
    }
    #[doc = "Bit 2 - AGT1 Underflow Deep Standby Cancel Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dagt1ie(&mut self) -> DAGT1IE_W<2> {
        DAGT1IE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Deep Standby Interrupt Enable Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpsier3](index.html) module"]
pub struct DPSIER3_SPEC;
impl crate::RegisterSpec for DPSIER3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dpsier3::R](R) reader structure"]
impl crate::Readable for DPSIER3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dpsier3::W](W) writer structure"]
impl crate::Writable for DPSIER3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPSIER3 to value 0"]
impl crate::Resettable for DPSIER3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
