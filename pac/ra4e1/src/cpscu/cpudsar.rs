#[doc = "Register `CPUDSAR` reader"]
pub struct R(crate::R<CPUDSAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPUDSAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPUDSAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPUDSAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPUDSAR` writer"]
pub struct W(crate::W<CPUDSAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPUDSAR_SPEC>;
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
impl From<crate::W<CPUDSAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPUDSAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPUDSA0` reader - CPU Debug Security Attribution 0"]
pub type CPUDSA0_R = crate::BitReader<CPUDSA0_A>;
#[doc = "CPU Debug Security Attribution 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPUDSA0_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<CPUDSA0_A> for bool {
    #[inline(always)]
    fn from(variant: CPUDSA0_A) -> Self {
        variant as u8 != 0
    }
}
impl CPUDSA0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPUDSA0_A {
        match self.bits {
            false => CPUDSA0_A::_0,
            true => CPUDSA0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPUDSA0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPUDSA0_A::_1
    }
}
#[doc = "Field `CPUDSA0` writer - CPU Debug Security Attribution 0"]
pub type CPUDSA0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPUDSAR_SPEC, CPUDSA0_A, O>;
impl<'a, const O: u8> CPUDSA0_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPUDSA0_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPUDSA0_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - CPU Debug Security Attribution 0"]
    #[inline(always)]
    pub fn cpudsa0(&self) -> CPUDSA0_R {
        CPUDSA0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU Debug Security Attribution 0"]
    #[inline(always)]
    #[must_use]
    pub fn cpudsa0(&mut self) -> CPUDSA0_W<0> {
        CPUDSA0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU Debug Security Attribution Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpudsar](index.html) module"]
pub struct CPUDSAR_SPEC;
impl crate::RegisterSpec for CPUDSAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpudsar::R](R) reader structure"]
impl crate::Readable for CPUDSAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpudsar::W](W) writer structure"]
impl crate::Writable for CPUDSAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPUDSAR to value 0xffff_fffe"]
impl crate::Resettable for CPUDSAR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_fffe;
}
