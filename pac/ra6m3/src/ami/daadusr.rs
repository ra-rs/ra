#[doc = "Register `DAADUSR` reader"]
pub struct R(crate::R<DAADUSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAADUSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAADUSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAADUSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAADUSR` writer"]
pub struct W(crate::W<DAADUSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAADUSR_SPEC>;
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
impl From<crate::W<DAADUSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAADUSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AMADSEL1` reader - A/D Unit 1 Select"]
pub type AMADSEL1_R = crate::BitReader<AMADSEL1_A>;
#[doc = "A/D Unit 1 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMADSEL1_A {
    #[doc = "0: Unit 1 is not selected."]
    _0 = 0,
    #[doc = "1: Unit 1 is selected."]
    _1 = 1,
}
impl From<AMADSEL1_A> for bool {
    #[inline(always)]
    fn from(variant: AMADSEL1_A) -> Self {
        variant as u8 != 0
    }
}
impl AMADSEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMADSEL1_A {
        match self.bits {
            false => AMADSEL1_A::_0,
            true => AMADSEL1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMADSEL1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMADSEL1_A::_1
    }
}
#[doc = "Field `AMADSEL1` writer - A/D Unit 1 Select"]
pub type AMADSEL1_W<'a, const O: u8> = crate::BitWriter<'a, u8, DAADUSR_SPEC, AMADSEL1_A, O>;
impl<'a, const O: u8> AMADSEL1_W<'a, O> {
    #[doc = "Unit 1 is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AMADSEL1_A::_0)
    }
    #[doc = "Unit 1 is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AMADSEL1_A::_1)
    }
}
impl R {
    #[doc = "Bit 1 - A/D Unit 1 Select"]
    #[inline(always)]
    pub fn amadsel1(&self) -> AMADSEL1_R {
        AMADSEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - A/D Unit 1 Select"]
    #[inline(always)]
    #[must_use]
    pub fn amadsel1(&mut self) -> AMADSEL1_W<1> {
        AMADSEL1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "D/A A/D Synchronous Unit Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [daadusr](index.html) module"]
pub struct DAADUSR_SPEC;
impl crate::RegisterSpec for DAADUSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [daadusr::R](R) reader structure"]
impl crate::Readable for DAADUSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [daadusr::W](W) writer structure"]
impl crate::Writable for DAADUSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAADUSR to value 0"]
impl crate::Resettable for DAADUSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
