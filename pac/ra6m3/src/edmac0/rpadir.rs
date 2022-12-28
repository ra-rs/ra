#[doc = "Register `RPADIR` reader"]
pub struct R(crate::R<RPADIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RPADIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RPADIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RPADIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RPADIR` writer"]
pub struct W(crate::W<RPADIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RPADIR_SPEC>;
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
impl From<crate::W<RPADIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RPADIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PADR` reader - Padding Slot"]
pub type PADR_R = crate::FieldReader<u8, PADR_A>;
#[doc = "Padding Slot\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PADR_A {
    #[doc = "0: Padding is inserted at the head of received data."]
    _00H = 0,
}
impl From<PADR_A> for u8 {
    #[inline(always)]
    fn from(variant: PADR_A) -> Self {
        variant as _
    }
}
impl PADR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PADR_A> {
        match self.bits {
            0 => Some(PADR_A::_00H),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00H`"]
    #[inline(always)]
    pub fn is_00h(&self) -> bool {
        *self == PADR_A::_00H
    }
}
#[doc = "Field `PADR` writer - Padding Slot"]
pub type PADR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RPADIR_SPEC, u8, PADR_A, 6, O>;
impl<'a, const O: u8> PADR_W<'a, O> {
    #[doc = "Padding is inserted at the head of received data."]
    #[inline(always)]
    pub fn _00h(self) -> &'a mut W {
        self.variant(PADR_A::_00H)
    }
}
#[doc = "Field `PADS` reader - Padding Size"]
pub type PADS_R = crate::FieldReader<u8, PADS_A>;
#[doc = "Padding Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PADS_A {
    #[doc = "0: No padding is inserted."]
    _00 = 0,
    #[doc = "1: 1 byte is inserted."]
    _01 = 1,
    #[doc = "2: 2 bytes are inserted."]
    _10 = 2,
    #[doc = "3: 3 bytes are inserted."]
    _11 = 3,
}
impl From<PADS_A> for u8 {
    #[inline(always)]
    fn from(variant: PADS_A) -> Self {
        variant as _
    }
}
impl PADS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PADS_A {
        match self.bits {
            0 => PADS_A::_00,
            1 => PADS_A::_01,
            2 => PADS_A::_10,
            3 => PADS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PADS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PADS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == PADS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == PADS_A::_11
    }
}
#[doc = "Field `PADS` writer - Padding Size"]
pub type PADS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, RPADIR_SPEC, u8, PADS_A, 2, O>;
impl<'a, const O: u8> PADS_W<'a, O> {
    #[doc = "No padding is inserted."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PADS_A::_00)
    }
    #[doc = "1 byte is inserted."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PADS_A::_01)
    }
    #[doc = "2 bytes are inserted."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PADS_A::_10)
    }
    #[doc = "3 bytes are inserted."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(PADS_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:5 - Padding Slot"]
    #[inline(always)]
    pub fn padr(&self) -> PADR_R {
        PADR_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:17 - Padding Size"]
    #[inline(always)]
    pub fn pads(&self) -> PADS_R {
        PADS_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Padding Slot"]
    #[inline(always)]
    #[must_use]
    pub fn padr(&mut self) -> PADR_W<0> {
        PADR_W::new(self)
    }
    #[doc = "Bits 16:17 - Padding Size"]
    #[inline(always)]
    #[must_use]
    pub fn pads(&mut self) -> PADS_W<16> {
        PADS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Data Padding Insert Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpadir](index.html) module"]
pub struct RPADIR_SPEC;
impl crate::RegisterSpec for RPADIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rpadir::R](R) reader structure"]
impl crate::Readable for RPADIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rpadir::W](W) writer structure"]
impl crate::Writable for RPADIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RPADIR to value 0"]
impl crate::Resettable for RPADIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
