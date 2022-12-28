#[doc = "Register `CFDTXQSTS` reader"]
pub struct R(crate::R<CFDTXQSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDTXQSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDTXQSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDTXQSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDTXQSTS` writer"]
pub struct W(crate::W<CFDTXQSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDTXQSTS_SPEC>;
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
impl From<crate::W<CFDTXQSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDTXQSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXQEMP` reader - TX Queue Empty"]
pub type TXQEMP_R = crate::BitReader<TXQEMP_A>;
#[doc = "TX Queue Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXQEMP_A {
    #[doc = "0: TX Queue not empty"]
    _0 = 0,
    #[doc = "1: TX Queue empty"]
    _1 = 1,
}
impl From<TXQEMP_A> for bool {
    #[inline(always)]
    fn from(variant: TXQEMP_A) -> Self {
        variant as u8 != 0
    }
}
impl TXQEMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXQEMP_A {
        match self.bits {
            false => TXQEMP_A::_0,
            true => TXQEMP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXQEMP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXQEMP_A::_1
    }
}
#[doc = "Field `TXQFLL` reader - TX Queue Full"]
pub type TXQFLL_R = crate::BitReader<TXQFLL_A>;
#[doc = "TX Queue Full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXQFLL_A {
    #[doc = "0: TX Queue not full"]
    _0 = 0,
    #[doc = "1: TX Queue full"]
    _1 = 1,
}
impl From<TXQFLL_A> for bool {
    #[inline(always)]
    fn from(variant: TXQFLL_A) -> Self {
        variant as u8 != 0
    }
}
impl TXQFLL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXQFLL_A {
        match self.bits {
            false => TXQFLL_A::_0,
            true => TXQFLL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXQFLL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXQFLL_A::_1
    }
}
#[doc = "Field `TXQTXIF` reader - TX Queue TX Interrupt Flag"]
pub type TXQTXIF_R = crate::BitReader<TXQTXIF_A>;
#[doc = "TX Queue TX Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXQTXIF_A {
    #[doc = "0: TX Queue interrupt condition not satisfied after a frame TX"]
    _0 = 0,
    #[doc = "1: TX Queue interrupt condition satisfied after a frame TX"]
    _1 = 1,
}
impl From<TXQTXIF_A> for bool {
    #[inline(always)]
    fn from(variant: TXQTXIF_A) -> Self {
        variant as u8 != 0
    }
}
impl TXQTXIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXQTXIF_A {
        match self.bits {
            false => TXQTXIF_A::_0,
            true => TXQTXIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXQTXIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXQTXIF_A::_1
    }
}
#[doc = "Field `TXQTXIF` writer - TX Queue TX Interrupt Flag"]
pub type TXQTXIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDTXQSTS_SPEC, TXQTXIF_A, O>;
impl<'a, const O: u8> TXQTXIF_W<'a, O> {
    #[doc = "TX Queue interrupt condition not satisfied after a frame TX"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXQTXIF_A::_0)
    }
    #[doc = "TX Queue interrupt condition satisfied after a frame TX"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXQTXIF_A::_1)
    }
}
#[doc = "Field `TXQMC` reader - TX Queue Message Count"]
pub type TXQMC_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - TX Queue Empty"]
    #[inline(always)]
    pub fn txqemp(&self) -> TXQEMP_R {
        TXQEMP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX Queue Full"]
    #[inline(always)]
    pub fn txqfll(&self) -> TXQFLL_R {
        TXQFLL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TX Queue TX Interrupt Flag"]
    #[inline(always)]
    pub fn txqtxif(&self) -> TXQTXIF_R {
        TXQTXIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:10 - TX Queue Message Count"]
    #[inline(always)]
    pub fn txqmc(&self) -> TXQMC_R {
        TXQMC_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - TX Queue TX Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txqtxif(&mut self) -> TXQTXIF_W<2> {
        TXQTXIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX Queue Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdtxqsts](index.html) module"]
pub struct CFDTXQSTS_SPEC;
impl crate::RegisterSpec for CFDTXQSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdtxqsts::R](R) reader structure"]
impl crate::Readable for CFDTXQSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdtxqsts::W](W) writer structure"]
impl crate::Writable for CFDTXQSTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDTXQSTS to value 0x01"]
impl crate::Resettable for CFDTXQSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
