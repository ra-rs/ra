#[doc = "Register `ADCHCR%s` reader"]
pub struct R(crate::R<ADCHCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCHCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCHCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCHCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCHCR%s` writer"]
pub struct W(crate::W<ADCHCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCHCR_SPEC>;
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
impl From<crate::W<ADCHCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCHCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SGSEL` reader - Scan Group Selection"]
pub type SGSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SGSEL` writer - Scan Group Selection"]
pub type SGSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCHCR_SPEC, u8, u8, 5, O>;
#[doc = "Field `CNVCS` reader - A/D Conversion Channel Selection"]
pub type CNVCS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CNVCS` writer - A/D Conversion Channel Selection"]
pub type CNVCS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCHCR_SPEC, u8, u8, 7, O>;
#[doc = "Field `AINMD` reader - Analog Input mode selection"]
pub type AINMD_R = crate::BitReader<AINMD_A>;
#[doc = "Analog Input mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AINMD_A {
    #[doc = "0: Single-ended input"]
    _0 = 0,
    #[doc = "1: Differential input"]
    _1 = 1,
}
impl From<AINMD_A> for bool {
    #[inline(always)]
    fn from(variant: AINMD_A) -> Self {
        variant as u8 != 0
    }
}
impl AINMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AINMD_A {
        match self.bits {
            false => AINMD_A::_0,
            true => AINMD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AINMD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AINMD_A::_1
    }
}
#[doc = "Field `AINMD` writer - Analog Input mode selection"]
pub type AINMD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCHCR_SPEC, AINMD_A, O>;
impl<'a, const O: u8> AINMD_W<'a, O> {
    #[doc = "Single-ended input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AINMD_A::_0)
    }
    #[doc = "Differential input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AINMD_A::_1)
    }
}
#[doc = "Field `SSTSEL` reader - Sampling State Table Selection"]
pub type SSTSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SSTSEL` writer - Sampling State Table Selection"]
pub type SSTSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCHCR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:4 - Scan Group Selection"]
    #[inline(always)]
    pub fn sgsel(&self) -> SGSEL_R {
        SGSEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:14 - A/D Conversion Channel Selection"]
    #[inline(always)]
    pub fn cnvcs(&self) -> CNVCS_R {
        CNVCS_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Analog Input mode selection"]
    #[inline(always)]
    pub fn ainmd(&self) -> AINMD_R {
        AINMD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Sampling State Table Selection"]
    #[inline(always)]
    pub fn sstsel(&self) -> SSTSEL_R {
        SSTSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Scan Group Selection"]
    #[inline(always)]
    #[must_use]
    pub fn sgsel(&mut self) -> SGSEL_W<0> {
        SGSEL_W::new(self)
    }
    #[doc = "Bits 8:14 - A/D Conversion Channel Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cnvcs(&mut self) -> CNVCS_W<8> {
        CNVCS_W::new(self)
    }
    #[doc = "Bit 15 - Analog Input mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn ainmd(&mut self) -> AINMD_W<15> {
        AINMD_W::new(self)
    }
    #[doc = "Bits 16:19 - Sampling State Table Selection"]
    #[inline(always)]
    #[must_use]
    pub fn sstsel(&mut self) -> SSTSEL_W<16> {
        SSTSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Conversion Channel Configuration Register %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adchcr](index.html) module"]
pub struct ADCHCR_SPEC;
impl crate::RegisterSpec for ADCHCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adchcr::R](R) reader structure"]
impl crate::Readable for ADCHCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adchcr::W](W) writer structure"]
impl crate::Writable for ADCHCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCHCR%s to value 0"]
impl crate::Resettable for ADCHCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
