#[doc = "Register `SSISCR` reader"]
pub struct R(crate::R<SSISCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSISCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSISCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSISCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSISCR` writer"]
pub struct W(crate::W<SSISCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSISCR_SPEC>;
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
impl From<crate::W<SSISCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSISCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDFS` reader - RDF Setting Condition Select"]
pub type RDFS_R = crate::FieldReader<u8, RDFS_A>;
#[doc = "RDF Setting Condition Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RDFS_A {
    #[doc = "0: SSIFRDR has one stage or more data size"]
    _00000 = 0,
    #[doc = "1: SSIFRDR has two stages or more data size (snip)"]
    _00001 = 1,
    #[doc = "30: SSIFRDR has thirty-one stages or more data size"]
    _11110 = 30,
    #[doc = "31: SSIFRDR has thirty-two stages or more data size."]
    _11111 = 31,
}
impl From<RDFS_A> for u8 {
    #[inline(always)]
    fn from(variant: RDFS_A) -> Self {
        variant as _
    }
}
impl RDFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RDFS_A> {
        match self.bits {
            0 => Some(RDFS_A::_00000),
            1 => Some(RDFS_A::_00001),
            30 => Some(RDFS_A::_11110),
            31 => Some(RDFS_A::_11111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00000`"]
    #[inline(always)]
    pub fn is_00000(&self) -> bool {
        *self == RDFS_A::_00000
    }
    #[doc = "Checks if the value of the field is `_00001`"]
    #[inline(always)]
    pub fn is_00001(&self) -> bool {
        *self == RDFS_A::_00001
    }
    #[doc = "Checks if the value of the field is `_11110`"]
    #[inline(always)]
    pub fn is_11110(&self) -> bool {
        *self == RDFS_A::_11110
    }
    #[doc = "Checks if the value of the field is `_11111`"]
    #[inline(always)]
    pub fn is_11111(&self) -> bool {
        *self == RDFS_A::_11111
    }
}
#[doc = "Field `RDFS` writer - RDF Setting Condition Select"]
pub type RDFS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SSISCR_SPEC, u8, RDFS_A, 5, O>;
impl<'a, const O: u8> RDFS_W<'a, O> {
    #[doc = "SSIFRDR has one stage or more data size"]
    #[inline(always)]
    pub fn _00000(self) -> &'a mut W {
        self.variant(RDFS_A::_00000)
    }
    #[doc = "SSIFRDR has two stages or more data size (snip)"]
    #[inline(always)]
    pub fn _00001(self) -> &'a mut W {
        self.variant(RDFS_A::_00001)
    }
    #[doc = "SSIFRDR has thirty-one stages or more data size"]
    #[inline(always)]
    pub fn _11110(self) -> &'a mut W {
        self.variant(RDFS_A::_11110)
    }
    #[doc = "SSIFRDR has thirty-two stages or more data size."]
    #[inline(always)]
    pub fn _11111(self) -> &'a mut W {
        self.variant(RDFS_A::_11111)
    }
}
#[doc = "Field `TDES` reader - TDE Setting Condition Select"]
pub type TDES_R = crate::FieldReader<u8, TDES_A>;
#[doc = "TDE Setting Condition Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TDES_A {
    #[doc = "0: SSIFTDR has one stage or more free space"]
    _00000 = 0,
    #[doc = "1: SSIFTDR has two stages or more free space (snip)"]
    _00001 = 1,
    #[doc = "30: SSIFTDR has thirty-one stages or more free space"]
    _11110 = 30,
    #[doc = "31: SSIFTDR has thirty-two stages or more free space."]
    _11111 = 31,
}
impl From<TDES_A> for u8 {
    #[inline(always)]
    fn from(variant: TDES_A) -> Self {
        variant as _
    }
}
impl TDES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TDES_A> {
        match self.bits {
            0 => Some(TDES_A::_00000),
            1 => Some(TDES_A::_00001),
            30 => Some(TDES_A::_11110),
            31 => Some(TDES_A::_11111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00000`"]
    #[inline(always)]
    pub fn is_00000(&self) -> bool {
        *self == TDES_A::_00000
    }
    #[doc = "Checks if the value of the field is `_00001`"]
    #[inline(always)]
    pub fn is_00001(&self) -> bool {
        *self == TDES_A::_00001
    }
    #[doc = "Checks if the value of the field is `_11110`"]
    #[inline(always)]
    pub fn is_11110(&self) -> bool {
        *self == TDES_A::_11110
    }
    #[doc = "Checks if the value of the field is `_11111`"]
    #[inline(always)]
    pub fn is_11111(&self) -> bool {
        *self == TDES_A::_11111
    }
}
#[doc = "Field `TDES` writer - TDE Setting Condition Select"]
pub type TDES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SSISCR_SPEC, u8, TDES_A, 5, O>;
impl<'a, const O: u8> TDES_W<'a, O> {
    #[doc = "SSIFTDR has one stage or more free space"]
    #[inline(always)]
    pub fn _00000(self) -> &'a mut W {
        self.variant(TDES_A::_00000)
    }
    #[doc = "SSIFTDR has two stages or more free space (snip)"]
    #[inline(always)]
    pub fn _00001(self) -> &'a mut W {
        self.variant(TDES_A::_00001)
    }
    #[doc = "SSIFTDR has thirty-one stages or more free space"]
    #[inline(always)]
    pub fn _11110(self) -> &'a mut W {
        self.variant(TDES_A::_11110)
    }
    #[doc = "SSIFTDR has thirty-two stages or more free space."]
    #[inline(always)]
    pub fn _11111(self) -> &'a mut W {
        self.variant(TDES_A::_11111)
    }
}
impl R {
    #[doc = "Bits 0:4 - RDF Setting Condition Select"]
    #[inline(always)]
    pub fn rdfs(&self) -> RDFS_R {
        RDFS_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - TDE Setting Condition Select"]
    #[inline(always)]
    pub fn tdes(&self) -> TDES_R {
        TDES_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - RDF Setting Condition Select"]
    #[inline(always)]
    #[must_use]
    pub fn rdfs(&mut self) -> RDFS_W<0> {
        RDFS_W::new(self)
    }
    #[doc = "Bits 8:12 - TDE Setting Condition Select"]
    #[inline(always)]
    #[must_use]
    pub fn tdes(&mut self) -> TDES_W<8> {
        TDES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssiscr](index.html) module"]
pub struct SSISCR_SPEC;
impl crate::RegisterSpec for SSISCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssiscr::R](R) reader structure"]
impl crate::Readable for SSISCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssiscr::W](W) writer structure"]
impl crate::Writable for SSISCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSISCR to value 0"]
impl crate::Resettable for SSISCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
