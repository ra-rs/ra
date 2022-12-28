#[doc = "Register `NRQTHCTL` reader"]
pub struct R(crate::R<NRQTHCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NRQTHCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NRQTHCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NRQTHCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NRQTHCTL` writer"]
pub struct W(crate::W<NRQTHCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NRQTHCTL_SPEC>;
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
impl From<crate::W<NRQTHCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NRQTHCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSQTH` reader - Normal Receive Status Queue Threshold"]
pub type RSQTH_R = crate::FieldReader<u8, RSQTH_A>;
#[doc = "Normal Receive Status Queue Threshold\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RSQTH_A {
    #[doc = "0: Interrupt is issued when Receive Status Queue contains 1 entry (DWORD)."]
    _0X00 = 0,
}
impl From<RSQTH_A> for u8 {
    #[inline(always)]
    fn from(variant: RSQTH_A) -> Self {
        variant as _
    }
}
impl RSQTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RSQTH_A> {
        match self.bits {
            0 => Some(RSQTH_A::_0X00),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == RSQTH_A::_0X00
    }
}
#[doc = "Field `RSQTH` writer - Normal Receive Status Queue Threshold"]
pub type RSQTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NRQTHCTL_SPEC, u8, RSQTH_A, 8, O>;
impl<'a, const O: u8> RSQTH_W<'a, O> {
    #[doc = "Interrupt is issued when Receive Status Queue contains 1 entry (DWORD)."]
    #[inline(always)]
    pub fn _0x00(self) -> &'a mut W {
        self.variant(RSQTH_A::_0X00)
    }
}
impl R {
    #[doc = "Bits 0:7 - Normal Receive Status Queue Threshold"]
    #[inline(always)]
    pub fn rsqth(&self) -> RSQTH_R {
        RSQTH_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Normal Receive Status Queue Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn rsqth(&mut self) -> RSQTH_W<0> {
        RSQTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Normal Receive Status Queue Threshold Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nrqthctl](index.html) module"]
pub struct NRQTHCTL_SPEC;
impl crate::RegisterSpec for NRQTHCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nrqthctl::R](R) reader structure"]
impl crate::Readable for NRQTHCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nrqthctl::W](W) writer structure"]
impl crate::Writable for NRQTHCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NRQTHCTL to value 0x01"]
impl crate::Resettable for NRQTHCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
