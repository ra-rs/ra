#[doc = "Register `ADCMPSR0` reader"]
pub struct R(crate::R<ADCMPSR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCMPSR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCMPSR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCMPSR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCMPSR0` writer"]
pub struct W(crate::W<ADCMPSR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCMPSR0_SPEC>;
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
impl From<crate::W<ADCMPSR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCMPSR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPSTCHAn` reader - Compare Window A Flag"]
pub type CMPSTCHAN_R = crate::FieldReader<u16, CMPSTCHAN_A>;
#[doc = "Compare Window A Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum CMPSTCHAN_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHAN_A> for u16 {
    #[inline(always)]
    fn from(variant: CMPSTCHAN_A) -> Self {
        variant as _
    }
}
impl CMPSTCHAN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMPSTCHAN_A> {
        match self.bits {
            0 => Some(CMPSTCHAN_A::_0),
            1 => Some(CMPSTCHAN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHAN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHAN_A::_1
    }
}
#[doc = "Field `CMPSTCHAn` writer - Compare Window A Flag"]
pub type CMPSTCHAN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, ADCMPSR0_SPEC, u16, CMPSTCHAN_A, 16, O>;
impl<'a, const O: u8> CMPSTCHAN_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHAN_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHAN_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:15 - Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstchan(&self) -> CMPSTCHAN_R {
        CMPSTCHAN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Compare Window A Flag"]
    #[inline(always)]
    pub fn cmpstchan(&mut self) -> CMPSTCHAN_W<0> {
        CMPSTCHAN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Compare Function Window A Channel Status Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcmpsr0](index.html) module"]
pub struct ADCMPSR0_SPEC;
impl crate::RegisterSpec for ADCMPSR0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adcmpsr0::R](R) reader structure"]
impl crate::Readable for ADCMPSR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcmpsr0::W](W) writer structure"]
impl crate::Writable for ADCMPSR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADCMPSR0 to value 0"]
impl crate::Resettable for ADCMPSR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
