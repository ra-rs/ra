#[doc = "Register `FBPROT0` reader"]
pub struct R(crate::R<FBPROT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FBPROT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FBPROT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FBPROT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FBPROT0` writer"]
pub struct W(crate::W<FBPROT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FBPROT0_SPEC>;
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
impl From<crate::W<FBPROT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FBPROT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BPCN0` reader - Block Protection for Non-secure Cancel"]
pub type BPCN0_R = crate::BitReader<BPCN0_A>;
#[doc = "Block Protection for Non-secure Cancel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BPCN0_A {
    #[doc = "0: Block protection is enabled"]
    _0 = 0,
    #[doc = "1: Block protection is disabled."]
    _1 = 1,
}
impl From<BPCN0_A> for bool {
    #[inline(always)]
    fn from(variant: BPCN0_A) -> Self {
        variant as u8 != 0
    }
}
impl BPCN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPCN0_A {
        match self.bits {
            false => BPCN0_A::_0,
            true => BPCN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPCN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPCN0_A::_1
    }
}
#[doc = "Field `BPCN0` writer - Block Protection for Non-secure Cancel"]
pub type BPCN0_W<'a, const O: u8> = crate::BitWriter<'a, u16, FBPROT0_SPEC, BPCN0_A, O>;
impl<'a, const O: u8> BPCN0_W<'a, O> {
    #[doc = "Block protection is enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPCN0_A::_0)
    }
    #[doc = "Block protection is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPCN0_A::_1)
    }
}
#[doc = "Field `KEY` writer - Key Code"]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u16, FBPROT0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Block Protection for Non-secure Cancel"]
    #[inline(always)]
    pub fn bpcn0(&self) -> BPCN0_R {
        BPCN0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Block Protection for Non-secure Cancel"]
    #[inline(always)]
    #[must_use]
    pub fn bpcn0(&mut self) -> BPCN0_W<0> {
        BPCN0_W::new(self)
    }
    #[doc = "Bits 8:15 - Key Code"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<8> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Block Protection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fbprot0](index.html) module"]
pub struct FBPROT0_SPEC;
impl crate::RegisterSpec for FBPROT0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [fbprot0::R](R) reader structure"]
impl crate::Readable for FBPROT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fbprot0::W](W) writer structure"]
impl crate::Writable for FBPROT0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FBPROT0 to value 0"]
impl crate::Resettable for FBPROT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
