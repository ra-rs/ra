#[doc = "Register `ADCMPANSR0` reader"]
pub struct R(crate::R<ADCMPANSR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCMPANSR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCMPANSR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCMPANSR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCMPANSR0` writer"]
pub struct W(crate::W<ADCMPANSR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCMPANSR0_SPEC>;
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
impl From<crate::W<ADCMPANSR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCMPANSR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPCHAn` reader - Compare Window A Channel Select"]
pub type CMPCHAN_R = crate::FieldReader<u16, CMPCHAN_A>;
#[doc = "Compare Window A Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum CMPCHAN_A {
    #[doc = "0: Disable compare function for associated input channel"]
    _0 = 0,
    #[doc = "1: Enable compare function for associated input channel"]
    _1 = 1,
}
impl From<CMPCHAN_A> for u16 {
    #[inline(always)]
    fn from(variant: CMPCHAN_A) -> Self {
        variant as _
    }
}
impl CMPCHAN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMPCHAN_A> {
        match self.bits {
            0 => Some(CMPCHAN_A::_0),
            1 => Some(CMPCHAN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHAN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHAN_A::_1
    }
}
#[doc = "Field `CMPCHAn` writer - Compare Window A Channel Select"]
pub type CMPCHAN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, ADCMPANSR0_SPEC, u16, CMPCHAN_A, 16, O>;
impl<'a, const O: u8> CMPCHAN_W<'a, O> {
    #[doc = "Disable compare function for associated input channel"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPCHAN_A::_0)
    }
    #[doc = "Enable compare function for associated input channel"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPCHAN_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:15 - Compare Window A Channel Select"]
    #[inline(always)]
    pub fn cmpchan(&self) -> CMPCHAN_R {
        CMPCHAN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Compare Window A Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmpchan(&mut self) -> CMPCHAN_W<0> {
        CMPCHAN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Compare Function Window A Channel Select Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcmpansr0](index.html) module"]
pub struct ADCMPANSR0_SPEC;
impl crate::RegisterSpec for ADCMPANSR0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adcmpansr0::R](R) reader structure"]
impl crate::Readable for ADCMPANSR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcmpansr0::W](W) writer structure"]
impl crate::Writable for ADCMPANSR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCMPANSR0 to value 0"]
impl crate::Resettable for ADCMPANSR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
