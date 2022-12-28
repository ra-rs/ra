#[doc = "Register `GR%s_VEN` reader"]
pub struct R(crate::R<GR_VEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GR_VEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GR_VEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GR_VEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GR%s_VEN` writer"]
pub struct W(crate::W<GR_VEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GR_VEN_SPEC>;
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
impl From<crate::W<GR_VEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GR_VEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PVEN` reader - Control of graphics n module register value reflection to internal operations.Reflection of the register values to the internal operation at the assertion of the vertical synchronization signal (VS).\n\nThe field is **modified** in some way after a read operation."]
pub type PVEN_R = crate::BitReader<PVEN_A>;
#[doc = "Control of graphics n module register value reflection to internal operations.Reflection of the register values to the internal operation at the assertion of the vertical synchronization signal (VS).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVEN_A {
    #[doc = "1: Enables reflection of the register values to the internal operation at the assertion of the vertical synchronization signal (VS)."]
    _1 = 1,
    #[doc = "0: Disables reflection of the register values to the internal operation at the assertion of the vertical synchronization signal (VS)."]
    _0 = 0,
}
impl From<PVEN_A> for bool {
    #[inline(always)]
    fn from(variant: PVEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PVEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PVEN_A {
        match self.bits {
            true => PVEN_A::_1,
            false => PVEN_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PVEN_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PVEN_A::_0
    }
}
#[doc = "Field `PVEN` writer - Control of graphics n module register value reflection to internal operations.Reflection of the register values to the internal operation at the assertion of the vertical synchronization signal (VS)."]
pub type PVEN_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, GR_VEN_SPEC, PVEN_A, O>;
impl<'a, const O: u8> PVEN_W<'a, O> {
    #[doc = "Enables reflection of the register values to the internal operation at the assertion of the vertical synchronization signal (VS)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PVEN_A::_1)
    }
    #[doc = "Disables reflection of the register values to the internal operation at the assertion of the vertical synchronization signal (VS)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PVEN_A::_0)
    }
}
impl R {
    #[doc = "Bit 0 - Control of graphics n module register value reflection to internal operations.Reflection of the register values to the internal operation at the assertion of the vertical synchronization signal (VS)."]
    #[inline(always)]
    pub fn pven(&self) -> PVEN_R {
        PVEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Control of graphics n module register value reflection to internal operations.Reflection of the register values to the internal operation at the assertion of the vertical synchronization signal (VS)."]
    #[inline(always)]
    #[must_use]
    pub fn pven(&mut self) -> PVEN_W<0> {
        PVEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Graphics %s Register Update Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gr_ven](index.html) module"]
pub struct GR_VEN_SPEC;
impl crate::RegisterSpec for GR_VEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gr_ven::R](R) reader structure"]
impl crate::Readable for GR_VEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gr_ven::W](W) writer structure"]
impl crate::Writable for GR_VEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x01;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GR%s_VEN to value 0"]
impl crate::Resettable for GR_VEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
