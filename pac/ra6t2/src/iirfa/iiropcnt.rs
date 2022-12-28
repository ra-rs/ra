#[doc = "Register `IIROPCNT` reader"]
pub struct R(crate::R<IIROPCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IIROPCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IIROPCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IIROPCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IIROPCNT` writer"]
pub struct W(crate::W<IIROPCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IIROPCNT_SPEC>;
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
impl From<crate::W<IIROPCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IIROPCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RND` reader - Setting for the rounding mode for addition and multiplication"]
pub type RND_R = crate::FieldReader<u8, RND_A>;
#[doc = "Setting for the rounding mode for addition and multiplication\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RND_A {
    #[doc = "0: Round to nearest"]
    _000 = 0,
    #[doc = "1: Round toward zero"]
    _001 = 1,
}
impl From<RND_A> for u8 {
    #[inline(always)]
    fn from(variant: RND_A) -> Self {
        variant as _
    }
}
impl RND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RND_A> {
        match self.bits {
            0 => Some(RND_A::_000),
            1 => Some(RND_A::_001),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == RND_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == RND_A::_001
    }
}
#[doc = "Field `RND` writer - Setting for the rounding mode for addition and multiplication"]
pub type RND_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IIROPCNT_SPEC, u8, RND_A, 3, O>;
impl<'a, const O: u8> RND_W<'a, O> {
    #[doc = "Round to nearest"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(RND_A::_000)
    }
    #[doc = "Round toward zero"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(RND_A::_001)
    }
}
impl R {
    #[doc = "Bits 0:2 - Setting for the rounding mode for addition and multiplication"]
    #[inline(always)]
    pub fn rnd(&self) -> RND_R {
        RND_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Setting for the rounding mode for addition and multiplication"]
    #[inline(always)]
    #[must_use]
    pub fn rnd(&mut self) -> RND_W<0> {
        RND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Operation Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iiropcnt](index.html) module"]
pub struct IIROPCNT_SPEC;
impl crate::RegisterSpec for IIROPCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iiropcnt::R](R) reader structure"]
impl crate::Readable for IIROPCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iiropcnt::W](W) writer structure"]
impl crate::Writable for IIROPCNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IIROPCNT to value 0"]
impl crate::Resettable for IIROPCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
