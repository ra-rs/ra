#[doc = "Register `GTINTAD` reader"]
pub struct R(crate::R<GTINTAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTINTAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTINTAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTINTAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTINTAD` writer"]
pub struct W(crate::W<GTINTAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTINTAD_SPEC>;
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
impl From<crate::W<GTINTAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTINTAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADTRAUEN` reader - GTADTRA Compare Match (Up-Counting) A/D Converter Start Request Interrupt Enable"]
pub type ADTRAUEN_R = crate::BitReader<ADTRAUEN_A>;
#[doc = "GTADTRA Compare Match (Up-Counting) A/D Converter Start Request Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADTRAUEN_A {
    #[doc = "0: Disable A/D converter start request"]
    _0 = 0,
    #[doc = "1: Enable A/D converter start request."]
    _1 = 1,
}
impl From<ADTRAUEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADTRAUEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADTRAUEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADTRAUEN_A {
        match self.bits {
            false => ADTRAUEN_A::_0,
            true => ADTRAUEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADTRAUEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADTRAUEN_A::_1
    }
}
#[doc = "Field `ADTRAUEN` writer - GTADTRA Compare Match (Up-Counting) A/D Converter Start Request Interrupt Enable"]
pub type ADTRAUEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTINTAD_SPEC, ADTRAUEN_A, O>;
impl<'a, const O: u8> ADTRAUEN_W<'a, O> {
    #[doc = "Disable A/D converter start request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADTRAUEN_A::_0)
    }
    #[doc = "Enable A/D converter start request."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADTRAUEN_A::_1)
    }
}
#[doc = "Field `ADTRADEN` reader - GTADTRA Compare Match (Down-Counting) A/D Converter Start Request Interrupt Enable"]
pub type ADTRADEN_R = crate::BitReader<ADTRADEN_A>;
#[doc = "GTADTRA Compare Match (Down-Counting) A/D Converter Start Request Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADTRADEN_A {
    #[doc = "0: Disable A/D converter start request"]
    _0 = 0,
    #[doc = "1: Enable A/D converter start request."]
    _1 = 1,
}
impl From<ADTRADEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADTRADEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADTRADEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADTRADEN_A {
        match self.bits {
            false => ADTRADEN_A::_0,
            true => ADTRADEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADTRADEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADTRADEN_A::_1
    }
}
#[doc = "Field `ADTRADEN` writer - GTADTRA Compare Match (Down-Counting) A/D Converter Start Request Interrupt Enable"]
pub type ADTRADEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTINTAD_SPEC, ADTRADEN_A, O>;
impl<'a, const O: u8> ADTRADEN_W<'a, O> {
    #[doc = "Disable A/D converter start request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADTRADEN_A::_0)
    }
    #[doc = "Enable A/D converter start request."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADTRADEN_A::_1)
    }
}
#[doc = "Field `ADTRBUEN` reader - GTADTRB Compare Match (Up-Counting) A/D Converter Start Request Interrupt Enable"]
pub type ADTRBUEN_R = crate::BitReader<ADTRBUEN_A>;
#[doc = "GTADTRB Compare Match (Up-Counting) A/D Converter Start Request Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADTRBUEN_A {
    #[doc = "0: Disable A/D converter start request"]
    _0 = 0,
    #[doc = "1: Enable A/D converter start request."]
    _1 = 1,
}
impl From<ADTRBUEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADTRBUEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADTRBUEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADTRBUEN_A {
        match self.bits {
            false => ADTRBUEN_A::_0,
            true => ADTRBUEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADTRBUEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADTRBUEN_A::_1
    }
}
#[doc = "Field `ADTRBUEN` writer - GTADTRB Compare Match (Up-Counting) A/D Converter Start Request Interrupt Enable"]
pub type ADTRBUEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTINTAD_SPEC, ADTRBUEN_A, O>;
impl<'a, const O: u8> ADTRBUEN_W<'a, O> {
    #[doc = "Disable A/D converter start request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADTRBUEN_A::_0)
    }
    #[doc = "Enable A/D converter start request."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADTRBUEN_A::_1)
    }
}
#[doc = "Field `ADTRBDEN` reader - GTADTRB Compare Match (Down-Counting) A/D Converter Start Request Interrupt Enable"]
pub type ADTRBDEN_R = crate::BitReader<ADTRBDEN_A>;
#[doc = "GTADTRB Compare Match (Down-Counting) A/D Converter Start Request Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADTRBDEN_A {
    #[doc = "0: Disable A/D converter start request"]
    _0 = 0,
    #[doc = "1: Enable A/D converter start request."]
    _1 = 1,
}
impl From<ADTRBDEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADTRBDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADTRBDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADTRBDEN_A {
        match self.bits {
            false => ADTRBDEN_A::_0,
            true => ADTRBDEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADTRBDEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADTRBDEN_A::_1
    }
}
#[doc = "Field `ADTRBDEN` writer - GTADTRB Compare Match (Down-Counting) A/D Converter Start Request Interrupt Enable"]
pub type ADTRBDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTINTAD_SPEC, ADTRBDEN_A, O>;
impl<'a, const O: u8> ADTRBDEN_W<'a, O> {
    #[doc = "Disable A/D converter start request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADTRBDEN_A::_0)
    }
    #[doc = "Enable A/D converter start request."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADTRBDEN_A::_1)
    }
}
#[doc = "Field `GRP` reader - Output Disable Source Select"]
pub type GRP_R = crate::FieldReader<u8, GRP_A>;
#[doc = "Output Disable Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GRP_A {
    #[doc = "0: Select Group A output disable request"]
    _00 = 0,
    #[doc = "1: Select Group B output disable request"]
    _01 = 1,
    #[doc = "2: Select Group C output disable request"]
    _10 = 2,
    #[doc = "3: Select Group D output disable request."]
    _11 = 3,
}
impl From<GRP_A> for u8 {
    #[inline(always)]
    fn from(variant: GRP_A) -> Self {
        variant as _
    }
}
impl GRP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GRP_A {
        match self.bits {
            0 => GRP_A::_00,
            1 => GRP_A::_01,
            2 => GRP_A::_10,
            3 => GRP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == GRP_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == GRP_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == GRP_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == GRP_A::_11
    }
}
#[doc = "Field `GRP` writer - Output Disable Source Select"]
pub type GRP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GTINTAD_SPEC, u8, GRP_A, 2, O>;
impl<'a, const O: u8> GRP_W<'a, O> {
    #[doc = "Select Group A output disable request"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(GRP_A::_00)
    }
    #[doc = "Select Group B output disable request"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(GRP_A::_01)
    }
    #[doc = "Select Group C output disable request"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(GRP_A::_10)
    }
    #[doc = "Select Group D output disable request."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(GRP_A::_11)
    }
}
#[doc = "Field `GRPDTE` reader - Dead Time Error Output Disable Request Enable"]
pub type GRPDTE_R = crate::BitReader<GRPDTE_A>;
#[doc = "Dead Time Error Output Disable Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GRPDTE_A {
    #[doc = "0: Disable dead time error output disable request"]
    _0 = 0,
    #[doc = "1: Enable dead time error output disable request"]
    _1 = 1,
}
impl From<GRPDTE_A> for bool {
    #[inline(always)]
    fn from(variant: GRPDTE_A) -> Self {
        variant as u8 != 0
    }
}
impl GRPDTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GRPDTE_A {
        match self.bits {
            false => GRPDTE_A::_0,
            true => GRPDTE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GRPDTE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GRPDTE_A::_1
    }
}
#[doc = "Field `GRPDTE` writer - Dead Time Error Output Disable Request Enable"]
pub type GRPDTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTINTAD_SPEC, GRPDTE_A, O>;
impl<'a, const O: u8> GRPDTE_W<'a, O> {
    #[doc = "Disable dead time error output disable request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GRPDTE_A::_0)
    }
    #[doc = "Enable dead time error output disable request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GRPDTE_A::_1)
    }
}
#[doc = "Field `GRPABH` reader - Same Time Output Level High Disable Request Enable"]
pub type GRPABH_R = crate::BitReader<GRPABH_A>;
#[doc = "Same Time Output Level High Disable Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GRPABH_A {
    #[doc = "0: Disable same time output level high disable request"]
    _0 = 0,
    #[doc = "1: Enable same time output level high disable request"]
    _1 = 1,
}
impl From<GRPABH_A> for bool {
    #[inline(always)]
    fn from(variant: GRPABH_A) -> Self {
        variant as u8 != 0
    }
}
impl GRPABH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GRPABH_A {
        match self.bits {
            false => GRPABH_A::_0,
            true => GRPABH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GRPABH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GRPABH_A::_1
    }
}
#[doc = "Field `GRPABH` writer - Same Time Output Level High Disable Request Enable"]
pub type GRPABH_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTINTAD_SPEC, GRPABH_A, O>;
impl<'a, const O: u8> GRPABH_W<'a, O> {
    #[doc = "Disable same time output level high disable request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GRPABH_A::_0)
    }
    #[doc = "Enable same time output level high disable request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GRPABH_A::_1)
    }
}
#[doc = "Field `GRPABL` reader - Same Time Output Level Low Disable Request Enable"]
pub type GRPABL_R = crate::BitReader<GRPABL_A>;
#[doc = "Same Time Output Level Low Disable Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GRPABL_A {
    #[doc = "0: Disable same time output level low disable request"]
    _0 = 0,
    #[doc = "1: Enable same time output level low disable request"]
    _1 = 1,
}
impl From<GRPABL_A> for bool {
    #[inline(always)]
    fn from(variant: GRPABL_A) -> Self {
        variant as u8 != 0
    }
}
impl GRPABL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GRPABL_A {
        match self.bits {
            false => GRPABL_A::_0,
            true => GRPABL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GRPABL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GRPABL_A::_1
    }
}
#[doc = "Field `GRPABL` writer - Same Time Output Level Low Disable Request Enable"]
pub type GRPABL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTINTAD_SPEC, GRPABL_A, O>;
impl<'a, const O: u8> GRPABL_W<'a, O> {
    #[doc = "Disable same time output level low disable request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GRPABL_A::_0)
    }
    #[doc = "Enable same time output level low disable request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GRPABL_A::_1)
    }
}
impl R {
    #[doc = "Bit 16 - GTADTRA Compare Match (Up-Counting) A/D Converter Start Request Interrupt Enable"]
    #[inline(always)]
    pub fn adtrauen(&self) -> ADTRAUEN_R {
        ADTRAUEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - GTADTRA Compare Match (Down-Counting) A/D Converter Start Request Interrupt Enable"]
    #[inline(always)]
    pub fn adtraden(&self) -> ADTRADEN_R {
        ADTRADEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GTADTRB Compare Match (Up-Counting) A/D Converter Start Request Interrupt Enable"]
    #[inline(always)]
    pub fn adtrbuen(&self) -> ADTRBUEN_R {
        ADTRBUEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GTADTRB Compare Match (Down-Counting) A/D Converter Start Request Interrupt Enable"]
    #[inline(always)]
    pub fn adtrbden(&self) -> ADTRBDEN_R {
        ADTRBDEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Output Disable Source Select"]
    #[inline(always)]
    pub fn grp(&self) -> GRP_R {
        GRP_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 28 - Dead Time Error Output Disable Request Enable"]
    #[inline(always)]
    pub fn grpdte(&self) -> GRPDTE_R {
        GRPDTE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Same Time Output Level High Disable Request Enable"]
    #[inline(always)]
    pub fn grpabh(&self) -> GRPABH_R {
        GRPABH_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Same Time Output Level Low Disable Request Enable"]
    #[inline(always)]
    pub fn grpabl(&self) -> GRPABL_R {
        GRPABL_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - GTADTRA Compare Match (Up-Counting) A/D Converter Start Request Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adtrauen(&mut self) -> ADTRAUEN_W<16> {
        ADTRAUEN_W::new(self)
    }
    #[doc = "Bit 17 - GTADTRA Compare Match (Down-Counting) A/D Converter Start Request Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adtraden(&mut self) -> ADTRADEN_W<17> {
        ADTRADEN_W::new(self)
    }
    #[doc = "Bit 18 - GTADTRB Compare Match (Up-Counting) A/D Converter Start Request Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adtrbuen(&mut self) -> ADTRBUEN_W<18> {
        ADTRBUEN_W::new(self)
    }
    #[doc = "Bit 19 - GTADTRB Compare Match (Down-Counting) A/D Converter Start Request Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adtrbden(&mut self) -> ADTRBDEN_W<19> {
        ADTRBDEN_W::new(self)
    }
    #[doc = "Bits 24:25 - Output Disable Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn grp(&mut self) -> GRP_W<24> {
        GRP_W::new(self)
    }
    #[doc = "Bit 28 - Dead Time Error Output Disable Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn grpdte(&mut self) -> GRPDTE_W<28> {
        GRPDTE_W::new(self)
    }
    #[doc = "Bit 29 - Same Time Output Level High Disable Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn grpabh(&mut self) -> GRPABH_W<29> {
        GRPABH_W::new(self)
    }
    #[doc = "Bit 30 - Same Time Output Level Low Disable Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn grpabl(&mut self) -> GRPABL_W<30> {
        GRPABL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Interrupt Output Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtintad](index.html) module"]
pub struct GTINTAD_SPEC;
impl crate::RegisterSpec for GTINTAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtintad::R](R) reader structure"]
impl crate::Readable for GTINTAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtintad::W](W) writer structure"]
impl crate::Writable for GTINTAD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTINTAD to value 0"]
impl crate::Resettable for GTINTAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
