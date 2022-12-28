#[doc = "Register `CFDTMC%s` reader"]
pub struct R(crate::R<CFDTMC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDTMC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDTMC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDTMC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDTMC%s` writer"]
pub struct W(crate::W<CFDTMC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDTMC_SPEC>;
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
impl From<crate::W<CFDTMC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDTMC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMTR` reader - TX Message Buffer Transmission Request"]
pub type TMTR_R = crate::BitReader<TMTR_A>;
#[doc = "TX Message Buffer Transmission Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMTR_A {
    #[doc = "0: TX Message buffer transmission not requested"]
    _0 = 0,
    #[doc = "1: TX message buffer transmission requested"]
    _1 = 1,
}
impl From<TMTR_A> for bool {
    #[inline(always)]
    fn from(variant: TMTR_A) -> Self {
        variant as u8 != 0
    }
}
impl TMTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMTR_A {
        match self.bits {
            false => TMTR_A::_0,
            true => TMTR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TMTR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TMTR_A::_1
    }
}
#[doc = "Field `TMTR` writer - TX Message Buffer Transmission Request"]
pub type TMTR_W<'a, const O: u8> = crate::BitWriter<'a, u8, CFDTMC_SPEC, TMTR_A, O>;
impl<'a, const O: u8> TMTR_W<'a, O> {
    #[doc = "TX Message buffer transmission not requested"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMTR_A::_0)
    }
    #[doc = "TX message buffer transmission requested"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMTR_A::_1)
    }
}
#[doc = "Field `TMTAR` reader - TX Message Buffer Transmission Abort Request"]
pub type TMTAR_R = crate::BitReader<TMTAR_A>;
#[doc = "TX Message Buffer Transmission Abort Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMTAR_A {
    #[doc = "0: TX message buffer transmission request abort not requested"]
    _0 = 0,
    #[doc = "1: TX message buffer transmission request abort requested"]
    _1 = 1,
}
impl From<TMTAR_A> for bool {
    #[inline(always)]
    fn from(variant: TMTAR_A) -> Self {
        variant as u8 != 0
    }
}
impl TMTAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMTAR_A {
        match self.bits {
            false => TMTAR_A::_0,
            true => TMTAR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TMTAR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TMTAR_A::_1
    }
}
#[doc = "Field `TMTAR` writer - TX Message Buffer Transmission Abort Request"]
pub type TMTAR_W<'a, const O: u8> = crate::BitWriter<'a, u8, CFDTMC_SPEC, TMTAR_A, O>;
impl<'a, const O: u8> TMTAR_W<'a, O> {
    #[doc = "TX message buffer transmission request abort not requested"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMTAR_A::_0)
    }
    #[doc = "TX message buffer transmission request abort requested"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMTAR_A::_1)
    }
}
#[doc = "Field `TMOM` reader - TX Message Buffer One-shot Mode"]
pub type TMOM_R = crate::BitReader<TMOM_A>;
#[doc = "TX Message Buffer One-shot Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMOM_A {
    #[doc = "0: TX message buffer not configured in one-shot mode"]
    _0 = 0,
    #[doc = "1: TX message buffer configured in one-shot mode"]
    _1 = 1,
}
impl From<TMOM_A> for bool {
    #[inline(always)]
    fn from(variant: TMOM_A) -> Self {
        variant as u8 != 0
    }
}
impl TMOM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMOM_A {
        match self.bits {
            false => TMOM_A::_0,
            true => TMOM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TMOM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TMOM_A::_1
    }
}
#[doc = "Field `TMOM` writer - TX Message Buffer One-shot Mode"]
pub type TMOM_W<'a, const O: u8> = crate::BitWriter<'a, u8, CFDTMC_SPEC, TMOM_A, O>;
impl<'a, const O: u8> TMOM_W<'a, O> {
    #[doc = "TX message buffer not configured in one-shot mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMOM_A::_0)
    }
    #[doc = "TX message buffer configured in one-shot mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMOM_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - TX Message Buffer Transmission Request"]
    #[inline(always)]
    pub fn tmtr(&self) -> TMTR_R {
        TMTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX Message Buffer Transmission Abort Request"]
    #[inline(always)]
    pub fn tmtar(&self) -> TMTAR_R {
        TMTAR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TX Message Buffer One-shot Mode"]
    #[inline(always)]
    pub fn tmom(&self) -> TMOM_R {
        TMOM_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX Message Buffer Transmission Request"]
    #[inline(always)]
    #[must_use]
    pub fn tmtr(&mut self) -> TMTR_W<0> {
        TMTR_W::new(self)
    }
    #[doc = "Bit 1 - TX Message Buffer Transmission Abort Request"]
    #[inline(always)]
    #[must_use]
    pub fn tmtar(&mut self) -> TMTAR_W<1> {
        TMTAR_W::new(self)
    }
    #[doc = "Bit 2 - TX Message Buffer One-shot Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tmom(&mut self) -> TMOM_W<2> {
        TMOM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX Message Buffer Control Registers %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdtmc](index.html) module"]
pub struct CFDTMC_SPEC;
impl crate::RegisterSpec for CFDTMC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cfdtmc::R](R) reader structure"]
impl crate::Readable for CFDTMC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdtmc::W](W) writer structure"]
impl crate::Writable for CFDTMC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDTMC%s to value 0"]
impl crate::Resettable for CFDTMC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
