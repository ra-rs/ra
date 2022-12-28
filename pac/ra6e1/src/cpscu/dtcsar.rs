#[doc = "Register `DTCSAR` reader"]
pub struct R(crate::R<DTCSAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTCSAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTCSAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTCSAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTCSAR` writer"]
pub struct W(crate::W<DTCSAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTCSAR_SPEC>;
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
impl From<crate::W<DTCSAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTCSAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTCSTSA` reader - DTC Security Attribution"]
pub type DTCSTSA_R = crate::BitReader<DTCSTSA_A>;
#[doc = "DTC Security Attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTCSTSA_A {
    #[doc = "0: Secure."]
    _0 = 0,
    #[doc = "1: Non-Secure."]
    _1 = 1,
}
impl From<DTCSTSA_A> for bool {
    #[inline(always)]
    fn from(variant: DTCSTSA_A) -> Self {
        variant as u8 != 0
    }
}
impl DTCSTSA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTCSTSA_A {
        match self.bits {
            false => DTCSTSA_A::_0,
            true => DTCSTSA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTCSTSA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTCSTSA_A::_1
    }
}
#[doc = "Field `DTCSTSA` writer - DTC Security Attribution"]
pub type DTCSTSA_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTCSAR_SPEC, DTCSTSA_A, O>;
impl<'a, const O: u8> DTCSTSA_W<'a, O> {
    #[doc = "Secure."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTCSTSA_A::_0)
    }
    #[doc = "Non-Secure."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTCSTSA_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - DTC Security Attribution"]
    #[inline(always)]
    pub fn dtcstsa(&self) -> DTCSTSA_R {
        DTCSTSA_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DTC Security Attribution"]
    #[inline(always)]
    #[must_use]
    pub fn dtcstsa(&mut self) -> DTCSTSA_W<0> {
        DTCSTSA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DTC Controller Security Attribution Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtcsar](index.html) module"]
pub struct DTCSAR_SPEC;
impl crate::RegisterSpec for DTCSAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtcsar::R](R) reader structure"]
impl crate::Readable for DTCSAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dtcsar::W](W) writer structure"]
impl crate::Writable for DTCSAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DTCSAR to value 0xffff_ffff"]
impl crate::Resettable for DTCSAR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
