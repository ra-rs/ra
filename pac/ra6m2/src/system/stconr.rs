#[doc = "Register `STCONR` reader"]
pub struct R(crate::R<STCONR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STCONR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STCONR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STCONR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STCONR` writer"]
pub struct W(crate::W<STCONR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STCONR_SPEC>;
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
impl From<crate::W<STCONR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STCONR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STCON` reader - SSTBY condition bit"]
pub type STCON_R = crate::FieldReader<u8, STCON_A>;
#[doc = "SSTBY condition bit\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STCON_A {
    #[doc = "0: set this value in case of transferring to Software Standby Mode in using HOCO."]
    _00 = 0,
    #[doc = "3: set this value in case of transferring to Software Standby Mode in using expect for HOCO."]
    _11 = 3,
}
impl From<STCON_A> for u8 {
    #[inline(always)]
    fn from(variant: STCON_A) -> Self {
        variant as _
    }
}
impl STCON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STCON_A> {
        match self.bits {
            0 => Some(STCON_A::_00),
            3 => Some(STCON_A::_11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == STCON_A::_00
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == STCON_A::_11
    }
}
#[doc = "Field `STCON` writer - SSTBY condition bit"]
pub type STCON_W<'a, const O: u8> = crate::FieldWriter<'a, u8, STCONR_SPEC, u8, STCON_A, 2, O>;
impl<'a, const O: u8> STCON_W<'a, O> {
    #[doc = "set this value in case of transferring to Software Standby Mode in using HOCO."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(STCON_A::_00)
    }
    #[doc = "set this value in case of transferring to Software Standby Mode in using expect for HOCO."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(STCON_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - SSTBY condition bit"]
    #[inline(always)]
    pub fn stcon(&self) -> STCON_R {
        STCON_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - SSTBY condition bit"]
    #[inline(always)]
    #[must_use]
    pub fn stcon(&mut self) -> STCON_W<0> {
        STCON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Standby Condition Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stconr](index.html) module"]
pub struct STCONR_SPEC;
impl crate::RegisterSpec for STCONR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [stconr::R](R) reader structure"]
impl crate::Readable for STCONR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stconr::W](W) writer structure"]
impl crate::Writable for STCONR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STCONR to value 0xc3"]
impl crate::Resettable for STCONR_SPEC {
    const RESET_VALUE: Self::Ux = 0xc3;
}
