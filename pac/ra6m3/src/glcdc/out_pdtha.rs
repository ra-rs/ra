#[doc = "Register `OUT_PDTHA` reader"]
pub struct R(crate::R<OUT_PDTHA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_PDTHA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_PDTHA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_PDTHA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT_PDTHA` writer"]
pub struct W(crate::W<OUT_PDTHA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_PDTHA_SPEC>;
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
impl From<crate::W<OUT_PDTHA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_PDTHA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PD` reader - Pattern value (D) of 2 x 2 pattern ditherUnsigned 2-bit integer"]
pub type PD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PD` writer - Pattern value (D) of 2 x 2 pattern ditherUnsigned 2-bit integer"]
pub type PD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OUT_PDTHA_SPEC, u8, u8, 2, O>;
#[doc = "Field `PC` reader - Pattern value (C) of 2 x 2 pattern ditherUnsigned 2-bit integer"]
pub type PC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PC` writer - Pattern value (C) of 2 x 2 pattern ditherUnsigned 2-bit integer"]
pub type PC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OUT_PDTHA_SPEC, u8, u8, 2, O>;
#[doc = "Field `PB` reader - Pattern value (B) of 2 x 2 pattern ditherUnsigned 2-bit integer"]
pub type PB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PB` writer - Pattern value (B) of 2 x 2 pattern ditherUnsigned 2-bit integer"]
pub type PB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OUT_PDTHA_SPEC, u8, u8, 2, O>;
#[doc = "Field `PA` reader - Pattern value (A) of 2 x 2 pattern ditherUnsigned 2-bit integer"]
pub type PA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PA` writer - Pattern value (A) of 2 x 2 pattern ditherUnsigned 2-bit integer"]
pub type PA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OUT_PDTHA_SPEC, u8, u8, 2, O>;
#[doc = "Field `FORM` reader - Output format select"]
pub type FORM_R = crate::FieldReader<u8, FORM_A>;
#[doc = "Output format select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FORM_A {
    #[doc = "3: Setting prohibited"]
    _11 = 3,
    #[doc = "2: RGB565; select RGB565 as output interface format."]
    _10 = 2,
    #[doc = "1: RGB666; select RGB666 as output interface format."]
    _01 = 1,
    #[doc = "0: RGB888; select RGB888 or serial RGB as output interface format."]
    _00 = 0,
}
impl From<FORM_A> for u8 {
    #[inline(always)]
    fn from(variant: FORM_A) -> Self {
        variant as _
    }
}
impl FORM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORM_A {
        match self.bits {
            3 => FORM_A::_11,
            2 => FORM_A::_10,
            1 => FORM_A::_01,
            0 => FORM_A::_00,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FORM_A::_11
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FORM_A::_10
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FORM_A::_01
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FORM_A::_00
    }
}
#[doc = "Field `FORM` writer - Output format select"]
pub type FORM_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, OUT_PDTHA_SPEC, u8, FORM_A, 2, O>;
impl<'a, const O: u8> FORM_W<'a, O> {
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FORM_A::_11)
    }
    #[doc = "RGB565; select RGB565 as output interface format."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FORM_A::_10)
    }
    #[doc = "RGB666; select RGB666 as output interface format."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FORM_A::_01)
    }
    #[doc = "RGB888; select RGB888 or serial RGB as output interface format."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FORM_A::_00)
    }
}
#[doc = "Field `SEL` reader - Operation mode"]
pub type SEL_R = crate::FieldReader<u8, SEL_A>;
#[doc = "Operation mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "3: Setting prohibited"]
    _11 = 3,
    #[doc = "2: 2x2 pattern dither"]
    _10 = 2,
    #[doc = "1: Round-off"]
    _01 = 1,
    #[doc = "0: Truncate"]
    _00 = 0,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as _
    }
}
impl SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_A {
        match self.bits {
            3 => SEL_A::_11,
            2 => SEL_A::_10,
            1 => SEL_A::_01,
            0 => SEL_A::_00,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SEL_A::_11
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SEL_A::_00
    }
}
#[doc = "Field `SEL` writer - Operation mode"]
pub type SEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, OUT_PDTHA_SPEC, u8, SEL_A, 2, O>;
impl<'a, const O: u8> SEL_W<'a, O> {
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(SEL_A::_11)
    }
    #[doc = "2x2 pattern dither"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SEL_A::_10)
    }
    #[doc = "Round-off"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(SEL_A::_01)
    }
    #[doc = "Truncate"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(SEL_A::_00)
    }
}
impl R {
    #[doc = "Bits 0:1 - Pattern value (D) of 2 x 2 pattern ditherUnsigned 2-bit integer"]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Pattern value (C) of 2 x 2 pattern ditherUnsigned 2-bit integer"]
    #[inline(always)]
    pub fn pc(&self) -> PC_R {
        PC_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pattern value (B) of 2 x 2 pattern ditherUnsigned 2-bit integer"]
    #[inline(always)]
    pub fn pb(&self) -> PB_R {
        PB_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Pattern value (A) of 2 x 2 pattern ditherUnsigned 2-bit integer"]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Output format select"]
    #[inline(always)]
    pub fn form(&self) -> FORM_R {
        FORM_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Operation mode"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Pattern value (D) of 2 x 2 pattern ditherUnsigned 2-bit integer"]
    #[inline(always)]
    #[must_use]
    pub fn pd(&mut self) -> PD_W<0> {
        PD_W::new(self)
    }
    #[doc = "Bits 4:5 - Pattern value (C) of 2 x 2 pattern ditherUnsigned 2-bit integer"]
    #[inline(always)]
    #[must_use]
    pub fn pc(&mut self) -> PC_W<4> {
        PC_W::new(self)
    }
    #[doc = "Bits 8:9 - Pattern value (B) of 2 x 2 pattern ditherUnsigned 2-bit integer"]
    #[inline(always)]
    #[must_use]
    pub fn pb(&mut self) -> PB_W<8> {
        PB_W::new(self)
    }
    #[doc = "Bits 12:13 - Pattern value (A) of 2 x 2 pattern ditherUnsigned 2-bit integer"]
    #[inline(always)]
    #[must_use]
    pub fn pa(&mut self) -> PA_W<12> {
        PA_W::new(self)
    }
    #[doc = "Bits 16:17 - Output format select"]
    #[inline(always)]
    #[must_use]
    pub fn form(&mut self) -> FORM_W<16> {
        FORM_W::new(self)
    }
    #[doc = "Bits 20:21 - Operation mode"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SEL_W<20> {
        SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output Control Block Panel Dither Correction Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_pdtha](index.html) module"]
pub struct OUT_PDTHA_SPEC;
impl crate::RegisterSpec for OUT_PDTHA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_pdtha::R](R) reader structure"]
impl crate::Readable for OUT_PDTHA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out_pdtha::W](W) writer structure"]
impl crate::Writable for OUT_PDTHA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUT_PDTHA to value 0"]
impl crate::Resettable for OUT_PDTHA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
