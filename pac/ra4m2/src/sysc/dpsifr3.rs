#[doc = "Register `DPSIFR3` reader"]
pub struct R(crate::R<DPSIFR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPSIFR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPSIFR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPSIFR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPSIFR3` writer"]
pub struct W(crate::W<DPSIFR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPSIFR3_SPEC>;
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
impl From<crate::W<DPSIFR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPSIFR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DUSBFS0IF` reader - USBFS0 Suspend/Resume Deep Standby Cancel Flag"]
pub type DUSBFS0IF_R = crate::BitReader<DUSBFS0IF_A>;
#[doc = "USBFS0 Suspend/Resume Deep Standby Cancel Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DUSBFS0IF_A {
    #[doc = "0: The cancel request is not generated."]
    _0 = 0,
    #[doc = "1: The cancel request is generated."]
    _1 = 1,
}
impl From<DUSBFS0IF_A> for bool {
    #[inline(always)]
    fn from(variant: DUSBFS0IF_A) -> Self {
        variant as u8 != 0
    }
}
impl DUSBFS0IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DUSBFS0IF_A {
        match self.bits {
            false => DUSBFS0IF_A::_0,
            true => DUSBFS0IF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DUSBFS0IF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DUSBFS0IF_A::_1
    }
}
#[doc = "Field `DUSBFS0IF` writer - USBFS0 Suspend/Resume Deep Standby Cancel Flag"]
pub type DUSBFS0IF_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIFR3_SPEC, DUSBFS0IF_A, O>;
impl<'a, const O: u8> DUSBFS0IF_W<'a, O> {
    #[doc = "The cancel request is not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DUSBFS0IF_A::_0)
    }
    #[doc = "The cancel request is generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DUSBFS0IF_A::_1)
    }
}
#[doc = "Field `DAGT1IF` reader - AGT1 Underflow Deep Standby Cancel Flag"]
pub type DAGT1IF_R = crate::BitReader<DAGT1IF_A>;
#[doc = "AGT1 Underflow Deep Standby Cancel Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAGT1IF_A {
    #[doc = "0: The cancel request is not generated."]
    _0 = 0,
    #[doc = "1: The cancel request is generated."]
    _1 = 1,
}
impl From<DAGT1IF_A> for bool {
    #[inline(always)]
    fn from(variant: DAGT1IF_A) -> Self {
        variant as u8 != 0
    }
}
impl DAGT1IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAGT1IF_A {
        match self.bits {
            false => DAGT1IF_A::_0,
            true => DAGT1IF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DAGT1IF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DAGT1IF_A::_1
    }
}
#[doc = "Field `DAGT1IF` writer - AGT1 Underflow Deep Standby Cancel Flag"]
pub type DAGT1IF_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIFR3_SPEC, DAGT1IF_A, O>;
impl<'a, const O: u8> DAGT1IF_W<'a, O> {
    #[doc = "The cancel request is not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DAGT1IF_A::_0)
    }
    #[doc = "The cancel request is generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DAGT1IF_A::_1)
    }
}
#[doc = "Field `DAGT3IF` reader - AGT3 Underflow Deep Standby Cancel Flag"]
pub type DAGT3IF_R = crate::BitReader<DAGT3IF_A>;
#[doc = "AGT3 Underflow Deep Standby Cancel Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAGT3IF_A {
    #[doc = "0: The cancel request is not generated."]
    _0 = 0,
    #[doc = "1: The cancel request is generated."]
    _1 = 1,
}
impl From<DAGT3IF_A> for bool {
    #[inline(always)]
    fn from(variant: DAGT3IF_A) -> Self {
        variant as u8 != 0
    }
}
impl DAGT3IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAGT3IF_A {
        match self.bits {
            false => DAGT3IF_A::_0,
            true => DAGT3IF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DAGT3IF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DAGT3IF_A::_1
    }
}
#[doc = "Field `DAGT3IF` writer - AGT3 Underflow Deep Standby Cancel Flag"]
pub type DAGT3IF_W<'a, const O: u8> = crate::BitWriter<'a, u8, DPSIFR3_SPEC, DAGT3IF_A, O>;
impl<'a, const O: u8> DAGT3IF_W<'a, O> {
    #[doc = "The cancel request is not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DAGT3IF_A::_0)
    }
    #[doc = "The cancel request is generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DAGT3IF_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - USBFS0 Suspend/Resume Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dusbfs0if(&self) -> DUSBFS0IF_R {
        DUSBFS0IF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - AGT1 Underflow Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dagt1if(&self) -> DAGT1IF_R {
        DAGT1IF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AGT3 Underflow Deep Standby Cancel Flag"]
    #[inline(always)]
    pub fn dagt3if(&self) -> DAGT3IF_R {
        DAGT3IF_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USBFS0 Suspend/Resume Deep Standby Cancel Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dusbfs0if(&mut self) -> DUSBFS0IF_W<0> {
        DUSBFS0IF_W::new(self)
    }
    #[doc = "Bit 2 - AGT1 Underflow Deep Standby Cancel Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dagt1if(&mut self) -> DAGT1IF_W<2> {
        DAGT1IF_W::new(self)
    }
    #[doc = "Bit 3 - AGT3 Underflow Deep Standby Cancel Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dagt3if(&mut self) -> DAGT3IF_W<3> {
        DAGT3IF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Deep Standby Interrupt Flag Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpsifr3](index.html) module"]
pub struct DPSIFR3_SPEC;
impl crate::RegisterSpec for DPSIFR3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dpsifr3::R](R) reader structure"]
impl crate::Readable for DPSIFR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dpsifr3::W](W) writer structure"]
impl crate::Writable for DPSIFR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPSIFR3 to value 0"]
impl crate::Resettable for DPSIFR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
