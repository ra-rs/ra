#[doc = "Register `KRCTL` reader"]
pub struct R(crate::R<KRCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KRCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KRCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KRCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KRCTL` writer"]
pub struct W(crate::W<KRCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KRCTL_SPEC>;
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
impl From<crate::W<KRCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KRCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KREG` reader - Detection Edge Selection (KR00 to KR03 pins)"]
pub type KREG_R = crate::BitReader<KREG_A>;
#[doc = "Detection Edge Selection (KR00 to KR03 pins)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KREG_A {
    #[doc = "0: Falling edge"]
    _0 = 0,
    #[doc = "1: Rising edge"]
    _1 = 1,
}
impl From<KREG_A> for bool {
    #[inline(always)]
    fn from(variant: KREG_A) -> Self {
        variant as u8 != 0
    }
}
impl KREG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KREG_A {
        match self.bits {
            false => KREG_A::_0,
            true => KREG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KREG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KREG_A::_1
    }
}
#[doc = "Field `KREG` writer - Detection Edge Selection (KR00 to KR03 pins)"]
pub type KREG_W<'a, const O: u8> = crate::BitWriter<'a, u8, KRCTL_SPEC, KREG_A, O>;
impl<'a, const O: u8> KREG_W<'a, O> {
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KREG_A::_0)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KREG_A::_1)
    }
}
#[doc = "Field `KRMD` reader - Usage of Key Interrupt Flags (KRF.KIF0 to KRF.KIF3)"]
pub type KRMD_R = crate::BitReader<KRMD_A>;
#[doc = "Usage of Key Interrupt Flags (KRF.KIF0 to KRF.KIF3)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KRMD_A {
    #[doc = "0: Do not use key interrupt flags"]
    _0 = 0,
    #[doc = "1: Use key interrupt flags"]
    _1 = 1,
}
impl From<KRMD_A> for bool {
    #[inline(always)]
    fn from(variant: KRMD_A) -> Self {
        variant as u8 != 0
    }
}
impl KRMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KRMD_A {
        match self.bits {
            false => KRMD_A::_0,
            true => KRMD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KRMD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KRMD_A::_1
    }
}
#[doc = "Field `KRMD` writer - Usage of Key Interrupt Flags (KRF.KIF0 to KRF.KIF3)"]
pub type KRMD_W<'a, const O: u8> = crate::BitWriter<'a, u8, KRCTL_SPEC, KRMD_A, O>;
impl<'a, const O: u8> KRMD_W<'a, O> {
    #[doc = "Do not use key interrupt flags"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KRMD_A::_0)
    }
    #[doc = "Use key interrupt flags"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KRMD_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Detection Edge Selection (KR00 to KR03 pins)"]
    #[inline(always)]
    pub fn kreg(&self) -> KREG_R {
        KREG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - Usage of Key Interrupt Flags (KRF.KIF0 to KRF.KIF3)"]
    #[inline(always)]
    pub fn krmd(&self) -> KRMD_R {
        KRMD_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Detection Edge Selection (KR00 to KR03 pins)"]
    #[inline(always)]
    #[must_use]
    pub fn kreg(&mut self) -> KREG_W<0> {
        KREG_W::new(self)
    }
    #[doc = "Bit 7 - Usage of Key Interrupt Flags (KRF.KIF0 to KRF.KIF3)"]
    #[inline(always)]
    #[must_use]
    pub fn krmd(&mut self) -> KRMD_W<7> {
        KRMD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Key Return Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [krctl](index.html) module"]
pub struct KRCTL_SPEC;
impl crate::RegisterSpec for KRCTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [krctl::R](R) reader structure"]
impl crate::Readable for KRCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [krctl::W](W) writer structure"]
impl crate::Writable for KRCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KRCTL to value 0"]
impl crate::Resettable for KRCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
