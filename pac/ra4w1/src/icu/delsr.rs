#[doc = "Register `DELSR%s` reader"]
pub struct R(crate::R<DELSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DELSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DELSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DELSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DELSR%s` writer"]
pub struct W(crate::W<DELSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DELSR_SPEC>;
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
impl From<crate::W<DELSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DELSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DELS` reader - Event selection to DMAC Start request"]
pub type DELS_R = crate::FieldReader<u8, DELS_A>;
#[doc = "Event selection to DMAC Start request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DELS_A {
    #[doc = "0: Nothing is selected."]
    _0X000 = 0,
}
impl From<DELS_A> for u8 {
    #[inline(always)]
    fn from(variant: DELS_A) -> Self {
        variant as _
    }
}
impl DELS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DELS_A> {
        match self.bits {
            0 => Some(DELS_A::_0X000),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X000`"]
    #[inline(always)]
    pub fn is_0x000(&self) -> bool {
        *self == DELS_A::_0X000
    }
}
#[doc = "Field `DELS` writer - Event selection to DMAC Start request"]
pub type DELS_W<'a, const O: u8> = crate::FieldWriter<'a, u16, DELSR_SPEC, u8, DELS_A, 8, O>;
impl<'a, const O: u8> DELS_W<'a, O> {
    #[doc = "Nothing is selected."]
    #[inline(always)]
    pub fn _0x000(self) -> &'a mut W {
        self.variant(DELS_A::_0X000)
    }
}
impl R {
    #[doc = "Bits 0:7 - Event selection to DMAC Start request"]
    #[inline(always)]
    pub fn dels(&self) -> DELS_R {
        DELS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Event selection to DMAC Start request"]
    #[inline(always)]
    #[must_use]
    pub fn dels(&mut self) -> DELS_W<0> {
        DELS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Event Link Setting Register %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [delsr](index.html) module"]
pub struct DELSR_SPEC;
impl crate::RegisterSpec for DELSR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [delsr::R](R) reader structure"]
impl crate::Readable for DELSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [delsr::W](W) writer structure"]
impl crate::Writable for DELSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DELSR%s to value 0"]
impl crate::Resettable for DELSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
