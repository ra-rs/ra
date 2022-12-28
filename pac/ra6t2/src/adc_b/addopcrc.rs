#[doc = "Register `ADDOPCRC%s` reader"]
pub struct R(crate::R<ADDOPCRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDOPCRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDOPCRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDOPCRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDOPCRC%s` writer"]
pub struct W(crate::W<ADDOPCRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDOPCRC_SPEC>;
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
impl From<crate::W<ADDOPCRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDOPCRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LIMTBLS` reader - Limiter Clip Table Selection"]
pub type LIMTBLS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LIMTBLS` writer - Limiter Clip Table Selection"]
pub type LIMTBLS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDOPCRC_SPEC, u8, u8, 4, O>;
#[doc = "Field `ADPRC` reader - A/D Conversion Data Format Selection"]
pub type ADPRC_R = crate::FieldReader<u8, ADPRC_A>;
#[doc = "A/D Conversion Data Format Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADPRC_A {
    #[doc = "0: Store the A/D conversion result as 16-bit data format"]
    _00 = 0,
    #[doc = "1: Store the A/D conversion result as 14-bit data format"]
    _01 = 1,
    #[doc = "2: Store the A/D conversion result as 12-bit data format"]
    _10 = 2,
    #[doc = "3: Store the A/D conversion result as 10-bit data format"]
    _11 = 3,
}
impl From<ADPRC_A> for u8 {
    #[inline(always)]
    fn from(variant: ADPRC_A) -> Self {
        variant as _
    }
}
impl ADPRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADPRC_A {
        match self.bits {
            0 => ADPRC_A::_00,
            1 => ADPRC_A::_01,
            2 => ADPRC_A::_10,
            3 => ADPRC_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == ADPRC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == ADPRC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == ADPRC_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == ADPRC_A::_11
    }
}
#[doc = "Field `ADPRC` writer - A/D Conversion Data Format Selection"]
pub type ADPRC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, ADDOPCRC_SPEC, u8, ADPRC_A, 2, O>;
impl<'a, const O: u8> ADPRC_W<'a, O> {
    #[doc = "Store the A/D conversion result as 16-bit data format"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(ADPRC_A::_00)
    }
    #[doc = "Store the A/D conversion result as 14-bit data format"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(ADPRC_A::_01)
    }
    #[doc = "Store the A/D conversion result as 12-bit data format"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(ADPRC_A::_10)
    }
    #[doc = "Store the A/D conversion result as 10-bit data format"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(ADPRC_A::_11)
    }
}
#[doc = "Field `SIGNSEL` reader - A/D Conversion Data Signed/Un-signed Selection"]
pub type SIGNSEL_R = crate::BitReader<SIGNSEL_A>;
#[doc = "A/D Conversion Data Signed/Un-signed Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SIGNSEL_A {
    #[doc = "0: Signed data format"]
    _0 = 0,
    #[doc = "1: Un-signed data format"]
    _1 = 1,
}
impl From<SIGNSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SIGNSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl SIGNSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIGNSEL_A {
        match self.bits {
            false => SIGNSEL_A::_0,
            true => SIGNSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SIGNSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SIGNSEL_A::_1
    }
}
#[doc = "Field `SIGNSEL` writer - A/D Conversion Data Signed/Un-signed Selection"]
pub type SIGNSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADDOPCRC_SPEC, SIGNSEL_A, O>;
impl<'a, const O: u8> SIGNSEL_W<'a, O> {
    #[doc = "Signed data format"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SIGNSEL_A::_0)
    }
    #[doc = "Un-signed data format"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SIGNSEL_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - Limiter Clip Table Selection"]
    #[inline(always)]
    pub fn limtbls(&self) -> LIMTBLS_R {
        LIMTBLS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - A/D Conversion Data Format Selection"]
    #[inline(always)]
    pub fn adprc(&self) -> ADPRC_R {
        ADPRC_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20 - A/D Conversion Data Signed/Un-signed Selection"]
    #[inline(always)]
    pub fn signsel(&self) -> SIGNSEL_R {
        SIGNSEL_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Limiter Clip Table Selection"]
    #[inline(always)]
    #[must_use]
    pub fn limtbls(&mut self) -> LIMTBLS_W<0> {
        LIMTBLS_W::new(self)
    }
    #[doc = "Bits 16:17 - A/D Conversion Data Format Selection"]
    #[inline(always)]
    #[must_use]
    pub fn adprc(&mut self) -> ADPRC_W<16> {
        ADPRC_W::new(self)
    }
    #[doc = "Bit 20 - A/D Conversion Data Signed/Un-signed Selection"]
    #[inline(always)]
    #[must_use]
    pub fn signsel(&mut self) -> SIGNSEL_W<20> {
        SIGNSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Conversion Data Operation Control C Register %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addopcrc](index.html) module"]
pub struct ADDOPCRC_SPEC;
impl crate::RegisterSpec for ADDOPCRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addopcrc::R](R) reader structure"]
impl crate::Readable for ADDOPCRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addopcrc::W](W) writer structure"]
impl crate::Writable for ADDOPCRC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDOPCRC%s to value 0"]
impl crate::Resettable for ADDOPCRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
