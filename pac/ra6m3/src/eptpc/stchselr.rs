#[doc = "Register `STCHSELR` reader"]
pub struct R(crate::R<STCHSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STCHSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STCHSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STCHSELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STCHSELR` writer"]
pub struct W(crate::W<STCHSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STCHSELR_SPEC>;
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
impl From<crate::W<STCHSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STCHSELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSEL` reader - Timer Information Input SelectNOTE: Do not change the value of this bit while the SYNSTARTR.STR bit is 1."]
pub type SYSEL_R = crate::BitReader<SYSEL_A>;
#[doc = "Timer Information Input SelectNOTE: Do not change the value of this bit while the SYNSTARTR.STR bit is 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSEL_A {
    #[doc = "0: Time information from synchronization from the SYNFP0 module is used."]
    _0 = 0,
    #[doc = "1: Time information from synchronization from the SYNFP1 module is used."]
    _1 = 1,
}
impl From<SYSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SYSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSEL_A {
        match self.bits {
            false => SYSEL_A::_0,
            true => SYSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SYSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SYSEL_A::_1
    }
}
#[doc = "Field `SYSEL` writer - Timer Information Input SelectNOTE: Do not change the value of this bit while the SYNSTARTR.STR bit is 1."]
pub type SYSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, STCHSELR_SPEC, SYSEL_A, O>;
impl<'a, const O: u8> SYSEL_W<'a, O> {
    #[doc = "Time information from synchronization from the SYNFP0 module is used."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYSEL_A::_0)
    }
    #[doc = "Time information from synchronization from the SYNFP1 module is used."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYSEL_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Timer Information Input SelectNOTE: Do not change the value of this bit while the SYNSTARTR.STR bit is 1."]
    #[inline(always)]
    pub fn sysel(&self) -> SYSEL_R {
        SYSEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer Information Input SelectNOTE: Do not change the value of this bit while the SYNSTARTR.STR bit is 1."]
    #[inline(always)]
    #[must_use]
    pub fn sysel(&mut self) -> SYSEL_W<0> {
        SYSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Time Synchronization Channel Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stchselr](index.html) module"]
pub struct STCHSELR_SPEC;
impl crate::RegisterSpec for STCHSELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stchselr::R](R) reader structure"]
impl crate::Readable for STCHSELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stchselr::W](W) writer structure"]
impl crate::Writable for STCHSELR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STCHSELR to value 0"]
impl crate::Resettable for STCHSELR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
