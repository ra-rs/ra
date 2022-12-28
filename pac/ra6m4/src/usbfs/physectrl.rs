#[doc = "Register `PHYSECTRL` reader"]
pub struct R(crate::R<PHYSECTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PHYSECTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PHYSECTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PHYSECTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PHYSECTRL` writer"]
pub struct W(crate::W<PHYSECTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PHYSECTRL_SPEC>;
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
impl From<crate::W<PHYSECTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PHYSECTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNEN` reader - Single-ended Receiver Enable"]
pub type CNEN_R = crate::BitReader<CNEN_A>;
#[doc = "Single-ended Receiver Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CNEN_A {
    #[doc = "0: Single-ended receiver operation is disabled"]
    _0 = 0,
    #[doc = "1: Single-ended receiver operation is enabled"]
    _1 = 1,
}
impl From<CNEN_A> for bool {
    #[inline(always)]
    fn from(variant: CNEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CNEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNEN_A {
        match self.bits {
            false => CNEN_A::_0,
            true => CNEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CNEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CNEN_A::_1
    }
}
#[doc = "Field `CNEN` writer - Single-ended Receiver Enable"]
pub type CNEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PHYSECTRL_SPEC, CNEN_A, O>;
impl<'a, const O: u8> CNEN_W<'a, O> {
    #[doc = "Single-ended receiver operation is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNEN_A::_0)
    }
    #[doc = "Single-ended receiver operation is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CNEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 4 - Single-ended Receiver Enable"]
    #[inline(always)]
    pub fn cnen(&self) -> CNEN_R {
        CNEN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Single-ended Receiver Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cnen(&mut self) -> CNEN_W<4> {
        CNEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PHY Single-ended Receiver Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [physectrl](index.html) module"]
pub struct PHYSECTRL_SPEC;
impl crate::RegisterSpec for PHYSECTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [physectrl::R](R) reader structure"]
impl crate::Readable for PHYSECTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [physectrl::W](W) writer structure"]
impl crate::Writable for PHYSECTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PHYSECTRL to value 0"]
impl crate::Resettable for PHYSECTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
