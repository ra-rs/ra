#[doc = "Register `ADREFCR` reader"]
pub struct R(crate::R<ADREFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADREFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADREFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADREFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADREFCR` writer"]
pub struct W(crate::W<ADREFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADREFCR_SPEC>;
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
impl From<crate::W<ADREFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADREFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VDE` reader - Internal Reference Voltage A/D Conversion Select"]
pub type VDE_R = crate::BitReader<VDE_A>;
#[doc = "Internal Reference Voltage A/D Conversion Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDE_A {
    #[doc = "0: Disable A/D conversion of internal reference voltage"]
    _0 = 0,
    #[doc = "1: Enable A/D conversion of internal reference voltage"]
    _1 = 1,
}
impl From<VDE_A> for bool {
    #[inline(always)]
    fn from(variant: VDE_A) -> Self {
        variant as u8 != 0
    }
}
impl VDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDE_A {
        match self.bits {
            false => VDE_A::_0,
            true => VDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VDE_A::_1
    }
}
#[doc = "Field `VDE` writer - Internal Reference Voltage A/D Conversion Select"]
pub type VDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADREFCR_SPEC, VDE_A, O>;
impl<'a, const O: u8> VDE_W<'a, O> {
    #[doc = "Disable A/D conversion of internal reference voltage"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VDE_A::_0)
    }
    #[doc = "Enable A/D conversion of internal reference voltage"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VDE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Internal Reference Voltage A/D Conversion Select"]
    #[inline(always)]
    pub fn vde(&self) -> VDE_R {
        VDE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal Reference Voltage A/D Conversion Select"]
    #[inline(always)]
    #[must_use]
    pub fn vde(&mut self) -> VDE_W<0> {
        VDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal Reference Voltage Monitor Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adrefcr](index.html) module"]
pub struct ADREFCR_SPEC;
impl crate::RegisterSpec for ADREFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adrefcr::R](R) reader structure"]
impl crate::Readable for ADREFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adrefcr::W](W) writer structure"]
impl crate::Writable for ADREFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADREFCR to value 0"]
impl crate::Resettable for ADREFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
