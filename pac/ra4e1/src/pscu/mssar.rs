#[doc = "Register `MSSAR` reader"]
pub struct R(crate::R<MSSAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSSAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSSAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSSAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSSAR` writer"]
pub struct W(crate::W<MSSAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSSAR_SPEC>;
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
impl From<crate::W<MSSAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSSAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSSAR0` reader - The MSTPCRC.MSTPC14 bit security attribution"]
pub type MSSAR0_R = crate::BitReader<MSSAR0_A>;
#[doc = "The MSTPCRC.MSTPC14 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSSAR0_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<MSSAR0_A> for bool {
    #[inline(always)]
    fn from(variant: MSSAR0_A) -> Self {
        variant as u8 != 0
    }
}
impl MSSAR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSSAR0_A {
        match self.bits {
            false => MSSAR0_A::_0,
            true => MSSAR0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSSAR0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSSAR0_A::_1
    }
}
#[doc = "Field `MSSAR0` writer - The MSTPCRC.MSTPC14 bit security attribution"]
pub type MSSAR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSSAR_SPEC, MSSAR0_A, O>;
impl<'a, const O: u8> MSSAR0_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSSAR0_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSSAR0_A::_1)
    }
}
#[doc = "Field `MSSAR1` reader - The MSTPCRA.MSTPA22 bit security attribution"]
pub type MSSAR1_R = crate::BitReader<MSSAR1_A>;
#[doc = "The MSTPCRA.MSTPA22 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSSAR1_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<MSSAR1_A> for bool {
    #[inline(always)]
    fn from(variant: MSSAR1_A) -> Self {
        variant as u8 != 0
    }
}
impl MSSAR1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSSAR1_A {
        match self.bits {
            false => MSSAR1_A::_0,
            true => MSSAR1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSSAR1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSSAR1_A::_1
    }
}
#[doc = "Field `MSSAR1` writer - The MSTPCRA.MSTPA22 bit security attribution"]
pub type MSSAR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSSAR_SPEC, MSSAR1_A, O>;
impl<'a, const O: u8> MSSAR1_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSSAR1_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSSAR1_A::_1)
    }
}
#[doc = "Field `MSSAR2` reader - The MSTPCRA.MSTPA7 bit security attribution"]
pub type MSSAR2_R = crate::BitReader<MSSAR2_A>;
#[doc = "The MSTPCRA.MSTPA7 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSSAR2_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<MSSAR2_A> for bool {
    #[inline(always)]
    fn from(variant: MSSAR2_A) -> Self {
        variant as u8 != 0
    }
}
impl MSSAR2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSSAR2_A {
        match self.bits {
            false => MSSAR2_A::_0,
            true => MSSAR2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSSAR2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSSAR2_A::_1
    }
}
#[doc = "Field `MSSAR2` writer - The MSTPCRA.MSTPA7 bit security attribution"]
pub type MSSAR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSSAR_SPEC, MSSAR2_A, O>;
impl<'a, const O: u8> MSSAR2_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSSAR2_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSSAR2_A::_1)
    }
}
#[doc = "Field `MSSAR3` reader - The MSTPCRA.MSTPA0 bit security attribution"]
pub type MSSAR3_R = crate::BitReader<MSSAR3_A>;
#[doc = "The MSTPCRA.MSTPA0 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSSAR3_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<MSSAR3_A> for bool {
    #[inline(always)]
    fn from(variant: MSSAR3_A) -> Self {
        variant as u8 != 0
    }
}
impl MSSAR3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSSAR3_A {
        match self.bits {
            false => MSSAR3_A::_0,
            true => MSSAR3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSSAR3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSSAR3_A::_1
    }
}
#[doc = "Field `MSSAR3` writer - The MSTPCRA.MSTPA0 bit security attribution"]
pub type MSSAR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSSAR_SPEC, MSSAR3_A, O>;
impl<'a, const O: u8> MSSAR3_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSSAR3_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSSAR3_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - The MSTPCRC.MSTPC14 bit security attribution"]
    #[inline(always)]
    pub fn mssar0(&self) -> MSSAR0_R {
        MSSAR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The MSTPCRA.MSTPA22 bit security attribution"]
    #[inline(always)]
    pub fn mssar1(&self) -> MSSAR1_R {
        MSSAR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The MSTPCRA.MSTPA7 bit security attribution"]
    #[inline(always)]
    pub fn mssar2(&self) -> MSSAR2_R {
        MSSAR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The MSTPCRA.MSTPA0 bit security attribution"]
    #[inline(always)]
    pub fn mssar3(&self) -> MSSAR3_R {
        MSSAR3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The MSTPCRC.MSTPC14 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn mssar0(&mut self) -> MSSAR0_W<0> {
        MSSAR0_W::new(self)
    }
    #[doc = "Bit 1 - The MSTPCRA.MSTPA22 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn mssar1(&mut self) -> MSSAR1_W<1> {
        MSSAR1_W::new(self)
    }
    #[doc = "Bit 2 - The MSTPCRA.MSTPA7 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn mssar2(&mut self) -> MSSAR2_W<2> {
        MSSAR2_W::new(self)
    }
    #[doc = "Bit 3 - The MSTPCRA.MSTPA0 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn mssar3(&mut self) -> MSSAR3_W<3> {
        MSSAR3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Module Stop Security Attribution Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mssar](index.html) module"]
pub struct MSSAR_SPEC;
impl crate::RegisterSpec for MSSAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mssar::R](R) reader structure"]
impl crate::Readable for MSSAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mssar::W](W) writer structure"]
impl crate::Writable for MSSAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSSAR to value 0xffff_ffff"]
impl crate::Resettable for MSSAR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
