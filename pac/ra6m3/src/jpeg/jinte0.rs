#[doc = "Register `JINTE0` reader"]
pub struct R(crate::R<JINTE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JINTE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JINTE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JINTE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JINTE0` writer"]
pub struct W(crate::W<JINTE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JINTE0_SPEC>;
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
impl From<crate::W<JINTE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JINTE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT3` reader - This bit enables an interrupt to be generated when it has been determined that the image size and the subsampling setting of the compressed data can be read through analyzing the data."]
pub type INT3_R = crate::BitReader<INT3_A>;
#[doc = "This bit enables an interrupt to be generated when it has been determined that the image size and the subsampling setting of the compressed data can be read through analyzing the data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT3_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<INT3_A> for bool {
    #[inline(always)]
    fn from(variant: INT3_A) -> Self {
        variant as u8 != 0
    }
}
impl INT3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT3_A {
        match self.bits {
            false => INT3_A::_0,
            true => INT3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INT3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INT3_A::_1
    }
}
#[doc = "Field `INT3` writer - This bit enables an interrupt to be generated when it has been determined that the image size and the subsampling setting of the compressed data can be read through analyzing the data."]
pub type INT3_W<'a, const O: u8> = crate::BitWriter<'a, u8, JINTE0_SPEC, INT3_A, O>;
impl<'a, const O: u8> INT3_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT3_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT3_A::_1)
    }
}
#[doc = "Field `INT5` reader - This bit enables an interrupt to be generated when the final number of MCU data in the Huffman-coding segment is not correct in decompression. When this bit is not set to enable interrupt generation, an error code is not returned."]
pub type INT5_R = crate::BitReader<INT5_A>;
#[doc = "This bit enables an interrupt to be generated when the final number of MCU data in the Huffman-coding segment is not correct in decompression. When this bit is not set to enable interrupt generation, an error code is not returned.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT5_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<INT5_A> for bool {
    #[inline(always)]
    fn from(variant: INT5_A) -> Self {
        variant as u8 != 0
    }
}
impl INT5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT5_A {
        match self.bits {
            false => INT5_A::_0,
            true => INT5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INT5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INT5_A::_1
    }
}
#[doc = "Field `INT5` writer - This bit enables an interrupt to be generated when the final number of MCU data in the Huffman-coding segment is not correct in decompression. When this bit is not set to enable interrupt generation, an error code is not returned."]
pub type INT5_W<'a, const O: u8> = crate::BitWriter<'a, u8, JINTE0_SPEC, INT5_A, O>;
impl<'a, const O: u8> INT5_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT5_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT5_A::_1)
    }
}
#[doc = "Field `INT6` reader - This bit enables an interrupt to be generated when the total number of data in the Huffman-coding segment is not correct in decompression. When this bit is not set to enable interrupt generation, an error code is not returned."]
pub type INT6_R = crate::BitReader<INT6_A>;
#[doc = "This bit enables an interrupt to be generated when the total number of data in the Huffman-coding segment is not correct in decompression. When this bit is not set to enable interrupt generation, an error code is not returned.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT6_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<INT6_A> for bool {
    #[inline(always)]
    fn from(variant: INT6_A) -> Self {
        variant as u8 != 0
    }
}
impl INT6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT6_A {
        match self.bits {
            false => INT6_A::_0,
            true => INT6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INT6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INT6_A::_1
    }
}
#[doc = "Field `INT6` writer - This bit enables an interrupt to be generated when the total number of data in the Huffman-coding segment is not correct in decompression. When this bit is not set to enable interrupt generation, an error code is not returned."]
pub type INT6_W<'a, const O: u8> = crate::BitWriter<'a, u8, JINTE0_SPEC, INT6_A, O>;
impl<'a, const O: u8> INT6_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT6_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT6_A::_1)
    }
}
#[doc = "Field `INT7` reader - This bit enables an interrupt to be generated when the number of data in the restart interval of the Huffman-coding segment is not correct in decompression.When this bit is not set to enable interrupt generation, an error code is not returned."]
pub type INT7_R = crate::BitReader<INT7_A>;
#[doc = "This bit enables an interrupt to be generated when the number of data in the restart interval of the Huffman-coding segment is not correct in decompression.When this bit is not set to enable interrupt generation, an error code is not returned.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT7_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<INT7_A> for bool {
    #[inline(always)]
    fn from(variant: INT7_A) -> Self {
        variant as u8 != 0
    }
}
impl INT7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT7_A {
        match self.bits {
            false => INT7_A::_0,
            true => INT7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INT7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INT7_A::_1
    }
}
#[doc = "Field `INT7` writer - This bit enables an interrupt to be generated when the number of data in the restart interval of the Huffman-coding segment is not correct in decompression.When this bit is not set to enable interrupt generation, an error code is not returned."]
pub type INT7_W<'a, const O: u8> = crate::BitWriter<'a, u8, JINTE0_SPEC, INT7_A, O>;
impl<'a, const O: u8> INT7_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT7_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT7_A::_1)
    }
}
impl R {
    #[doc = "Bit 3 - This bit enables an interrupt to be generated when it has been determined that the image size and the subsampling setting of the compressed data can be read through analyzing the data."]
    #[inline(always)]
    pub fn int3(&self) -> INT3_R {
        INT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit enables an interrupt to be generated when the final number of MCU data in the Huffman-coding segment is not correct in decompression. When this bit is not set to enable interrupt generation, an error code is not returned."]
    #[inline(always)]
    pub fn int5(&self) -> INT5_R {
        INT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit enables an interrupt to be generated when the total number of data in the Huffman-coding segment is not correct in decompression. When this bit is not set to enable interrupt generation, an error code is not returned."]
    #[inline(always)]
    pub fn int6(&self) -> INT6_R {
        INT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit enables an interrupt to be generated when the number of data in the restart interval of the Huffman-coding segment is not correct in decompression.When this bit is not set to enable interrupt generation, an error code is not returned."]
    #[inline(always)]
    pub fn int7(&self) -> INT7_R {
        INT7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - This bit enables an interrupt to be generated when it has been determined that the image size and the subsampling setting of the compressed data can be read through analyzing the data."]
    #[inline(always)]
    #[must_use]
    pub fn int3(&mut self) -> INT3_W<3> {
        INT3_W::new(self)
    }
    #[doc = "Bit 5 - This bit enables an interrupt to be generated when the final number of MCU data in the Huffman-coding segment is not correct in decompression. When this bit is not set to enable interrupt generation, an error code is not returned."]
    #[inline(always)]
    #[must_use]
    pub fn int5(&mut self) -> INT5_W<5> {
        INT5_W::new(self)
    }
    #[doc = "Bit 6 - This bit enables an interrupt to be generated when the total number of data in the Huffman-coding segment is not correct in decompression. When this bit is not set to enable interrupt generation, an error code is not returned."]
    #[inline(always)]
    #[must_use]
    pub fn int6(&mut self) -> INT6_W<6> {
        INT6_W::new(self)
    }
    #[doc = "Bit 7 - This bit enables an interrupt to be generated when the number of data in the restart interval of the Huffman-coding segment is not correct in decompression.When this bit is not set to enable interrupt generation, an error code is not returned."]
    #[inline(always)]
    #[must_use]
    pub fn int7(&mut self) -> INT7_W<7> {
        INT7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG Interrupt Enable Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jinte0](index.html) module"]
pub struct JINTE0_SPEC;
impl crate::RegisterSpec for JINTE0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [jinte0::R](R) reader structure"]
impl crate::Readable for JINTE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jinte0::W](W) writer structure"]
impl crate::Writable for JINTE0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets JINTE0 to value 0"]
impl crate::Resettable for JINTE0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
