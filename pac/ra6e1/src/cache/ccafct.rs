#[doc = "Register `CCAFCT` reader"]
pub struct R(crate::R<CCAFCT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCAFCT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCAFCT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCAFCT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCAFCT` writer"]
pub struct W(crate::W<CCAFCT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCAFCT_SPEC>;
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
impl From<crate::W<CCAFCT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCAFCT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FC` reader - C-Cache Flush"]
pub type FC_R = crate::BitReader<FC_A>;
#[doc = "C-Cache Flush\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FC_A {
    #[doc = "0: No action"]
    _0 = 0,
    #[doc = "1: C-cache line flush (all lines invalidated)"]
    _1 = 1,
}
impl From<FC_A> for bool {
    #[inline(always)]
    fn from(variant: FC_A) -> Self {
        variant as u8 != 0
    }
}
impl FC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC_A {
        match self.bits {
            false => FC_A::_0,
            true => FC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FC_A::_1
    }
}
#[doc = "Field `FC` writer - C-Cache Flush"]
pub type FC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCAFCT_SPEC, FC_A, O>;
impl<'a, const O: u8> FC_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FC_A::_0)
    }
    #[doc = "C-cache line flush (all lines invalidated)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FC_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - C-Cache Flush"]
    #[inline(always)]
    pub fn fc(&self) -> FC_R {
        FC_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - C-Cache Flush"]
    #[inline(always)]
    #[must_use]
    pub fn fc(&mut self) -> FC_W<0> {
        FC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "C-Cache Flush Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccafct](index.html) module"]
pub struct CCAFCT_SPEC;
impl crate::RegisterSpec for CCAFCT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccafct::R](R) reader structure"]
impl crate::Readable for CCAFCT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccafct::W](W) writer structure"]
impl crate::Writable for CCAFCT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCAFCT to value 0"]
impl crate::Resettable for CCAFCT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
