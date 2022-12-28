#[doc = "Register `DMCRB` reader"]
pub struct R(crate::R<DMCRB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMCRB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMCRB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMCRB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMCRB` writer"]
pub struct W(crate::W<DMCRB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMCRB_SPEC>;
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
impl From<crate::W<DMCRB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMCRB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMCRB` reader - Specifies the number of block transfer operations or repeat transfer operations."]
pub type DMCRB_R = crate::FieldReader<u16, DMCRB_A>;
#[doc = "Specifies the number of block transfer operations or repeat transfer operations.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum DMCRB_A {
    #[doc = "0: 65,536 blocks"]
    _0000 = 0,
}
impl From<DMCRB_A> for u16 {
    #[inline(always)]
    fn from(variant: DMCRB_A) -> Self {
        variant as _
    }
}
impl DMCRB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DMCRB_A> {
        match self.bits {
            0 => Some(DMCRB_A::_0000),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == DMCRB_A::_0000
    }
}
#[doc = "Field `DMCRB` writer - Specifies the number of block transfer operations or repeat transfer operations."]
pub type DMCRB_W<'a, const O: u8> = crate::FieldWriter<'a, u16, DMCRB_SPEC, u16, DMCRB_A, 16, O>;
impl<'a, const O: u8> DMCRB_W<'a, O> {
    #[doc = "65,536 blocks"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(DMCRB_A::_0000)
    }
}
impl R {
    #[doc = "Bits 0:15 - Specifies the number of block transfer operations or repeat transfer operations."]
    #[inline(always)]
    pub fn dmcrb(&self) -> DMCRB_R {
        DMCRB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Specifies the number of block transfer operations or repeat transfer operations."]
    #[inline(always)]
    #[must_use]
    pub fn dmcrb(&mut self) -> DMCRB_W<0> {
        DMCRB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Block Transfer Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmcrb](index.html) module"]
pub struct DMCRB_SPEC;
impl crate::RegisterSpec for DMCRB_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dmcrb::R](R) reader structure"]
impl crate::Readable for DMCRB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmcrb::W](W) writer structure"]
impl crate::Writable for DMCRB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMCRB to value 0"]
impl crate::Resettable for DMCRB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
