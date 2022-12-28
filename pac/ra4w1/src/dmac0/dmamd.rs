#[doc = "Register `DMAMD` reader"]
pub struct R(crate::R<DMAMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAMD` writer"]
pub struct W(crate::W<DMAMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAMD_SPEC>;
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
impl From<crate::W<DMAMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DARA` reader - Destination Address Extended Repeat Area Specifies the extended repeat area on the destination address. For details on the settings."]
pub type DARA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DARA` writer - Destination Address Extended Repeat Area Specifies the extended repeat area on the destination address. For details on the settings."]
pub type DARA_W<'a, const O: u8> = crate::FieldWriter<'a, u16, DMAMD_SPEC, u8, u8, 5, O>;
#[doc = "Field `DM` reader - Destination Address Update Mode"]
pub type DM_R = crate::FieldReader<u8, DM_A>;
#[doc = "Destination Address Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DM_A {
    #[doc = "0: Fixed address"]
    _00 = 0,
    #[doc = "1: Offset addition"]
    _01 = 1,
    #[doc = "2: Incremented address"]
    _10 = 2,
    #[doc = "3: Decremented address."]
    _11 = 3,
}
impl From<DM_A> for u8 {
    #[inline(always)]
    fn from(variant: DM_A) -> Self {
        variant as _
    }
}
impl DM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DM_A {
        match self.bits {
            0 => DM_A::_00,
            1 => DM_A::_01,
            2 => DM_A::_10,
            3 => DM_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DM_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DM_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DM_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DM_A::_11
    }
}
#[doc = "Field `DM` writer - Destination Address Update Mode"]
pub type DM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, DMAMD_SPEC, u8, DM_A, 2, O>;
impl<'a, const O: u8> DM_W<'a, O> {
    #[doc = "Fixed address"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(DM_A::_00)
    }
    #[doc = "Offset addition"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(DM_A::_01)
    }
    #[doc = "Incremented address"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DM_A::_10)
    }
    #[doc = "Decremented address."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(DM_A::_11)
    }
}
#[doc = "Field `SARA` reader - Source Address Extended Repeat Area Specifies the extended repeat area on the source address. For details on the settings."]
pub type SARA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SARA` writer - Source Address Extended Repeat Area Specifies the extended repeat area on the source address. For details on the settings."]
pub type SARA_W<'a, const O: u8> = crate::FieldWriter<'a, u16, DMAMD_SPEC, u8, u8, 5, O>;
#[doc = "Field `SM` reader - Source Address Update Mode"]
pub type SM_R = crate::FieldReader<u8, SM_A>;
#[doc = "Source Address Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SM_A {
    #[doc = "0: Fixed address"]
    _00 = 0,
    #[doc = "1: Offset addition"]
    _01 = 1,
    #[doc = "2: Incremented address"]
    _10 = 2,
    #[doc = "3: Decremented address."]
    _11 = 3,
}
impl From<SM_A> for u8 {
    #[inline(always)]
    fn from(variant: SM_A) -> Self {
        variant as _
    }
}
impl SM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SM_A {
        match self.bits {
            0 => SM_A::_00,
            1 => SM_A::_01,
            2 => SM_A::_10,
            3 => SM_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SM_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SM_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SM_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SM_A::_11
    }
}
#[doc = "Field `SM` writer - Source Address Update Mode"]
pub type SM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, DMAMD_SPEC, u8, SM_A, 2, O>;
impl<'a, const O: u8> SM_W<'a, O> {
    #[doc = "Fixed address"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(SM_A::_00)
    }
    #[doc = "Offset addition"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(SM_A::_01)
    }
    #[doc = "Incremented address"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SM_A::_10)
    }
    #[doc = "Decremented address."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(SM_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:4 - Destination Address Extended Repeat Area Specifies the extended repeat area on the destination address. For details on the settings."]
    #[inline(always)]
    pub fn dara(&self) -> DARA_R {
        DARA_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - Destination Address Update Mode"]
    #[inline(always)]
    pub fn dm(&self) -> DM_R {
        DM_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - Source Address Extended Repeat Area Specifies the extended repeat area on the source address. For details on the settings."]
    #[inline(always)]
    pub fn sara(&self) -> SARA_R {
        SARA_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 14:15 - Source Address Update Mode"]
    #[inline(always)]
    pub fn sm(&self) -> SM_R {
        SM_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Destination Address Extended Repeat Area Specifies the extended repeat area on the destination address. For details on the settings."]
    #[inline(always)]
    #[must_use]
    pub fn dara(&mut self) -> DARA_W<0> {
        DARA_W::new(self)
    }
    #[doc = "Bits 6:7 - Destination Address Update Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dm(&mut self) -> DM_W<6> {
        DM_W::new(self)
    }
    #[doc = "Bits 8:12 - Source Address Extended Repeat Area Specifies the extended repeat area on the source address. For details on the settings."]
    #[inline(always)]
    #[must_use]
    pub fn sara(&mut self) -> SARA_W<8> {
        SARA_W::new(self)
    }
    #[doc = "Bits 14:15 - Source Address Update Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sm(&mut self) -> SM_W<14> {
        SM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Address Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamd](index.html) module"]
pub struct DMAMD_SPEC;
impl crate::RegisterSpec for DMAMD_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dmamd::R](R) reader structure"]
impl crate::Readable for DMAMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmamd::W](W) writer structure"]
impl crate::Writable for DMAMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAMD to value 0"]
impl crate::Resettable for DMAMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
