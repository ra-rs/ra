#[doc = "Register `SDIR` reader"]
pub struct R(crate::R<SDIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDIR` writer"]
pub struct W(crate::W<SDIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDIR_SPEC>;
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
impl From<crate::W<SDIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARFI` reader - Initialization Auto-Refresh Interval (ARFI+3 cycles )"]
pub type ARFI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ARFI` writer - Initialization Auto-Refresh Interval (ARFI+3 cycles )"]
pub type ARFI_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SDIR_SPEC, u8, u8, 4, O>;
#[doc = "Field `ARFC` reader - Initialization Auto-Refresh Count"]
pub type ARFC_R = crate::FieldReader<u8, ARFC_A>;
#[doc = "Initialization Auto-Refresh Count\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ARFC_A {
    #[doc = "0: Setting prohibited"]
    _0X0 = 0,
}
impl From<ARFC_A> for u8 {
    #[inline(always)]
    fn from(variant: ARFC_A) -> Self {
        variant as _
    }
}
impl ARFC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ARFC_A> {
        match self.bits {
            0 => Some(ARFC_A::_0X0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X0`"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == ARFC_A::_0X0
    }
}
#[doc = "Field `ARFC` writer - Initialization Auto-Refresh Count"]
pub type ARFC_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SDIR_SPEC, u8, ARFC_A, 4, O>;
impl<'a, const O: u8> ARFC_W<'a, O> {
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut W {
        self.variant(ARFC_A::_0X0)
    }
}
#[doc = "Field `PRC` reader - Initialization Precharge Cycle Count (PRC+3 cycles)"]
pub type PRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRC` writer - Initialization Precharge Cycle Count (PRC+3 cycles)"]
pub type PRC_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SDIR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:3 - Initialization Auto-Refresh Interval (ARFI+3 cycles )"]
    #[inline(always)]
    pub fn arfi(&self) -> ARFI_R {
        ARFI_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Initialization Auto-Refresh Count"]
    #[inline(always)]
    pub fn arfc(&self) -> ARFC_R {
        ARFC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - Initialization Precharge Cycle Count (PRC+3 cycles)"]
    #[inline(always)]
    pub fn prc(&self) -> PRC_R {
        PRC_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Initialization Auto-Refresh Interval (ARFI+3 cycles )"]
    #[inline(always)]
    #[must_use]
    pub fn arfi(&mut self) -> ARFI_W<0> {
        ARFI_W::new(self)
    }
    #[doc = "Bits 4:7 - Initialization Auto-Refresh Count"]
    #[inline(always)]
    #[must_use]
    pub fn arfc(&mut self) -> ARFC_W<4> {
        ARFC_W::new(self)
    }
    #[doc = "Bits 8:10 - Initialization Precharge Cycle Count (PRC+3 cycles)"]
    #[inline(always)]
    #[must_use]
    pub fn prc(&mut self) -> PRC_W<8> {
        PRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDRAM Initialization Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdir](index.html) module"]
pub struct SDIR_SPEC;
impl crate::RegisterSpec for SDIR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sdir::R](R) reader structure"]
impl crate::Readable for SDIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdir::W](W) writer structure"]
impl crate::Writable for SDIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDIR to value 0x10"]
impl crate::Resettable for SDIR_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
