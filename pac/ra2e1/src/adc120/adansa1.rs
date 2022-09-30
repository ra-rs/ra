#[doc = "Register `ADANSA1` reader"]
pub struct R(crate::R<ADANSA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADANSA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADANSA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADANSA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADANSA1` writer"]
pub struct W(crate::W<ADANSA1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADANSA1_SPEC>;
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
impl From<crate::W<ADANSA1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADANSA1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ANSAn` reader - A/D Conversion Channels Select"]
pub type ANSAN_R = crate::FieldReader<u16, ANSAN_A>;
#[doc = "A/D Conversion Channels Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum ANSAN_A {
    #[doc = "0: Do not select associated input channel."]
    _0 = 0,
    #[doc = "1: Select associated input channel."]
    _1 = 1,
}
impl From<ANSAN_A> for u16 {
    #[inline(always)]
    fn from(variant: ANSAN_A) -> Self {
        variant as _
    }
}
impl ANSAN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ANSAN_A> {
        match self.bits {
            0 => Some(ANSAN_A::_0),
            1 => Some(ANSAN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSAN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSAN_A::_1
    }
}
#[doc = "Field `ANSAn` writer - A/D Conversion Channels Select"]
pub type ANSAN_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ADANSA1_SPEC, u16, ANSAN_A, 16, O>;
impl<'a, const O: u8> ANSAN_W<'a, O> {
    #[doc = "Do not select associated input channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANSAN_A::_0)
    }
    #[doc = "Select associated input channel."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANSAN_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:15 - A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansan(&self) -> ANSAN_R {
        ANSAN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - A/D Conversion Channels Select"]
    #[inline(always)]
    pub fn ansan(&mut self) -> ANSAN_W<0> {
        ANSAN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Channel Select Register A1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adansa1](index.html) module"]
pub struct ADANSA1_SPEC;
impl crate::RegisterSpec for ADANSA1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adansa1::R](R) reader structure"]
impl crate::Readable for ADANSA1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adansa1::W](W) writer structure"]
impl crate::Writable for ADANSA1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADANSA1 to value 0"]
impl crate::Resettable for ADANSA1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
