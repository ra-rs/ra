#[doc = "Register `FENTRYR` reader"]
pub struct R(crate::R<FENTRYR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FENTRYR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FENTRYR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FENTRYR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FENTRYR` writer"]
pub struct W(crate::W<FENTRYR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FENTRYR_SPEC>;
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
impl From<crate::W<FENTRYR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FENTRYR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FENTRY0` reader - Code Flash P/E Mode Entry 0"]
pub type FENTRY0_R = crate::BitReader<FENTRY0_A>;
#[doc = "Code Flash P/E Mode Entry 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FENTRY0_A {
    #[doc = "0: The code flash is the read mode"]
    _0 = 0,
    #[doc = "1: The code flash is the P/E mode."]
    _1 = 1,
}
impl From<FENTRY0_A> for bool {
    #[inline(always)]
    fn from(variant: FENTRY0_A) -> Self {
        variant as u8 != 0
    }
}
impl FENTRY0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FENTRY0_A {
        match self.bits {
            false => FENTRY0_A::_0,
            true => FENTRY0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FENTRY0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FENTRY0_A::_1
    }
}
#[doc = "Field `FENTRY0` writer - Code Flash P/E Mode Entry 0"]
pub type FENTRY0_W<'a, const O: u8> = crate::BitWriter<'a, u16, FENTRYR_SPEC, FENTRY0_A, O>;
impl<'a, const O: u8> FENTRY0_W<'a, O> {
    #[doc = "The code flash is the read mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FENTRY0_A::_0)
    }
    #[doc = "The code flash is the P/E mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FENTRY0_A::_1)
    }
}
#[doc = "Field `FENTRYD` reader - Data Flash P/E Mode Entry"]
pub type FENTRYD_R = crate::BitReader<FENTRYD_A>;
#[doc = "Data Flash P/E Mode Entry\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FENTRYD_A {
    #[doc = "0: The data flash is the read mode"]
    _0 = 0,
    #[doc = "1: The data flash is the P/E mode."]
    _1 = 1,
}
impl From<FENTRYD_A> for bool {
    #[inline(always)]
    fn from(variant: FENTRYD_A) -> Self {
        variant as u8 != 0
    }
}
impl FENTRYD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FENTRYD_A {
        match self.bits {
            false => FENTRYD_A::_0,
            true => FENTRYD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FENTRYD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FENTRYD_A::_1
    }
}
#[doc = "Field `FENTRYD` writer - Data Flash P/E Mode Entry"]
pub type FENTRYD_W<'a, const O: u8> = crate::BitWriter<'a, u16, FENTRYR_SPEC, FENTRYD_A, O>;
impl<'a, const O: u8> FENTRYD_W<'a, O> {
    #[doc = "The data flash is the read mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FENTRYD_A::_0)
    }
    #[doc = "The data flash is the P/E mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FENTRYD_A::_1)
    }
}
#[doc = "Field `FEKEY` writer - Key Code"]
pub type FEKEY_W<'a, const O: u8> = crate::FieldWriter<'a, u16, FENTRYR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Code Flash P/E Mode Entry 0"]
    #[inline(always)]
    pub fn fentry0(&self) -> FENTRY0_R {
        FENTRY0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - Data Flash P/E Mode Entry"]
    #[inline(always)]
    pub fn fentryd(&self) -> FENTRYD_R {
        FENTRYD_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Code Flash P/E Mode Entry 0"]
    #[inline(always)]
    #[must_use]
    pub fn fentry0(&mut self) -> FENTRY0_W<0> {
        FENTRY0_W::new(self)
    }
    #[doc = "Bit 7 - Data Flash P/E Mode Entry"]
    #[inline(always)]
    #[must_use]
    pub fn fentryd(&mut self) -> FENTRYD_W<7> {
        FENTRYD_W::new(self)
    }
    #[doc = "Bits 8:15 - Key Code"]
    #[inline(always)]
    #[must_use]
    pub fn fekey(&mut self) -> FEKEY_W<8> {
        FEKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash P/E Mode Entry Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fentryr](index.html) module"]
pub struct FENTRYR_SPEC;
impl crate::RegisterSpec for FENTRYR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [fentryr::R](R) reader structure"]
impl crate::Readable for FENTRYR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fentryr::W](W) writer structure"]
impl crate::Writable for FENTRYR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FENTRYR to value 0"]
impl crate::Resettable for FENTRYR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
