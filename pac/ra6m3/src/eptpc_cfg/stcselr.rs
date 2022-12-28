#[doc = "Register `STCSELR` reader"]
pub struct R(crate::R<STCSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STCSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STCSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STCSELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STCSELR` writer"]
pub struct W(crate::W<STCSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STCSELR_SPEC>;
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
impl From<crate::W<STCSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STCSELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCLKDIV` reader - PCLKA Clock Frequency Division"]
pub type SCLKDIV_R = crate::FieldReader<u8, SCLKDIV_A>;
#[doc = "PCLKA Clock Frequency Division\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SCLKDIV_A {
    #[doc = "1: 1"]
    _001 = 1,
    #[doc = "2: 1/2"]
    _010 = 2,
    #[doc = "3: 1/3"]
    _011 = 3,
    #[doc = "4: 1/4"]
    _100 = 4,
    #[doc = "5: 1/5"]
    _101 = 5,
    #[doc = "6: 1/6"]
    _110 = 6,
}
impl From<SCLKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: SCLKDIV_A) -> Self {
        variant as _
    }
}
impl SCLKDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SCLKDIV_A> {
        match self.bits {
            1 => Some(SCLKDIV_A::_001),
            2 => Some(SCLKDIV_A::_010),
            3 => Some(SCLKDIV_A::_011),
            4 => Some(SCLKDIV_A::_100),
            5 => Some(SCLKDIV_A::_101),
            6 => Some(SCLKDIV_A::_110),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SCLKDIV_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SCLKDIV_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == SCLKDIV_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == SCLKDIV_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == SCLKDIV_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == SCLKDIV_A::_110
    }
}
#[doc = "Field `SCLKDIV` writer - PCLKA Clock Frequency Division"]
pub type SCLKDIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STCSELR_SPEC, u8, SCLKDIV_A, 3, O>;
impl<'a, const O: u8> SCLKDIV_W<'a, O> {
    #[doc = "1"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(SCLKDIV_A::_001)
    }
    #[doc = "1/2"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(SCLKDIV_A::_010)
    }
    #[doc = "1/3"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(SCLKDIV_A::_011)
    }
    #[doc = "1/4"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(SCLKDIV_A::_100)
    }
    #[doc = "1/5"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(SCLKDIV_A::_101)
    }
    #[doc = "1/6"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(SCLKDIV_A::_110)
    }
}
#[doc = "Field `SCLKSEL` reader - STCA Clock Select"]
pub type SCLKSEL_R = crate::FieldReader<u8, SCLKSEL_A>;
#[doc = "STCA Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SCLKSEL_A {
    #[doc = "0: PCLKA clock divided by 1 to 6"]
    _000 = 0,
    #[doc = "2: Input clock from the REF50CK0 pin"]
    _010 = 2,
}
impl From<SCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SCLKSEL_A) -> Self {
        variant as _
    }
}
impl SCLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SCLKSEL_A> {
        match self.bits {
            0 => Some(SCLKSEL_A::_000),
            2 => Some(SCLKSEL_A::_010),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SCLKSEL_A::_000
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SCLKSEL_A::_010
    }
}
#[doc = "Field `SCLKSEL` writer - STCA Clock Select"]
pub type SCLKSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STCSELR_SPEC, u8, SCLKSEL_A, 3, O>;
impl<'a, const O: u8> SCLKSEL_W<'a, O> {
    #[doc = "PCLKA clock divided by 1 to 6"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(SCLKSEL_A::_000)
    }
    #[doc = "Input clock from the REF50CK0 pin"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(SCLKSEL_A::_010)
    }
}
impl R {
    #[doc = "Bits 0:2 - PCLKA Clock Frequency Division"]
    #[inline(always)]
    pub fn sclkdiv(&self) -> SCLKDIV_R {
        SCLKDIV_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - STCA Clock Select"]
    #[inline(always)]
    pub fn sclksel(&self) -> SCLKSEL_R {
        SCLKSEL_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - PCLKA Clock Frequency Division"]
    #[inline(always)]
    #[must_use]
    pub fn sclkdiv(&mut self) -> SCLKDIV_W<0> {
        SCLKDIV_W::new(self)
    }
    #[doc = "Bits 8:10 - STCA Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn sclksel(&mut self) -> SCLKSEL_W<8> {
        SCLKSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "STCA Clock Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stcselr](index.html) module"]
pub struct STCSELR_SPEC;
impl crate::RegisterSpec for STCSELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stcselr::R](R) reader structure"]
impl crate::Readable for STCSELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stcselr::W](W) writer structure"]
impl crate::Writable for STCSELR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STCSELR to value 0x06"]
impl crate::Resettable for STCSELR_SPEC {
    const RESET_VALUE: Self::Ux = 0x06;
}
