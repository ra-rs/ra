#[doc = "Register `NQTHCTL` reader"]
pub struct R(crate::R<NQTHCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NQTHCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NQTHCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NQTHCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NQTHCTL` writer"]
pub struct W(crate::W<NQTHCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NQTHCTL_SPEC>;
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
impl From<crate::W<NQTHCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NQTHCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMDQTH` reader - Normal Command Ready Queue Threshold"]
pub type CMDQTH_R = crate::FieldReader<u8, CMDQTH_A>;
#[doc = "Normal Command Ready Queue Threshold\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMDQTH_A {
    #[doc = "0: Interrupt is issued when Command Queue is completely empty."]
    _0X00 = 0,
}
impl From<CMDQTH_A> for u8 {
    #[inline(always)]
    fn from(variant: CMDQTH_A) -> Self {
        variant as _
    }
}
impl CMDQTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMDQTH_A> {
        match self.bits {
            0 => Some(CMDQTH_A::_0X00),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == CMDQTH_A::_0X00
    }
}
#[doc = "Field `CMDQTH` writer - Normal Command Ready Queue Threshold"]
pub type CMDQTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NQTHCTL_SPEC, u8, CMDQTH_A, 8, O>;
impl<'a, const O: u8> CMDQTH_W<'a, O> {
    #[doc = "Interrupt is issued when Command Queue is completely empty."]
    #[inline(always)]
    pub fn _0x00(self) -> &'a mut W {
        self.variant(CMDQTH_A::_0X00)
    }
}
#[doc = "Field `RSPQTH` reader - Normal Response Queue Threshold"]
pub type RSPQTH_R = crate::FieldReader<u8, RSPQTH_A>;
#[doc = "Normal Response Queue Threshold\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RSPQTH_A {
    #[doc = "0: Interrupt is issued when Response Queue contains 1 entry (DWORD)."]
    _0X00 = 0,
}
impl From<RSPQTH_A> for u8 {
    #[inline(always)]
    fn from(variant: RSPQTH_A) -> Self {
        variant as _
    }
}
impl RSPQTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RSPQTH_A> {
        match self.bits {
            0 => Some(RSPQTH_A::_0X00),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == RSPQTH_A::_0X00
    }
}
#[doc = "Field `RSPQTH` writer - Normal Response Queue Threshold"]
pub type RSPQTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NQTHCTL_SPEC, u8, RSPQTH_A, 8, O>;
impl<'a, const O: u8> RSPQTH_W<'a, O> {
    #[doc = "Interrupt is issued when Response Queue contains 1 entry (DWORD)."]
    #[inline(always)]
    pub fn _0x00(self) -> &'a mut W {
        self.variant(RSPQTH_A::_0X00)
    }
}
#[doc = "Field `IBIDSSZ` reader - Normal IBI Data Segment Size"]
pub type IBIDSSZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IBIDSSZ` writer - Normal IBI Data Segment Size"]
pub type IBIDSSZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NQTHCTL_SPEC, u8, u8, 8, O>;
#[doc = "Field `IBIQTH` reader - Normal IBI Queue Threshold"]
pub type IBIQTH_R = crate::FieldReader<u8, IBIQTH_A>;
#[doc = "Normal IBI Queue Threshold\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IBIQTH_A {
    #[doc = "0: I3C Protocol mode (Master): Interrupt is generated when the Outstanding IBI Status count is 1 or more. I3C Protocol mode (Slave): Interrupt is issued when IBI Data Buffer is completely empty."]
    _0X00 = 0,
}
impl From<IBIQTH_A> for u8 {
    #[inline(always)]
    fn from(variant: IBIQTH_A) -> Self {
        variant as _
    }
}
impl IBIQTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IBIQTH_A> {
        match self.bits {
            0 => Some(IBIQTH_A::_0X00),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == IBIQTH_A::_0X00
    }
}
#[doc = "Field `IBIQTH` writer - Normal IBI Queue Threshold"]
pub type IBIQTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NQTHCTL_SPEC, u8, IBIQTH_A, 8, O>;
impl<'a, const O: u8> IBIQTH_W<'a, O> {
    #[doc = "I3C Protocol mode (Master): Interrupt is generated when the Outstanding IBI Status count is 1 or more. I3C Protocol mode (Slave): Interrupt is issued when IBI Data Buffer is completely empty."]
    #[inline(always)]
    pub fn _0x00(self) -> &'a mut W {
        self.variant(IBIQTH_A::_0X00)
    }
}
impl R {
    #[doc = "Bits 0:7 - Normal Command Ready Queue Threshold"]
    #[inline(always)]
    pub fn cmdqth(&self) -> CMDQTH_R {
        CMDQTH_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Normal Response Queue Threshold"]
    #[inline(always)]
    pub fn rspqth(&self) -> RSPQTH_R {
        RSPQTH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Normal IBI Data Segment Size"]
    #[inline(always)]
    pub fn ibidssz(&self) -> IBIDSSZ_R {
        IBIDSSZ_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Normal IBI Queue Threshold"]
    #[inline(always)]
    pub fn ibiqth(&self) -> IBIQTH_R {
        IBIQTH_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Normal Command Ready Queue Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn cmdqth(&mut self) -> CMDQTH_W<0> {
        CMDQTH_W::new(self)
    }
    #[doc = "Bits 8:15 - Normal Response Queue Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn rspqth(&mut self) -> RSPQTH_W<8> {
        RSPQTH_W::new(self)
    }
    #[doc = "Bits 16:23 - Normal IBI Data Segment Size"]
    #[inline(always)]
    #[must_use]
    pub fn ibidssz(&mut self) -> IBIDSSZ_W<16> {
        IBIDSSZ_W::new(self)
    }
    #[doc = "Bits 24:31 - Normal IBI Queue Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn ibiqth(&mut self) -> IBIQTH_W<24> {
        IBIQTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Normal Queue Threshold Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nqthctl](index.html) module"]
pub struct NQTHCTL_SPEC;
impl crate::RegisterSpec for NQTHCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nqthctl::R](R) reader structure"]
impl crate::Readable for NQTHCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nqthctl::W](W) writer structure"]
impl crate::Writable for NQTHCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NQTHCTL to value 0x0101_0101"]
impl crate::Resettable for NQTHCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0101_0101;
}
