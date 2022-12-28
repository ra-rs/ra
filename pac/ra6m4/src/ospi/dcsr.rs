#[doc = "Register `DCSR` reader"]
pub struct R(crate::R<DCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCSR` writer"]
pub struct W(crate::W<DCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCSR_SPEC>;
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
impl From<crate::W<DCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DALEN` reader - Transfer data length setting"]
pub type DALEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DALEN` writer - Transfer data length setting"]
pub type DALEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCSR_SPEC, u8, u8, 8, O>;
#[doc = "Field `DMLEN` reader - Dummy cycle setting"]
pub type DMLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMLEN` writer - Dummy cycle setting"]
pub type DMLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCSR_SPEC, u8, u8, 8, O>;
#[doc = "Field `ACDV` reader - Access Device setting"]
pub type ACDV_R = crate::BitReader<ACDV_A>;
#[doc = "Access Device setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACDV_A {
    #[doc = "0: Send commands to device 0."]
    _0 = 0,
    #[doc = "1: Send commands to device 1."]
    _1 = 1,
}
impl From<ACDV_A> for bool {
    #[inline(always)]
    fn from(variant: ACDV_A) -> Self {
        variant as u8 != 0
    }
}
impl ACDV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACDV_A {
        match self.bits {
            false => ACDV_A::_0,
            true => ACDV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACDV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACDV_A::_1
    }
}
#[doc = "Field `ACDV` writer - Access Device setting"]
pub type ACDV_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCSR_SPEC, ACDV_A, O>;
impl<'a, const O: u8> ACDV_W<'a, O> {
    #[doc = "Send commands to device 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACDV_A::_0)
    }
    #[doc = "Send commands to device 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACDV_A::_1)
    }
}
#[doc = "Field `CMDLEN` reader - Transfer command length setting"]
pub type CMDLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMDLEN` writer - Transfer command length setting"]
pub type CMDLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCSR_SPEC, u8, u8, 3, O>;
#[doc = "Field `DAOR` reader - Data order setting"]
pub type DAOR_R = crate::BitReader<DAOR_A>;
#[doc = "Data order setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAOR_A {
    #[doc = "0: byte0, byte1, byte2, byte3"]
    _0 = 0,
    #[doc = "1: byte1, byte0, byte3, byte2"]
    _1 = 1,
}
impl From<DAOR_A> for bool {
    #[inline(always)]
    fn from(variant: DAOR_A) -> Self {
        variant as u8 != 0
    }
}
impl DAOR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAOR_A {
        match self.bits {
            false => DAOR_A::_0,
            true => DAOR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DAOR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DAOR_A::_1
    }
}
#[doc = "Field `DAOR` writer - Data order setting"]
pub type DAOR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCSR_SPEC, DAOR_A, O>;
impl<'a, const O: u8> DAOR_W<'a, O> {
    #[doc = "byte0, byte1, byte2, byte3"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DAOR_A::_0)
    }
    #[doc = "byte1, byte0, byte3, byte2"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DAOR_A::_1)
    }
}
#[doc = "Field `ADLEN` reader - Transfer address length setting"]
pub type ADLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADLEN` writer - Transfer address length setting"]
pub type ADLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCSR_SPEC, u8, u8, 3, O>;
#[doc = "Field `DOPI` reader - DOPI single byte setting"]
pub type DOPI_R = crate::BitReader<DOPI_A>;
#[doc = "DOPI single byte setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOPI_A {
    #[doc = "0: Each cycle has two bytes data. (normal DOPI mode)"]
    _0 = 0,
    #[doc = "1: Each cycle has one byte data. (The byte data changes at the rising edge of the clock and does not change at the falling edge of the clock.)"]
    _1 = 1,
}
impl From<DOPI_A> for bool {
    #[inline(always)]
    fn from(variant: DOPI_A) -> Self {
        variant as u8 != 0
    }
}
impl DOPI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOPI_A {
        match self.bits {
            false => DOPI_A::_0,
            true => DOPI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DOPI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DOPI_A::_1
    }
}
#[doc = "Field `DOPI` writer - DOPI single byte setting"]
pub type DOPI_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCSR_SPEC, DOPI_A, O>;
impl<'a, const O: u8> DOPI_W<'a, O> {
    #[doc = "Each cycle has two bytes data. (normal DOPI mode)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOPI_A::_0)
    }
    #[doc = "Each cycle has one byte data. (The byte data changes at the rising edge of the clock and does not change at the falling edge of the clock.)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOPI_A::_1)
    }
}
#[doc = "Field `ACDA` reader - Data Access Control"]
pub type ACDA_R = crate::BitReader<ACDA_A>;
#[doc = "Data Access Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACDA_A {
    #[doc = "0: Register access Do not arrange the transfer data."]
    _0 = 0,
    #[doc = "1: Data access"]
    _1 = 1,
}
impl From<ACDA_A> for bool {
    #[inline(always)]
    fn from(variant: ACDA_A) -> Self {
        variant as u8 != 0
    }
}
impl ACDA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACDA_A {
        match self.bits {
            false => ACDA_A::_0,
            true => ACDA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACDA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACDA_A::_1
    }
}
#[doc = "Field `ACDA` writer - Data Access Control"]
pub type ACDA_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCSR_SPEC, ACDA_A, O>;
impl<'a, const O: u8> ACDA_W<'a, O> {
    #[doc = "Register access Do not arrange the transfer data."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACDA_A::_0)
    }
    #[doc = "Data access"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACDA_A::_1)
    }
}
#[doc = "Field `PREN` reader - Preamble bit enable for OctaRAM"]
pub type PREN_R = crate::BitReader<PREN_A>;
#[doc = "Preamble bit enable for OctaRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PREN_A {
    #[doc = "0: No check preamble bit from OctaRAM"]
    _0 = 0,
    #[doc = "1: Check preamble bit from OctaRAM"]
    _1 = 1,
}
impl From<PREN_A> for bool {
    #[inline(always)]
    fn from(variant: PREN_A) -> Self {
        variant as u8 != 0
    }
}
impl PREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PREN_A {
        match self.bits {
            false => PREN_A::_0,
            true => PREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PREN_A::_1
    }
}
#[doc = "Field `PREN` writer - Preamble bit enable for OctaRAM"]
pub type PREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCSR_SPEC, PREN_A, O>;
impl<'a, const O: u8> PREN_W<'a, O> {
    #[doc = "No check preamble bit from OctaRAM"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PREN_A::_0)
    }
    #[doc = "Check preamble bit from OctaRAM"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PREN_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Transfer data length setting"]
    #[inline(always)]
    pub fn dalen(&self) -> DALEN_R {
        DALEN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Dummy cycle setting"]
    #[inline(always)]
    pub fn dmlen(&self) -> DMLEN_R {
        DMLEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 19 - Access Device setting"]
    #[inline(always)]
    pub fn acdv(&self) -> ACDV_R {
        ACDV_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - Transfer command length setting"]
    #[inline(always)]
    pub fn cmdlen(&self) -> CMDLEN_R {
        CMDLEN_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - Data order setting"]
    #[inline(always)]
    pub fn daor(&self) -> DAOR_R {
        DAOR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Transfer address length setting"]
    #[inline(always)]
    pub fn adlen(&self) -> ADLEN_R {
        ADLEN_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - DOPI single byte setting"]
    #[inline(always)]
    pub fn dopi(&self) -> DOPI_R {
        DOPI_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Data Access Control"]
    #[inline(always)]
    pub fn acda(&self) -> ACDA_R {
        ACDA_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Preamble bit enable for OctaRAM"]
    #[inline(always)]
    pub fn pren(&self) -> PREN_R {
        PREN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transfer data length setting"]
    #[inline(always)]
    #[must_use]
    pub fn dalen(&mut self) -> DALEN_W<0> {
        DALEN_W::new(self)
    }
    #[doc = "Bits 8:15 - Dummy cycle setting"]
    #[inline(always)]
    #[must_use]
    pub fn dmlen(&mut self) -> DMLEN_W<8> {
        DMLEN_W::new(self)
    }
    #[doc = "Bit 19 - Access Device setting"]
    #[inline(always)]
    #[must_use]
    pub fn acdv(&mut self) -> ACDV_W<19> {
        ACDV_W::new(self)
    }
    #[doc = "Bits 20:22 - Transfer command length setting"]
    #[inline(always)]
    #[must_use]
    pub fn cmdlen(&mut self) -> CMDLEN_W<20> {
        CMDLEN_W::new(self)
    }
    #[doc = "Bit 23 - Data order setting"]
    #[inline(always)]
    #[must_use]
    pub fn daor(&mut self) -> DAOR_W<23> {
        DAOR_W::new(self)
    }
    #[doc = "Bits 24:26 - Transfer address length setting"]
    #[inline(always)]
    #[must_use]
    pub fn adlen(&mut self) -> ADLEN_W<24> {
        ADLEN_W::new(self)
    }
    #[doc = "Bit 27 - DOPI single byte setting"]
    #[inline(always)]
    #[must_use]
    pub fn dopi(&mut self) -> DOPI_W<27> {
        DOPI_W::new(self)
    }
    #[doc = "Bit 28 - Data Access Control"]
    #[inline(always)]
    #[must_use]
    pub fn acda(&mut self) -> ACDA_W<28> {
        ACDA_W::new(self)
    }
    #[doc = "Bit 29 - Preamble bit enable for OctaRAM"]
    #[inline(always)]
    #[must_use]
    pub fn pren(&mut self) -> PREN_W<29> {
        PREN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Command Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcsr](index.html) module"]
pub struct DCSR_SPEC;
impl crate::RegisterSpec for DCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcsr::R](R) reader structure"]
impl crate::Readable for DCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcsr::W](W) writer structure"]
impl crate::Writable for DCSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCSR to value 0"]
impl crate::Resettable for DCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
