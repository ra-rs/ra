#[doc = "Register `FBCCNT` reader"]
pub struct R(crate::R<FBCCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FBCCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FBCCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FBCCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FBCCNT` writer"]
pub struct W(crate::W<FBCCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FBCCNT_SPEC>;
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
impl From<crate::W<FBCCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FBCCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BCDIR` reader - Blank Check Direction"]
pub type BCDIR_R = crate::BitReader<BCDIR_A>;
#[doc = "Blank Check Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BCDIR_A {
    #[doc = "0: Blank checking is executed from the lower addresses to the higher addresses (incremental mode)"]
    _0 = 0,
    #[doc = "1: Blank checking is executed from the higher addresses to the lower addresses (decremental mode)."]
    _1 = 1,
}
impl From<BCDIR_A> for bool {
    #[inline(always)]
    fn from(variant: BCDIR_A) -> Self {
        variant as u8 != 0
    }
}
impl BCDIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCDIR_A {
        match self.bits {
            false => BCDIR_A::_0,
            true => BCDIR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BCDIR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BCDIR_A::_1
    }
}
#[doc = "Field `BCDIR` writer - Blank Check Direction"]
pub type BCDIR_W<'a, const O: u8> = crate::BitWriter<'a, u8, FBCCNT_SPEC, BCDIR_A, O>;
impl<'a, const O: u8> BCDIR_W<'a, O> {
    #[doc = "Blank checking is executed from the lower addresses to the higher addresses (incremental mode)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BCDIR_A::_0)
    }
    #[doc = "Blank checking is executed from the higher addresses to the lower addresses (decremental mode)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BCDIR_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Blank Check Direction"]
    #[inline(always)]
    pub fn bcdir(&self) -> BCDIR_R {
        BCDIR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Blank Check Direction"]
    #[inline(always)]
    #[must_use]
    pub fn bcdir(&mut self) -> BCDIR_W<0> {
        BCDIR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Blank Check Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fbccnt](index.html) module"]
pub struct FBCCNT_SPEC;
impl crate::RegisterSpec for FBCCNT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fbccnt::R](R) reader structure"]
impl crate::Readable for FBCCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fbccnt::W](W) writer structure"]
impl crate::Writable for FBCCNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FBCCNT to value 0"]
impl crate::Resettable for FBCCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
