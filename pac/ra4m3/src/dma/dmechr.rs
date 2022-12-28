#[doc = "Register `DMECHR` reader"]
pub struct R(crate::R<DMECHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMECHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMECHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMECHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMECHR` writer"]
pub struct W(crate::W<DMECHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMECHR_SPEC>;
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
impl From<crate::W<DMECHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMECHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMECH` reader - DMAC Error channel"]
pub type DMECH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMECHSAM` reader - DMAC Error channel Security Attribution Monitor"]
pub type DMECHSAM_R = crate::BitReader<DMECHSAM_A>;
#[doc = "DMAC Error channel Security Attribution Monitor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMECHSAM_A {
    #[doc = "0: secure channel"]
    _0 = 0,
    #[doc = "1: non-secure channel"]
    _1 = 1,
}
impl From<DMECHSAM_A> for bool {
    #[inline(always)]
    fn from(variant: DMECHSAM_A) -> Self {
        variant as u8 != 0
    }
}
impl DMECHSAM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMECHSAM_A {
        match self.bits {
            false => DMECHSAM_A::_0,
            true => DMECHSAM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMECHSAM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMECHSAM_A::_1
    }
}
#[doc = "Field `DMESTA` reader - DMAC Error Status"]
pub type DMESTA_R = crate::BitReader<DMESTA_A>;
#[doc = "DMAC Error Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMESTA_A {
    #[doc = "0: No DMA transfer error occurred"]
    _0 = 0,
    #[doc = "1: DMA transfer error occurred"]
    _1 = 1,
}
impl From<DMESTA_A> for bool {
    #[inline(always)]
    fn from(variant: DMESTA_A) -> Self {
        variant as u8 != 0
    }
}
impl DMESTA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMESTA_A {
        match self.bits {
            false => DMESTA_A::_0,
            true => DMESTA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMESTA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMESTA_A::_1
    }
}
#[doc = "Field `DMESTA` writer - DMAC Error Status"]
pub type DMESTA_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMECHR_SPEC, DMESTA_A, O>;
impl<'a, const O: u8> DMESTA_W<'a, O> {
    #[doc = "No DMA transfer error occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMESTA_A::_0)
    }
    #[doc = "DMA transfer error occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMESTA_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - DMAC Error channel"]
    #[inline(always)]
    pub fn dmech(&self) -> DMECH_R {
        DMECH_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - DMAC Error channel Security Attribution Monitor"]
    #[inline(always)]
    pub fn dmechsam(&self) -> DMECHSAM_R {
        DMECHSAM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - DMAC Error Status"]
    #[inline(always)]
    pub fn dmesta(&self) -> DMESTA_R {
        DMESTA_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - DMAC Error Status"]
    #[inline(always)]
    #[must_use]
    pub fn dmesta(&mut self) -> DMESTA_W<16> {
        DMESTA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Error Channel Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmechr](index.html) module"]
pub struct DMECHR_SPEC;
impl crate::RegisterSpec for DMECHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmechr::R](R) reader structure"]
impl crate::Readable for DMECHR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmechr::W](W) writer structure"]
impl crate::Writable for DMECHR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMECHR to value 0"]
impl crate::Resettable for DMECHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
