#[doc = "Register `ADADS1` reader"]
pub struct R(crate::R<ADADS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADADS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADADS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADADS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADADS1` writer"]
pub struct W(crate::W<ADADS1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADADS1_SPEC>;
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
impl From<crate::W<ADADS1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADADS1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADSn` reader - A/D-Converted Value Addition/Average Channel Select"]
pub type ADSN_R = crate::FieldReader<u16, ADSN_A>;
#[doc = "A/D-Converted Value Addition/Average Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum ADSN_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ADSN_A> for u16 {
    #[inline(always)]
    fn from(variant: ADSN_A) -> Self {
        variant as _
    }
}
impl ADSN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADSN_A> {
        match self.bits {
            0 => Some(ADSN_A::_0),
            1 => Some(ADSN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADSN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADSN_A::_1
    }
}
#[doc = "Field `ADSn` writer - A/D-Converted Value Addition/Average Channel Select"]
pub type ADSN_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ADADS1_SPEC, u16, ADSN_A, 16, O>;
impl<'a, const O: u8> ADSN_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADSN_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADSN_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:15 - A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    pub fn adsn(&self) -> ADSN_R {
        ADSN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - A/D-Converted Value Addition/Average Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn adsn(&mut self) -> ADSN_W<0> {
        ADSN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D-Converted Value Addition/Average Channel Select Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adads1](index.html) module"]
pub struct ADADS1_SPEC;
impl crate::RegisterSpec for ADADS1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adads1::R](R) reader structure"]
impl crate::Readable for ADADS1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adads1::W](W) writer structure"]
impl crate::Writable for ADADS1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADADS1 to value 0"]
impl crate::Resettable for ADADS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
