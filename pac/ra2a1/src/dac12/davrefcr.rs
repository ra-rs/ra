#[doc = "Register `DAVREFCR` reader"]
pub struct R(crate::R<DAVREFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAVREFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAVREFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAVREFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAVREFCR` writer"]
pub struct W(crate::W<DAVREFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAVREFCR_SPEC>;
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
impl From<crate::W<DAVREFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAVREFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REF` reader - D/A Reference Voltage Select"]
pub type REF_R = crate::FieldReader<u8, REF_A>;
#[doc = "D/A Reference Voltage Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REF_A {
    #[doc = "0: Not selected"]
    _000 = 0,
    #[doc = "1: AVCC0/AVSS0"]
    _001 = 1,
    #[doc = "3: Internal reference voltage/AVSS0"]
    _011 = 3,
    #[doc = "6: VREFH/VREFL"]
    _110 = 6,
}
impl From<REF_A> for u8 {
    #[inline(always)]
    fn from(variant: REF_A) -> Self {
        variant as _
    }
}
impl REF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REF_A> {
        match self.bits {
            0 => Some(REF_A::_000),
            1 => Some(REF_A::_001),
            3 => Some(REF_A::_011),
            6 => Some(REF_A::_110),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == REF_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == REF_A::_001
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == REF_A::_011
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == REF_A::_110
    }
}
#[doc = "Field `REF` writer - D/A Reference Voltage Select"]
pub type REF_W<'a, const O: u8> = crate::FieldWriter<'a, u8, DAVREFCR_SPEC, u8, REF_A, 3, O>;
impl<'a, const O: u8> REF_W<'a, O> {
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(REF_A::_000)
    }
    #[doc = "AVCC0/AVSS0"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(REF_A::_001)
    }
    #[doc = "Internal reference voltage/AVSS0"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(REF_A::_011)
    }
    #[doc = "VREFH/VREFL"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(REF_A::_110)
    }
}
impl R {
    #[doc = "Bits 0:2 - D/A Reference Voltage Select"]
    #[inline(always)]
    pub fn ref_(&self) -> REF_R {
        REF_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - D/A Reference Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn ref_(&mut self) -> REF_W<0> {
        REF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "D/A VREF Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [davrefcr](index.html) module"]
pub struct DAVREFCR_SPEC;
impl crate::RegisterSpec for DAVREFCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [davrefcr::R](R) reader structure"]
impl crate::Readable for DAVREFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [davrefcr::W](W) writer structure"]
impl crate::Writable for DAVREFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAVREFCR to value 0"]
impl crate::Resettable for DAVREFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
