#[doc = "Register `ADCSR` reader"]
pub struct R(crate::R<ADCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCSR` writer"]
pub struct W(crate::W<ADCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCSR_SPEC>;
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
impl From<crate::W<ADCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBLANS` reader - Double Trigger Channel Select"]
pub type DBLANS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBLANS` writer - Double Trigger Channel Select"]
pub type DBLANS_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ADCSR_SPEC, u8, u8, 5, O>;
#[doc = "Field `GBADIE` reader - Group B Scan End Interrupt and ELC Event Enable"]
pub type GBADIE_R = crate::BitReader<GBADIE_A>;
#[doc = "Group B Scan End Interrupt and ELC Event Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GBADIE_A {
    #[doc = "0: Disable ADC120_GBADI interrupt generation on group B scan completion."]
    _0 = 0,
    #[doc = "1: Enable ADC120_GBADI interrupt generation on group B scan completion."]
    _1 = 1,
}
impl From<GBADIE_A> for bool {
    #[inline(always)]
    fn from(variant: GBADIE_A) -> Self {
        variant as u8 != 0
    }
}
impl GBADIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GBADIE_A {
        match self.bits {
            false => GBADIE_A::_0,
            true => GBADIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GBADIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GBADIE_A::_1
    }
}
#[doc = "Field `GBADIE` writer - Group B Scan End Interrupt and ELC Event Enable"]
pub type GBADIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCSR_SPEC, GBADIE_A, O>;
impl<'a, const O: u8> GBADIE_W<'a, O> {
    #[doc = "Disable ADC120_GBADI interrupt generation on group B scan completion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GBADIE_A::_0)
    }
    #[doc = "Enable ADC120_GBADI interrupt generation on group B scan completion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GBADIE_A::_1)
    }
}
#[doc = "Field `DBLE` reader - Double Trigger Mode Select"]
pub type DBLE_R = crate::BitReader<DBLE_A>;
#[doc = "Double Trigger Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBLE_A {
    #[doc = "0: Deselect double-trigger mode."]
    _0 = 0,
    #[doc = "1: Select double-trigger mode."]
    _1 = 1,
}
impl From<DBLE_A> for bool {
    #[inline(always)]
    fn from(variant: DBLE_A) -> Self {
        variant as u8 != 0
    }
}
impl DBLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBLE_A {
        match self.bits {
            false => DBLE_A::_0,
            true => DBLE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DBLE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DBLE_A::_1
    }
}
#[doc = "Field `DBLE` writer - Double Trigger Mode Select"]
pub type DBLE_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCSR_SPEC, DBLE_A, O>;
impl<'a, const O: u8> DBLE_W<'a, O> {
    #[doc = "Deselect double-trigger mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBLE_A::_0)
    }
    #[doc = "Select double-trigger mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBLE_A::_1)
    }
}
#[doc = "Field `EXTRG` reader - Trigger Select"]
pub type EXTRG_R = crate::BitReader<EXTRG_A>;
#[doc = "Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTRG_A {
    #[doc = "0: Start A/D conversion by the synchronous trigger (ELC)."]
    _0 = 0,
    #[doc = "1: Start A/D conversion by the asynchronous trigger (ADTRG0)."]
    _1 = 1,
}
impl From<EXTRG_A> for bool {
    #[inline(always)]
    fn from(variant: EXTRG_A) -> Self {
        variant as u8 != 0
    }
}
impl EXTRG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTRG_A {
        match self.bits {
            false => EXTRG_A::_0,
            true => EXTRG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EXTRG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EXTRG_A::_1
    }
}
#[doc = "Field `EXTRG` writer - Trigger Select"]
pub type EXTRG_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCSR_SPEC, EXTRG_A, O>;
impl<'a, const O: u8> EXTRG_W<'a, O> {
    #[doc = "Start A/D conversion by the synchronous trigger (ELC)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EXTRG_A::_0)
    }
    #[doc = "Start A/D conversion by the asynchronous trigger (ADTRG0)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EXTRG_A::_1)
    }
}
#[doc = "Field `TRGE` reader - Trigger Start Enable"]
pub type TRGE_R = crate::BitReader<TRGE_A>;
#[doc = "Trigger Start Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGE_A {
    #[doc = "0: Disable A/D conversion to be started by the synchronous or asynchronous trigger"]
    _0 = 0,
    #[doc = "1: Enable A/D conversion to be started by the synchronous or asynchronous trigger"]
    _1 = 1,
}
impl From<TRGE_A> for bool {
    #[inline(always)]
    fn from(variant: TRGE_A) -> Self {
        variant as u8 != 0
    }
}
impl TRGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGE_A {
        match self.bits {
            false => TRGE_A::_0,
            true => TRGE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRGE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRGE_A::_1
    }
}
#[doc = "Field `TRGE` writer - Trigger Start Enable"]
pub type TRGE_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCSR_SPEC, TRGE_A, O>;
impl<'a, const O: u8> TRGE_W<'a, O> {
    #[doc = "Disable A/D conversion to be started by the synchronous or asynchronous trigger"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGE_A::_0)
    }
    #[doc = "Enable A/D conversion to be started by the synchronous or asynchronous trigger"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGE_A::_1)
    }
}
#[doc = "Field `ADHSC` reader - A/D Conversion Mode Select"]
pub type ADHSC_R = crate::BitReader<ADHSC_A>;
#[doc = "A/D Conversion Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADHSC_A {
    #[doc = "0: High-speed A/D conversion mode"]
    _0 = 0,
    #[doc = "1: Low-power A/D conversion mode"]
    _1 = 1,
}
impl From<ADHSC_A> for bool {
    #[inline(always)]
    fn from(variant: ADHSC_A) -> Self {
        variant as u8 != 0
    }
}
impl ADHSC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADHSC_A {
        match self.bits {
            false => ADHSC_A::_0,
            true => ADHSC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADHSC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADHSC_A::_1
    }
}
#[doc = "Field `ADHSC` writer - A/D Conversion Mode Select"]
pub type ADHSC_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCSR_SPEC, ADHSC_A, O>;
impl<'a, const O: u8> ADHSC_W<'a, O> {
    #[doc = "High-speed A/D conversion mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADHSC_A::_0)
    }
    #[doc = "Low-power A/D conversion mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADHSC_A::_1)
    }
}
#[doc = "Field `ADCS` reader - Scan Mode Select"]
pub type ADCS_R = crate::FieldReader<u8, ADCS_A>;
#[doc = "Scan Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCS_A {
    #[doc = "0: Single scan mode"]
    _00 = 0,
    #[doc = "1: Group scan mode"]
    _01 = 1,
    #[doc = "2: Continuous scan mode"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<ADCS_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCS_A) -> Self {
        variant as _
    }
}
impl ADCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCS_A {
        match self.bits {
            0 => ADCS_A::_00,
            1 => ADCS_A::_01,
            2 => ADCS_A::_10,
            3 => ADCS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == ADCS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == ADCS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == ADCS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == ADCS_A::_11
    }
}
#[doc = "Field `ADCS` writer - Scan Mode Select"]
pub type ADCS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, ADCSR_SPEC, u8, ADCS_A, 2, O>;
impl<'a, const O: u8> ADCS_W<'a, O> {
    #[doc = "Single scan mode"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(ADCS_A::_00)
    }
    #[doc = "Group scan mode"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(ADCS_A::_01)
    }
    #[doc = "Continuous scan mode"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(ADCS_A::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(ADCS_A::_11)
    }
}
#[doc = "Field `ADST` reader - A/D Conversion Start"]
pub type ADST_R = crate::BitReader<ADST_A>;
#[doc = "A/D Conversion Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADST_A {
    #[doc = "0: Stop A/D conversion process."]
    _0 = 0,
    #[doc = "1: Start A/D conversion process."]
    _1 = 1,
}
impl From<ADST_A> for bool {
    #[inline(always)]
    fn from(variant: ADST_A) -> Self {
        variant as u8 != 0
    }
}
impl ADST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADST_A {
        match self.bits {
            false => ADST_A::_0,
            true => ADST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADST_A::_1
    }
}
#[doc = "Field `ADST` writer - A/D Conversion Start"]
pub type ADST_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCSR_SPEC, ADST_A, O>;
impl<'a, const O: u8> ADST_W<'a, O> {
    #[doc = "Stop A/D conversion process."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADST_A::_0)
    }
    #[doc = "Start A/D conversion process."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADST_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:4 - Double Trigger Channel Select"]
    #[inline(always)]
    pub fn dblans(&self) -> DBLANS_R {
        DBLANS_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Group B Scan End Interrupt and ELC Event Enable"]
    #[inline(always)]
    pub fn gbadie(&self) -> GBADIE_R {
        GBADIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Double Trigger Mode Select"]
    #[inline(always)]
    pub fn dble(&self) -> DBLE_R {
        DBLE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Trigger Select"]
    #[inline(always)]
    pub fn extrg(&self) -> EXTRG_R {
        EXTRG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Trigger Start Enable"]
    #[inline(always)]
    pub fn trge(&self) -> TRGE_R {
        TRGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - A/D Conversion Mode Select"]
    #[inline(always)]
    pub fn adhsc(&self) -> ADHSC_R {
        ADHSC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Scan Mode Select"]
    #[inline(always)]
    pub fn adcs(&self) -> ADCS_R {
        ADCS_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - A/D Conversion Start"]
    #[inline(always)]
    pub fn adst(&self) -> ADST_R {
        ADST_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Double Trigger Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn dblans(&mut self) -> DBLANS_W<0> {
        DBLANS_W::new(self)
    }
    #[doc = "Bit 6 - Group B Scan End Interrupt and ELC Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gbadie(&mut self) -> GBADIE_W<6> {
        GBADIE_W::new(self)
    }
    #[doc = "Bit 7 - Double Trigger Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn dble(&mut self) -> DBLE_W<7> {
        DBLE_W::new(self)
    }
    #[doc = "Bit 8 - Trigger Select"]
    #[inline(always)]
    #[must_use]
    pub fn extrg(&mut self) -> EXTRG_W<8> {
        EXTRG_W::new(self)
    }
    #[doc = "Bit 9 - Trigger Start Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trge(&mut self) -> TRGE_W<9> {
        TRGE_W::new(self)
    }
    #[doc = "Bit 10 - A/D Conversion Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn adhsc(&mut self) -> ADHSC_W<10> {
        ADHSC_W::new(self)
    }
    #[doc = "Bits 13:14 - Scan Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn adcs(&mut self) -> ADCS_W<13> {
        ADCS_W::new(self)
    }
    #[doc = "Bit 15 - A/D Conversion Start"]
    #[inline(always)]
    #[must_use]
    pub fn adst(&mut self) -> ADST_W<15> {
        ADST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcsr](index.html) module"]
pub struct ADCSR_SPEC;
impl crate::RegisterSpec for ADCSR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adcsr::R](R) reader structure"]
impl crate::Readable for ADCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcsr::W](W) writer structure"]
impl crate::Writable for ADCSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCSR to value 0"]
impl crate::Resettable for ADCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
