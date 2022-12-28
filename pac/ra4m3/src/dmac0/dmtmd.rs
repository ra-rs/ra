#[doc = "Register `DMTMD` reader"]
pub struct R(crate::R<DMTMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMTMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMTMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMTMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMTMD` writer"]
pub struct W(crate::W<DMTMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMTMD_SPEC>;
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
impl From<crate::W<DMTMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMTMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCTG` reader - Transfer Request Source Select"]
pub type DCTG_R = crate::FieldReader<u8, DCTG_A>;
#[doc = "Transfer Request Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DCTG_A {
    #[doc = "0: Software request"]
    _00 = 0,
    #[doc = "1: Hardware request"]
    _01 = 1,
    #[doc = "2: Setting prohibited"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<DCTG_A> for u8 {
    #[inline(always)]
    fn from(variant: DCTG_A) -> Self {
        variant as _
    }
}
impl DCTG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCTG_A {
        match self.bits {
            0 => DCTG_A::_00,
            1 => DCTG_A::_01,
            2 => DCTG_A::_10,
            3 => DCTG_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DCTG_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DCTG_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DCTG_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DCTG_A::_11
    }
}
#[doc = "Field `DCTG` writer - Transfer Request Source Select"]
pub type DCTG_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, DMTMD_SPEC, u8, DCTG_A, 2, O>;
impl<'a, const O: u8> DCTG_W<'a, O> {
    #[doc = "Software request"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(DCTG_A::_00)
    }
    #[doc = "Hardware request"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(DCTG_A::_01)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DCTG_A::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(DCTG_A::_11)
    }
}
#[doc = "Field `SZ` reader - Transfer Data Size Select"]
pub type SZ_R = crate::FieldReader<u8, SZ_A>;
#[doc = "Transfer Data Size Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SZ_A {
    #[doc = "0: 8 bits"]
    _00 = 0,
    #[doc = "1: 16 bits"]
    _01 = 1,
    #[doc = "2: 32 bits"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<SZ_A> for u8 {
    #[inline(always)]
    fn from(variant: SZ_A) -> Self {
        variant as _
    }
}
impl SZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SZ_A {
        match self.bits {
            0 => SZ_A::_00,
            1 => SZ_A::_01,
            2 => SZ_A::_10,
            3 => SZ_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SZ_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SZ_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SZ_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SZ_A::_11
    }
}
#[doc = "Field `SZ` writer - Transfer Data Size Select"]
pub type SZ_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, DMTMD_SPEC, u8, SZ_A, 2, O>;
impl<'a, const O: u8> SZ_W<'a, O> {
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(SZ_A::_00)
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(SZ_A::_01)
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SZ_A::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(SZ_A::_11)
    }
}
#[doc = "Field `TKP` reader - Transfer Keeping"]
pub type TKP_R = crate::BitReader<TKP_A>;
#[doc = "Transfer Keeping\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TKP_A {
    #[doc = "0: Transfer is stopped by completion of specified total number of transfer operations."]
    _0 = 0,
    #[doc = "1: Transfer is not stopped by completion of specified total number of transfer operations. (free-running)"]
    _1 = 1,
}
impl From<TKP_A> for bool {
    #[inline(always)]
    fn from(variant: TKP_A) -> Self {
        variant as u8 != 0
    }
}
impl TKP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TKP_A {
        match self.bits {
            false => TKP_A::_0,
            true => TKP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TKP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TKP_A::_1
    }
}
#[doc = "Field `TKP` writer - Transfer Keeping"]
pub type TKP_W<'a, const O: u8> = crate::BitWriter<'a, u16, DMTMD_SPEC, TKP_A, O>;
impl<'a, const O: u8> TKP_W<'a, O> {
    #[doc = "Transfer is stopped by completion of specified total number of transfer operations."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TKP_A::_0)
    }
    #[doc = "Transfer is not stopped by completion of specified total number of transfer operations. (free-running)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TKP_A::_1)
    }
}
#[doc = "Field `DTS` reader - Repeat Area Select"]
pub type DTS_R = crate::FieldReader<u8, DTS_A>;
#[doc = "Repeat Area Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DTS_A {
    #[doc = "0: The destination is specified as the repeat area or block area"]
    _00 = 0,
    #[doc = "1: The source is specified as the repeat area or block area"]
    _01 = 1,
    #[doc = "2: The repeat area or block area is not specified"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<DTS_A> for u8 {
    #[inline(always)]
    fn from(variant: DTS_A) -> Self {
        variant as _
    }
}
impl DTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTS_A {
        match self.bits {
            0 => DTS_A::_00,
            1 => DTS_A::_01,
            2 => DTS_A::_10,
            3 => DTS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DTS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DTS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DTS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DTS_A::_11
    }
}
#[doc = "Field `DTS` writer - Repeat Area Select"]
pub type DTS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, DMTMD_SPEC, u8, DTS_A, 2, O>;
impl<'a, const O: u8> DTS_W<'a, O> {
    #[doc = "The destination is specified as the repeat area or block area"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(DTS_A::_00)
    }
    #[doc = "The source is specified as the repeat area or block area"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(DTS_A::_01)
    }
    #[doc = "The repeat area or block area is not specified"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DTS_A::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(DTS_A::_11)
    }
}
#[doc = "Field `MD` reader - Transfer Mode Select"]
pub type MD_R = crate::FieldReader<u8, MD_A>;
#[doc = "Transfer Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MD_A {
    #[doc = "0: Normal transfer"]
    _00 = 0,
    #[doc = "1: Repeat transfer"]
    _01 = 1,
    #[doc = "2: Block transfer"]
    _10 = 2,
    #[doc = "3: Repeat-block transfer"]
    _11 = 3,
}
impl From<MD_A> for u8 {
    #[inline(always)]
    fn from(variant: MD_A) -> Self {
        variant as _
    }
}
impl MD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MD_A {
        match self.bits {
            0 => MD_A::_00,
            1 => MD_A::_01,
            2 => MD_A::_10,
            3 => MD_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == MD_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == MD_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == MD_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == MD_A::_11
    }
}
#[doc = "Field `MD` writer - Transfer Mode Select"]
pub type MD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, DMTMD_SPEC, u8, MD_A, 2, O>;
impl<'a, const O: u8> MD_W<'a, O> {
    #[doc = "Normal transfer"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(MD_A::_00)
    }
    #[doc = "Repeat transfer"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(MD_A::_01)
    }
    #[doc = "Block transfer"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(MD_A::_10)
    }
    #[doc = "Repeat-block transfer"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(MD_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - Transfer Request Source Select"]
    #[inline(always)]
    pub fn dctg(&self) -> DCTG_R {
        DCTG_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Transfer Data Size Select"]
    #[inline(always)]
    pub fn sz(&self) -> SZ_R {
        SZ_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Transfer Keeping"]
    #[inline(always)]
    pub fn tkp(&self) -> TKP_R {
        TKP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Repeat Area Select"]
    #[inline(always)]
    pub fn dts(&self) -> DTS_R {
        DTS_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Transfer Mode Select"]
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Transfer Request Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn dctg(&mut self) -> DCTG_W<0> {
        DCTG_W::new(self)
    }
    #[doc = "Bits 8:9 - Transfer Data Size Select"]
    #[inline(always)]
    #[must_use]
    pub fn sz(&mut self) -> SZ_W<8> {
        SZ_W::new(self)
    }
    #[doc = "Bit 10 - Transfer Keeping"]
    #[inline(always)]
    #[must_use]
    pub fn tkp(&mut self) -> TKP_W<10> {
        TKP_W::new(self)
    }
    #[doc = "Bits 12:13 - Repeat Area Select"]
    #[inline(always)]
    #[must_use]
    pub fn dts(&mut self) -> DTS_W<12> {
        DTS_W::new(self)
    }
    #[doc = "Bits 14:15 - Transfer Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn md(&mut self) -> MD_W<14> {
        MD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Transfer Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmtmd](index.html) module"]
pub struct DMTMD_SPEC;
impl crate::RegisterSpec for DMTMD_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dmtmd::R](R) reader structure"]
impl crate::Readable for DMTMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmtmd::W](W) writer structure"]
impl crate::Writable for DMTMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMTMD to value 0"]
impl crate::Resettable for DMTMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
