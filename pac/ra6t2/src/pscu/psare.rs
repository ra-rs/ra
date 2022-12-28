#[doc = "Register `PSARE` reader"]
pub struct R(crate::R<PSARE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSARE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSARE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSARE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSARE` writer"]
pub struct W(crate::W<PSARE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSARE_SPEC>;
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
impl From<crate::W<PSARE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSARE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSARE0` reader - WDT security attribution"]
pub type PSARE0_R = crate::BitReader<PSARE0_A>;
#[doc = "WDT security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARE0_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARE0_A> for bool {
    #[inline(always)]
    fn from(variant: PSARE0_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARE0_A {
        match self.bits {
            false => PSARE0_A::_0,
            true => PSARE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARE0_A::_1
    }
}
#[doc = "Field `PSARE0` writer - WDT security attribution"]
pub type PSARE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARE_SPEC, PSARE0_A, O>;
impl<'a, const O: u8> PSARE0_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARE0_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARE0_A::_1)
    }
}
#[doc = "Field `PSARE1` reader - IWDT security attribution"]
pub type PSARE1_R = crate::BitReader<PSARE1_A>;
#[doc = "IWDT security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARE1_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARE1_A> for bool {
    #[inline(always)]
    fn from(variant: PSARE1_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARE1_A {
        match self.bits {
            false => PSARE1_A::_0,
            true => PSARE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARE1_A::_1
    }
}
#[doc = "Field `PSARE1` writer - IWDT security attribution"]
pub type PSARE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARE_SPEC, PSARE1_A, O>;
impl<'a, const O: u8> PSARE1_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARE1_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARE1_A::_1)
    }
}
#[doc = "Field `PSARE4` reader - KINT and the MSTPCRE.MSTPE4 bit security attribution"]
pub type PSARE4_R = crate::BitReader<PSARE4_A>;
#[doc = "KINT and the MSTPCRE.MSTPE4 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARE4_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARE4_A> for bool {
    #[inline(always)]
    fn from(variant: PSARE4_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARE4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARE4_A {
        match self.bits {
            false => PSARE4_A::_0,
            true => PSARE4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARE4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARE4_A::_1
    }
}
#[doc = "Field `PSARE4` writer - KINT and the MSTPCRE.MSTPE4 bit security attribution"]
pub type PSARE4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARE_SPEC, PSARE4_A, O>;
impl<'a, const O: u8> PSARE4_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARE4_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARE4_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - WDT security attribution"]
    #[inline(always)]
    pub fn psare0(&self) -> PSARE0_R {
        PSARE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IWDT security attribution"]
    #[inline(always)]
    pub fn psare1(&self) -> PSARE1_R {
        PSARE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - KINT and the MSTPCRE.MSTPE4 bit security attribution"]
    #[inline(always)]
    pub fn psare4(&self) -> PSARE4_R {
        PSARE4_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WDT security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psare0(&mut self) -> PSARE0_W<0> {
        PSARE0_W::new(self)
    }
    #[doc = "Bit 1 - IWDT security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psare1(&mut self) -> PSARE1_W<1> {
        PSARE1_W::new(self)
    }
    #[doc = "Bit 4 - KINT and the MSTPCRE.MSTPE4 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psare4(&mut self) -> PSARE4_W<4> {
        PSARE4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Security Attribution Register E\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psare](index.html) module"]
pub struct PSARE_SPEC;
impl crate::RegisterSpec for PSARE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psare::R](R) reader structure"]
impl crate::Readable for PSARE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psare::W](W) writer structure"]
impl crate::Writable for PSARE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSARE to value 0xffff_ffff"]
impl crate::Resettable for PSARE_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
