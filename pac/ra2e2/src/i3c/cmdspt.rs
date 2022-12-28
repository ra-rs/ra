#[doc = "Register `CMDSPT` reader"]
pub struct R(crate::R<CMDSPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMDSPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMDSPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMDSPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMDSPT` writer"]
pub struct W(crate::W<CMDSPT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMDSPT_SPEC>;
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
impl From<crate::W<CMDSPT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMDSPT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MRTTIM` reader - Maximum Read Turnaround Time"]
pub type MRTTIM_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MRTTIM` writer - Maximum Read Turnaround Time"]
pub type MRTTIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMDSPT_SPEC, u32, u32, 24, O>;
#[doc = "Field `MRTE` reader - Maximum Read Turnaround Time Enable"]
pub type MRTE_R = crate::BitReader<MRTE_A>;
#[doc = "Maximum Read Turnaround Time Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MRTE_A {
    #[doc = "0: Disables transmission of the Maximum Read Turnaround Time. (GETMXDS Format 1: Without Turnaround)"]
    _0 = 0,
    #[doc = "1: Enables transmission of the Maximum Read Turnaround Time. (GETMXDS Format 2: With Turnaround)"]
    _1 = 1,
}
impl From<MRTE_A> for bool {
    #[inline(always)]
    fn from(variant: MRTE_A) -> Self {
        variant as u8 != 0
    }
}
impl MRTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MRTE_A {
        match self.bits {
            false => MRTE_A::_0,
            true => MRTE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MRTE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MRTE_A::_1
    }
}
#[doc = "Field `MRTE` writer - Maximum Read Turnaround Time Enable"]
pub type MRTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMDSPT_SPEC, MRTE_A, O>;
impl<'a, const O: u8> MRTE_W<'a, O> {
    #[doc = "Disables transmission of the Maximum Read Turnaround Time. (GETMXDS Format 1: Without Turnaround)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MRTE_A::_0)
    }
    #[doc = "Enables transmission of the Maximum Read Turnaround Time. (GETMXDS Format 2: With Turnaround)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MRTE_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:23 - Maximum Read Turnaround Time"]
    #[inline(always)]
    pub fn mrttim(&self) -> MRTTIM_R {
        MRTTIM_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 31 - Maximum Read Turnaround Time Enable"]
    #[inline(always)]
    pub fn mrte(&self) -> MRTE_R {
        MRTE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Maximum Read Turnaround Time"]
    #[inline(always)]
    #[must_use]
    pub fn mrttim(&mut self) -> MRTTIM_W<0> {
        MRTTIM_W::new(self)
    }
    #[doc = "Bit 31 - Maximum Read Turnaround Time Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mrte(&mut self) -> MRTE_W<31> {
        MRTE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CCC Max Data Speed T (Turnaround) Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdspt](index.html) module"]
pub struct CMDSPT_SPEC;
impl crate::RegisterSpec for CMDSPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmdspt::R](R) reader structure"]
impl crate::Readable for CMDSPT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmdspt::W](W) writer structure"]
impl crate::Writable for CMDSPT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMDSPT to value 0"]
impl crate::Resettable for CMDSPT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
