#[doc = "Register `MESR` reader"]
pub struct R(crate::R<MESR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MESR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MESR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MESR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MESR` writer"]
pub struct W(crate::W<MESR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MESR_SPEC>;
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
impl From<crate::W<MESR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MESR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PFER` reader - Preface Error flag"]
pub type PFER_R = crate::BitReader<PFER_A>;
#[doc = "Preface Error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PFER_A {
    #[doc = "0: No preface error detected"]
    _0 = 0,
    #[doc = "1: Preface error detected"]
    _1 = 1,
}
impl From<PFER_A> for bool {
    #[inline(always)]
    fn from(variant: PFER_A) -> Self {
        variant as u8 != 0
    }
}
impl PFER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFER_A {
        match self.bits {
            false => PFER_A::_0,
            true => PFER_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PFER_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PFER_A::_1
    }
}
#[doc = "Field `PFER` writer - Preface Error flag"]
pub type PFER_W<'a, const O: u8> = crate::BitWriter<'a, u8, MESR_SPEC, PFER_A, O>;
impl<'a, const O: u8> PFER_W<'a, O> {
    #[doc = "No preface error detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PFER_A::_0)
    }
    #[doc = "Preface error detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PFER_A::_1)
    }
}
#[doc = "Field `SYER` reader - SYNC Error flag"]
pub type SYER_R = crate::BitReader<SYER_A>;
#[doc = "SYNC Error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYER_A {
    #[doc = "0: No receive SYNC error detected"]
    _0 = 0,
    #[doc = "1: Receive SYNC error detected"]
    _1 = 1,
}
impl From<SYER_A> for bool {
    #[inline(always)]
    fn from(variant: SYER_A) -> Self {
        variant as u8 != 0
    }
}
impl SYER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYER_A {
        match self.bits {
            false => SYER_A::_0,
            true => SYER_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SYER_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SYER_A::_1
    }
}
#[doc = "Field `SYER` writer - SYNC Error flag"]
pub type SYER_W<'a, const O: u8> = crate::BitWriter<'a, u8, MESR_SPEC, SYER_A, O>;
impl<'a, const O: u8> SYER_W<'a, O> {
    #[doc = "No receive SYNC error detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYER_A::_0)
    }
    #[doc = "Receive SYNC error detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYER_A::_1)
    }
}
#[doc = "Field `SBER` reader - Start Bit Error flag"]
pub type SBER_R = crate::BitReader<SBER_A>;
#[doc = "Start Bit Error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBER_A {
    #[doc = "0: No start bit error detected"]
    _0 = 0,
    #[doc = "1: Start bit error detected"]
    _1 = 1,
}
impl From<SBER_A> for bool {
    #[inline(always)]
    fn from(variant: SBER_A) -> Self {
        variant as u8 != 0
    }
}
impl SBER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBER_A {
        match self.bits {
            false => SBER_A::_0,
            true => SBER_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SBER_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SBER_A::_1
    }
}
#[doc = "Field `SBER` writer - Start Bit Error flag"]
pub type SBER_W<'a, const O: u8> = crate::BitWriter<'a, u8, MESR_SPEC, SBER_A, O>;
impl<'a, const O: u8> SBER_W<'a, O> {
    #[doc = "No start bit error detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SBER_A::_0)
    }
    #[doc = "Start bit error detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SBER_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Preface Error flag"]
    #[inline(always)]
    pub fn pfer(&self) -> PFER_R {
        PFER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SYNC Error flag"]
    #[inline(always)]
    pub fn syer(&self) -> SYER_R {
        SYER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Start Bit Error flag"]
    #[inline(always)]
    pub fn sber(&self) -> SBER_R {
        SBER_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Preface Error flag"]
    #[inline(always)]
    #[must_use]
    pub fn pfer(&mut self) -> PFER_W<0> {
        PFER_W::new(self)
    }
    #[doc = "Bit 1 - SYNC Error flag"]
    #[inline(always)]
    #[must_use]
    pub fn syer(&mut self) -> SYER_W<1> {
        SYER_W::new(self)
    }
    #[doc = "Bit 2 - Start Bit Error flag"]
    #[inline(always)]
    #[must_use]
    pub fn sber(&mut self) -> SBER_W<2> {
        SBER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Manchester Extended Error Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mesr](index.html) module"]
pub struct MESR_SPEC;
impl crate::RegisterSpec for MESR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mesr::R](R) reader structure"]
impl crate::Readable for MESR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mesr::W](W) writer structure"]
impl crate::Writable for MESR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MESR to value 0"]
impl crate::Resettable for MESR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
