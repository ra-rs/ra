#[doc = "Register `GAM%s_LATCH` reader"]
pub struct R(crate::R<GAM_LATCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GAM_LATCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GAM_LATCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GAM_LATCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GAM%s_LATCH` writer"]
pub struct W(crate::W<GAM_LATCH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GAM_LATCH_SPEC>;
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
impl From<crate::W<GAM_LATCH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GAM_LATCH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VEN` reader - Control of gamma correction x module register value reflection to internal operations.The register values to be reflected to the internal operations at the assertion of the vertical synchronization signal (VS).\n\nThe field is **modified** in some way after a read operation."]
pub type VEN_R = crate::BitReader<VEN_A>;
#[doc = "Control of gamma correction x module register value reflection to internal operations.The register values to be reflected to the internal operations at the assertion of the vertical synchronization signal (VS).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VEN_A {
    #[doc = "1: Enables the register values to be reflected to the internal operations at the assertion of the vertical synchronization signal (VS)."]
    _1 = 1,
    #[doc = "0: Disables the register values to be reflected to the internal operations at the assertion of the vertical synchronization signal (VS)."]
    _0 = 0,
}
impl From<VEN_A> for bool {
    #[inline(always)]
    fn from(variant: VEN_A) -> Self {
        variant as u8 != 0
    }
}
impl VEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VEN_A {
        match self.bits {
            true => VEN_A::_1,
            false => VEN_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VEN_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VEN_A::_0
    }
}
#[doc = "Field `VEN` writer - Control of gamma correction x module register value reflection to internal operations.The register values to be reflected to the internal operations at the assertion of the vertical synchronization signal (VS)."]
pub type VEN_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, GAM_LATCH_SPEC, VEN_A, O>;
impl<'a, const O: u8> VEN_W<'a, O> {
    #[doc = "Enables the register values to be reflected to the internal operations at the assertion of the vertical synchronization signal (VS)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VEN_A::_1)
    }
    #[doc = "Disables the register values to be reflected to the internal operations at the assertion of the vertical synchronization signal (VS)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VEN_A::_0)
    }
}
impl R {
    #[doc = "Bit 0 - Control of gamma correction x module register value reflection to internal operations.The register values to be reflected to the internal operations at the assertion of the vertical synchronization signal (VS)."]
    #[inline(always)]
    pub fn ven(&self) -> VEN_R {
        VEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Control of gamma correction x module register value reflection to internal operations.The register values to be reflected to the internal operations at the assertion of the vertical synchronization signal (VS)."]
    #[inline(always)]
    #[must_use]
    pub fn ven(&mut self) -> VEN_W<0> {
        VEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Gamma %s Register Update Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gam_latch](index.html) module"]
pub struct GAM_LATCH_SPEC;
impl crate::RegisterSpec for GAM_LATCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gam_latch::R](R) reader structure"]
impl crate::Readable for GAM_LATCH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gam_latch::W](W) writer structure"]
impl crate::Writable for GAM_LATCH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x01;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GAM%s_LATCH to value 0"]
impl crate::Resettable for GAM_LATCH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
