#[doc = "Register `FASR` reader"]
pub struct R(crate::R<FASR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FASR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FASR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FASR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FASR` writer"]
pub struct W(crate::W<FASR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FASR_SPEC>;
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
impl From<crate::W<FASR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FASR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXS` reader - Extra Area Select"]
pub type EXS_R = crate::BitReader<EXS_A>;
#[doc = "Extra Area Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXS_A {
    #[doc = "0: User area or data area"]
    _0 = 0,
    #[doc = "1: Extra area."]
    _1 = 1,
}
impl From<EXS_A> for bool {
    #[inline(always)]
    fn from(variant: EXS_A) -> Self {
        variant as u8 != 0
    }
}
impl EXS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXS_A {
        match self.bits {
            false => EXS_A::_0,
            true => EXS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EXS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EXS_A::_1
    }
}
#[doc = "Field `EXS` writer - Extra Area Select"]
pub type EXS_W<'a, const O: u8> = crate::BitWriter<'a, u8, FASR_SPEC, EXS_A, O>;
impl<'a, const O: u8> EXS_W<'a, O> {
    #[doc = "User area or data area"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EXS_A::_0)
    }
    #[doc = "Extra area."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EXS_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Extra Area Select"]
    #[inline(always)]
    pub fn exs(&self) -> EXS_R {
        EXS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Extra Area Select"]
    #[inline(always)]
    pub fn exs(&mut self) -> EXS_W<0> {
        EXS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Area Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fasr](index.html) module"]
pub struct FASR_SPEC;
impl crate::RegisterSpec for FASR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fasr::R](R) reader structure"]
impl crate::Readable for FASR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fasr::W](W) writer structure"]
impl crate::Writable for FASR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FASR to value 0"]
impl crate::Resettable for FASR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
