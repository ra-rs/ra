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
#[doc = "Field `AMADSEL0` reader - A/D Unit 0 Select"]
pub type AMADSEL0_R = crate::BitReader<AMADSEL0_A>;
#[doc = "A/D Unit 0 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMADSEL0_A {
    #[doc = "0: Do not select unit 0"]
    _0 = 0,
    #[doc = "1: Select unit 0"]
    _1 = 1,
}
impl From<AMADSEL0_A> for bool {
    #[inline(always)]
    fn from(variant: AMADSEL0_A) -> Self {
        variant as u8 != 0
    }
}
impl AMADSEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMADSEL0_A {
        match self.bits {
            false => AMADSEL0_A::_0,
            true => AMADSEL0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMADSEL0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMADSEL0_A::_1
    }
}
#[doc = "Field `AMADSEL0` writer - A/D Unit 0 Select"]
pub type AMADSEL0_W<'a, const O: u8> = crate::BitWriter<'a, u8, DAADUSR_SPEC, AMADSEL0_A, O>;
impl<'a, const O: u8> AMADSEL0_W<'a, O> {
    #[doc = "Do not select unit 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AMADSEL0_A::_0)
    }
    #[doc = "Select unit 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AMADSEL0_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - A/D Unit 0 Select"]
    #[inline(always)]
    pub fn amadsel0(&self) -> AMADSEL0_R {
        AMADSEL0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A/D Unit 0 Select"]
    #[inline(always)]
    #[must_use]
    pub fn amadsel0(&mut self) -> AMADSEL0_W<0> {
        AMADSEL0_W::new(self)
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
