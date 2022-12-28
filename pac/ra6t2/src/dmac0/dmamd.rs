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
#[doc = "Field `DARA` reader - Destination Address Extended Repeat Area"]
pub type DARA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DARA` writer - Destination Address Extended Repeat Area"]
pub type DARA_W<'a, const O: u8> = crate::FieldWriter<'a, u16, DMAMD_SPEC, u8, u8, 5, O>;
#[doc = "Field `DADR` reader - Destination Address Update Select After Reload"]
pub type DADR_R = crate::BitReader<DADR_A>;
#[doc = "Destination Address Update Select After Reload\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DADR_A {
    #[doc = "0: Only reloading."]
    _0 = 0,
    #[doc = "1: Add index after reloading."]
    _1 = 1,
}
impl From<DADR_A> for bool {
    #[inline(always)]
    fn from(variant: DADR_A) -> Self {
        variant as u8 != 0
    }
}
impl DADR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DADR_A {
        match self.bits {
            false => DADR_A::_0,
            true => DADR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DADR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DADR_A::_1
    }
}
#[doc = "Field `DADR` writer - Destination Address Update Select After Reload"]
pub type DADR_W<'a, const O: u8> = crate::BitWriter<'a, u16, DMAMD_SPEC, DADR_A, O>;
impl<'a, const O: u8> DADR_W<'a, O> {
    #[doc = "Only reloading."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DADR_A::_0)
    }
    #[doc = "Add index after reloading."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DADR_A::_1)
    }
}
#[doc = "Field `DM` reader - Destination Address Update Mode"]
pub type DM_R = crate::FieldReader<u8, DM_A>;
#[doc = "Destination Address Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DM_A {
    #[doc = "0: Destination address is fixed."]
    _00 = 0,
    #[doc = "1: Offset addition."]
    _01 = 1,
    #[doc = "2: Destination address is incremented."]
    _10 = 2,
    #[doc = "3: Destination address is decremented."]
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
    #[doc = "Destination address is fixed."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(DM_A::_00)
    }
    #[doc = "Offset addition."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(DM_A::_01)
    }
    #[doc = "Destination address is incremented."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DM_A::_10)
    }
    #[doc = "Destination address is decremented."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(DM_A::_11)
    }
}
#[doc = "Field `SARA` reader - Source Address Extended Repeat Area"]
pub type SARA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SARA` writer - Source Address Extended Repeat Area"]
pub type SARA_W<'a, const O: u8> = crate::FieldWriter<'a, u16, DMAMD_SPEC, u8, u8, 5, O>;
#[doc = "Field `SADR` reader - Source Address Update Select After Reload"]
pub type SADR_R = crate::BitReader<SADR_A>;
#[doc = "Source Address Update Select After Reload\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SADR_A {
    #[doc = "0: Only reloading."]
    _0 = 0,
    #[doc = "1: Add index after reloading."]
    _1 = 1,
}
impl From<SADR_A> for bool {
    #[inline(always)]
    fn from(variant: SADR_A) -> Self {
        variant as u8 != 0
    }
}
impl SADR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SADR_A {
        match self.bits {
            false => SADR_A::_0,
            true => SADR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SADR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SADR_A::_1
    }
}
#[doc = "Field `SADR` writer - Source Address Update Select After Reload"]
pub type SADR_W<'a, const O: u8> = crate::BitWriter<'a, u16, DMAMD_SPEC, SADR_A, O>;
impl<'a, const O: u8> SADR_W<'a, O> {
    #[doc = "Only reloading."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SADR_A::_0)
    }
    #[doc = "Add index after reloading."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SADR_A::_1)
    }
}
#[doc = "Field `SM` reader - Source Address Update Mode"]
pub type SM_R = crate::FieldReader<u8, SM_A>;
#[doc = "Source Address Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SM_A {
    #[doc = "0: Source address is fixed."]
    _00 = 0,
    #[doc = "1: Offset addition."]
    _01 = 1,
    #[doc = "2: Source address is incremented."]
    _10 = 2,
    #[doc = "3: Source address is decremented."]
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
    #[doc = "Source address is fixed."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(SM_A::_00)
    }
    #[doc = "Offset addition."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(SM_A::_01)
    }
    #[doc = "Source address is incremented."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SM_A::_10)
    }
    #[doc = "Source address is decremented."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(SM_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:4 - Destination Address Extended Repeat Area"]
    #[inline(always)]
    pub fn dara(&self) -> DARA_R {
        DARA_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Destination Address Update Select After Reload"]
    #[inline(always)]
    pub fn dadr(&self) -> DADR_R {
        DADR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Destination Address Update Mode"]
    #[inline(always)]
    pub fn dm(&self) -> DM_R {
        DM_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - Source Address Extended Repeat Area"]
    #[inline(always)]
    pub fn sara(&self) -> SARA_R {
        SARA_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 13 - Source Address Update Select After Reload"]
    #[inline(always)]
    pub fn sadr(&self) -> SADR_R {
        SADR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Source Address Update Mode"]
    #[inline(always)]
    pub fn sm(&self) -> SM_R {
        SM_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Destination Address Extended Repeat Area"]
    #[inline(always)]
    #[must_use]
    pub fn dara(&mut self) -> DARA_W<0> {
        DARA_W::new(self)
    }
    #[doc = "Bit 5 - Destination Address Update Select After Reload"]
    #[inline(always)]
    #[must_use]
    pub fn dadr(&mut self) -> DADR_W<5> {
        DADR_W::new(self)
    }
    #[doc = "Bits 6:7 - Destination Address Update Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dm(&mut self) -> DM_W<6> {
        DM_W::new(self)
    }
    #[doc = "Bits 8:12 - Source Address Extended Repeat Area"]
    #[inline(always)]
    #[must_use]
    pub fn sara(&mut self) -> SARA_W<8> {
        SARA_W::new(self)
    }
    #[doc = "Bit 13 - Source Address Update Select After Reload"]
    #[inline(always)]
    #[must_use]
    pub fn sadr(&mut self) -> SADR_W<13> {
        SADR_W::new(self)
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
