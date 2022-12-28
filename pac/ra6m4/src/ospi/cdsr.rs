#[doc = "Register `CDSR` reader"]
pub struct R(crate::R<CDSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CDSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CDSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CDSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CDSR` writer"]
pub struct W(crate::W<CDSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CDSR_SPEC>;
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
impl From<crate::W<CDSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CDSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DV0TTYP` reader - Device0_transfer_type setting"]
pub type DV0TTYP_R = crate::FieldReader<u8, DV0TTYP_A>;
#[doc = "Device0_transfer_type setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DV0TTYP_A {
    #[doc = "0: SPI mode"]
    _00 = 0,
    #[doc = "1: SOPI mode"]
    _01 = 1,
    #[doc = "2: DOPI mode"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<DV0TTYP_A> for u8 {
    #[inline(always)]
    fn from(variant: DV0TTYP_A) -> Self {
        variant as _
    }
}
impl DV0TTYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DV0TTYP_A {
        match self.bits {
            0 => DV0TTYP_A::_00,
            1 => DV0TTYP_A::_01,
            2 => DV0TTYP_A::_10,
            3 => DV0TTYP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DV0TTYP_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DV0TTYP_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DV0TTYP_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DV0TTYP_A::_11
    }
}
#[doc = "Field `DV0TTYP` writer - Device0_transfer_type setting"]
pub type DV0TTYP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CDSR_SPEC, u8, DV0TTYP_A, 2, O>;
impl<'a, const O: u8> DV0TTYP_W<'a, O> {
    #[doc = "SPI mode"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(DV0TTYP_A::_00)
    }
    #[doc = "SOPI mode"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(DV0TTYP_A::_01)
    }
    #[doc = "DOPI mode"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DV0TTYP_A::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(DV0TTYP_A::_11)
    }
}
#[doc = "Field `DV1TTYP` reader - Device1_transfer_type setting"]
pub type DV1TTYP_R = crate::FieldReader<u8, DV1TTYP_A>;
#[doc = "Device1_transfer_type setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DV1TTYP_A {
    #[doc = "0: SPI mode"]
    _00 = 0,
    #[doc = "1: SOPI mode"]
    _01 = 1,
    #[doc = "2: DOPI mode"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<DV1TTYP_A> for u8 {
    #[inline(always)]
    fn from(variant: DV1TTYP_A) -> Self {
        variant as _
    }
}
impl DV1TTYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DV1TTYP_A {
        match self.bits {
            0 => DV1TTYP_A::_00,
            1 => DV1TTYP_A::_01,
            2 => DV1TTYP_A::_10,
            3 => DV1TTYP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DV1TTYP_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DV1TTYP_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DV1TTYP_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DV1TTYP_A::_11
    }
}
#[doc = "Field `DV1TTYP` writer - Device1_transfer_type setting"]
pub type DV1TTYP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CDSR_SPEC, u8, DV1TTYP_A, 2, O>;
impl<'a, const O: u8> DV1TTYP_W<'a, O> {
    #[doc = "SPI mode"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(DV1TTYP_A::_00)
    }
    #[doc = "SOPI mode"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(DV1TTYP_A::_01)
    }
    #[doc = "DOPI mode"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DV1TTYP_A::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(DV1TTYP_A::_11)
    }
}
#[doc = "Field `DV0PC` reader - Device0_memory precycle setting"]
pub type DV0PC_R = crate::BitReader<DV0PC_A>;
#[doc = "Device0_memory precycle setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DV0PC_A {
    #[doc = "0: Disable"]
    _0 = 0,
    #[doc = "1: Enable"]
    _1 = 1,
}
impl From<DV0PC_A> for bool {
    #[inline(always)]
    fn from(variant: DV0PC_A) -> Self {
        variant as u8 != 0
    }
}
impl DV0PC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DV0PC_A {
        match self.bits {
            false => DV0PC_A::_0,
            true => DV0PC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DV0PC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DV0PC_A::_1
    }
}
#[doc = "Field `DV0PC` writer - Device0_memory precycle setting"]
pub type DV0PC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CDSR_SPEC, DV0PC_A, O>;
impl<'a, const O: u8> DV0PC_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DV0PC_A::_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DV0PC_A::_1)
    }
}
#[doc = "Field `DV1PC` reader - Device1_memory precycle setting"]
pub type DV1PC_R = crate::BitReader<DV1PC_A>;
#[doc = "Device1_memory precycle setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DV1PC_A {
    #[doc = "0: Disable"]
    _0 = 0,
    #[doc = "1: Enable"]
    _1 = 1,
}
impl From<DV1PC_A> for bool {
    #[inline(always)]
    fn from(variant: DV1PC_A) -> Self {
        variant as u8 != 0
    }
}
impl DV1PC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DV1PC_A {
        match self.bits {
            false => DV1PC_A::_0,
            true => DV1PC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DV1PC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DV1PC_A::_1
    }
}
#[doc = "Field `DV1PC` writer - Device1_memory precycle setting"]
pub type DV1PC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CDSR_SPEC, DV1PC_A, O>;
impl<'a, const O: u8> DV1PC_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DV1PC_A::_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DV1PC_A::_1)
    }
}
#[doc = "Field `ACMEME0` reader - Automatic calibration memory enable setting for device 0"]
pub type ACMEME0_R = crate::BitReader<ACMEME0_A>;
#[doc = "Automatic calibration memory enable setting for device 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACMEME0_A {
    #[doc = "0: Disable"]
    _0 = 0,
    #[doc = "1: Enable"]
    _1 = 1,
}
impl From<ACMEME0_A> for bool {
    #[inline(always)]
    fn from(variant: ACMEME0_A) -> Self {
        variant as u8 != 0
    }
}
impl ACMEME0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMEME0_A {
        match self.bits {
            false => ACMEME0_A::_0,
            true => ACMEME0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACMEME0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACMEME0_A::_1
    }
}
#[doc = "Field `ACMEME0` writer - Automatic calibration memory enable setting for device 0"]
pub type ACMEME0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CDSR_SPEC, ACMEME0_A, O>;
impl<'a, const O: u8> ACMEME0_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACMEME0_A::_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACMEME0_A::_1)
    }
}
#[doc = "Field `ACMEME1` reader - Automatic calibration memory enable setting for device 1"]
pub type ACMEME1_R = crate::BitReader<ACMEME1_A>;
#[doc = "Automatic calibration memory enable setting for device 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACMEME1_A {
    #[doc = "0: Disable"]
    _0 = 0,
    #[doc = "1: Enable"]
    _1 = 1,
}
impl From<ACMEME1_A> for bool {
    #[inline(always)]
    fn from(variant: ACMEME1_A) -> Self {
        variant as u8 != 0
    }
}
impl ACMEME1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMEME1_A {
        match self.bits {
            false => ACMEME1_A::_0,
            true => ACMEME1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACMEME1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACMEME1_A::_1
    }
}
#[doc = "Field `ACMEME1` writer - Automatic calibration memory enable setting for device 1"]
pub type ACMEME1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CDSR_SPEC, ACMEME1_A, O>;
impl<'a, const O: u8> ACMEME1_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACMEME1_A::_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACMEME1_A::_1)
    }
}
#[doc = "Field `ACMODE` reader - Automatic calibration mode"]
pub type ACMODE_R = crate::FieldReader<u8, ACMODE_A>;
#[doc = "Automatic calibration mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACMODE_A {
    #[doc = "0: Automatic calibration is disabled"]
    _00 = 0,
    #[doc = "1: Automatic calibration is enabled and modify MDTR"]
    _01 = 1,
    #[doc = "2: Automatic calibration immediately is executed for all trim code, but it will not modify MDTR"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<ACMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: ACMODE_A) -> Self {
        variant as _
    }
}
impl ACMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMODE_A {
        match self.bits {
            0 => ACMODE_A::_00,
            1 => ACMODE_A::_01,
            2 => ACMODE_A::_10,
            3 => ACMODE_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == ACMODE_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == ACMODE_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == ACMODE_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == ACMODE_A::_11
    }
}
#[doc = "Field `ACMODE` writer - Automatic calibration mode"]
pub type ACMODE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CDSR_SPEC, u8, ACMODE_A, 2, O>;
impl<'a, const O: u8> ACMODE_W<'a, O> {
    #[doc = "Automatic calibration is disabled"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(ACMODE_A::_00)
    }
    #[doc = "Automatic calibration is enabled and modify MDTR"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(ACMODE_A::_01)
    }
    #[doc = "Automatic calibration immediately is executed for all trim code, but it will not modify MDTR"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(ACMODE_A::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(ACMODE_A::_11)
    }
}
#[doc = "Field `DLFT` reader - Deadlock Free Timer Enable"]
pub type DLFT_R = crate::BitReader<DLFT_A>;
#[doc = "Deadlock Free Timer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLFT_A {
    #[doc = "0: Enable timer"]
    _0 = 0,
    #[doc = "1: Disable timer"]
    _1 = 1,
}
impl From<DLFT_A> for bool {
    #[inline(always)]
    fn from(variant: DLFT_A) -> Self {
        variant as u8 != 0
    }
}
impl DLFT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLFT_A {
        match self.bits {
            false => DLFT_A::_0,
            true => DLFT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLFT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLFT_A::_1
    }
}
#[doc = "Field `DLFT` writer - Deadlock Free Timer Enable"]
pub type DLFT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CDSR_SPEC, DLFT_A, O>;
impl<'a, const O: u8> DLFT_W<'a, O> {
    #[doc = "Enable timer"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DLFT_A::_0)
    }
    #[doc = "Disable timer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DLFT_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Device0_transfer_type setting"]
    #[inline(always)]
    pub fn dv0ttyp(&self) -> DV0TTYP_R {
        DV0TTYP_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Device1_transfer_type setting"]
    #[inline(always)]
    pub fn dv1ttyp(&self) -> DV1TTYP_R {
        DV1TTYP_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Device0_memory precycle setting"]
    #[inline(always)]
    pub fn dv0pc(&self) -> DV0PC_R {
        DV0PC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Device1_memory precycle setting"]
    #[inline(always)]
    pub fn dv1pc(&self) -> DV1PC_R {
        DV1PC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 10 - Automatic calibration memory enable setting for device 0"]
    #[inline(always)]
    pub fn acmeme0(&self) -> ACMEME0_R {
        ACMEME0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Automatic calibration memory enable setting for device 1"]
    #[inline(always)]
    pub fn acmeme1(&self) -> ACMEME1_R {
        ACMEME1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Automatic calibration mode"]
    #[inline(always)]
    pub fn acmode(&self) -> ACMODE_R {
        ACMODE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 31 - Deadlock Free Timer Enable"]
    #[inline(always)]
    pub fn dlft(&self) -> DLFT_R {
        DLFT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Device0_transfer_type setting"]
    #[inline(always)]
    #[must_use]
    pub fn dv0ttyp(&mut self) -> DV0TTYP_W<0> {
        DV0TTYP_W::new(self)
    }
    #[doc = "Bits 2:3 - Device1_transfer_type setting"]
    #[inline(always)]
    #[must_use]
    pub fn dv1ttyp(&mut self) -> DV1TTYP_W<2> {
        DV1TTYP_W::new(self)
    }
    #[doc = "Bit 4 - Device0_memory precycle setting"]
    #[inline(always)]
    #[must_use]
    pub fn dv0pc(&mut self) -> DV0PC_W<4> {
        DV0PC_W::new(self)
    }
    #[doc = "Bit 5 - Device1_memory precycle setting"]
    #[inline(always)]
    #[must_use]
    pub fn dv1pc(&mut self) -> DV1PC_W<5> {
        DV1PC_W::new(self)
    }
    #[doc = "Bit 10 - Automatic calibration memory enable setting for device 0"]
    #[inline(always)]
    #[must_use]
    pub fn acmeme0(&mut self) -> ACMEME0_W<10> {
        ACMEME0_W::new(self)
    }
    #[doc = "Bit 11 - Automatic calibration memory enable setting for device 1"]
    #[inline(always)]
    #[must_use]
    pub fn acmeme1(&mut self) -> ACMEME1_W<11> {
        ACMEME1_W::new(self)
    }
    #[doc = "Bits 12:13 - Automatic calibration mode"]
    #[inline(always)]
    #[must_use]
    pub fn acmode(&mut self) -> ACMODE_W<12> {
        ACMODE_W::new(self)
    }
    #[doc = "Bit 31 - Deadlock Free Timer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dlft(&mut self) -> DLFT_W<31> {
        DLFT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controller and Device Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdsr](index.html) module"]
pub struct CDSR_SPEC;
impl crate::RegisterSpec for CDSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cdsr::R](R) reader structure"]
impl crate::Readable for CDSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cdsr::W](W) writer structure"]
impl crate::Writable for CDSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CDSR to value 0"]
impl crate::Resettable for CDSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
