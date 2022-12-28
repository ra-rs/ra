#[doc = "Register `CMDSPW` reader"]
pub struct R(crate::R<CMDSPW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMDSPW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMDSPW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMDSPW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMDSPW` writer"]
pub struct W(crate::W<CMDSPW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMDSPW_SPEC>;
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
impl From<crate::W<CMDSPW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMDSPW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSWDR` reader - Maximum Sustained Write Data Rate"]
pub type MSWDR_R = crate::FieldReader<u8, MSWDR_A>;
#[doc = "Maximum Sustained Write Data Rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSWDR_A {
    #[doc = "0: fscl Max (default value)"]
    _000 = 0,
    #[doc = "1: 8 MHz"]
    _001 = 1,
    #[doc = "2: 6 MHz"]
    _010 = 2,
    #[doc = "3: 4 MHz"]
    _011 = 3,
    #[doc = "4: 2 MHz"]
    _100 = 4,
}
impl From<MSWDR_A> for u8 {
    #[inline(always)]
    fn from(variant: MSWDR_A) -> Self {
        variant as _
    }
}
impl MSWDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MSWDR_A> {
        match self.bits {
            0 => Some(MSWDR_A::_000),
            1 => Some(MSWDR_A::_001),
            2 => Some(MSWDR_A::_010),
            3 => Some(MSWDR_A::_011),
            4 => Some(MSWDR_A::_100),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == MSWDR_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == MSWDR_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == MSWDR_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == MSWDR_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == MSWDR_A::_100
    }
}
#[doc = "Field `MSWDR` writer - Maximum Sustained Write Data Rate"]
pub type MSWDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMDSPW_SPEC, u8, MSWDR_A, 3, O>;
impl<'a, const O: u8> MSWDR_W<'a, O> {
    #[doc = "fscl Max (default value)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(MSWDR_A::_000)
    }
    #[doc = "8 MHz"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(MSWDR_A::_001)
    }
    #[doc = "6 MHz"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(MSWDR_A::_010)
    }
    #[doc = "4 MHz"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(MSWDR_A::_011)
    }
    #[doc = "2 MHz"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(MSWDR_A::_100)
    }
}
impl R {
    #[doc = "Bits 0:2 - Maximum Sustained Write Data Rate"]
    #[inline(always)]
    pub fn mswdr(&self) -> MSWDR_R {
        MSWDR_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Maximum Sustained Write Data Rate"]
    #[inline(always)]
    #[must_use]
    pub fn mswdr(&mut self) -> MSWDR_W<0> {
        MSWDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CCC Max Data Speed W (Write) Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdspw](index.html) module"]
pub struct CMDSPW_SPEC;
impl crate::RegisterSpec for CMDSPW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmdspw::R](R) reader structure"]
impl crate::Readable for CMDSPW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmdspw::W](W) writer structure"]
impl crate::Writable for CMDSPW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMDSPW to value 0"]
impl crate::Resettable for CMDSPW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
