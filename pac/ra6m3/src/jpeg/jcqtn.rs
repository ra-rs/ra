#[doc = "Register `JCQTN` reader"]
pub struct R(crate::R<JCQTN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JCQTN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JCQTN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JCQTN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JCQTN` writer"]
pub struct W(crate::W<JCQTN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JCQTN_SPEC>;
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
impl From<crate::W<JCQTN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JCQTN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QT1` reader - Quantization table number for the first color componentNOTE: Read-only in Decompression."]
pub type QT1_R = crate::FieldReader<u8, QT1_A>;
#[doc = "Quantization table number for the first color componentNOTE: Read-only in Decompression.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum QT1_A {
    #[doc = "0: Use quantization table No.0 (JCQTBL0) as the first color component."]
    _00 = 0,
    #[doc = "1: Use quantization table No.1 (JCQTBL1) as the first color component."]
    _01 = 1,
    #[doc = "2: Use quantization table No.2 (JCQTBL2) as the first color component."]
    _10 = 2,
    #[doc = "3: Use quantization table No.3 (JCQTBL3) as the first color component."]
    _11 = 3,
}
impl From<QT1_A> for u8 {
    #[inline(always)]
    fn from(variant: QT1_A) -> Self {
        variant as _
    }
}
impl QT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QT1_A {
        match self.bits {
            0 => QT1_A::_00,
            1 => QT1_A::_01,
            2 => QT1_A::_10,
            3 => QT1_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == QT1_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == QT1_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == QT1_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == QT1_A::_11
    }
}
#[doc = "Field `QT1` writer - Quantization table number for the first color componentNOTE: Read-only in Decompression."]
pub type QT1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, JCQTN_SPEC, u8, QT1_A, 2, O>;
impl<'a, const O: u8> QT1_W<'a, O> {
    #[doc = "Use quantization table No.0 (JCQTBL0) as the first color component."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(QT1_A::_00)
    }
    #[doc = "Use quantization table No.1 (JCQTBL1) as the first color component."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(QT1_A::_01)
    }
    #[doc = "Use quantization table No.2 (JCQTBL2) as the first color component."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(QT1_A::_10)
    }
    #[doc = "Use quantization table No.3 (JCQTBL3) as the first color component."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(QT1_A::_11)
    }
}
#[doc = "Field `QT2` reader - Quantization table number for the second color component NOTE: Read-only in Decompression."]
pub type QT2_R = crate::FieldReader<u8, QT2_A>;
#[doc = "Quantization table number for the second color component NOTE: Read-only in Decompression.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum QT2_A {
    #[doc = "0: Use quantization table No.0 (JCQTBL0) as the second color component."]
    _00 = 0,
    #[doc = "1: Use quantization table No.1 (JCQTBL1) as the second color component."]
    _01 = 1,
    #[doc = "2: Use quantization table No.2 (JCQTBL2) as the second color component."]
    _10 = 2,
    #[doc = "3: Use quantization table No.3 (JCQTBL3) as the second color component."]
    _11 = 3,
}
impl From<QT2_A> for u8 {
    #[inline(always)]
    fn from(variant: QT2_A) -> Self {
        variant as _
    }
}
impl QT2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QT2_A {
        match self.bits {
            0 => QT2_A::_00,
            1 => QT2_A::_01,
            2 => QT2_A::_10,
            3 => QT2_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == QT2_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == QT2_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == QT2_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == QT2_A::_11
    }
}
#[doc = "Field `QT2` writer - Quantization table number for the second color component NOTE: Read-only in Decompression."]
pub type QT2_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, JCQTN_SPEC, u8, QT2_A, 2, O>;
impl<'a, const O: u8> QT2_W<'a, O> {
    #[doc = "Use quantization table No.0 (JCQTBL0) as the second color component."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(QT2_A::_00)
    }
    #[doc = "Use quantization table No.1 (JCQTBL1) as the second color component."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(QT2_A::_01)
    }
    #[doc = "Use quantization table No.2 (JCQTBL2) as the second color component."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(QT2_A::_10)
    }
    #[doc = "Use quantization table No.3 (JCQTBL3) as the second color component."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(QT2_A::_11)
    }
}
#[doc = "Field `QT3` reader - Quantization table number for the third color component NOTE: Read-only in Decompression."]
pub type QT3_R = crate::FieldReader<u8, QT3_A>;
#[doc = "Quantization table number for the third color component NOTE: Read-only in Decompression.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum QT3_A {
    #[doc = "0: Use quantization table No.0 (JCQTBL0) as the third color component."]
    _00 = 0,
    #[doc = "1: Use quantization table No.1 (JCQTBL1) as the third color component."]
    _01 = 1,
    #[doc = "2: Use quantization table No.2 (JCQTBL2) as the third color component."]
    _10 = 2,
    #[doc = "3: Use quantization table No.3 (JCQTBL3) as the third color component."]
    _11 = 3,
}
impl From<QT3_A> for u8 {
    #[inline(always)]
    fn from(variant: QT3_A) -> Self {
        variant as _
    }
}
impl QT3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QT3_A {
        match self.bits {
            0 => QT3_A::_00,
            1 => QT3_A::_01,
            2 => QT3_A::_10,
            3 => QT3_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == QT3_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == QT3_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == QT3_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == QT3_A::_11
    }
}
#[doc = "Field `QT3` writer - Quantization table number for the third color component NOTE: Read-only in Decompression."]
pub type QT3_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, JCQTN_SPEC, u8, QT3_A, 2, O>;
impl<'a, const O: u8> QT3_W<'a, O> {
    #[doc = "Use quantization table No.0 (JCQTBL0) as the third color component."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(QT3_A::_00)
    }
    #[doc = "Use quantization table No.1 (JCQTBL1) as the third color component."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(QT3_A::_01)
    }
    #[doc = "Use quantization table No.2 (JCQTBL2) as the third color component."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(QT3_A::_10)
    }
    #[doc = "Use quantization table No.3 (JCQTBL3) as the third color component."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(QT3_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - Quantization table number for the first color componentNOTE: Read-only in Decompression."]
    #[inline(always)]
    pub fn qt1(&self) -> QT1_R {
        QT1_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Quantization table number for the second color component NOTE: Read-only in Decompression."]
    #[inline(always)]
    pub fn qt2(&self) -> QT2_R {
        QT2_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - Quantization table number for the third color component NOTE: Read-only in Decompression."]
    #[inline(always)]
    pub fn qt3(&self) -> QT3_R {
        QT3_R::new((self.bits >> 4) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Quantization table number for the first color componentNOTE: Read-only in Decompression."]
    #[inline(always)]
    #[must_use]
    pub fn qt1(&mut self) -> QT1_W<0> {
        QT1_W::new(self)
    }
    #[doc = "Bits 2:3 - Quantization table number for the second color component NOTE: Read-only in Decompression."]
    #[inline(always)]
    #[must_use]
    pub fn qt2(&mut self) -> QT2_W<2> {
        QT2_W::new(self)
    }
    #[doc = "Bits 4:5 - Quantization table number for the third color component NOTE: Read-only in Decompression."]
    #[inline(always)]
    #[must_use]
    pub fn qt3(&mut self) -> QT3_W<4> {
        QT3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG Code Quantization Table Number Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jcqtn](index.html) module"]
pub struct JCQTN_SPEC;
impl crate::RegisterSpec for JCQTN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [jcqtn::R](R) reader structure"]
impl crate::Readable for JCQTN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jcqtn::W](W) writer structure"]
impl crate::Writable for JCQTN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets JCQTN to value 0"]
impl crate::Resettable for JCQTN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
