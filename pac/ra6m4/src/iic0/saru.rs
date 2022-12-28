#[doc = "Register `SARU%s` reader"]
pub struct R(crate::R<SARU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SARU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SARU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SARU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SARU%s` writer"]
pub struct W(crate::W<SARU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SARU_SPEC>;
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
impl From<crate::W<SARU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SARU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FS` reader - 7-bit/10-bit Address Format Select"]
pub type FS_R = crate::BitReader<FS_A>;
#[doc = "7-bit/10-bit Address Format Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FS_A {
    #[doc = "0: Select 7-bit address format"]
    _0 = 0,
    #[doc = "1: Select 10-bit address format"]
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
#[doc = "Field `FS` writer - 7-bit/10-bit Address Format Select"]
pub type FS_W<'a, const O: u8> = crate::BitWriter<'a, u8, SARU_SPEC, FS_A, O>;
impl<'a, const O: u8> FS_W<'a, O> {
    #[doc = "Select 7-bit address format"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FS_A::_0)
    }
    #[doc = "Select 10-bit address format"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FS_A::_1)
    }
}
#[doc = "Field `SVA` reader - 10-bit Address Upper Bits"]
pub type SVA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SVA` writer - 10-bit Address Upper Bits"]
pub type SVA_W<'a, const O: u8> = crate::FieldWriter<'a, u8, SARU_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - 7-bit/10-bit Address Format Select"]
    #[inline(always)]
    pub fn fs(&self) -> FS_R {
        FS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - 10-bit Address Upper Bits"]
    #[inline(always)]
    pub fn sva(&self) -> SVA_R {
        SVA_R::new((self.bits >> 1) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - 7-bit/10-bit Address Format Select"]
    #[inline(always)]
    #[must_use]
    pub fn fs(&mut self) -> FS_W<0> {
        FS_W::new(self)
    }
    #[doc = "Bits 1:2 - 10-bit Address Upper Bits"]
    #[inline(always)]
    #[must_use]
    pub fn sva(&mut self) -> SVA_W<1> {
        SVA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Address Register Uy\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [saru](index.html) module"]
pub struct SARU_SPEC;
impl crate::RegisterSpec for SARU_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [saru::R](R) reader structure"]
impl crate::Readable for SARU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [saru::W](W) writer structure"]
impl crate::Writable for SARU_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SARU%s to value 0"]
impl crate::Resettable for SARU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
