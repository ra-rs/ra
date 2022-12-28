#[doc = "Register `SELSR0` reader"]
pub struct R(crate::R<SELSR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SELSR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SELSR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SELSR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SELSR0` writer"]
pub struct W(crate::W<SELSR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SELSR0_SPEC>;
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
impl From<crate::W<SELSR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SELSR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SELS` reader - SYS Event Link Select"]
pub type SELS_R = crate::FieldReader<u8, SELS_A>;
#[doc = "SYS Event Link Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SELS_A {
    #[doc = "0: Disable event output to the associated low-power mode module"]
    _0X00 = 0,
}
impl From<SELS_A> for u8 {
    #[inline(always)]
    fn from(variant: SELS_A) -> Self {
        variant as _
    }
}
impl SELS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SELS_A> {
        match self.bits {
            0 => Some(SELS_A::_0X00),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == SELS_A::_0X00
    }
}
#[doc = "Field `SELS` writer - SYS Event Link Select"]
pub type SELS_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SELSR0_SPEC, u8, SELS_A, 8, O>;
impl<'a, const O: u8> SELS_W<'a, O> {
    #[doc = "Disable event output to the associated low-power mode module"]
    #[inline(always)]
    pub fn _0x00(self) -> &'a mut W {
        self.variant(SELS_A::_0X00)
    }
}
impl R {
    #[doc = "Bits 0:7 - SYS Event Link Select"]
    #[inline(always)]
    pub fn sels(&self) -> SELS_R {
        SELS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SYS Event Link Select"]
    #[inline(always)]
    #[must_use]
    pub fn sels(&mut self) -> SELS_W<0> {
        SELS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYS Event Link Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [selsr0](index.html) module"]
pub struct SELSR0_SPEC;
impl crate::RegisterSpec for SELSR0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [selsr0::R](R) reader structure"]
impl crate::Readable for SELSR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [selsr0::W](W) writer structure"]
impl crate::Writable for SELSR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SELSR0 to value 0"]
impl crate::Resettable for SELSR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
