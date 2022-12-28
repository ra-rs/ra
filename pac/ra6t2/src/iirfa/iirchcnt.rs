#[doc = "Register `IIRCH%sCNT` reader"]
pub struct R(crate::R<IIRCHCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IIRCHCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IIRCHCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IIRCHCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IIRCH%sCNT` writer"]
pub struct W(crate::W<IIRCHCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IIRCHCNT_SPEC>;
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
impl From<crate::W<IIRCHCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IIRCHCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STGSEL` reader - Stage selection bit"]
pub type STGSEL_R = crate::FieldReader<u32, STGSEL_A>;
#[doc = "Stage selection bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum STGSEL_A {
    #[doc = "0: The corresponding stage is not used for channel n."]
    _0 = 0,
    #[doc = "1: The corresponding stage is used for channel n."]
    _1 = 1,
}
impl From<STGSEL_A> for u32 {
    #[inline(always)]
    fn from(variant: STGSEL_A) -> Self {
        variant as _
    }
}
impl STGSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STGSEL_A> {
        match self.bits {
            0 => Some(STGSEL_A::_0),
            1 => Some(STGSEL_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STGSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STGSEL_A::_1
    }
}
#[doc = "Field `STGSEL` writer - Stage selection bit"]
pub type STGSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IIRCHCNT_SPEC, u32, STGSEL_A, 32, O>;
impl<'a, const O: u8> STGSEL_W<'a, O> {
    #[doc = "The corresponding stage is not used for channel n."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STGSEL_A::_0)
    }
    #[doc = "The corresponding stage is used for channel n."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STGSEL_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:31 - Stage selection bit"]
    #[inline(always)]
    pub fn stgsel(&self) -> STGSEL_R {
        STGSEL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Stage selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn stgsel(&mut self) -> STGSEL_W<0> {
        STGSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel %s Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iirchcnt](index.html) module"]
pub struct IIRCHCNT_SPEC;
impl crate::RegisterSpec for IIRCHCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iirchcnt::R](R) reader structure"]
impl crate::Readable for IIRCHCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iirchcnt::W](W) writer structure"]
impl crate::Writable for IIRCHCNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IIRCH%sCNT to value 0"]
impl crate::Resettable for IIRCHCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
