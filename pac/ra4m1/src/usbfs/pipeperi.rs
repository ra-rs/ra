#[doc = "Register `PIPEPERI` reader"]
pub struct R(crate::R<PIPEPERI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIPEPERI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIPEPERI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIPEPERI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIPEPERI` writer"]
pub struct W(crate::W<PIPEPERI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIPEPERI_SPEC>;
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
impl From<crate::W<PIPEPERI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIPEPERI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IITV` reader - Interval Error Detection Interval Specifies the interval error detection timing for the selected pipe in terms of frames, which is expressed as nth power of 2."]
pub type IITV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IITV` writer - Interval Error Detection Interval Specifies the interval error detection timing for the selected pipe in terms of frames, which is expressed as nth power of 2."]
pub type IITV_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PIPEPERI_SPEC, u8, u8, 3, O>;
#[doc = "Field `IFIS` reader - Isochronous IN Buffer Flush"]
pub type IFIS_R = crate::BitReader<IFIS_A>;
#[doc = "Isochronous IN Buffer Flush\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IFIS_A {
    #[doc = "0: The buffer is not flushed."]
    _0 = 0,
    #[doc = "1: The buffer is flushed."]
    _1 = 1,
}
impl From<IFIS_A> for bool {
    #[inline(always)]
    fn from(variant: IFIS_A) -> Self {
        variant as u8 != 0
    }
}
impl IFIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IFIS_A {
        match self.bits {
            false => IFIS_A::_0,
            true => IFIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IFIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IFIS_A::_1
    }
}
#[doc = "Field `IFIS` writer - Isochronous IN Buffer Flush"]
pub type IFIS_W<'a, const O: u8> = crate::BitWriter<'a, u16, PIPEPERI_SPEC, IFIS_A, O>;
impl<'a, const O: u8> IFIS_W<'a, O> {
    #[doc = "The buffer is not flushed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IFIS_A::_0)
    }
    #[doc = "The buffer is flushed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IFIS_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Interval Error Detection Interval Specifies the interval error detection timing for the selected pipe in terms of frames, which is expressed as nth power of 2."]
    #[inline(always)]
    pub fn iitv(&self) -> IITV_R {
        IITV_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 12 - Isochronous IN Buffer Flush"]
    #[inline(always)]
    pub fn ifis(&self) -> IFIS_R {
        IFIS_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Interval Error Detection Interval Specifies the interval error detection timing for the selected pipe in terms of frames, which is expressed as nth power of 2."]
    #[inline(always)]
    #[must_use]
    pub fn iitv(&mut self) -> IITV_W<0> {
        IITV_W::new(self)
    }
    #[doc = "Bit 12 - Isochronous IN Buffer Flush"]
    #[inline(always)]
    #[must_use]
    pub fn ifis(&mut self) -> IFIS_W<12> {
        IFIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pipe Cycle Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pipeperi](index.html) module"]
pub struct PIPEPERI_SPEC;
impl crate::RegisterSpec for PIPEPERI_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pipeperi::R](R) reader structure"]
impl crate::Readable for PIPEPERI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pipeperi::W](W) writer structure"]
impl crate::Writable for PIPEPERI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PIPEPERI to value 0"]
impl crate::Resettable for PIPEPERI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
