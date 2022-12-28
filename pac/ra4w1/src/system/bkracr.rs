#[doc = "Register `BKRACR` reader"]
pub struct R(crate::R<BKRACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BKRACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BKRACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BKRACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BKRACR` writer"]
pub struct W(crate::W<BKRACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BKRACR_SPEC>;
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
impl From<crate::W<BKRACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BKRACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BKRACS` reader - Backup Register Access Control Register"]
pub type BKRACS_R = crate::FieldReader<u8, BKRACS_A>;
#[doc = "Backup Register Access Control Register\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BKRACS_A {
    #[doc = "0: Access control disable. When System clock source is SOSC or LOCO."]
    _000 = 0,
    #[doc = "6: Access control enable. System clock source is other than SOSC or LOCO."]
    _110 = 6,
}
impl From<BKRACS_A> for u8 {
    #[inline(always)]
    fn from(variant: BKRACS_A) -> Self {
        variant as _
    }
}
impl BKRACS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BKRACS_A> {
        match self.bits {
            0 => Some(BKRACS_A::_000),
            6 => Some(BKRACS_A::_110),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == BKRACS_A::_000
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == BKRACS_A::_110
    }
}
#[doc = "Field `BKRACS` writer - Backup Register Access Control Register"]
pub type BKRACS_W<'a, const O: u8> = crate::FieldWriter<'a, u8, BKRACR_SPEC, u8, BKRACS_A, 3, O>;
impl<'a, const O: u8> BKRACS_W<'a, O> {
    #[doc = "Access control disable. When System clock source is SOSC or LOCO."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(BKRACS_A::_000)
    }
    #[doc = "Access control enable. System clock source is other than SOSC or LOCO."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(BKRACS_A::_110)
    }
}
impl R {
    #[doc = "Bits 0:2 - Backup Register Access Control Register"]
    #[inline(always)]
    pub fn bkracs(&self) -> BKRACS_R {
        BKRACS_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Backup Register Access Control Register"]
    #[inline(always)]
    #[must_use]
    pub fn bkracs(&mut self) -> BKRACS_W<0> {
        BKRACS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Backup Register Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkracr](index.html) module"]
pub struct BKRACR_SPEC;
impl crate::RegisterSpec for BKRACR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [bkracr::R](R) reader structure"]
impl crate::Readable for BKRACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bkracr::W](W) writer structure"]
impl crate::Writable for BKRACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BKRACR to value 0x06"]
impl crate::Resettable for BKRACR_SPEC {
    const RESET_VALUE: Self::Ux = 0x06;
}
