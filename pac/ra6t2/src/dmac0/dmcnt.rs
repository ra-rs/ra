#[doc = "Register `DMCNT` reader"]
pub struct R(crate::R<DMCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMCNT` writer"]
pub struct W(crate::W<DMCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMCNT_SPEC>;
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
impl From<crate::W<DMCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTE` reader - DMA Transfer Enable"]
pub type DTE_R = crate::BitReader<DTE_A>;
#[doc = "DMA Transfer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTE_A {
    #[doc = "0: Disables DMA transfer."]
    _0 = 0,
    #[doc = "1: Enables DMA transfer."]
    _1 = 1,
}
impl From<DTE_A> for bool {
    #[inline(always)]
    fn from(variant: DTE_A) -> Self {
        variant as u8 != 0
    }
}
impl DTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTE_A {
        match self.bits {
            false => DTE_A::_0,
            true => DTE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTE_A::_1
    }
}
#[doc = "Field `DTE` writer - DMA Transfer Enable"]
pub type DTE_W<'a, const O: u8> = crate::BitWriter<'a, u8, DMCNT_SPEC, DTE_A, O>;
impl<'a, const O: u8> DTE_W<'a, O> {
    #[doc = "Disables DMA transfer."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTE_A::_0)
    }
    #[doc = "Enables DMA transfer."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - DMA Transfer Enable"]
    #[inline(always)]
    pub fn dte(&self) -> DTE_R {
        DTE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Transfer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dte(&mut self) -> DTE_W<0> {
        DTE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Transfer Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmcnt](index.html) module"]
pub struct DMCNT_SPEC;
impl crate::RegisterSpec for DMCNT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dmcnt::R](R) reader structure"]
impl crate::Readable for DMCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmcnt::W](W) writer structure"]
impl crate::Writable for DMCNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMCNT to value 0"]
impl crate::Resettable for DMCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
