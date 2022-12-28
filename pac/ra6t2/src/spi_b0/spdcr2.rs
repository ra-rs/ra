#[doc = "Register `SPDCR2` reader"]
pub struct R(crate::R<SPDCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPDCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPDCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPDCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPDCR2` writer"]
pub struct W(crate::W<SPDCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPDCR2_SPEC>;
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
impl From<crate::W<SPDCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPDCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTRG` reader - Receive FIFO threshold setting"]
pub type RTRG_R = crate::FieldReader<u8, RTRG_A>;
#[doc = "Receive FIFO threshold setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTRG_A {
    #[doc = "0: threshold 0"]
    _00 = 0,
    #[doc = "1: threshold 1"]
    _01 = 1,
    #[doc = "2: threshold 2"]
    _10 = 2,
    #[doc = "3: threshold 3"]
    _11 = 3,
}
impl From<RTRG_A> for u8 {
    #[inline(always)]
    fn from(variant: RTRG_A) -> Self {
        variant as _
    }
}
impl RTRG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTRG_A {
        match self.bits {
            0 => RTRG_A::_00,
            1 => RTRG_A::_01,
            2 => RTRG_A::_10,
            3 => RTRG_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == RTRG_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == RTRG_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == RTRG_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == RTRG_A::_11
    }
}
#[doc = "Field `RTRG` writer - Receive FIFO threshold setting"]
pub type RTRG_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SPDCR2_SPEC, u8, RTRG_A, 2, O>;
impl<'a, const O: u8> RTRG_W<'a, O> {
    #[doc = "threshold 0"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(RTRG_A::_00)
    }
    #[doc = "threshold 1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(RTRG_A::_01)
    }
    #[doc = "threshold 2"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(RTRG_A::_10)
    }
    #[doc = "threshold 3"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(RTRG_A::_11)
    }
}
#[doc = "Field `TTRG` reader - Transmission FIFO threshold setting"]
pub type TTRG_R = crate::FieldReader<u8, TTRG_A>;
#[doc = "Transmission FIFO threshold setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TTRG_A {
    #[doc = "0: threshold 0"]
    _00 = 0,
    #[doc = "1: threshold 1"]
    _01 = 1,
    #[doc = "2: threshold 2"]
    _10 = 2,
    #[doc = "3: threshold 3"]
    _11 = 3,
}
impl From<TTRG_A> for u8 {
    #[inline(always)]
    fn from(variant: TTRG_A) -> Self {
        variant as _
    }
}
impl TTRG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TTRG_A {
        match self.bits {
            0 => TTRG_A::_00,
            1 => TTRG_A::_01,
            2 => TTRG_A::_10,
            3 => TTRG_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TTRG_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TTRG_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TTRG_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TTRG_A::_11
    }
}
#[doc = "Field `TTRG` writer - Transmission FIFO threshold setting"]
pub type TTRG_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SPDCR2_SPEC, u8, TTRG_A, 2, O>;
impl<'a, const O: u8> TTRG_W<'a, O> {
    #[doc = "threshold 0"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TTRG_A::_00)
    }
    #[doc = "threshold 1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TTRG_A::_01)
    }
    #[doc = "threshold 2"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TTRG_A::_10)
    }
    #[doc = "threshold 3"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TTRG_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - Receive FIFO threshold setting"]
    #[inline(always)]
    pub fn rtrg(&self) -> RTRG_R {
        RTRG_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Transmission FIFO threshold setting"]
    #[inline(always)]
    pub fn ttrg(&self) -> TTRG_R {
        TTRG_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Receive FIFO threshold setting"]
    #[inline(always)]
    #[must_use]
    pub fn rtrg(&mut self) -> RTRG_W<0> {
        RTRG_W::new(self)
    }
    #[doc = "Bits 8:9 - Transmission FIFO threshold setting"]
    #[inline(always)]
    #[must_use]
    pub fn ttrg(&mut self) -> TTRG_W<8> {
        TTRG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Data Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spdcr2](index.html) module"]
pub struct SPDCR2_SPEC;
impl crate::RegisterSpec for SPDCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spdcr2::R](R) reader structure"]
impl crate::Readable for SPDCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spdcr2::W](W) writer structure"]
impl crate::Writable for SPDCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPDCR2 to value 0"]
impl crate::Resettable for SPDCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
