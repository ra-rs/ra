#[doc = "Register `FSUACR` reader"]
pub struct R(crate::R<FSUACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSUACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSUACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSUACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSUACR` writer"]
pub struct W(crate::W<FSUACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSUACR_SPEC>;
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
impl From<crate::W<FSUACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSUACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAS` reader - Startup Area Select"]
pub type SAS_R = crate::FieldReader<u8, SAS_A>;
#[doc = "Startup Area Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SAS_A {
    #[doc = "0: Startup area is selected by BTFLG bit"]
    _00 = 0,
    #[doc = "1: Startup area is selected by BTFLG bit"]
    _01 = 1,
    #[doc = "2: Startup area is temporarily switched to the default area (block 0)"]
    _10 = 2,
    #[doc = "3: Startup area is temporarily switched to the alternate area (block 1)."]
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
    pub fn variant(&self) -> SAS_A {
        match self.bits {
            0 => SAS_A::_00,
            1 => SAS_A::_01,
            2 => SAS_A::_10,
            3 => SAS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SAS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SAS_A::_01
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
pub type SAS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, FSUACR_SPEC, u8, SAS_A, 2, O>;
impl<'a, const O: u8> SAS_W<'a, O> {
    #[doc = "Startup area is selected by BTFLG bit"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(SAS_A::_00)
    }
    #[doc = "Startup area is selected by BTFLG bit"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(SAS_A::_01)
    }
    #[doc = "Startup area is temporarily switched to the default area (block 0)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SAS_A::_10)
    }
    #[doc = "Startup area is temporarily switched to the alternate area (block 1)."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(SAS_A::_11)
    }
}
#[doc = "Field `KEY` writer - Key Code"]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u16, FSUACR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:1 - Startup Area Select"]
    #[inline(always)]
    pub fn sas(&self) -> SAS_R {
        SAS_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Startup Area Select"]
    #[inline(always)]
    #[must_use]
    pub fn sas(&mut self) -> SAS_W<0> {
        SAS_W::new(self)
    }
    #[doc = "Bits 8:15 - Key Code"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<8> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Startup Area Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsuacr](index.html) module"]
pub struct FSUACR_SPEC;
impl crate::RegisterSpec for FSUACR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [fsuacr::R](R) reader structure"]
impl crate::Readable for FSUACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsuacr::W](W) writer structure"]
impl crate::Writable for FSUACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSUACR to value 0"]
impl crate::Resettable for FSUACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
