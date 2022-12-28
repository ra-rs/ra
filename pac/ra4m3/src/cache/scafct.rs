#[doc = "Register `SCAFCT` reader"]
pub struct R(crate::R<SCAFCT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCAFCT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCAFCT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCAFCT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCAFCT` writer"]
pub struct W(crate::W<SCAFCT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCAFCT_SPEC>;
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
impl From<crate::W<SCAFCT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCAFCT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FS` reader - S-Cache Flush"]
pub type FS_R = crate::BitReader<FS_A>;
#[doc = "S-Cache Flush\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FS_A {
    #[doc = "0: No action"]
    _0 = 0,
    #[doc = "1: S-cache line flush (all lines invalidated)"]
    _1 = 1,
}
impl From<FS_A> for bool {
    #[inline(always)]
    fn from(variant: FS_A) -> Self {
        variant as u8 != 0
    }
}
impl FS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FS_A {
        match self.bits {
            false => FS_A::_0,
            true => FS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FS_A::_1
    }
}
#[doc = "Field `FS` writer - S-Cache Flush"]
pub type FS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCAFCT_SPEC, FS_A, O>;
impl<'a, const O: u8> FS_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FS_A::_0)
    }
    #[doc = "S-cache line flush (all lines invalidated)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FS_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - S-Cache Flush"]
    #[inline(always)]
    pub fn fs(&self) -> FS_R {
        FS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - S-Cache Flush"]
    #[inline(always)]
    #[must_use]
    pub fn fs(&mut self) -> FS_W<0> {
        FS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "S-Cache Flush Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scafct](index.html) module"]
pub struct SCAFCT_SPEC;
impl crate::RegisterSpec for SCAFCT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scafct::R](R) reader structure"]
impl crate::Readable for SCAFCT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scafct::W](W) writer structure"]
impl crate::Writable for SCAFCT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCAFCT to value 0"]
impl crate::Resettable for SCAFCT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
