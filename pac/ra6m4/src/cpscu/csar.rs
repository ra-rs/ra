#[doc = "Register `CSAR` reader"]
pub struct R(crate::R<CSAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSAR` writer"]
pub struct W(crate::W<CSAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSAR_SPEC>;
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
impl From<crate::W<CSAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CACHESA` reader - Security Attributes of Registers for Cache Control"]
pub type CACHESA_R = crate::BitReader<CACHESA_A>;
#[doc = "Security Attributes of Registers for Cache Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CACHESA_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<CACHESA_A> for bool {
    #[inline(always)]
    fn from(variant: CACHESA_A) -> Self {
        variant as u8 != 0
    }
}
impl CACHESA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CACHESA_A {
        match self.bits {
            false => CACHESA_A::_0,
            true => CACHESA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CACHESA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CACHESA_A::_1
    }
}
#[doc = "Field `CACHESA` writer - Security Attributes of Registers for Cache Control"]
pub type CACHESA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSAR_SPEC, CACHESA_A, O>;
impl<'a, const O: u8> CACHESA_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CACHESA_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CACHESA_A::_1)
    }
}
#[doc = "Field `CACHELSA` reader - Security Attributes of Registers for Cache Line Configuration"]
pub type CACHELSA_R = crate::BitReader<CACHELSA_A>;
#[doc = "Security Attributes of Registers for Cache Line Configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CACHELSA_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<CACHELSA_A> for bool {
    #[inline(always)]
    fn from(variant: CACHELSA_A) -> Self {
        variant as u8 != 0
    }
}
impl CACHELSA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CACHELSA_A {
        match self.bits {
            false => CACHELSA_A::_0,
            true => CACHELSA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CACHELSA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CACHELSA_A::_1
    }
}
#[doc = "Field `CACHELSA` writer - Security Attributes of Registers for Cache Line Configuration"]
pub type CACHELSA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSAR_SPEC, CACHELSA_A, O>;
impl<'a, const O: u8> CACHELSA_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CACHELSA_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CACHELSA_A::_1)
    }
}
#[doc = "Field `CACHEESA` reader - Security Attributes of Registers for Cache Error"]
pub type CACHEESA_R = crate::BitReader<CACHEESA_A>;
#[doc = "Security Attributes of Registers for Cache Error\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CACHEESA_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<CACHEESA_A> for bool {
    #[inline(always)]
    fn from(variant: CACHEESA_A) -> Self {
        variant as u8 != 0
    }
}
impl CACHEESA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CACHEESA_A {
        match self.bits {
            false => CACHEESA_A::_0,
            true => CACHEESA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CACHEESA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CACHEESA_A::_1
    }
}
#[doc = "Field `CACHEESA` writer - Security Attributes of Registers for Cache Error"]
pub type CACHEESA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSAR_SPEC, CACHEESA_A, O>;
impl<'a, const O: u8> CACHEESA_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CACHEESA_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CACHEESA_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Security Attributes of Registers for Cache Control"]
    #[inline(always)]
    pub fn cachesa(&self) -> CACHESA_R {
        CACHESA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Security Attributes of Registers for Cache Line Configuration"]
    #[inline(always)]
    pub fn cachelsa(&self) -> CACHELSA_R {
        CACHELSA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Security Attributes of Registers for Cache Error"]
    #[inline(always)]
    pub fn cacheesa(&self) -> CACHEESA_R {
        CACHEESA_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Security Attributes of Registers for Cache Control"]
    #[inline(always)]
    #[must_use]
    pub fn cachesa(&mut self) -> CACHESA_W<0> {
        CACHESA_W::new(self)
    }
    #[doc = "Bit 1 - Security Attributes of Registers for Cache Line Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn cachelsa(&mut self) -> CACHELSA_W<1> {
        CACHELSA_W::new(self)
    }
    #[doc = "Bit 2 - Security Attributes of Registers for Cache Error"]
    #[inline(always)]
    #[must_use]
    pub fn cacheesa(&mut self) -> CACHEESA_W<2> {
        CACHEESA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache Security Attribution Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csar](index.html) module"]
pub struct CSAR_SPEC;
impl crate::RegisterSpec for CSAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csar::R](R) reader structure"]
impl crate::Readable for CSAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csar::W](W) writer structure"]
impl crate::Writable for CSAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSAR to value 0xffff_ffff"]
impl crate::Resettable for CSAR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
