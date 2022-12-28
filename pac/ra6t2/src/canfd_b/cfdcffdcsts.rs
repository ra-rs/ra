#[doc = "Register `CFDCFFDCSTS` reader"]
pub struct R(crate::R<CFDCFFDCSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDCFFDCSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDCFFDCSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDCFFDCSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDCFFDCSTS` writer"]
pub struct W(crate::W<CFDCFFDCSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDCFFDCSTS_SPEC>;
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
impl From<crate::W<CFDCFFDCSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDCFFDCSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFESI` reader - Error State Indicator bit"]
pub type CFESI_R = crate::BitReader<CFESI_A>;
#[doc = "Error State Indicator bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFESI_A {
    #[doc = "0: CANFD frame received or to transmit by error active node"]
    _0 = 0,
    #[doc = "1: CANFD frame received or to transmit by error passive node"]
    _1 = 1,
}
impl From<CFESI_A> for bool {
    #[inline(always)]
    fn from(variant: CFESI_A) -> Self {
        variant as u8 != 0
    }
}
impl CFESI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFESI_A {
        match self.bits {
            false => CFESI_A::_0,
            true => CFESI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFESI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFESI_A::_1
    }
}
#[doc = "Field `CFESI` writer - Error State Indicator bit"]
pub type CFESI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDCFFDCSTS_SPEC, CFESI_A, O>;
impl<'a, const O: u8> CFESI_W<'a, O> {
    #[doc = "CANFD frame received or to transmit by error active node"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFESI_A::_0)
    }
    #[doc = "CANFD frame received or to transmit by error passive node"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFESI_A::_1)
    }
}
#[doc = "Field `CFBRS` reader - Bit Rate Switch bit"]
pub type CFBRS_R = crate::BitReader<CFBRS_A>;
#[doc = "Bit Rate Switch bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFBRS_A {
    #[doc = "0: CANFD frame received or to transmit with no bit rate switch"]
    _0 = 0,
    #[doc = "1: CANFD frame received or to transmit with bit rate switch"]
    _1 = 1,
}
impl From<CFBRS_A> for bool {
    #[inline(always)]
    fn from(variant: CFBRS_A) -> Self {
        variant as u8 != 0
    }
}
impl CFBRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFBRS_A {
        match self.bits {
            false => CFBRS_A::_0,
            true => CFBRS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFBRS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFBRS_A::_1
    }
}
#[doc = "Field `CFBRS` writer - Bit Rate Switch bit"]
pub type CFBRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDCFFDCSTS_SPEC, CFBRS_A, O>;
impl<'a, const O: u8> CFBRS_W<'a, O> {
    #[doc = "CANFD frame received or to transmit with no bit rate switch"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFBRS_A::_0)
    }
    #[doc = "CANFD frame received or to transmit with bit rate switch"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFBRS_A::_1)
    }
}
#[doc = "Field `CFFDF` reader - CAN FD Format bit"]
pub type CFFDF_R = crate::BitReader<CFFDF_A>;
#[doc = "CAN FD Format bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFFDF_A {
    #[doc = "0: Non CANFD frame received or to transmit"]
    _0 = 0,
    #[doc = "1: CANFD frame received or to transmit"]
    _1 = 1,
}
impl From<CFFDF_A> for bool {
    #[inline(always)]
    fn from(variant: CFFDF_A) -> Self {
        variant as u8 != 0
    }
}
impl CFFDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFFDF_A {
        match self.bits {
            false => CFFDF_A::_0,
            true => CFFDF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFFDF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFFDF_A::_1
    }
}
#[doc = "Field `CFFDF` writer - CAN FD Format bit"]
pub type CFFDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDCFFDCSTS_SPEC, CFFDF_A, O>;
impl<'a, const O: u8> CFFDF_W<'a, O> {
    #[doc = "Non CANFD frame received or to transmit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFFDF_A::_0)
    }
    #[doc = "CANFD frame received or to transmit"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFFDF_A::_1)
    }
}
#[doc = "Field `CFIFL` reader - COMMON FIFO Buffer Information Label Field"]
pub type CFIFL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFIFL` writer - COMMON FIFO Buffer Information Label Field"]
pub type CFIFL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDCFFDCSTS_SPEC, u8, u8, 2, O>;
#[doc = "Field `CFPTR` reader - Common FIFO Buffer Pointer Field"]
pub type CFPTR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CFPTR` writer - Common FIFO Buffer Pointer Field"]
pub type CFPTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDCFFDCSTS_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - Error State Indicator bit"]
    #[inline(always)]
    pub fn cfesi(&self) -> CFESI_R {
        CFESI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bit Rate Switch bit"]
    #[inline(always)]
    pub fn cfbrs(&self) -> CFBRS_R {
        CFBRS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CAN FD Format bit"]
    #[inline(always)]
    pub fn cffdf(&self) -> CFFDF_R {
        CFFDF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:9 - COMMON FIFO Buffer Information Label Field"]
    #[inline(always)]
    pub fn cfifl(&self) -> CFIFL_R {
        CFIFL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:31 - Common FIFO Buffer Pointer Field"]
    #[inline(always)]
    pub fn cfptr(&self) -> CFPTR_R {
        CFPTR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Error State Indicator bit"]
    #[inline(always)]
    #[must_use]
    pub fn cfesi(&mut self) -> CFESI_W<0> {
        CFESI_W::new(self)
    }
    #[doc = "Bit 1 - Bit Rate Switch bit"]
    #[inline(always)]
    #[must_use]
    pub fn cfbrs(&mut self) -> CFBRS_W<1> {
        CFBRS_W::new(self)
    }
    #[doc = "Bit 2 - CAN FD Format bit"]
    #[inline(always)]
    #[must_use]
    pub fn cffdf(&mut self) -> CFFDF_W<2> {
        CFFDF_W::new(self)
    }
    #[doc = "Bits 8:9 - COMMON FIFO Buffer Information Label Field"]
    #[inline(always)]
    #[must_use]
    pub fn cfifl(&mut self) -> CFIFL_W<8> {
        CFIFL_W::new(self)
    }
    #[doc = "Bits 16:31 - Common FIFO Buffer Pointer Field"]
    #[inline(always)]
    #[must_use]
    pub fn cfptr(&mut self) -> CFPTR_W<16> {
        CFPTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Common FIFO Access CANFD Control/Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdcffdcsts](index.html) module"]
pub struct CFDCFFDCSTS_SPEC;
impl crate::RegisterSpec for CFDCFFDCSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdcffdcsts::R](R) reader structure"]
impl crate::Readable for CFDCFFDCSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdcffdcsts::W](W) writer structure"]
impl crate::Writable for CFDCFFDCSTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDCFFDCSTS to value 0"]
impl crate::Resettable for CFDCFFDCSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
