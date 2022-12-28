#[doc = "Register `CEACTST` reader"]
pub struct R(crate::R<CEACTST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CEACTST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CEACTST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CEACTST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CEACTST` writer"]
pub struct W(crate::W<CEACTST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CEACTST_SPEC>;
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
impl From<crate::W<CEACTST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CEACTST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACTST` reader - Activity State"]
pub type ACTST_R = crate::FieldReader<u8, ACTST_A>;
#[doc = "Activity State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACTST_A {
    #[doc = "1: ENTAS0 (1µs: Latency-free operation)"]
    _0X1 = 1,
    #[doc = "2: ENTAS1 (100 µs)"]
    _0X2 = 2,
    #[doc = "4: ENTAS2 (2 ms)"]
    _0X4 = 4,
    #[doc = "8: ENTAS3 (50 ms: Lowest-activity operation)"]
    _0X8 = 8,
}
impl From<ACTST_A> for u8 {
    #[inline(always)]
    fn from(variant: ACTST_A) -> Self {
        variant as _
    }
}
impl ACTST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ACTST_A> {
        match self.bits {
            1 => Some(ACTST_A::_0X1),
            2 => Some(ACTST_A::_0X2),
            4 => Some(ACTST_A::_0X4),
            8 => Some(ACTST_A::_0X8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X1`"]
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == ACTST_A::_0X1
    }
    #[doc = "Checks if the value of the field is `_0X2`"]
    #[inline(always)]
    pub fn is_0x2(&self) -> bool {
        *self == ACTST_A::_0X2
    }
    #[doc = "Checks if the value of the field is `_0X4`"]
    #[inline(always)]
    pub fn is_0x4(&self) -> bool {
        *self == ACTST_A::_0X4
    }
    #[doc = "Checks if the value of the field is `_0X8`"]
    #[inline(always)]
    pub fn is_0x8(&self) -> bool {
        *self == ACTST_A::_0X8
    }
}
#[doc = "Field `ACTST` writer - Activity State"]
pub type ACTST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CEACTST_SPEC, u8, ACTST_A, 4, O>;
impl<'a, const O: u8> ACTST_W<'a, O> {
    #[doc = "ENTAS0 (1µs: Latency-free operation)"]
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut W {
        self.variant(ACTST_A::_0X1)
    }
    #[doc = "ENTAS1 (100 µs)"]
    #[inline(always)]
    pub fn _0x2(self) -> &'a mut W {
        self.variant(ACTST_A::_0X2)
    }
    #[doc = "ENTAS2 (2 ms)"]
    #[inline(always)]
    pub fn _0x4(self) -> &'a mut W {
        self.variant(ACTST_A::_0X4)
    }
    #[doc = "ENTAS3 (50 ms: Lowest-activity operation)"]
    #[inline(always)]
    pub fn _0x8(self) -> &'a mut W {
        self.variant(ACTST_A::_0X8)
    }
}
impl R {
    #[doc = "Bits 0:3 - Activity State"]
    #[inline(always)]
    pub fn actst(&self) -> ACTST_R {
        ACTST_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Activity State"]
    #[inline(always)]
    #[must_use]
    pub fn actst(&mut self) -> ACTST_W<0> {
        ACTST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CCC Enter Activity State Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ceactst](index.html) module"]
pub struct CEACTST_SPEC;
impl crate::RegisterSpec for CEACTST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ceactst::R](R) reader structure"]
impl crate::Readable for CEACTST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ceactst::W](W) writer structure"]
impl crate::Writable for CEACTST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CEACTST to value 0"]
impl crate::Resettable for CEACTST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
