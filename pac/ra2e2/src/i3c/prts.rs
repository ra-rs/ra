#[doc = "Register `PRTS` reader"]
pub struct R(crate::R<PRTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRTS` writer"]
pub struct W(crate::W<PRTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRTS_SPEC>;
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
impl From<crate::W<PRTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRTMD` reader - Protocol Mode"]
pub type PRTMD_R = crate::BitReader<PRTMD_A>;
#[doc = "Protocol Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRTMD_A {
    #[doc = "0: I3C protocol mode"]
    _0 = 0,
    #[doc = "1: I2C protocol mode"]
    _1 = 1,
}
impl From<PRTMD_A> for bool {
    #[inline(always)]
    fn from(variant: PRTMD_A) -> Self {
        variant as u8 != 0
    }
}
impl PRTMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRTMD_A {
        match self.bits {
            false => PRTMD_A::_0,
            true => PRTMD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PRTMD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PRTMD_A::_1
    }
}
#[doc = "Field `PRTMD` writer - Protocol Mode"]
pub type PRTMD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRTS_SPEC, PRTMD_A, O>;
impl<'a, const O: u8> PRTMD_W<'a, O> {
    #[doc = "I3C protocol mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PRTMD_A::_0)
    }
    #[doc = "I2C protocol mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PRTMD_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Protocol Mode"]
    #[inline(always)]
    pub fn prtmd(&self) -> PRTMD_R {
        PRTMD_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Protocol Mode"]
    #[inline(always)]
    #[must_use]
    pub fn prtmd(&mut self) -> PRTMD_W<0> {
        PRTMD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Protocol Selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prts](index.html) module"]
pub struct PRTS_SPEC;
impl crate::RegisterSpec for PRTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prts::R](R) reader structure"]
impl crate::Readable for PRTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prts::W](W) writer structure"]
impl crate::Writable for PRTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRTS to value 0x01"]
impl crate::Resettable for PRTS_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
