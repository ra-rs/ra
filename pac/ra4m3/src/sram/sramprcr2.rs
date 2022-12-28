#[doc = "Register `SRAMPRCR2` reader"]
pub struct R(crate::R<SRAMPRCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRAMPRCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRAMPRCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRAMPRCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRAMPRCR2` writer"]
pub struct W(crate::W<SRAMPRCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRAMPRCR2_SPEC>;
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
impl From<crate::W<SRAMPRCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRAMPRCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRAMPRCR2` reader - Register Write Control"]
pub type SRAMPRCR2_R = crate::BitReader<SRAMPRCR2_A>;
#[doc = "Register Write Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAMPRCR2_A {
    #[doc = "0: Disable writes to the protectedregisters"]
    _0 = 0,
    #[doc = "1: Enable writes to the protected registers"]
    _1 = 1,
}
impl From<SRAMPRCR2_A> for bool {
    #[inline(always)]
    fn from(variant: SRAMPRCR2_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAMPRCR2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAMPRCR2_A {
        match self.bits {
            false => SRAMPRCR2_A::_0,
            true => SRAMPRCR2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SRAMPRCR2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SRAMPRCR2_A::_1
    }
}
#[doc = "Field `SRAMPRCR2` writer - Register Write Control"]
pub type SRAMPRCR2_W<'a, const O: u8> = crate::BitWriter<'a, u8, SRAMPRCR2_SPEC, SRAMPRCR2_A, O>;
impl<'a, const O: u8> SRAMPRCR2_W<'a, O> {
    #[doc = "Disable writes to the protectedregisters"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SRAMPRCR2_A::_0)
    }
    #[doc = "Enable writes to the protected registers"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SRAMPRCR2_A::_1)
    }
}
#[doc = "Field `KW` writer - Write Key Code"]
pub type KW_W<'a, const O: u8> = crate::FieldWriter<'a, u8, SRAMPRCR2_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 0 - Register Write Control"]
    #[inline(always)]
    pub fn sramprcr2(&self) -> SRAMPRCR2_R {
        SRAMPRCR2_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Register Write Control"]
    #[inline(always)]
    #[must_use]
    pub fn sramprcr2(&mut self) -> SRAMPRCR2_W<0> {
        SRAMPRCR2_W::new(self)
    }
    #[doc = "Bits 1:7 - Write Key Code"]
    #[inline(always)]
    #[must_use]
    pub fn kw(&mut self) -> KW_W<1> {
        KW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM Protection Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sramprcr2](index.html) module"]
pub struct SRAMPRCR2_SPEC;
impl crate::RegisterSpec for SRAMPRCR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sramprcr2::R](R) reader structure"]
impl crate::Readable for SRAMPRCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sramprcr2::W](W) writer structure"]
impl crate::Writable for SRAMPRCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRAMPRCR2 to value 0"]
impl crate::Resettable for SRAMPRCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
