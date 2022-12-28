#[doc = "Register `CFDGAFLM%s` reader"]
pub struct R(crate::R<CFDGAFLM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDGAFLM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDGAFLM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDGAFLM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDGAFLM%s` writer"]
pub struct W(crate::W<CFDGAFLM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDGAFLM_SPEC>;
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
impl From<crate::W<CFDGAFLM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDGAFLM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GAFLIDM` reader - Global Acceptance Filter List ID Mask Field"]
pub type GAFLIDM_R = crate::FieldReader<u32, u32>;
#[doc = "Field `GAFLIDM` writer - Global Acceptance Filter List ID Mask Field"]
pub type GAFLIDM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDGAFLM_SPEC, u32, u32, 29, O>;
#[doc = "Field `GAFLIFL1` reader - Global Acceptance Filter List Information Label 1"]
pub type GAFLIFL1_R = crate::BitReader<bool>;
#[doc = "Field `GAFLIFL1` writer - Global Acceptance Filter List Information Label 1"]
pub type GAFLIFL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDGAFLM_SPEC, bool, O>;
#[doc = "Field `GAFLRTRM` reader - Global Acceptance Filter List Entry RTR Mask"]
pub type GAFLRTRM_R = crate::BitReader<GAFLRTRM_A>;
#[doc = "Global Acceptance Filter List Entry RTR Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GAFLRTRM_A {
    #[doc = "0: RTR bit is not used for ID matching"]
    _0 = 0,
    #[doc = "1: RTR bit is used for ID matching"]
    _1 = 1,
}
impl From<GAFLRTRM_A> for bool {
    #[inline(always)]
    fn from(variant: GAFLRTRM_A) -> Self {
        variant as u8 != 0
    }
}
impl GAFLRTRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GAFLRTRM_A {
        match self.bits {
            false => GAFLRTRM_A::_0,
            true => GAFLRTRM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GAFLRTRM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GAFLRTRM_A::_1
    }
}
#[doc = "Field `GAFLRTRM` writer - Global Acceptance Filter List Entry RTR Mask"]
pub type GAFLRTRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDGAFLM_SPEC, GAFLRTRM_A, O>;
impl<'a, const O: u8> GAFLRTRM_W<'a, O> {
    #[doc = "RTR bit is not used for ID matching"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GAFLRTRM_A::_0)
    }
    #[doc = "RTR bit is used for ID matching"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GAFLRTRM_A::_1)
    }
}
#[doc = "Field `GAFLIDEM` reader - Global Acceptance Filter List IDE Mask"]
pub type GAFLIDEM_R = crate::BitReader<GAFLIDEM_A>;
#[doc = "Global Acceptance Filter List IDE Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GAFLIDEM_A {
    #[doc = "0: IDE bit is not used for ID matching"]
    _0 = 0,
    #[doc = "1: IDE bit is used for ID matching"]
    _1 = 1,
}
impl From<GAFLIDEM_A> for bool {
    #[inline(always)]
    fn from(variant: GAFLIDEM_A) -> Self {
        variant as u8 != 0
    }
}
impl GAFLIDEM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GAFLIDEM_A {
        match self.bits {
            false => GAFLIDEM_A::_0,
            true => GAFLIDEM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GAFLIDEM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GAFLIDEM_A::_1
    }
}
#[doc = "Field `GAFLIDEM` writer - Global Acceptance Filter List IDE Mask"]
pub type GAFLIDEM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDGAFLM_SPEC, GAFLIDEM_A, O>;
impl<'a, const O: u8> GAFLIDEM_W<'a, O> {
    #[doc = "IDE bit is not used for ID matching"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GAFLIDEM_A::_0)
    }
    #[doc = "IDE bit is used for ID matching"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GAFLIDEM_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:28 - Global Acceptance Filter List ID Mask Field"]
    #[inline(always)]
    pub fn gaflidm(&self) -> GAFLIDM_R {
        GAFLIDM_R::new(self.bits & 0x1fff_ffff)
    }
    #[doc = "Bit 29 - Global Acceptance Filter List Information Label 1"]
    #[inline(always)]
    pub fn gaflifl1(&self) -> GAFLIFL1_R {
        GAFLIFL1_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Global Acceptance Filter List Entry RTR Mask"]
    #[inline(always)]
    pub fn gaflrtrm(&self) -> GAFLRTRM_R {
        GAFLRTRM_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Global Acceptance Filter List IDE Mask"]
    #[inline(always)]
    pub fn gaflidem(&self) -> GAFLIDEM_R {
        GAFLIDEM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:28 - Global Acceptance Filter List ID Mask Field"]
    #[inline(always)]
    #[must_use]
    pub fn gaflidm(&mut self) -> GAFLIDM_W<0> {
        GAFLIDM_W::new(self)
    }
    #[doc = "Bit 29 - Global Acceptance Filter List Information Label 1"]
    #[inline(always)]
    #[must_use]
    pub fn gaflifl1(&mut self) -> GAFLIFL1_W<29> {
        GAFLIFL1_W::new(self)
    }
    #[doc = "Bit 30 - Global Acceptance Filter List Entry RTR Mask"]
    #[inline(always)]
    #[must_use]
    pub fn gaflrtrm(&mut self) -> GAFLRTRM_W<30> {
        GAFLRTRM_W::new(self)
    }
    #[doc = "Bit 31 - Global Acceptance Filter List IDE Mask"]
    #[inline(always)]
    #[must_use]
    pub fn gaflidem(&mut self) -> GAFLIDEM_W<31> {
        GAFLIDEM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Acceptance Filter List Mask Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdgaflm](index.html) module"]
pub struct CFDGAFLM_SPEC;
impl crate::RegisterSpec for CFDGAFLM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdgaflm::R](R) reader structure"]
impl crate::Readable for CFDGAFLM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdgaflm::W](W) writer structure"]
impl crate::Writable for CFDGAFLM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDGAFLM%s to value 0"]
impl crate::Resettable for CFDGAFLM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
