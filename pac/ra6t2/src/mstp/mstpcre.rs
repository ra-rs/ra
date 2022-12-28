#[doc = "Register `MSTPCRE` reader"]
pub struct R(crate::R<MSTPCRE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSTPCRE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSTPCRE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSTPCRE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSTPCRE` writer"]
pub struct W(crate::W<MSTPCRE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSTPCRE_SPEC>;
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
impl From<crate::W<MSTPCRE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSTPCRE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSTPE4` reader - Key Interrupt Function Module Stop"]
pub type MSTPE4_R = crate::BitReader<MSTPE4_A>;
#[doc = "Key Interrupt Function Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPE4_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPE4_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPE4_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPE4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPE4_A {
        match self.bits {
            false => MSTPE4_A::_0,
            true => MSTPE4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPE4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPE4_A::_1
    }
}
#[doc = "Field `MSTPE4` writer - Key Interrupt Function Module Stop"]
pub type MSTPE4_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRE_SPEC, MSTPE4_A, O>;
impl<'a, const O: u8> MSTPE4_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPE4_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPE4_A::_1)
    }
}
#[doc = "Field `MSTPE31` reader - General PWM Timer and PWM Delay Generation Circuit Module Stop"]
pub type MSTPE31_R = crate::BitReader<MSTPE31_A>;
#[doc = "General PWM Timer and PWM Delay Generation Circuit Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPE31_A {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<MSTPE31_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPE31_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPE31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPE31_A {
        match self.bits {
            false => MSTPE31_A::_0,
            true => MSTPE31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPE31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPE31_A::_1
    }
}
#[doc = "Field `MSTPE31` writer - General PWM Timer and PWM Delay Generation Circuit Module Stop"]
pub type MSTPE31_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTPCRE_SPEC, MSTPE31_A, O>;
impl<'a, const O: u8> MSTPE31_W<'a, O> {
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTPE31_A::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTPE31_A::_1)
    }
}
impl R {
    #[doc = "Bit 4 - Key Interrupt Function Module Stop"]
    #[inline(always)]
    pub fn mstpe4(&self) -> MSTPE4_R {
        MSTPE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 31 - General PWM Timer and PWM Delay Generation Circuit Module Stop"]
    #[inline(always)]
    pub fn mstpe31(&self) -> MSTPE31_R {
        MSTPE31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Key Interrupt Function Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpe4(&mut self) -> MSTPE4_W<4> {
        MSTPE4_W::new(self)
    }
    #[doc = "Bit 31 - General PWM Timer and PWM Delay Generation Circuit Module Stop"]
    #[inline(always)]
    #[must_use]
    pub fn mstpe31(&mut self) -> MSTPE31_W<31> {
        MSTPE31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Module Stop Control Register E\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mstpcre](index.html) module"]
pub struct MSTPCRE_SPEC;
impl crate::RegisterSpec for MSTPCRE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mstpcre::R](R) reader structure"]
impl crate::Readable for MSTPCRE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mstpcre::W](W) writer structure"]
impl crate::Writable for MSTPCRE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSTPCRE to value 0xffff_ffff"]
impl crate::Resettable for MSTPCRE_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
