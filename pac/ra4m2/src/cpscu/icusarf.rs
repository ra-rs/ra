#[doc = "Register `ICUSARF` reader"]
pub struct R(crate::R<ICUSARF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICUSARF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICUSARF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICUSARF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICUSARF` writer"]
pub struct W(crate::W<ICUSARF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICUSARF_SPEC>;
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
impl From<crate::W<ICUSARF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICUSARF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAAGT3UDWUP` reader - Security attributes of registers for WUPEN1.b0"]
pub type SAAGT3UDWUP_R = crate::BitReader<SAAGT3UDWUP_A>;
#[doc = "Security attributes of registers for WUPEN1.b0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAAGT3UDWUP_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAAGT3UDWUP_A> for bool {
    #[inline(always)]
    fn from(variant: SAAGT3UDWUP_A) -> Self {
        variant as u8 != 0
    }
}
impl SAAGT3UDWUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAAGT3UDWUP_A {
        match self.bits {
            false => SAAGT3UDWUP_A::_0,
            true => SAAGT3UDWUP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAAGT3UDWUP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAAGT3UDWUP_A::_1
    }
}
#[doc = "Field `SAAGT3UDWUP` writer - Security attributes of registers for WUPEN1.b0"]
pub type SAAGT3UDWUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARF_SPEC, SAAGT3UDWUP_A, O>;
impl<'a, const O: u8> SAAGT3UDWUP_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAAGT3UDWUP_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAAGT3UDWUP_A::_1)
    }
}
#[doc = "Field `SAAGT3CAWUP` reader - Security attributes of registers for WUPEN1.b1"]
pub type SAAGT3CAWUP_R = crate::BitReader<SAAGT3CAWUP_A>;
#[doc = "Security attributes of registers for WUPEN1.b1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAAGT3CAWUP_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAAGT3CAWUP_A> for bool {
    #[inline(always)]
    fn from(variant: SAAGT3CAWUP_A) -> Self {
        variant as u8 != 0
    }
}
impl SAAGT3CAWUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAAGT3CAWUP_A {
        match self.bits {
            false => SAAGT3CAWUP_A::_0,
            true => SAAGT3CAWUP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAAGT3CAWUP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAAGT3CAWUP_A::_1
    }
}
#[doc = "Field `SAAGT3CAWUP` writer - Security attributes of registers for WUPEN1.b1"]
pub type SAAGT3CAWUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARF_SPEC, SAAGT3CAWUP_A, O>;
impl<'a, const O: u8> SAAGT3CAWUP_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAAGT3CAWUP_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAAGT3CAWUP_A::_1)
    }
}
#[doc = "Field `SAAGT3CBWUP` reader - Security attributes of registers for WUPEN1.b2"]
pub type SAAGT3CBWUP_R = crate::BitReader<SAAGT3CBWUP_A>;
#[doc = "Security attributes of registers for WUPEN1.b2\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAAGT3CBWUP_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAAGT3CBWUP_A> for bool {
    #[inline(always)]
    fn from(variant: SAAGT3CBWUP_A) -> Self {
        variant as u8 != 0
    }
}
impl SAAGT3CBWUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAAGT3CBWUP_A {
        match self.bits {
            false => SAAGT3CBWUP_A::_0,
            true => SAAGT3CBWUP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAAGT3CBWUP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAAGT3CBWUP_A::_1
    }
}
#[doc = "Field `SAAGT3CBWUP` writer - Security attributes of registers for WUPEN1.b2"]
pub type SAAGT3CBWUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARF_SPEC, SAAGT3CBWUP_A, O>;
impl<'a, const O: u8> SAAGT3CBWUP_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAAGT3CBWUP_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAAGT3CBWUP_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Security attributes of registers for WUPEN1.b0"]
    #[inline(always)]
    pub fn saagt3udwup(&self) -> SAAGT3UDWUP_R {
        SAAGT3UDWUP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Security attributes of registers for WUPEN1.b1"]
    #[inline(always)]
    pub fn saagt3cawup(&self) -> SAAGT3CAWUP_R {
        SAAGT3CAWUP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Security attributes of registers for WUPEN1.b2"]
    #[inline(always)]
    pub fn saagt3cbwup(&self) -> SAAGT3CBWUP_R {
        SAAGT3CBWUP_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Security attributes of registers for WUPEN1.b0"]
    #[inline(always)]
    #[must_use]
    pub fn saagt3udwup(&mut self) -> SAAGT3UDWUP_W<0> {
        SAAGT3UDWUP_W::new(self)
    }
    #[doc = "Bit 1 - Security attributes of registers for WUPEN1.b1"]
    #[inline(always)]
    #[must_use]
    pub fn saagt3cawup(&mut self) -> SAAGT3CAWUP_W<1> {
        SAAGT3CAWUP_W::new(self)
    }
    #[doc = "Bit 2 - Security attributes of registers for WUPEN1.b2"]
    #[inline(always)]
    #[must_use]
    pub fn saagt3cbwup(&mut self) -> SAAGT3CBWUP_W<2> {
        SAAGT3CBWUP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Controller Unit Security Attribution Register F\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icusarf](index.html) module"]
pub struct ICUSARF_SPEC;
impl crate::RegisterSpec for ICUSARF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icusarf::R](R) reader structure"]
impl crate::Readable for ICUSARF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icusarf::W](W) writer structure"]
impl crate::Writable for ICUSARF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICUSARF to value 0xffff_ffff"]
impl crate::Resettable for ICUSARF_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
