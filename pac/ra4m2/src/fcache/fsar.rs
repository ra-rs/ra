#[doc = "Register `FSAR` reader"]
pub struct R(crate::R<FSAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSAR` writer"]
pub struct W(crate::W<FSAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSAR_SPEC>;
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
impl From<crate::W<FSAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLWTSA` reader - FLWT Security Attribution"]
pub type FLWTSA_R = crate::BitReader<FLWTSA_A>;
#[doc = "FLWT Security Attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLWTSA_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-Secure"]
    _1 = 1,
}
impl From<FLWTSA_A> for bool {
    #[inline(always)]
    fn from(variant: FLWTSA_A) -> Self {
        variant as u8 != 0
    }
}
impl FLWTSA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLWTSA_A {
        match self.bits {
            false => FLWTSA_A::_0,
            true => FLWTSA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLWTSA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLWTSA_A::_1
    }
}
#[doc = "Field `FLWTSA` writer - FLWT Security Attribution"]
pub type FLWTSA_W<'a, const O: u8> = crate::BitWriter<'a, u16, FSAR_SPEC, FLWTSA_A, O>;
impl<'a, const O: u8> FLWTSA_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLWTSA_A::_0)
    }
    #[doc = "Non-Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLWTSA_A::_1)
    }
}
#[doc = "Field `FCKMHZSA` reader - FCKMHZ Security Attribution"]
pub type FCKMHZSA_R = crate::BitReader<FCKMHZSA_A>;
#[doc = "FCKMHZ Security Attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCKMHZSA_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-Secure"]
    _1 = 1,
}
impl From<FCKMHZSA_A> for bool {
    #[inline(always)]
    fn from(variant: FCKMHZSA_A) -> Self {
        variant as u8 != 0
    }
}
impl FCKMHZSA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCKMHZSA_A {
        match self.bits {
            false => FCKMHZSA_A::_0,
            true => FCKMHZSA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FCKMHZSA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FCKMHZSA_A::_1
    }
}
#[doc = "Field `FCKMHZSA` writer - FCKMHZ Security Attribution"]
pub type FCKMHZSA_W<'a, const O: u8> = crate::BitWriter<'a, u16, FSAR_SPEC, FCKMHZSA_A, O>;
impl<'a, const O: u8> FCKMHZSA_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FCKMHZSA_A::_0)
    }
    #[doc = "Non-Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FCKMHZSA_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - FLWT Security Attribution"]
    #[inline(always)]
    pub fn flwtsa(&self) -> FLWTSA_R {
        FLWTSA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - FCKMHZ Security Attribution"]
    #[inline(always)]
    pub fn fckmhzsa(&self) -> FCKMHZSA_R {
        FCKMHZSA_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FLWT Security Attribution"]
    #[inline(always)]
    #[must_use]
    pub fn flwtsa(&mut self) -> FLWTSA_W<0> {
        FLWTSA_W::new(self)
    }
    #[doc = "Bit 8 - FCKMHZ Security Attribution"]
    #[inline(always)]
    #[must_use]
    pub fn fckmhzsa(&mut self) -> FCKMHZSA_W<8> {
        FCKMHZSA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Security Attribution Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsar](index.html) module"]
pub struct FSAR_SPEC;
impl crate::RegisterSpec for FSAR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [fsar::R](R) reader structure"]
impl crate::Readable for FSAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsar::W](W) writer structure"]
impl crate::Writable for FSAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSAR to value 0xffff"]
impl crate::Resettable for FSAR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
