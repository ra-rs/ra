#[doc = "Register `ELCSARA` reader"]
pub struct R(crate::R<ELCSARA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ELCSARA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ELCSARA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ELCSARA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ELCSARA` writer"]
pub struct W(crate::W<ELCSARA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ELCSARA_SPEC>;
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
impl From<crate::W<ELCSARA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ELCSARA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ELCR` reader - Event Link Controller Register Security Attribution"]
pub type ELCR_R = crate::BitReader<ELCR_A>;
#[doc = "Event Link Controller Register Security Attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELCR_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<ELCR_A> for bool {
    #[inline(always)]
    fn from(variant: ELCR_A) -> Self {
        variant as u8 != 0
    }
}
impl ELCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELCR_A {
        match self.bits {
            false => ELCR_A::_0,
            true => ELCR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELCR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELCR_A::_1
    }
}
#[doc = "Field `ELCR` writer - Event Link Controller Register Security Attribution"]
pub type ELCR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELCSARA_SPEC, ELCR_A, O>;
impl<'a, const O: u8> ELCR_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ELCR_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ELCR_A::_1)
    }
}
#[doc = "Field `ELSEGR0` reader - Event Link Software Event Generation Register 0 Security Attribution"]
pub type ELSEGR0_R = crate::BitReader<ELSEGR0_A>;
#[doc = "Event Link Software Event Generation Register 0 Security Attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELSEGR0_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<ELSEGR0_A> for bool {
    #[inline(always)]
    fn from(variant: ELSEGR0_A) -> Self {
        variant as u8 != 0
    }
}
impl ELSEGR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELSEGR0_A {
        match self.bits {
            false => ELSEGR0_A::_0,
            true => ELSEGR0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELSEGR0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELSEGR0_A::_1
    }
}
#[doc = "Field `ELSEGR0` writer - Event Link Software Event Generation Register 0 Security Attribution"]
pub type ELSEGR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELCSARA_SPEC, ELSEGR0_A, O>;
impl<'a, const O: u8> ELSEGR0_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ELSEGR0_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ELSEGR0_A::_1)
    }
}
#[doc = "Field `ELSEGR1` reader - Event Link Software Event Generation Register 1 Security Attribution"]
pub type ELSEGR1_R = crate::BitReader<ELSEGR1_A>;
#[doc = "Event Link Software Event Generation Register 1 Security Attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELSEGR1_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<ELSEGR1_A> for bool {
    #[inline(always)]
    fn from(variant: ELSEGR1_A) -> Self {
        variant as u8 != 0
    }
}
impl ELSEGR1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELSEGR1_A {
        match self.bits {
            false => ELSEGR1_A::_0,
            true => ELSEGR1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELSEGR1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELSEGR1_A::_1
    }
}
#[doc = "Field `ELSEGR1` writer - Event Link Software Event Generation Register 1 Security Attribution"]
pub type ELSEGR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELCSARA_SPEC, ELSEGR1_A, O>;
impl<'a, const O: u8> ELSEGR1_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ELSEGR1_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ELSEGR1_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Event Link Controller Register Security Attribution"]
    #[inline(always)]
    pub fn elcr(&self) -> ELCR_R {
        ELCR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Event Link Software Event Generation Register 0 Security Attribution"]
    #[inline(always)]
    pub fn elsegr0(&self) -> ELSEGR0_R {
        ELSEGR0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Event Link Software Event Generation Register 1 Security Attribution"]
    #[inline(always)]
    pub fn elsegr1(&self) -> ELSEGR1_R {
        ELSEGR1_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event Link Controller Register Security Attribution"]
    #[inline(always)]
    #[must_use]
    pub fn elcr(&mut self) -> ELCR_W<0> {
        ELCR_W::new(self)
    }
    #[doc = "Bit 1 - Event Link Software Event Generation Register 0 Security Attribution"]
    #[inline(always)]
    #[must_use]
    pub fn elsegr0(&mut self) -> ELSEGR0_W<1> {
        ELSEGR0_W::new(self)
    }
    #[doc = "Bit 2 - Event Link Software Event Generation Register 1 Security Attribution"]
    #[inline(always)]
    #[must_use]
    pub fn elsegr1(&mut self) -> ELSEGR1_W<2> {
        ELSEGR1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Link Controller Security Attribution Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [elcsara](index.html) module"]
pub struct ELCSARA_SPEC;
impl crate::RegisterSpec for ELCSARA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [elcsara::R](R) reader structure"]
impl crate::Readable for ELCSARA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [elcsara::W](W) writer structure"]
impl crate::Writable for ELCSARA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ELCSARA to value 0xffff_ffff"]
impl crate::Resettable for ELCSARA_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
