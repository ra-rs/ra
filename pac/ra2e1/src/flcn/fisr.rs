#[doc = "Register `FISR` reader"]
pub struct R(crate::R<FISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FISR` writer"]
pub struct W(crate::W<FISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FISR_SPEC>;
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
impl From<crate::W<FISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCKA` reader - Flash-IF Clock Notification"]
pub type PCKA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCKA` writer - Flash-IF Clock Notification"]
pub type PCKA_W<'a, const O: u8> = crate::FieldWriter<'a, u8, FISR_SPEC, u8, u8, 6, O>;
#[doc = "Field `SAS` reader - Startup Area Select"]
pub type SAS_R = crate::FieldReader<u8, SAS_A>;
#[doc = "Startup Area Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SAS_A {
    #[doc = "2: The startup area is switched to the default area temporarily"]
    _10 = 2,
    #[doc = "3: The startup area is switched to the alternate area temporarily."]
    _11 = 3,
}
impl From<SAS_A> for u8 {
    #[inline(always)]
    fn from(variant: SAS_A) -> Self {
        variant as _
    }
}
impl SAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SAS_A> {
        match self.bits {
            2 => Some(SAS_A::_10),
            3 => Some(SAS_A::_11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SAS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SAS_A::_11
    }
}
#[doc = "Field `SAS` writer - Startup Area Select"]
pub type SAS_W<'a, const O: u8> = crate::FieldWriter<'a, u8, FISR_SPEC, u8, SAS_A, 2, O>;
impl<'a, const O: u8> SAS_W<'a, O> {
    #[doc = "The startup area is switched to the default area temporarily"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SAS_A::_10)
    }
    #[doc = "The startup area is switched to the alternate area temporarily."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(SAS_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:5 - Flash-IF Clock Notification"]
    #[inline(always)]
    pub fn pcka(&self) -> PCKA_R {
        PCKA_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Startup Area Select"]
    #[inline(always)]
    pub fn sas(&self) -> SAS_R {
        SAS_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Flash-IF Clock Notification"]
    #[inline(always)]
    pub fn pcka(&mut self) -> PCKA_W<0> {
        PCKA_W::new(self)
    }
    #[doc = "Bits 6:7 - Startup Area Select"]
    #[inline(always)]
    pub fn sas(&mut self) -> SAS_W<6> {
        SAS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Initial Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fisr](index.html) module"]
pub struct FISR_SPEC;
impl crate::RegisterSpec for FISR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fisr::R](R) reader structure"]
impl crate::Readable for FISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fisr::W](W) writer structure"]
impl crate::Writable for FISR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FISR to value 0"]
impl crate::Resettable for FISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
