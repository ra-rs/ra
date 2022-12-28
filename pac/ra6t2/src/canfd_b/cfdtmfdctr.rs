#[doc = "Register `CFDTMFDCTR%s` reader"]
pub struct R(crate::R<CFDTMFDCTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDTMFDCTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDTMFDCTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDTMFDCTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDTMFDCTR%s` writer"]
pub struct W(crate::W<CFDTMFDCTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDTMFDCTR_SPEC>;
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
impl From<crate::W<CFDTMFDCTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDTMFDCTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMESI` reader - Error State Indicator bit"]
pub type TMESI_R = crate::BitReader<TMESI_A>;
#[doc = "Error State Indicator bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMESI_A {
    #[doc = "0: CANFD frame to transmit by error active node"]
    _0 = 0,
    #[doc = "1: CANFD frame to transmit by error passive node"]
    _1 = 1,
}
impl From<TMESI_A> for bool {
    #[inline(always)]
    fn from(variant: TMESI_A) -> Self {
        variant as u8 != 0
    }
}
impl TMESI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMESI_A {
        match self.bits {
            false => TMESI_A::_0,
            true => TMESI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TMESI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TMESI_A::_1
    }
}
#[doc = "Field `TMESI` writer - Error State Indicator bit"]
pub type TMESI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDTMFDCTR_SPEC, TMESI_A, O>;
impl<'a, const O: u8> TMESI_W<'a, O> {
    #[doc = "CANFD frame to transmit by error active node"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMESI_A::_0)
    }
    #[doc = "CANFD frame to transmit by error passive node"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMESI_A::_1)
    }
}
#[doc = "Field `TMBRS` reader - Bit Rate Switch bit"]
pub type TMBRS_R = crate::BitReader<TMBRS_A>;
#[doc = "Bit Rate Switch bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMBRS_A {
    #[doc = "0: CANFD frame to transmit with no bit rate switch"]
    _0 = 0,
    #[doc = "1: CANFD frame to transmit with bit rate switch"]
    _1 = 1,
}
impl From<TMBRS_A> for bool {
    #[inline(always)]
    fn from(variant: TMBRS_A) -> Self {
        variant as u8 != 0
    }
}
impl TMBRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMBRS_A {
        match self.bits {
            false => TMBRS_A::_0,
            true => TMBRS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TMBRS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TMBRS_A::_1
    }
}
#[doc = "Field `TMBRS` writer - Bit Rate Switch bit"]
pub type TMBRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDTMFDCTR_SPEC, TMBRS_A, O>;
impl<'a, const O: u8> TMBRS_W<'a, O> {
    #[doc = "CANFD frame to transmit with no bit rate switch"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMBRS_A::_0)
    }
    #[doc = "CANFD frame to transmit with bit rate switch"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMBRS_A::_1)
    }
}
#[doc = "Field `TMFDF` reader - CAN FD Format bit"]
pub type TMFDF_R = crate::BitReader<TMFDF_A>;
#[doc = "CAN FD Format bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMFDF_A {
    #[doc = "0: Non CANFD frame to transmit"]
    _0 = 0,
    #[doc = "1: CANFD frame to transmit"]
    _1 = 1,
}
impl From<TMFDF_A> for bool {
    #[inline(always)]
    fn from(variant: TMFDF_A) -> Self {
        variant as u8 != 0
    }
}
impl TMFDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMFDF_A {
        match self.bits {
            false => TMFDF_A::_0,
            true => TMFDF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TMFDF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TMFDF_A::_1
    }
}
#[doc = "Field `TMFDF` writer - CAN FD Format bit"]
pub type TMFDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDTMFDCTR_SPEC, TMFDF_A, O>;
impl<'a, const O: u8> TMFDF_W<'a, O> {
    #[doc = "Non CANFD frame to transmit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMFDF_A::_0)
    }
    #[doc = "CANFD frame to transmit"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMFDF_A::_1)
    }
}
#[doc = "Field `TMIFL` reader - TX Message Buffer Information Label Field"]
pub type TMIFL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TMIFL` writer - TX Message Buffer Information Label Field"]
pub type TMIFL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDTMFDCTR_SPEC, u8, u8, 2, O>;
#[doc = "Field `TMPTR` reader - TX Message Buffer Pointer Field"]
pub type TMPTR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TMPTR` writer - TX Message Buffer Pointer Field"]
pub type TMPTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDTMFDCTR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - Error State Indicator bit"]
    #[inline(always)]
    pub fn tmesi(&self) -> TMESI_R {
        TMESI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bit Rate Switch bit"]
    #[inline(always)]
    pub fn tmbrs(&self) -> TMBRS_R {
        TMBRS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CAN FD Format bit"]
    #[inline(always)]
    pub fn tmfdf(&self) -> TMFDF_R {
        TMFDF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:9 - TX Message Buffer Information Label Field"]
    #[inline(always)]
    pub fn tmifl(&self) -> TMIFL_R {
        TMIFL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:31 - TX Message Buffer Pointer Field"]
    #[inline(always)]
    pub fn tmptr(&self) -> TMPTR_R {
        TMPTR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Error State Indicator bit"]
    #[inline(always)]
    #[must_use]
    pub fn tmesi(&mut self) -> TMESI_W<0> {
        TMESI_W::new(self)
    }
    #[doc = "Bit 1 - Bit Rate Switch bit"]
    #[inline(always)]
    #[must_use]
    pub fn tmbrs(&mut self) -> TMBRS_W<1> {
        TMBRS_W::new(self)
    }
    #[doc = "Bit 2 - CAN FD Format bit"]
    #[inline(always)]
    #[must_use]
    pub fn tmfdf(&mut self) -> TMFDF_W<2> {
        TMFDF_W::new(self)
    }
    #[doc = "Bits 8:9 - TX Message Buffer Information Label Field"]
    #[inline(always)]
    #[must_use]
    pub fn tmifl(&mut self) -> TMIFL_W<8> {
        TMIFL_W::new(self)
    }
    #[doc = "Bits 16:31 - TX Message Buffer Pointer Field"]
    #[inline(always)]
    #[must_use]
    pub fn tmptr(&mut self) -> TMPTR_W<16> {
        TMPTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX Message Buffer CANFD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdtmfdctr](index.html) module"]
pub struct CFDTMFDCTR_SPEC;
impl crate::RegisterSpec for CFDTMFDCTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdtmfdctr::R](R) reader structure"]
impl crate::Readable for CFDTMFDCTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdtmfdctr::W](W) writer structure"]
impl crate::Writable for CFDTMFDCTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDTMFDCTR%s to value 0"]
impl crate::Resettable for CFDTMFDCTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
