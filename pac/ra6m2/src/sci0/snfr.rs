#[doc = "Register `SNFR` reader"]
pub struct R(crate::R<SNFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SNFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SNFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SNFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SNFR` writer"]
pub struct W(crate::W<SNFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SNFR_SPEC>;
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
impl From<crate::W<SNFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SNFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NFCS` reader - Noise Filter Clock Select"]
pub type NFCS_R = crate::FieldReader<u8, NFCS_A>;
#[doc = "Noise Filter Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NFCS_A {
    #[doc = "0: The clock signal divided by 1 is used with the noise filter.(In asynchronous mode)"]
    _000 = 0,
    #[doc = "1: The clock signal divided by 1 is used with the noise filter.(In simple I2C mode)"]
    _001 = 1,
    #[doc = "2: The clock signal divided by 2 is used with the noise filter.(In simple I2C mode)"]
    _010 = 2,
    #[doc = "3: The clock signal divided by 4 is used with the noise filter.(In simple I2C mode)"]
    _011 = 3,
    #[doc = "4: The clock signal divided by 8 is used with the noise filter.(In simple I2C mode)"]
    _100 = 4,
}
impl From<NFCS_A> for u8 {
    #[inline(always)]
    fn from(variant: NFCS_A) -> Self {
        variant as _
    }
}
impl NFCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NFCS_A> {
        match self.bits {
            0 => Some(NFCS_A::_000),
            1 => Some(NFCS_A::_001),
            2 => Some(NFCS_A::_010),
            3 => Some(NFCS_A::_011),
            4 => Some(NFCS_A::_100),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == NFCS_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == NFCS_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == NFCS_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == NFCS_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == NFCS_A::_100
    }
}
#[doc = "Field `NFCS` writer - Noise Filter Clock Select"]
pub type NFCS_W<'a, const O: u8> = crate::FieldWriter<'a, u8, SNFR_SPEC, u8, NFCS_A, 3, O>;
impl<'a, const O: u8> NFCS_W<'a, O> {
    #[doc = "The clock signal divided by 1 is used with the noise filter.(In asynchronous mode)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(NFCS_A::_000)
    }
    #[doc = "The clock signal divided by 1 is used with the noise filter.(In simple I2C mode)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(NFCS_A::_001)
    }
    #[doc = "The clock signal divided by 2 is used with the noise filter.(In simple I2C mode)"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(NFCS_A::_010)
    }
    #[doc = "The clock signal divided by 4 is used with the noise filter.(In simple I2C mode)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(NFCS_A::_011)
    }
    #[doc = "The clock signal divided by 8 is used with the noise filter.(In simple I2C mode)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(NFCS_A::_100)
    }
}
impl R {
    #[doc = "Bits 0:2 - Noise Filter Clock Select"]
    #[inline(always)]
    pub fn nfcs(&self) -> NFCS_R {
        NFCS_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Noise Filter Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn nfcs(&mut self) -> NFCS_W<0> {
        NFCS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Noise Filter Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [snfr](index.html) module"]
pub struct SNFR_SPEC;
impl crate::RegisterSpec for SNFR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [snfr::R](R) reader structure"]
impl crate::Readable for SNFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [snfr::W](W) writer structure"]
impl crate::Writable for SNFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SNFR to value 0"]
impl crate::Resettable for SNFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
