#[doc = "Register `DMBWR` reader"]
pub struct R(crate::R<DMBWR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMBWR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMBWR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMBWR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMBWR` writer"]
pub struct W(crate::W<DMBWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMBWR_SPEC>;
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
impl From<crate::W<DMBWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMBWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BWE` reader - Bufferable Write Enable"]
pub type BWE_R = crate::BitReader<BWE_A>;
#[doc = "Bufferable Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BWE_A {
    #[doc = "0: Disables Bufferable Write"]
    _0 = 0,
    #[doc = "1: Enables Bufferable Write"]
    _1 = 1,
}
impl From<BWE_A> for bool {
    #[inline(always)]
    fn from(variant: BWE_A) -> Self {
        variant as u8 != 0
    }
}
impl BWE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWE_A {
        match self.bits {
            false => BWE_A::_0,
            true => BWE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BWE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BWE_A::_1
    }
}
#[doc = "Field `BWE` writer - Bufferable Write Enable"]
pub type BWE_W<'a, const O: u8> = crate::BitWriter<'a, u8, DMBWR_SPEC, BWE_A, O>;
impl<'a, const O: u8> BWE_W<'a, O> {
    #[doc = "Disables Bufferable Write"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BWE_A::_0)
    }
    #[doc = "Enables Bufferable Write"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BWE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Bufferable Write Enable"]
    #[inline(always)]
    pub fn bwe(&self) -> BWE_R {
        BWE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bufferable Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bwe(&mut self) -> BWE_W<0> {
        BWE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Bufferable Write Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmbwr](index.html) module"]
pub struct DMBWR_SPEC;
impl crate::RegisterSpec for DMBWR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dmbwr::R](R) reader structure"]
impl crate::Readable for DMBWR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmbwr::W](W) writer structure"]
impl crate::Writable for DMBWR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMBWR to value 0"]
impl crate::Resettable for DMBWR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
