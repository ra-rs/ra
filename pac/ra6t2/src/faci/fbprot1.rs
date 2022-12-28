#[doc = "Register `FBPROT1` reader"]
pub struct R(crate::R<FBPROT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FBPROT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FBPROT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FBPROT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FBPROT1` writer"]
pub struct W(crate::W<FBPROT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FBPROT1_SPEC>;
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
impl From<crate::W<FBPROT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FBPROT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BPCN1` reader - Block Protection for Secure Cancel"]
pub type BPCN1_R = crate::BitReader<BPCN1_A>;
#[doc = "Block Protection for Secure Cancel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BPCN1_A {
    #[doc = "0: Block protection is enabled"]
    _0 = 0,
    #[doc = "1: Block protection is disabled."]
    _1 = 1,
}
impl From<BPCN1_A> for bool {
    #[inline(always)]
    fn from(variant: BPCN1_A) -> Self {
        variant as u8 != 0
    }
}
impl BPCN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPCN1_A {
        match self.bits {
            false => BPCN1_A::_0,
            true => BPCN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPCN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPCN1_A::_1
    }
}
#[doc = "Field `BPCN1` writer - Block Protection for Secure Cancel"]
pub type BPCN1_W<'a, const O: u8> = crate::BitWriter<'a, u16, FBPROT1_SPEC, BPCN1_A, O>;
impl<'a, const O: u8> BPCN1_W<'a, O> {
    #[doc = "Block protection is enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPCN1_A::_0)
    }
    #[doc = "Block protection is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPCN1_A::_1)
    }
}
#[doc = "Field `KEY` writer - Key Code"]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u16, FBPROT1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Block Protection for Secure Cancel"]
    #[inline(always)]
    pub fn bpcn1(&self) -> BPCN1_R {
        BPCN1_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Block Protection for Secure Cancel"]
    #[inline(always)]
    #[must_use]
    pub fn bpcn1(&mut self) -> BPCN1_W<0> {
        BPCN1_W::new(self)
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
#[doc = "Flash Block Protection for Secure Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fbprot1](index.html) module"]
pub struct FBPROT1_SPEC;
impl crate::RegisterSpec for FBPROT1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [fbprot1::R](R) reader structure"]
impl crate::Readable for FBPROT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fbprot1::W](W) writer structure"]
impl crate::Writable for FBPROT1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FBPROT1 to value 0"]
impl crate::Resettable for FBPROT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
