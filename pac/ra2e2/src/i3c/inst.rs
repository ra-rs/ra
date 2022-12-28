#[doc = "Register `INST` reader"]
pub struct R(crate::R<INST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INST` writer"]
pub struct W(crate::W<INST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INST_SPEC>;
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
impl From<crate::W<INST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INEF` reader - Internal Error Flag"]
pub type INEF_R = crate::BitReader<INEF_A>;
#[doc = "Internal Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INEF_A {
    #[doc = "0: I3C Internal Error has not detected."]
    _0 = 0,
    #[doc = "1: I3C Internal Error has detected."]
    _1 = 1,
}
impl From<INEF_A> for bool {
    #[inline(always)]
    fn from(variant: INEF_A) -> Self {
        variant as u8 != 0
    }
}
impl INEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INEF_A {
        match self.bits {
            false => INEF_A::_0,
            true => INEF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INEF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INEF_A::_1
    }
}
#[doc = "Field `INEF` writer - Internal Error Flag"]
pub type INEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, INST_SPEC, INEF_A, O>;
impl<'a, const O: u8> INEF_W<'a, O> {
    #[doc = "I3C Internal Error has not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INEF_A::_0)
    }
    #[doc = "I3C Internal Error has detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INEF_A::_1)
    }
}
impl R {
    #[doc = "Bit 10 - Internal Error Flag"]
    #[inline(always)]
    pub fn inef(&self) -> INEF_R {
        INEF_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - Internal Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn inef(&mut self) -> INEF_W<10> {
        INEF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inst](index.html) module"]
pub struct INST_SPEC;
impl crate::RegisterSpec for INST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inst::R](R) reader structure"]
impl crate::Readable for INST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inst::W](W) writer structure"]
impl crate::Writable for INST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INST to value 0"]
impl crate::Resettable for INST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
