#[doc = "Register `ADSHMSR` reader"]
pub struct R(crate::R<ADSHMSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADSHMSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADSHMSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADSHMSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADSHMSR` writer"]
pub struct W(crate::W<ADSHMSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADSHMSR_SPEC>;
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
impl From<crate::W<ADSHMSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADSHMSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHMD` reader - Channel-Dedicated Sample-and-Hold Circuit Operation Mode Select"]
pub type SHMD_R = crate::BitReader<SHMD_A>;
#[doc = "Channel-Dedicated Sample-and-Hold Circuit Operation Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHMD_A {
    #[doc = "0: Sampling by channel-dedicated sample-and-hold circuit is disable."]
    _0 = 0,
    #[doc = "1: Sampling by channel-dedicated sample-and-hold circuit is enable."]
    _1 = 1,
}
impl From<SHMD_A> for bool {
    #[inline(always)]
    fn from(variant: SHMD_A) -> Self {
        variant as u8 != 0
    }
}
impl SHMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHMD_A {
        match self.bits {
            false => SHMD_A::_0,
            true => SHMD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SHMD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SHMD_A::_1
    }
}
#[doc = "Field `SHMD` writer - Channel-Dedicated Sample-and-Hold Circuit Operation Mode Select"]
pub type SHMD_W<'a, const O: u8> = crate::BitWriter<'a, u8, ADSHMSR_SPEC, SHMD_A, O>;
impl<'a, const O: u8> SHMD_W<'a, O> {
    #[doc = "Sampling by channel-dedicated sample-and-hold circuit is disable."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SHMD_A::_0)
    }
    #[doc = "Sampling by channel-dedicated sample-and-hold circuit is enable."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SHMD_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Channel-Dedicated Sample-and-Hold Circuit Operation Mode Select"]
    #[inline(always)]
    pub fn shmd(&self) -> SHMD_R {
        SHMD_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel-Dedicated Sample-and-Hold Circuit Operation Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn shmd(&mut self) -> SHMD_W<0> {
        SHMD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Sample and Hold Operation Mode Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adshmsr](index.html) module"]
pub struct ADSHMSR_SPEC;
impl crate::RegisterSpec for ADSHMSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [adshmsr::R](R) reader structure"]
impl crate::Readable for ADSHMSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adshmsr::W](W) writer structure"]
impl crate::Writable for ADSHMSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADSHMSR to value 0"]
impl crate::Resettable for ADSHMSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
